use anyhow::Result;

use crate::Client;

pub struct WorkspaceItems {
    pub client: Client,
}

impl WorkspaceItems {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        WorkspaceItems { client }
    }

    /**
     * List workspace folder contents.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/workspaces/{workspaceId}/folders/{folderId}` endpoint.
     *
     * This method returns the contents of a workspace folder, which can include sub-folders and files.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `folder_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `workspace_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_files: &str` -- When set to **true**, the response includes file information (in addition to folder information). The default is **false**.
     * * `include_sub_folders: &str` -- When set to **true**, the response includes information about the sub-folders of the current folder. The default is **false**.
     * * `include_thumbnails: &str` -- When set to **true**, the response returns thumbnails.  The default is **false**.
     * * `include_user_detail: &str` -- When set to **true**, the response includes extended details about the user. The default is **false**.
     * * `start_position: &str` -- The position within the total result set from which to start returning values.
     * * `workspace_user_id: &str` -- If set, the response only includes results associated with the `userId` that you specify.
     */
    pub async fn workspace_folder_get(
        &self,
        account_id: &str,
        folder_id: &str,
        workspace_id: &str,
        count: &str,
        include_files: &str,
        include_sub_folders: &str,
        include_thumbnails: &str,
        include_user_detail: &str,
        start_position: &str,
        workspace_user_id: &str,
    ) -> Result<crate::types::WorkspaceFolderContents> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !count.is_empty() {
            query_args.push(format!("count={}", count));
        }
        if !include_files.is_empty() {
            query_args.push(format!("include_files={}", include_files));
        }
        if !include_sub_folders.is_empty() {
            query_args.push(format!("include_sub_folders={}", include_sub_folders));
        }
        if !include_thumbnails.is_empty() {
            query_args.push(format!("include_thumbnails={}", include_thumbnails));
        }
        if !include_user_detail.is_empty() {
            query_args.push(format!("include_user_detail={}", include_user_detail));
        }
        if !start_position.is_empty() {
            query_args.push(format!("start_position={}", start_position));
        }
        if !workspace_user_id.is_empty() {
            query_args.push(format!("workspace_user_id={}", workspace_user_id));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/workspaces/{}/folders/{}?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&workspace_id.to_string()),
            crate::progenitor_support::encode_path(&folder_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Deletes files or sub-folders from a workspace.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/workspaces/{workspaceId}/folders/{folderId}` endpoint.
     *
     * This method deletes one or more files or sub-folders from a workspace folder or root.
     *
     * Note: To delete items from a workspace, the `status` of the workspace must be `active`.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `folder_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `workspace_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn workspace_folder_delete_items(
        &self,
        account_id: &str,
        folder_id: &str,
        workspace_id: &str,
        body: &crate::types::WorkspaceItemList,
    ) -> Result<()> {
        let url = format!(
            "/v2.1/accounts/{}/workspaces/{}/folders/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&workspace_id.to_string()),
            crate::progenitor_support::encode_path(&folder_id.to_string()),
        );

        self.client
            .delete(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Creates a workspace file.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/workspaces/{workspaceId}/folders/{folderId}/files` endpoint.
     *
     * This method adds a file to a workspace.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `folder_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `workspace_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn workspace_file_post_files(
        &self,
        account_id: &str,
        folder_id: &str,
        workspace_id: &str,
    ) -> Result<crate::types::WorkspaceItem> {
        let url = format!(
            "/v2.1/accounts/{}/workspaces/{}/folders/{}/files",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&workspace_id.to_string()),
            crate::progenitor_support::encode_path(&folder_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
     * Gets a workspace file.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/workspaces/{workspaceId}/folders/{folderId}/files/{fileId}` endpoint.
     *
     * This method returns a binary version of a file in a workspace.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `file_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `folder_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `workspace_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `is_download: &str` -- When set to **true**, the `Content-Disposition` header is set in the response. The value of the header provides the filename of the file. The default is **false**.
     * * `pdf_version: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn workspace_file_get(
        &self,
        account_id: &str,
        file_id: &str,
        folder_id: &str,
        workspace_id: &str,
        is_download: &str,
        pdf_version: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !is_download.is_empty() {
            query_args.push(format!("is_download={}", is_download));
        }
        if !pdf_version.is_empty() {
            query_args.push(format!("pdf_version={}", pdf_version));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/workspaces/{}/folders/{}/files/{}?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&workspace_id.to_string()),
            crate::progenitor_support::encode_path(&folder_id.to_string()),
            crate::progenitor_support::encode_path(&file_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update workspace file or folder metadata.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/workspaces/{workspaceId}/folders/{folderId}/files/{fileId}` endpoint.
     *
     * This method updates the metadata for one or more specific files or folders in a workspace.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `file_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `folder_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `workspace_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn workspace_file_put(
        &self,
        account_id: &str,
        file_id: &str,
        folder_id: &str,
        workspace_id: &str,
    ) -> Result<crate::types::WorkspaceItem> {
        let url = format!(
            "/v2.1/accounts/{}/workspaces/{}/folders/{}/files/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&workspace_id.to_string()),
            crate::progenitor_support::encode_path(&folder_id.to_string()),
            crate::progenitor_support::encode_path(&file_id.to_string()),
        );

        self.client.put(&url, None).await
    }

    /**
     * List File Pages.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/workspaces/{workspaceId}/folders/{folderId}/files/{fileId}/pages` endpoint.
     *
     * This method returns a workspace file as rasterized pages.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `file_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `folder_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `workspace_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `dpi: &str` -- The number of dots per inch (DPI) for the resulting images. Valid values are 1-310 DPI. The default value is 94.
     * * `max_height: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `max_width: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- The position within the total result set from which to start returning values. The value **thumbnail** may be used to return the page image.
     */
    pub async fn workspace_file_pages_get(
        &self,
        account_id: &str,
        file_id: &str,
        folder_id: &str,
        workspace_id: &str,
        count: &str,
        dpi: &str,
        max_height: &str,
        max_width: &str,
        start_position: &str,
    ) -> Result<crate::types::PageImages> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !count.is_empty() {
            query_args.push(format!("count={}", count));
        }
        if !dpi.is_empty() {
            query_args.push(format!("dpi={}", dpi));
        }
        if !max_height.is_empty() {
            query_args.push(format!("max_height={}", max_height));
        }
        if !max_width.is_empty() {
            query_args.push(format!("max_width={}", max_width));
        }
        if !start_position.is_empty() {
            query_args.push(format!("start_position={}", start_position));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/workspaces/{}/folders/{}/files/{}/pages?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&workspace_id.to_string()),
            crate::progenitor_support::encode_path(&folder_id.to_string()),
            crate::progenitor_support::encode_path(&file_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
