use crate::Client;
use crate::ClientResult;

pub struct EventHooks {
    pub client: Client,
}

impl EventHooks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EventHooks { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/eventHooks` endpoint.
     *
     * Success
     */
    pub async fn list(&self) -> ClientResult<Vec<crate::types::EventHook>> {
        let url = self.client.url("/api/v1/eventHooks", None);
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
     * This function performs a `GET` to the `/api/v1/eventHooks` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all(&self) -> ClientResult<Vec<crate::types::EventHook>> {
        let url = self.client.url("/api/v1/eventHooks", None);
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
     * This function performs a `POST` to the `/api/v1/eventHooks` endpoint.
     *
     * Success
     */
    pub async fn create(
        &self,
        body: &crate::types::EventHook,
    ) -> ClientResult<crate::types::EventHook> {
        let url = self.client.url("/api/v1/eventHooks", None);
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
     * This function performs a `GET` to the `/api/v1/eventHooks/{eventHookId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `event_hook_id: &str`
     */
    pub async fn get(&self, event_hook_id: &str) -> ClientResult<crate::types::EventHook> {
        let url = self.client.url(
            &format!(
                "/api/v1/eventHooks/{}",
                crate::progenitor_support::encode_path(event_hook_id),
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
     * This function performs a `PUT` to the `/api/v1/eventHooks/{eventHookId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `event_hook_id: &str`
     */
    pub async fn update(
        &self,
        event_hook_id: &str,
        body: &crate::types::EventHook,
    ) -> ClientResult<crate::types::EventHook> {
        let url = self.client.url(
            &format!(
                "/api/v1/eventHooks/{}",
                crate::progenitor_support::encode_path(event_hook_id),
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
     * This function performs a `DELETE` to the `/api/v1/eventHooks/{eventHookId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `event_hook_id: &str`
     */
    pub async fn delete(&self, event_hook_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/eventHooks/{}",
                crate::progenitor_support::encode_path(event_hook_id),
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
     * This function performs a `POST` to the `/api/v1/eventHooks/{eventHookId}/lifecycle/activate` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `event_hook_id: &str`
     */
    pub async fn activate(&self, event_hook_id: &str) -> ClientResult<crate::types::EventHook> {
        let url = self.client.url(
            &format!(
                "/api/v1/eventHooks/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(event_hook_id),
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
     * This function performs a `POST` to the `/api/v1/eventHooks/{eventHookId}/lifecycle/deactivate` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `event_hook_id: &str`
     */
    pub async fn deactivate(&self, event_hook_id: &str) -> ClientResult<crate::types::EventHook> {
        let url = self.client.url(
            &format!(
                "/api/v1/eventHooks/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(event_hook_id),
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
     * This function performs a `POST` to the `/api/v1/eventHooks/{eventHookId}/lifecycle/verify` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `event_hook_id: &str`
     */
    pub async fn verify(&self, event_hook_id: &str) -> ClientResult<crate::types::EventHook> {
        let url = self.client.url(
            &format!(
                "/api/v1/eventHooks/{}/lifecycle/verify",
                crate::progenitor_support::encode_path(event_hook_id),
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
