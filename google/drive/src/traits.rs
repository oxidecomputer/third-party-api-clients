#![allow(clippy::field_reassign_with_default)]
use anyhow::{anyhow, Result};

#[async_trait::async_trait]
pub trait PermissionOps {
    /// Add a permission if it does not already exist.
    ///
    /// `role`: The role granted by this permission. While new values may be supported in the future, the following are currently allowed:
    /// - `owner`
    /// - `organizer`
    /// - `fileOrganizer`
    /// - `writer`
    /// - `commenter`
    /// - `reader`
    ///
    /// `type_`: The type of the grantee. Valid values are:
    /// - `user`
    /// - `group`
    /// - `domain`
    /// - `anyone`
    /// When creating a permission, if type is user or group, you must provide an emailAddress for the user or group. When type is domain, you must provide a domain. There isn't extra information required for a anyone type.
    async fn add_if_not_exists(
        &self,
        file_id: &str,
        email_address: &str,
        email_message: &str,
        role: &str,
        type_: &str,
        use_domain_admin_access: bool,
        send_notification_email: bool,
    ) -> Result<crate::types::Permission>;
}

#[async_trait::async_trait]
impl PermissionOps for crate::permissions::Permissions {
    /// Add a permission if it does not already exist.
    ///
    /// `role`: The role granted by this permission. While new values may be supported in the future, the following are currently allowed:
    /// - `owner`
    /// - `organizer`
    /// - `fileOrganizer`
    /// - `writer`
    /// - `commenter`
    /// - `reader`
    ///
    /// `type_`: The type of the grantee. Valid values are:
    /// - `user`
    /// - `group`
    /// - `domain`
    /// - `anyone`
    /// When creating a permission, if type is user or group, you must provide an emailAddress for the user or group. When type is domain, you must provide a domain. There isn't extra information required for a anyone type.
    async fn add_if_not_exists(
        &self,
        file_id: &str,
        email_address: &str,
        email_message: &str,
        role: &str,
        type_: &str,
        use_domain_admin_access: bool,
        send_notification_email: bool,
    ) -> Result<crate::types::Permission> {
        // First let's check if the permission already exists.
        // List all the permissions for a file.
        let perms = self
            .list_all(
                file_id,
                "",   // include_permissions_for_view
                true, // supports_all_drives
                true, // supports_team_drives
                use_domain_admin_access,
            )
            .await?;

        // Iterate over our permissions and see if we have ours.
        for perm in perms {
            if perm.email_address == email_address && perm.role == role && perm.type_ == type_ {
                // We found the permission, return it.
                return Ok(perm);
            }
        }

        // If we got here we could not find the permission so let's create it.
        let perm = crate::types::Permission {
            allow_file_discovery: None,
            deleted: None,
            display_name: String::new(),
            domain: String::new(),
            email_address: email_address.to_string(),
            expiration_time: None,
            id: String::new(),
            kind: String::new(),
            permission_details: Default::default(),
            photo_link: String::new(),
            role: role.to_string(),
            team_drive_permission_details: Default::default(),
            type_: type_.to_string(),
            view: String::new(),
        };

        // Create the permission and return it.
        self.create(
            file_id,
            email_message,
            false, // move_to_new_owners_root
            send_notification_email,
            true,  // supports_all_drives
            true,  // supports_team_drives
            false, // transfer_ownership
            use_domain_admin_access,
            &perm,
        )
        .await
    }
}

#[async_trait::async_trait]
pub trait FileOps {
    /// Get a file by it's name.
    async fn get_by_name(
        &self,
        drive_id: &str,
        parent_id: &str,
        name: &str,
    ) -> Result<Vec<crate::types::File>>;

    /// Create or update a file in a drive.
    /// If the file already exists, it will update it.
    /// If the file does not exist, it will create it.
    async fn create_or_update(
        &self,
        drive_id: &str,
        parent_id: &str,
        name: &str,
        mime_type: &str,
        contents: &[u8],
    ) -> Result<crate::types::File>;

    /// Download a file by it's ID.
    async fn download_by_id(&self, id: &str) -> Result<bytes::Bytes>;

    /// Create a folder, if it doesn't exist, returns the ID of the folder.
    async fn create_folder(&self, drive_id: &str, parent_id: &str, name: &str) -> Result<String>;

    /// Get a file's contents by it's ID. Only works for Google Docs.
    async fn get_contents_by_id(&self, id: &str) -> Result<String>;

    /// Delete a file by its name.
    async fn delete_by_name(&self, drive_id: &str, parent_id: &str, name: &str) -> Result<()>;
}

#[async_trait::async_trait]
impl FileOps for crate::files::Files {
    /// Get a file by it's name.
    async fn get_by_name(
        &self,
        drive_id: &str,
        parent_id: &str,
        name: &str,
    ) -> Result<Vec<crate::types::File>> {
        let mut query = format!("name = '{}'", name);
        if !parent_id.is_empty() {
            query = format!("{} and '{}' in parents", query, parent_id);
        }

        self.list_all(
            "drive",  // corpora
            drive_id, // drive id
            true,     // include_items_from_all_drives
            "",       // include_permissions_for_view
            false,    // include_team_drive_items
            "",       // order_by
            &query,   // query
            "",       // spaces
            true,     // supports_all_drives
            false,    // supports_team_drives
            "",       // team_drive_id
        )
        .await
    }

