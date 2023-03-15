use crate::Client;
use crate::ClientResult;

pub struct Folders {
    pub client: Client,
}

impl Folders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Folders { client }
    }

    /**
     * This function performs a `GET` to the `/v2/folders` endpoint.
     *
     * Lists the Folders that are direct descendants of supplied parent resource. List provides a strongly consistent view of the Folders underneath the specified parent resource. List returns Folders sorted based upon the (ascending) lexical ordering of their display_name. The caller must have `resourcemanager.folders.list` permission on the identified parent.
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
     * * `page_token: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     * * `parent: &str` -- Required. The resource name of the Organization or Folder whose Folders are being listed. Must be of the form `folders/{folder_id}` or `organizations/{org_id}`. Access to this method is controlled by checking the `resourcemanager.folders.list` permission on the `parent`.
     * * `show_deleted: bool` -- True if the project can be retrieved using `GetProject`. No other operations on the project are guaranteed to work until the project creation is complete.
     */
    pub async fn list(
        &self,
        page_size: i64,
        page_token: &str,
        parent: &str,
        show_deleted: bool,
    ) -> ClientResult<Vec<crate::types::Folder>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !parent.is_empty() {
            query_args.push(("parent".to_string(), parent.to_string()));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v2/folders?{}", query_), None);
        let resp: crate::types::ListFoldersResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.folders.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v2/folders` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists the Folders that are direct descendants of supplied parent resource. List provides a strongly consistent view of the Folders underneath the specified parent resource. List returns Folders sorted based upon the (ascending) lexical ordering of their display_name. The caller must have `resourcemanager.folders.list` permission on the identified parent.
     */
    pub async fn list_all(
        &self,
        parent: &str,
        show_deleted: bool,
    ) -> ClientResult<Vec<crate::types::Folder>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !parent.is_empty() {
            query_args.push(("parent".to_string(), parent.to_string()));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v2/folders?{}", query_), None);
        let mut resp: crate::types::ListFoldersResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut folders = resp.folders;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                resp = self
                    .client
                    .get(
                        &format!("{}&pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            folders.append(&mut resp.folders);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(folders)
    }
    /**
     * This function performs a `POST` to the `/v2/folders` endpoint.
     *
     * Creates a Folder in the resource hierarchy. Returns an Operation which can be used to track the progress of the folder creation workflow. Upon success the Operation.response field will be populated with the created Folder. In order to succeed, the addition of this new Folder must not violate the Folder naming, height or fanout constraints. + The Folder's display_name must be distinct from all other Folders that share its parent. + The addition of the Folder must not cause the active Folder hierarchy to exceed a height of 10. Note, the full active + deleted Folder hierarchy is allowed to reach a height of 20; this provides additional headroom when moving folders that contain deleted folders. + The addition of the Folder must not cause the total number of Folders under its parent to exceed 300. If the operation fails due to a folder constraint violation, some errors may be returned by the CreateFolder request, with status code FAILED_PRECONDITION and an error description. Other folder constraint violations will be communicated in the Operation, with the specific PreconditionFailure returned via the details list in the Operation.error field. The caller must have `resourcemanager.folders.create` permission on the identified parent.
     *
     * **Parameters:**
     *
     * * `parent: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn create(
        &self,
        parent: &str,
        body: &crate::types::Folder,
    ) -> ClientResult<crate::types::Operation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !parent.is_empty() {
            query_args.push(("parent".to_string(), parent.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v2/folders?{}", query_), None);
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
     * This function performs a `POST` to the `/v2/folders:search` endpoint.
     *
     * Search for folders that match specific filter criteria. Search provides an eventually consistent view of the folders a user has access to which meet the specified filter criteria. This will only return folders on which the caller has the permission `resourcemanager.folders.get`.
     */
    pub async fn search(
        &self,
        body: &crate::types::SearchFoldersRequest,
    ) -> ClientResult<Vec<crate::types::Folder>> {
        let url = self.client.url("/v2/folders:search", None);
        let resp: crate::types::SearchFoldersResponse = self
            .client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.folders.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v2/{name}` endpoint.
     *
     * Retrieves a Folder identified by the supplied resource name. Valid Folder resource names have the format `folders/{folder_id}` (for example, `folders/1234`). The caller must have `resourcemanager.folders.get` permission on the identified folder.
     *
     * **Parameters:**
     *
     * * `name: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn get(&self, name: &str) -> ClientResult<crate::types::Folder> {
        let url = self.client.url(
            &format!("/v2/{}", crate::progenitor_support::encode_path(name),),
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
     * This function performs a `DELETE` to the `/v2/{name}` endpoint.
     *
     * Requests deletion of a Folder. The Folder is moved into the DELETE_REQUESTED state immediately, and is deleted approximately 30 days later. This method may only be called on an empty Folder in the ACTIVE state, where a Folder is empty if it doesn't contain any Folders or Projects in the ACTIVE state. The caller must have `resourcemanager.folders.delete` permission on the identified folder.
     *
     * **Parameters:**
     *
     * * `name: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn delete(&self, name: &str) -> ClientResult<crate::types::Folder> {
        let url = self.client.url(
            &format!("/v2/{}", crate::progenitor_support::encode_path(name),),
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
     * This function performs a `PATCH` to the `/v2/{name}` endpoint.
     *
     * Updates a Folder, changing its display_name. Changes to the folder display_name will be rejected if they violate either the display_name formatting rules or naming constraints described in the CreateFolder documentation. The Folder's display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be between 3 and 30 characters. This is captured by the regular expression: `\p{L}\p{N}{1,28}[\p{L}\p{N}]`. The caller must have `resourcemanager.folders.update` permission on the identified folder. If the update fails due to the unique name constraint then a PreconditionFailure explaining this violation will be returned in the Status.details field.
     *
     * **Parameters:**
     *
     * * `name: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     * * `update_mask: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn patch(
        &self,
        name: &str,
        update_mask: &str,
        body: &crate::types::Folder,
    ) -> ClientResult<crate::types::Folder> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !update_mask.is_empty() {
            query_args.push(("updateMask".to_string(), update_mask.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2/{}?{}",
                crate::progenitor_support::encode_path(name),
                query_
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v2/{name}:move` endpoint.
     *
     * Moves a Folder under a new resource parent. Returns an Operation which can be used to track the progress of the folder move workflow. Upon success the Operation.response field will be populated with the moved Folder. Upon failure, a FolderOperationError categorizing the failure cause will be returned - if the failure occurs synchronously then the FolderOperationError will be returned via the Status.details field and if it occurs asynchronously then the FolderOperation will be returned via the Operation.error field. In addition, the Operation.metadata field will be populated with a FolderOperation message as an aid to stateless clients. Folder moves will be rejected if they violate either the naming, height or fanout constraints described in the CreateFolder documentation. The caller must have `resourcemanager.folders.move` permission on the folder's current and proposed new parent.
     *
     * **Parameters:**
     *
     * * `name: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn mv(
        &self,
        name: &str,
        body: &crate::types::MoveFolderRequest,
    ) -> ClientResult<crate::types::Operation> {
        let url = self.client.url(
            &format!("/v2/{}/move", crate::progenitor_support::encode_path(name),),
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
     * This function performs a `POST` to the `/v2/{name}:undelete` endpoint.
     *
     * Cancels the deletion request for a Folder. This method may only be called on a Folder in the DELETE_REQUESTED state. In order to succeed, the Folder's parent must be in the ACTIVE state. In addition, reintroducing the folder into the tree must not violate folder naming, height and fanout constraints described in the CreateFolder documentation. The caller must have `resourcemanager.folders.undelete` permission on the identified folder.
     *
     * **Parameters:**
     *
     * * `name: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn undelete(
        &self,
        name: &str,
        body: &crate::types::MoveProjectMetadata,
    ) -> ClientResult<crate::types::Folder> {
        let url = self.client.url(
            &format!(
                "/v2/{}/undelete",
                crate::progenitor_support::encode_path(name),
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
     * This function performs a `POST` to the `/v2/{resource}:getIamPolicy` endpoint.
     *
     * Gets the access control policy for a Folder. The returned policy may be empty if no such policy or resource exists. The `resource` field should be the Folder's resource name, e.g. "folders/1234". The caller must have `resourcemanager.folders.getIamPolicy` permission on the identified folder.
     *
     * **Parameters:**
     *
     * * `resource: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn get_iam_policy(
        &self,
        resource: &str,
        body: &crate::types::GetIamPolicyRequest,
    ) -> ClientResult<crate::types::Policy> {
        let url = self.client.url(
            &format!(
                "/v2/{}/getIamPolicy",
                crate::progenitor_support::encode_path(resource),
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
     * This function performs a `POST` to the `/v2/{resource}:setIamPolicy` endpoint.
     *
     * Sets the access control policy on a Folder, replacing any existing policy. The `resource` field should be the Folder's resource name, e.g. "folders/1234". The caller must have `resourcemanager.folders.setIamPolicy` permission on the identified folder.
     *
     * **Parameters:**
     *
     * * `resource: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn set_iam_policy(
        &self,
        resource: &str,
        body: &crate::types::SetIamPolicyRequest,
    ) -> ClientResult<crate::types::Policy> {
        let url = self.client.url(
            &format!(
                "/v2/{}/setIamPolicy",
                crate::progenitor_support::encode_path(resource),
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
     * This function performs a `POST` to the `/v2/{resource}:testIamPermissions` endpoint.
     *
     * Returns permissions that a caller has on the specified Folder. The `resource` field should be the Folder's resource name, e.g. "folders/1234". There are no permissions required for making this API call.
     *
     * **Parameters:**
     *
     * * `resource: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn test_iam_permissions(
        &self,
        resource: &str,
        body: &crate::types::TestIamPermissionsRequest,
    ) -> ClientResult<crate::types::TestIamPermissionsResponse> {
        let url = self.client.url(
            &format!(
                "/v2/{}/testIamPermissions",
                crate::progenitor_support::encode_path(resource),
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
}
