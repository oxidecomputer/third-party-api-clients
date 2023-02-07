use anyhow::Result;

use crate::Client;

pub struct FileManager {
    pub client: Client,
}

impl FileManager {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        FileManager { client }
    }

    /**
    * List stored files.
    *
    * This function performs a `GET` to the `/file-manager/files` endpoint.
    *
    * Get a list of available images and files stored in the File Manager for the account.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `type_: &str` -- The file type for the File Manager file.
    * * `created_by: &str` -- The Mailchimp account user who created the File Manager file.
    * * `before_created_at: &str` -- Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_created_at: &str` -- Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `sort_field: crate::types::GetFileManagerFilesSortField` -- Returns files sorted by the specified field.
    * * `sort_dir: crate::types::SortDir` -- Determines the order direction for sorted results.
    */
    pub async fn get_file(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        type_: &str,
        created_by: &str,
        before_created_at: &str,
        since_created_at: &str,
        sort_field: crate::types::GetFileManagerFilesSortField,
        sort_dir: crate::types::SortDir,
    ) -> Result<crate::types::FileManager> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !before_created_at.is_empty() {
            query_args.push((
                "before_created_at".to_string(),
                before_created_at.to_string(),
            ));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !created_by.is_empty() {
            query_args.push(("created_by".to_string(), created_by.to_string()));
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
        if !since_created_at.is_empty() {
            query_args.push(("since_created_at".to_string(), since_created_at.to_string()));
        }
        if !sort_dir.to_string().is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/file-manager/files?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Add file.
    *
    * This function performs a `POST` to the `/file-manager/files` endpoint.
    *
    * Upload a new image or file to the File Manager.
    */
    pub async fn post(&self, body: &crate::types::GalleryFile) -> Result<crate::types::Files> {
        let url = "/file-manager/files".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get file.
    *
    * This function performs a `GET` to the `/file-manager/files/{file_id}` endpoint.
    *
    * Get information about a specific file in the File Manager.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `file_id: &str` -- The unique id for the File Manager file.
    */
    pub async fn get_file_file_manager(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        file_id: &str,
    ) -> Result<crate::types::Files> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/file-manager/files/{}?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete file.
    *
    * This function performs a `DELETE` to the `/file-manager/files/{file_id}` endpoint.
    *
    * Remove a specific file from the File Manager.
    *
    * **Parameters:**
    *
    * * `file_id: &str` -- The unique id for the File Manager file.
    */
    pub async fn delete_files(&self, file_id: &str) -> Result<()> {
        let url = format!(
            "/file-manager/files/{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update file.
    *
    * This function performs a `PATCH` to the `/file-manager/files/{file_id}` endpoint.
    *
    * Update a file in the File Manager.
    *
    * **Parameters:**
    *
    * * `file_id: &str` -- The unique id for the File Manager file.
    */
    pub async fn patch_files(
        &self,
        file_id: &str,
        body: &crate::types::GalleryFileData,
    ) -> Result<crate::types::Files> {
        let url = format!(
            "/file-manager/files/{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List folders.
    *
    * This function performs a `GET` to the `/file-manager/folders` endpoint.
    *
    * Get a list of all folders in the File Manager.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `created_by: &str` -- The Mailchimp account user who created the File Manager file.
    * * `before_created_at: &str` -- Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_created_at: &str` -- Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    */
    pub async fn get_folder(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        created_by: &str,
        before_created_at: &str,
        since_created_at: &str,
    ) -> Result<crate::types::FileManagerFolders> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !before_created_at.is_empty() {
            query_args.push((
                "before_created_at".to_string(),
                before_created_at.to_string(),
            ));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !created_by.is_empty() {
            query_args.push(("created_by".to_string(), created_by.to_string()));
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
        if !since_created_at.is_empty() {
            query_args.push(("since_created_at".to_string(), since_created_at.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/file-manager/folders?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Add folder.
    *
    * This function performs a `POST` to the `/file-manager/folders` endpoint.
    *
    * Create a new folder in the File Manager.
    */
    pub async fn post_folder(
        &self,
        body: &crate::types::GalleryFolder,
    ) -> Result<crate::types::FileManagerFoldersGalleryFolder> {
        let url = "/file-manager/folders".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get folder.
    *
    * This function performs a `GET` to the `/file-manager/folders/{folder_id}` endpoint.
    *
    * Get information about a specific folder in the File Manager.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `folder_id: &str` -- The unique id for the File Manager folder.
    */
    pub async fn get_folder_file_manager(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        folder_id: &str,
    ) -> Result<crate::types::FileManagerFoldersGalleryFolder> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/file-manager/folders/{}?{}",
            crate::progenitor_support::encode_path(&folder_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete folder.
    *
    * This function performs a `DELETE` to the `/file-manager/folders/{folder_id}` endpoint.
    *
    * Delete a specific folder in the File Manager.
    *
    * **Parameters:**
    *
    * * `folder_id: &str` -- The unique id for the File Manager folder.
    */
    pub async fn delete_folders(&self, folder_id: &str) -> Result<()> {
        let url = format!(
            "/file-manager/folders/{}",
            crate::progenitor_support::encode_path(&folder_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update folder.
    *
    * This function performs a `PATCH` to the `/file-manager/folders/{folder_id}` endpoint.
    *
    * Update a specific File Manager folder.
    *
    * **Parameters:**
    *
    * * `folder_id: &str` -- The unique id for the File Manager folder.
    */
    pub async fn patch_folders(
        &self,
        folder_id: &str,
        body: &crate::types::GalleryFolder,
    ) -> Result<crate::types::FileManagerFoldersGalleryFolder> {
        let url = format!(
            "/file-manager/folders/{}",
            crate::progenitor_support::encode_path(&folder_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