    /// Create or update a file in a drive.
    /// If the file already exists, it will update it.
    /// If the file does not exist, it will create it.
    async fn create_or_update(
        &self,
        drive_id: &str,
        parent_id: &str,
        name: &str,
        mime_type: &str,
        contents: &[u8],
    ) -> Result<crate::types::File> {
        // Create the file.
        let mut f: crate::types::File = Default::default();
        let mut method = reqwest::Method::POST;
        let mut uri = "https://www.googleapis.com/upload/drive/v3/files".to_string();

        // Check if the file exists.
        let files = self
            .get_by_name(drive_id, parent_id, name)
            .await
            .unwrap_or_default();
        if files.is_empty() {
            // Set the name,
            f.name = name.to_string();
            f.mime_type = mime_type.to_string();
            if !parent_id.is_empty() {
                f.parents = vec![parent_id.to_string()];
            } else {
                f.parents = vec![drive_id.to_string()];
            }

            uri += "?uploadType=resumable&supportsAllDrives=true&includeItemsFromAllDrives=true";

            // Create the file.
        } else if let Some(f) = files.get(0) {
            method = reqwest::Method::PATCH;
            let mut f = f.clone();
            uri += &format!(
                "/{}?uploadType=resumable&supportsAllDrives=true&includeItemsFromAllDrives=true",
                f.id
            );

            f.id = "".to_string();
            f.drive_id = "".to_string();
            f.kind = "".to_string();
            f.original_filename = f.name.to_string();
        } else {
            // Set the name,
            f.name = name.to_string();
            f.mime_type = mime_type.to_string();
            if !parent_id.is_empty() {
                f.parents = vec![parent_id.to_string()];
            } else {
                f.parents = vec![drive_id.to_string()];
            }

            uri += "?uploadType=resumable&supportsAllDrives=true&includeItemsFromAllDrives=true";

            // Create the file.
        }

        // Build the request to get the URL upload location if we need to create the file.
        let resp = self
            .client
            .request_raw(
                method,
                &uri,
                Some(reqwest::Body::from(serde_json::to_vec(&f)?)),
            )
            .await?;

        // Get the "Location" header.
        let location = match resp.headers().get("Location") {
            Some(location) => location.to_str()?,
            None => anyhow::bail!("No Location header"),
        };

        // Now upload the file to that location.
        self.client
            .request_with_mime(reqwest::Method::PUT, location, contents, mime_type)
            .await
    }

    /// Download a file by it's ID.
    async fn download_by_id(&self, id: &str) -> Result<bytes::Bytes> {
        let resp = self
            .client
            .request_raw(
                reqwest::Method::GET,
                &format!("/files/{}?supportsAllDrives=true&alt=media", id),
                None,
            )
            .await?;

        Ok(resp.bytes().await?)
    }

    /// Create a folder, if it doesn't exist, returns the ID of the folder.
    async fn create_folder(&self, drive_id: &str, parent_id: &str, name: &str) -> Result<String> {
        let folder_mime_type = "application/vnd.google-apps.folder";
        let mut file: crate::types::File = Default::default();
        // Set the name,
        file.name = name.to_string();
        file.mime_type = folder_mime_type.to_string();
        if !parent_id.is_empty() {
            file.parents = vec![parent_id.to_string()];
        } else {
            file.parents = vec![drive_id.to_string()];
        }

        let mut query = format!(
            "name = '{}' and mimeType = 'application/vnd.google-apps.folder'",
            name
        );
        if !parent_id.is_empty() {
            query = format!("{} and '{}' in parents", query, parent_id);
        }

        // Check if the folder exists.
        let folders = self
            .list_all(
                "drive",  // corpora
                drive_id, // drive id
                true,     // include_items_from_all_drives
                "",       // include_permissions_for_view
                false,    // include_team_drive_items
                "",       // order_by
                &query,   // query
                "",       // spaces
                true,     // supports_all_drives
                false,    // supports_team_drives
                "",       // team_drive_id
            )
            .await
            .unwrap_or_default();
        if !folders.is_empty() {
            let f = folders.get(0).unwrap().clone();
            return Ok(f.id);
        }

        // Make the request and return the ID.
        let folder: crate::types::File = self
            .client
            .post(
                "/files?supportsAllDrives=true&includeItemsFromAllDrives=true",
                Some(reqwest::Body::from(serde_json::to_vec(&file)?)),
            )
            .await?;

        Ok(folder.id)
    }

    /// Get a file's contents by it's ID. Only works for Google Docs.
    // TODO: make binary content work in the actual library.
    async fn get_contents_by_id(&self, id: &str) -> Result<String> {
        let mut query_ = String::new();
        let query_args = vec!["mime_type=text/plain".to_string()];
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/files/{}/export?{}",
            crate::progenitor_support::encode_path(id),
            query_
        );
        let resp = self
            .client
            .request_raw(reqwest::Method::GET, &url, None)
            .await?;

        Ok(resp.text().await?)
    }

    /// Delete a file by its name.
    async fn delete_by_name(&self, drive_id: &str, parent_id: &str, name: &str) -> Result<()> {
        // Check if the file exists.
        let files = self
            .get_by_name(drive_id, parent_id, name)
            .await
            .unwrap_or_default();
        if files.is_empty() {
            // The file does not exist.
            return Ok(());
        }

        // Delete the file.
        self.delete(
            &files.get(0).unwrap().id,
            true, // supports all drives
            true, // supports team drives
        )
        .await
    }
}

#[async_trait::async_trait]
pub trait DriveOps {
    /// Get a drive by it's name.
    async fn get_by_name(&self, name: &str) -> Result<crate::types::Drive>;
}

#[async_trait::async_trait]
impl DriveOps for crate::drives::Drives {
    /// Get a drive by it's name.
    async fn get_by_name(&self, name: &str) -> Result<crate::types::Drive> {
        let drives = self
            .list_all(
                //&format!("name = '{}'", name), // query
                "", true, // use domain admin access
            )
            .await?;

        for drive in drives {
            if drive.name == name {
                return Ok(drive);
            }
        }

        Err(anyhow!("could not find drive with name: {:?}", name))
    }
}
