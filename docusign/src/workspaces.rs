use anyhow::Result;

use crate::Client;

pub struct Workspaces {
    client: Client,
}

impl Workspaces {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Workspaces {
            client,
        }
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
pub async fn workspace_get(
&self,
account_id: &str,
) -> Result<crate::types::WorkspaceList> {
let url =
format!("/v2.1/accounts/{}/workspaces",
crate::progenitor_support::encode_path(&account_id.to_string()),);

self.client.get(&url, None).await
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
pub async fn workspace_post(
&self,
account_id: &str,
body: &crate::types::Workspace
) -> Result<crate::types::Workspace> {
let url =
format!("/v2.1/accounts/{}/workspaces",
crate::progenitor_support::encode_path(&account_id.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
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
pub async fn workspace_get_workspaces(
&self,
account_id: &str, workspace_id: &str,
) -> Result<crate::types::Workspace> {
let url =
format!("/v2.1/accounts/{}/workspaces/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&workspace_id.to_string()),);

self.client.get(&url, None).await
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
pub async fn workspace_put(
&self,
account_id: &str, workspace_id: &str,
body: &crate::types::Workspace
) -> Result<crate::types::Workspace> {
let url =
format!("/v2.1/accounts/{}/workspaces/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&workspace_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
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
pub async fn workspace_delete(
&self,
account_id: &str, workspace_id: &str,
) -> Result<crate::types::Workspace> {
let url =
format!("/v2.1/accounts/{}/workspaces/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&workspace_id.to_string()),);

self.client.delete(&url, None).await
}


}