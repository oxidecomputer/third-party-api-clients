use anyhow::Result;

use crate::Client;

pub struct Changes {
    pub client: Client,
}

impl Changes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Changes { client }
    }

    /**
     * This function performs a `GET` to the `/changes` endpoint.
     *
     * Lists the changes for a user or shared drive.
     *
     * **Parameters:**
     *
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
     * * `drive_id: &str` -- The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.
     * * `include_corpus_removals: bool` -- Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.
     * * `include_items_from_all_drives: bool` -- Whether both My Drive and shared drive items should be included in results.
     * * `include_permissions_for_view: &str` -- Specifies which additional view's permissions to include in the response. Only 'published' is supported.
     * * `include_removed: bool` -- Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.
     * * `include_team_drive_items: bool` -- Whether the user has installed the requesting app.
     * * `page_size: i64` -- A map of maximum import sizes by MIME type, in bytes.
     * * `restrict_to_my_drive: bool` -- Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
     * * `spaces: &str` -- A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `team_drive_id: &str` -- A link to this theme's background image.
     */
    pub async fn list(
        &self,
        page_token: &str,
        drive_id: &str,
        include_corpus_removals: bool,
        include_items_from_all_drives: bool,
        include_permissions_for_view: &str,
        include_removed: bool,
        include_team_drive_items: bool,
        page_size: i64,
        restrict_to_my_drive: bool,
        spaces: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        team_drive_id: &str,
    ) -> Result<Vec<crate::types::Change>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !drive_id.is_empty() {
            query_args.push(("driveId".to_string(), drive_id.to_string()));
        }
        if include_corpus_removals {
            query_args.push((
                "includeCorpusRemovals".to_string(),
                include_corpus_removals.to_string(),
            ));
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
        if include_removed {
            query_args.push(("includeRemoved".to_string(), include_removed.to_string()));
        }
        if include_team_drive_items {
            query_args.push((
                "includeTeamDriveItems".to_string(),
                include_team_drive_items.to_string(),
            ));
        }
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if restrict_to_my_drive {
            query_args.push((
                "restrictToMyDrive".to_string(),
                restrict_to_my_drive.to_string(),
            ));
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
        let url = format!("/changes?{}", query_);

        let resp: crate::types::ChangeList = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.changes)
    }

    /**
     * This function performs a `GET` to the `/changes` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists the changes for a user or shared drive.
     */
    pub async fn list_all(
        &self,
        drive_id: &str,
        include_corpus_removals: bool,
        include_items_from_all_drives: bool,
        include_permissions_for_view: &str,
        include_removed: bool,
        include_team_drive_items: bool,
        restrict_to_my_drive: bool,
        spaces: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        team_drive_id: &str,
    ) -> Result<Vec<crate::types::Change>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !drive_id.is_empty() {
            query_args.push(("driveId".to_string(), drive_id.to_string()));
        }
        if include_corpus_removals {
            query_args.push((
                "includeCorpusRemovals".to_string(),
                include_corpus_removals.to_string(),
            ));
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
        if include_removed {
            query_args.push(("includeRemoved".to_string(), include_removed.to_string()));
        }
        if include_team_drive_items {
            query_args.push((
                "includeTeamDriveItems".to_string(),
                include_team_drive_items.to_string(),
            ));
        }
        if restrict_to_my_drive {
            query_args.push((
                "restrictToMyDrive".to_string(),
                restrict_to_my_drive.to_string(),
            ));
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
        let url = format!("/changes?{}", query_);

        let mut resp: crate::types::ChangeList = self.client.get(&url, None).await?;

        let mut changes = resp.changes;
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

            changes.append(&mut resp.changes);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(changes)
    }

    /**
     * This function performs a `GET` to the `/changes/startPageToken` endpoint.
     *
     * Gets the starting pageToken for listing future changes.
     *
     * **Parameters:**
     *
     * * `drive_id: &str` -- The ID of the shared drive for which the starting pageToken for listing future changes from that shared drive is returned.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `team_drive_id: &str` -- A link to this theme's background image.
     */
    pub async fn get_start_page_token(
        &self,
        drive_id: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        team_drive_id: &str,
    ) -> Result<crate::types::StartPageToken> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !drive_id.is_empty() {
            query_args.push(("driveId".to_string(), drive_id.to_string()));
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
        let url = format!("/changes/startPageToken?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/changes/watch` endpoint.
     *
     * Subscribes to changes for a user.
     *
     * **Parameters:**
     *
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.
     * * `drive_id: &str` -- The shared drive from which changes are returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.
     * * `include_corpus_removals: bool` -- Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.
     * * `include_items_from_all_drives: bool` -- Whether both My Drive and shared drive items should be included in results.
     * * `include_permissions_for_view: &str` -- Specifies which additional view's permissions to include in the response. Only 'published' is supported.
     * * `include_removed: bool` -- Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.
     * * `include_team_drive_items: bool` -- Whether the user has installed the requesting app.
     * * `page_size: i64` -- A map of maximum import sizes by MIME type, in bytes.
     * * `restrict_to_my_drive: bool` -- Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
     * * `spaces: &str` -- A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `team_drive_id: &str` -- A link to this theme's background image.
     */
    pub async fn watch(
        &self,
        page_token: &str,
        drive_id: &str,
        include_corpus_removals: bool,
        include_items_from_all_drives: bool,
        include_permissions_for_view: &str,
        include_removed: bool,
        include_team_drive_items: bool,
        page_size: i64,
        restrict_to_my_drive: bool,
        spaces: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        team_drive_id: &str,
        body: &crate::types::Channel,
    ) -> Result<crate::types::Channel> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !drive_id.is_empty() {
            query_args.push(("driveId".to_string(), drive_id.to_string()));
        }
        if include_corpus_removals {
            query_args.push((
                "includeCorpusRemovals".to_string(),
                include_corpus_removals.to_string(),
            ));
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
        if include_removed {
            query_args.push(("includeRemoved".to_string(), include_removed.to_string()));
        }
        if include_team_drive_items {
            query_args.push((
                "includeTeamDriveItems".to_string(),
                include_team_drive_items.to_string(),
            ));
        }
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if restrict_to_my_drive {
            query_args.push((
                "restrictToMyDrive".to_string(),
                restrict_to_my_drive.to_string(),
            ));
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
        let url = format!("/changes/watch?{}", query_);

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
