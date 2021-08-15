use anyhow::Result;

use crate::Client;

pub struct Changes {
    client: Client,
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
     * * `page_size: i64` -- The maximum number of changes to return per page.
     * * `restrict_to_my_drive: bool` -- Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
     * * `spaces: &str` -- A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `team_drive_id: &str` -- A link to this theme's background image.
     */
    pub async fn drive_list(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
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
    ) -> Result<crate::types::ChangeList> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !drive_id.is_empty() {
            query_args.push(format!("drive_id={}", drive_id));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if include_corpus_removals {
            query_args.push(format!(
                "include_corpus_removals={}",
                include_corpus_removals
            ));
        }
        if include_items_from_all_drives {
            query_args.push(format!(
                "include_items_from_all_drives={}",
                include_items_from_all_drives
            ));
        }
        if !include_permissions_for_view.is_empty() {
            query_args.push(format!(
                "include_permissions_for_view={}",
                include_permissions_for_view
            ));
        }
        if include_removed {
            query_args.push(format!("include_removed={}", include_removed));
        }
        if include_team_drive_items {
            query_args.push(format!(
                "include_team_drive_items={}",
                include_team_drive_items
            ));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if restrict_to_my_drive {
            query_args.push(format!("restrict_to_my_drive={}", restrict_to_my_drive));
        }
        if !spaces.is_empty() {
            query_args.push(format!("spaces={}", spaces));
        }
        if supports_all_drives {
            query_args.push(format!("supports_all_drives={}", supports_all_drives));
        }
        if supports_team_drives {
            query_args.push(format!("supports_team_drives={}", supports_team_drives));
        }
        if !team_drive_id.is_empty() {
            query_args.push(format!("team_drive_id={}", team_drive_id));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/changes?{}", query_);

        self.client.get(&url, None).await
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
    pub async fn drive_get_start_page_token(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        drive_id: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        team_drive_id: &str,
    ) -> Result<crate::types::StartPageToken> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !drive_id.is_empty() {
            query_args.push(format!("drive_id={}", drive_id));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if supports_all_drives {
            query_args.push(format!("supports_all_drives={}", supports_all_drives));
        }
        if supports_team_drives {
            query_args.push(format!("supports_team_drives={}", supports_team_drives));
        }
        if !team_drive_id.is_empty() {
            query_args.push(format!("team_drive_id={}", team_drive_id));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
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
     * * `page_size: i64` -- The maximum number of changes to return per page.
     * * `restrict_to_my_drive: bool` -- Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.
     * * `spaces: &str` -- A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `team_drive_id: &str` -- A link to this theme's background image.
     */
    pub async fn drive_watch(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
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
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !drive_id.is_empty() {
            query_args.push(format!("drive_id={}", drive_id));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if include_corpus_removals {
            query_args.push(format!(
                "include_corpus_removals={}",
                include_corpus_removals
            ));
        }
        if include_items_from_all_drives {
            query_args.push(format!(
                "include_items_from_all_drives={}",
                include_items_from_all_drives
            ));
        }
        if !include_permissions_for_view.is_empty() {
            query_args.push(format!(
                "include_permissions_for_view={}",
                include_permissions_for_view
            ));
        }
        if include_removed {
            query_args.push(format!("include_removed={}", include_removed));
        }
        if include_team_drive_items {
            query_args.push(format!(
                "include_team_drive_items={}",
                include_team_drive_items
            ));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if restrict_to_my_drive {
            query_args.push(format!("restrict_to_my_drive={}", restrict_to_my_drive));
        }
        if !spaces.is_empty() {
            query_args.push(format!("spaces={}", spaces));
        }
        if supports_all_drives {
            query_args.push(format!("supports_all_drives={}", supports_all_drives));
        }
        if supports_team_drives {
            query_args.push(format!("supports_team_drives={}", supports_team_drives));
        }
        if !team_drive_id.is_empty() {
            query_args.push(format!("team_drive_id={}", team_drive_id));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/changes/watch?{}", query_);

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
