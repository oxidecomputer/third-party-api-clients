use anyhow::Result;

#[async_trait::async_trait]
pub trait Operations {
    /// Get a file by it's name.
    async fn get_by_name(&self, drive_id: &str, name: &str) -> Result<Vec<crate::types::File>>;

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
}

#[async_trait::async_trait]
impl Operations for crate::files::Files {
    /// Get a file by it's name.
    async fn get_by_name(&self, drive_id: &str, name: &str) -> Result<Vec<crate::types::File>> {
        self.drive_list_files(
            "drive",                       // corpora
            crate::types::Corpus::Noop,    // corpus
            drive_id,                      // drive id
            true,                          // include_items_from_all_drives
            "",                            // include_permissions_for_view
            false,                         // include_team_drive_items
            "",                            // order_by
            &format!("name = '{}'", name), // query
            "",                            // spaces
            true,                          // supports_all_drives
            false,                         // supports_team_drives
            "",                            // team_drive_id
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
        let files = self.get_by_name(drive_id, name).await.unwrap_or_default();
        if files.is_empty() {
            // Set the name,
            f.name = name.to_string();
            f.mime_type = mime_type.to_string();
            if !parent_id.is_empty() {
                f.parents = vec![parent_id.to_string()];
            } else {
                f.parents = vec![drive_id.to_string()];
            }

            // Create the file.
        } else {
            method = reqwest::Method::PATCH;

            f = files.get(0).unwrap().clone();
            uri += &format!(
                "/{}?uploadType=resumable&supportsAllDrives=true&includeItemsFromAllDrives=true",
                f.id
            );

            f.id = "".to_string();
            f.drive_id = "".to_string();
            f.kind = "".to_string();
            f.original_filename = f.name.to_string();
        }

        // Build the request to get the URL upload location if we need to create the file.
        let resp = self
            .client
            .request_raw(
                method,
                &uri,
                Some(reqwest::Body::from(serde_json::to_vec(&f).unwrap())),
            )
            .await
            .unwrap();

        // Get the "Location" header.
        let location = resp.headers().get("Location").unwrap().to_str().unwrap();

        // Now upload the file to that location.
        Ok(self
            .client
            .request_with_mime(reqwest::Method::PUT, &location, contents, mime_type)
            .await
            .unwrap())
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
            .await
            .unwrap();

        Ok(resp.bytes().await.unwrap())
    }
}
