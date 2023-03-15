use crate::Client;
use crate::ClientResult;

pub struct Workspaces {
    pub client: Client,
}

impl Workspaces {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Workspaces { client }
    }

    /**
     * List Workspaces.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/workspaces` endpoint.
     *
     * Gets information about the Workspaces that have been created.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(&self, account_id: &str) -> ClientResult<crate::types::WorkspaceList> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/workspaces",
                crate::progenitor_support::encode_path(account_id),
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
     * Create a Workspace.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/workspaces` endpoint.
     *
     * This method creates a new workspace.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn post(
        &self,
        account_id: &str,
        body: &crate::types::Workspace,
    ) -> ClientResult<crate::types::Workspace> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/workspaces",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
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
     * Get Workspace.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/workspaces/{workspaceId}` endpoint.
     *
     * Retrives properties about a workspace given a unique workspaceId.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `workspace_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get_workspaces(
        &self,
        account_id: &str,
        workspace_id: &str,
    ) -> ClientResult<crate::types::Workspace> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/workspaces/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(workspace_id),
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
     * Update Workspace.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/workspaces/{workspaceId}` endpoint.
     *
     * Updates information about a specific workspace.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `workspace_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        workspace_id: &str,
        body: &crate::types::Workspace,
    ) -> ClientResult<crate::types::Workspace> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/workspaces/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(workspace_id),
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
     * Delete Workspace.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/workspaces/{workspaceId}` endpoint.
     *
     * Deletes an existing workspace (logically).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `workspace_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete(
        &self,
        account_id: &str,
        workspace_id: &str,
    ) -> ClientResult<crate::types::Workspace> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/workspaces/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(workspace_id),
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
}
