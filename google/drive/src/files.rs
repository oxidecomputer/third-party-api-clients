use anyhow::Result;

use crate::Client;

pub struct Files {
    pub client: Client,
}

impl Files {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Files { client }
    }

    /**
     * This function performs a `GET` to the `/files` endpoint.
     *
     * Lists or searches files.
     *
     * **Parameters:**
     *
     * * `corpora: &str` -- Groupings of files to which the query applies. Supported groupings are: 'user' (files created by, opened by, or shared directly with the user), 'drive' (files in the specified shared drive as indicated by the 'driveId'), 'domain' (files shared to the user's domain), and 'allDrives' (A combination of 'user' and 'drive' for all drives where the user is a member). When able, use 'user' or 'drive', instead of 'allDrives', for efficiency.
     * * `corpus: crate::types::Corpus` -- The source of files to list. Deprecated: use 'corpora' instead.
     * * `drive_id: &str` -- A link to this theme's background image.
     * * `include_items_from_all_drives: bool` -- Whether both My Drive and shared drive items should be included in results.
     * * `include_permissions_for_view: &str` -- Specifies which additional view's permissions to include in the response. Only 'published' is supported.
     * * `include_team_drive_items: bool` -- Whether the user has installed the requesting app.
     * * `order_by: &str` -- A comma-separated list of sort keys. Valid keys are 'createdTime', 'folder', 'modifiedByMeTime', 'modifiedTime', 'name', 'name_natural', 'quotaBytesUsed', 'recency', 'sharedWithMeTime', 'starred', and 'viewedByMeTime'. Each key sorts ascending by default, but may be reversed with the 'desc' modifier. Example usage: ?orderBy=folder,modifiedTime desc,name. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored.
     * * `page_size: i64` -- The maximum number of files to return per page. Partial or empty result pages are possible even before the end of the files list has been reached.
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
     * * `q: &str` -- A query for filtering the file results. See the "Search for Files" guide for supported syntax.
     * * `spaces: &str` -- A comma-separated list of spaces to query within the corpus. Supported values are 'drive' and 'appDataFolder'.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `team_drive_id: &str` -- A link to this theme's background image.
     */
    pub async fn list(
        &self,
        corpora: &str,
        drive_id: &str,
        include_items_from_all_drives: bool,
        include_permissions_for_view: &str,
        include_team_drive_items: bool,
        order_by: &str,
        page_size: i64,
        page_token: &str,
        q: &str,
        spaces: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        team_drive_id: &str,
    ) -> Result<Vec<crate::types::File>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !corpora.is_empty() {
            query_args.push(("corpora".to_string(), corpora.to_string()));
        }
        if !drive_id.is_empty() {
            query_args.push(("driveId".to_string(), drive_id.to_string()));
        }
        if include_items_from_all_drives {
            query_args.push((
                "includeItemsFromAllDrives".to_string(),
                include_items_from_all_drives.to_string(),
            ));
        }
        if !include_permissions_for_view.is_empty() {
            query_args.push((
                "includePermissionsForView".to_string(),
                include_permissions_for_view.to_string(),
            ));
        }
        if include_team_drive_items {
            query_args.push((
                "includeTeamDriveItems".to_string(),
                include_team_drive_items.to_string(),
            ));
        }
        if !order_by.is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !spaces.is_empty() {
            query_args.push(("spaces".to_string(), spaces.to_string()));
        }
        if supports_all_drives {
            query_args.push((
                "supportsAllDrives".to_string(),
                supports_all_drives.to_string(),
            ));
        }
        if supports_team_drives {
            query_args.push((
                "supportsTeamDrives".to_string(),
                supports_team_drives.to_string(),
            ));
        }
        if !team_drive_id.is_empty() {
            query_args.push(("teamDriveId".to_string(), team_drive_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/files?{}", query_);

        let resp: crate::types::FileList = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.files)
    }

    /**
     * This function performs a `GET` to the `/files` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists or searches files.
     */
    pub async fn list_all(
        &self,
        corpora: &str,
        drive_id: &str,
        include_items_from_all_drives: bool,
        include_permissions_for_view: &str,
        include_team_drive_items: bool,
        order_by: &str,
        q: &str,
        spaces: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        team_drive_id: &str,
    ) -> Result<Vec<crate::types::File>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !corpora.is_empty() {
            query_args.push(("corpora".to_string(), corpora.to_string()));
        }
        if !drive_id.is_empty() {
            query_args.push(("driveId".to_string(), drive_id.to_string()));
        }
        if include_items_from_all_drives {
            query_args.push((
                "includeItemsFromAllDrives".to_string(),
                include_items_from_all_drives.to_string(),
            ));
        }
        if !include_permissions_for_view.is_empty() {
            query_args.push((
                "includePermissionsForView".to_string(),
                include_permissions_for_view.to_string(),
            ));
        }
        if include_team_drive_items {
            query_args.push((
                "includeTeamDriveItems".to_string(),
                include_team_drive_items.to_string(),
            ));
        }
        if !order_by.is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !spaces.is_empty() {
            query_args.push(("spaces".to_string(), spaces.to_string()));
        }
        if supports_all_drives {
            query_args.push((
                "supportsAllDrives".to_string(),
                supports_all_drives.to_string(),
            ));
        }
        if supports_team_drives {
            query_args.push((
                "supportsTeamDrives".to_string(),
                supports_team_drives.to_string(),
            ));
        }
        if !team_drive_id.is_empty() {
            query_args.push(("teamDriveId".to_string(), team_drive_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/files?{}", query_);

        let mut resp: crate::types::FileList = self.client.get(&url, None).await?;

        let mut files = resp.files;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?pageToken={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&pageToken={}", url, page), None)
                    .await?;
            }

            files.append(&mut resp.files);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(files)
    }

    /**
     * This function performs a `POST` to the `/files` endpoint.
     *
     * Creates a new file.
     *
     * **Parameters:**
     *
     * * `enforce_single_parent: bool` -- Deprecated. Creating files in multiple folders is no longer supported.
     * * `ignore_default_visibility: bool` -- Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.
     * * `include_permissions_for_view: &str` -- Specifies which additional view's permissions to include in the response. Only 'published' is supported.
     * * `keep_revision_forever: bool` -- Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.
     * * `ocr_language: &str` -- A language hint for OCR processing during image import (ISO 639-1 code).
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `use_content_as_indexable_text: bool` -- Whether to use the uploaded content as indexable text.
     */
    pub async fn create(
        &self,
        ignore_default_visibility: bool,
        include_permissions_for_view: &str,
        keep_revision_forever: bool,
        ocr_language: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        use_content_as_indexable_text: bool,
        body: &crate::types::File,
    ) -> Result<crate::types::File> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if ignore_default_visibility {
            query_args.push((
                "ignoreDefaultVisibility".to_string(),
                ignore_default_visibility.to_string(),
            ));
        }
        if !include_permissions_for_view.is_empty() {
            query_args.push((
                "includePermissionsForView".to_string(),
                include_permissions_for_view.to_string(),
            ));
        }
        if keep_revision_forever {
            query_args.push((
                "keepRevisionForever".to_string(),
                keep_revision_forever.to_string(),
            ));
        }
        if !ocr_language.is_empty() {
            query_args.push(("ocrLanguage".to_string(), ocr_language.to_string()));
        }
        if supports_all_drives {
            query_args.push((
                "supportsAllDrives".to_string(),
                supports_all_drives.to_string(),
            ));
        }
        if supports_team_drives {
            query_args.push((
                "supportsTeamDrives".to_string(),
                supports_team_drives.to_string(),
            ));
        }
        if use_content_as_indexable_text {
            query_args.push((
                "useContentAsIndexableText".to_string(),
                use_content_as_indexable_text.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/files?{}", query_);

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/files/generateIds` endpoint.
     *
     * Generates a set of file IDs which can be provided in create or copy requests.
     *
     * **Parameters:**
     *
     * * `count: i64` -- A map of maximum import sizes by MIME type, in bytes.
     * * `space: &str` -- The space in which the IDs can be used to create new files. Supported values are 'drive' and 'appDataFolder'. (Default: 'drive').
     * * `type_: &str` -- The type of items which the IDs can be used for. Supported values are 'files' and 'shortcuts'. Note that 'shortcuts' are only supported in the drive 'space'. (Default: 'files').
     */
    pub async fn generate_id(
        &self,
        count: i64,
        space: &str,
        type_: &str,
    ) -> Result<crate::types::GeneratedIds> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !space.is_empty() {
            query_args.push(("space".to_string(), space.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/files/generateIds?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/files/trash` endpoint.
     *
     * Permanently deletes all of the user's trashed files.
     *
     * **Parameters:**
     *
     * * `enforce_single_parent: bool` -- Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root.
     */
    pub async fn empty_trash(&self) -> Result<()> {
        let url = "/files/trash".to_string();
        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}` endpoint.
     *
     * Gets a file's metadata or content by ID.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `acknowledge_abuse: bool` -- Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
     * * `include_permissions_for_view: &str` -- Specifies which additional view's permissions to include in the response. Only 'published' is supported.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     */
    pub async fn get(
        &self,
        file_id: &str,
        acknowledge_abuse: bool,
        include_permissions_for_view: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
    ) -> Result<crate::types::File> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if acknowledge_abuse {
            query_args.push((
                "acknowledgeAbuse".to_string(),
                acknowledge_abuse.to_string(),
            ));
        }
        if !include_permissions_for_view.is_empty() {
            query_args.push((
                "includePermissionsForView".to_string(),
                include_permissions_for_view.to_string(),
            ));
        }
        if supports_all_drives {
            query_args.push((
                "supportsAllDrives".to_string(),
                supports_all_drives.to_string(),
            ));
        }
        if supports_team_drives {
            query_args.push((
                "supportsTeamDrives".to_string(),
                supports_team_drives.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/files/{fileId}` endpoint.
     *
     * Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `enforce_single_parent: bool` -- Deprecated. If an item is not in a shared drive and its last parent is deleted but the item itself is not, the item will be placed under its owner's root.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     */
    pub async fn delete(
        &self,
        file_id: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if supports_all_drives {
            query_args.push((
                "supportsAllDrives".to_string(),
                supports_all_drives.to_string(),
            ));
        }
        if supports_team_drives {
            query_args.push((
                "supportsTeamDrives".to_string(),
                supports_team_drives.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/files/{fileId}` endpoint.
     *
     * Updates a file's metadata and/or content. This method supports patch semantics.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `add_parents: &str` -- A comma-separated list of parent IDs to add.
     * * `enforce_single_parent: bool` -- Deprecated. Adding files to multiple folders is no longer supported. Use shortcuts instead.
     * * `include_permissions_for_view: &str` -- Specifies which additional view's permissions to include in the response. Only 'published' is supported.
     * * `keep_revision_forever: bool` -- Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.
     * * `ocr_language: &str` -- A language hint for OCR processing during image import (ISO 639-1 code).
     * * `remove_parents: &str` -- A comma-separated list of parent IDs to remove.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `use_content_as_indexable_text: bool` -- Whether to use the uploaded content as indexable text.
     */
    pub async fn update(
        &self,
        file_id: &str,
        add_parents: &str,
        include_permissions_for_view: &str,
        keep_revision_forever: bool,
        ocr_language: &str,
        remove_parents: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        use_content_as_indexable_text: bool,
        body: &crate::types::File,
    ) -> Result<crate::types::File> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !add_parents.is_empty() {
            query_args.push(("addParents".to_string(), add_parents.to_string()));
        }
        if !include_permissions_for_view.is_empty() {
            query_args.push((
                "includePermissionsForView".to_string(),
                include_permissions_for_view.to_string(),
            ));
        }
        if keep_revision_forever {
            query_args.push((
                "keepRevisionForever".to_string(),
                keep_revision_forever.to_string(),
            ));
        }
        if !ocr_language.is_empty() {
            query_args.push(("ocrLanguage".to_string(), ocr_language.to_string()));
        }
        if !remove_parents.is_empty() {
            query_args.push(("removeParents".to_string(), remove_parents.to_string()));
        }
        if supports_all_drives {
            query_args.push((
                "supportsAllDrives".to_string(),
                supports_all_drives.to_string(),
            ));
        }
        if supports_team_drives {
            query_args.push((
                "supportsTeamDrives".to_string(),
                supports_team_drives.to_string(),
            ));
        }
        if use_content_as_indexable_text {
            query_args.push((
                "useContentAsIndexableText".to_string(),
                use_content_as_indexable_text.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `POST` to the `/files/{fileId}/copy` endpoint.
     *
     * Creates a copy of a file and applies any requested updates with patch semantics. Folders cannot be copied.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `enforce_single_parent: bool` -- Deprecated. Copying files into multiple folders is no longer supported. Use shortcuts instead.
     * * `ignore_default_visibility: bool` -- Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.
     * * `include_permissions_for_view: &str` -- Specifies which additional view's permissions to include in the response. Only 'published' is supported.
     * * `keep_revision_forever: bool` -- Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.
     * * `ocr_language: &str` -- A language hint for OCR processing during image import (ISO 639-1 code).
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     */
    pub async fn copy(
        &self,
        file_id: &str,
        ignore_default_visibility: bool,
        include_permissions_for_view: &str,
        keep_revision_forever: bool,
        ocr_language: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        body: &crate::types::File,
    ) -> Result<crate::types::File> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if ignore_default_visibility {
            query_args.push((
                "ignoreDefaultVisibility".to_string(),
                ignore_default_visibility.to_string(),
            ));
        }
        if !include_permissions_for_view.is_empty() {
            query_args.push((
                "includePermissionsForView".to_string(),
                include_permissions_for_view.to_string(),
            ));
        }
        if keep_revision_forever {
            query_args.push((
                "keepRevisionForever".to_string(),
                keep_revision_forever.to_string(),
            ));
        }
        if !ocr_language.is_empty() {
            query_args.push(("ocrLanguage".to_string(), ocr_language.to_string()));
        }
        if supports_all_drives {
            query_args.push((
                "supportsAllDrives".to_string(),
                supports_all_drives.to_string(),
            ));
        }
        if supports_team_drives {
            query_args.push((
                "supportsTeamDrives".to_string(),
                supports_team_drives.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/copy?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/export` endpoint.
     *
     * Exports a Google Doc to the requested MIME type and returns the exported content. Please note that the exported content is limited to 10MB.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `mime_type: &str` -- The MIME type of the format requested for this export.
     */
    pub async fn export(&self, file_id: &str, mime_type: &str) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !mime_type.is_empty() {
            query_args.push(("mimeType".to_string(), mime_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/export?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/files/{fileId}/watch` endpoint.
     *
     * Subscribes to changes to a file
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `acknowledge_abuse: bool` -- Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
     * * `include_permissions_for_view: &str` -- Specifies which additional view's permissions to include in the response. Only 'published' is supported.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     */
    pub async fn watch(
        &self,
        file_id: &str,
        acknowledge_abuse: bool,
        include_permissions_for_view: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        body: &crate::types::Channel,
    ) -> Result<crate::types::Channel> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if acknowledge_abuse {
            query_args.push((
                "acknowledgeAbuse".to_string(),
                acknowledge_abuse.to_string(),
            ));
        }
        if !include_permissions_for_view.is_empty() {
            query_args.push((
                "includePermissionsForView".to_string(),
                include_permissions_for_view.to_string(),
            ));
        }
        if supports_all_drives {
            query_args.push((
                "supportsAllDrives".to_string(),
                supports_all_drives.to_string(),
            ));
        }
        if supports_team_drives {
            query_args.push((
                "supportsTeamDrives".to_string(),
                supports_team_drives.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/watch?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
