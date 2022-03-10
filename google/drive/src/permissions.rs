use anyhow::Result;

use crate::Client;

pub struct Permissions {
    pub client: Client,
}

impl Permissions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Permissions { client }
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/permissions` endpoint.
     *
     * Lists a file's or shared drive's permissions.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `include_permissions_for_view: &str` -- Specifies which additional view's permissions to include in the response. Only 'published' is supported.
     * * `page_size: i64` -- The maximum number of permissions to return per page. When not set for files in a shared drive, at most 100 results will be returned. When not set for files that are not in a shared drive, the entire list will be returned.
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `use_domain_admin_access: bool` -- Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
     */
    pub async fn list(
        &self,
        file_id: &str,
        include_permissions_for_view: &str,
        page_size: i64,
        page_token: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        use_domain_admin_access: bool,
    ) -> Result<Vec<crate::types::Permission>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_permissions_for_view.is_empty() {
            query_args.push((
                "includePermissionsForView".to_string(),
                include_permissions_for_view.to_string(),
            ));
        }
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
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
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/permissions?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        let resp: crate::types::PermissionList = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.permissions)
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/permissions` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists a file's or shared drive's permissions.
     */
    pub async fn list_all(
        &self,
        file_id: &str,
        include_permissions_for_view: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        use_domain_admin_access: bool,
    ) -> Result<Vec<crate::types::Permission>> {
        let mut query_args: Vec<(String, String)> = Default::default();
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
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/permissions?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        let mut resp: crate::types::PermissionList = self.client.get(&url, None).await?;

        let mut permissions = resp.permissions;
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

            permissions.append(&mut resp.permissions);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(permissions)
    }

    /**
     * This function performs a `POST` to the `/files/{fileId}/permissions` endpoint.
     *
     * Creates a permission for a file or shared drive.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `email_message: &str` -- A plain text custom message to include in the notification email.
     * * `enforce_single_parent: bool` -- Whether the user has installed the requesting app.
     * * `move_to_new_owners_root: bool` -- This parameter will only take effect if the item is not in a shared drive and the request is attempting to transfer the ownership of the item. If set to true, the item will be moved to the new owner's My Drive root folder and all prior parents removed. If set to false, parents are not changed.
     * * `send_notification_email: bool` -- Whether to send a notification email when sharing to users or groups. This defaults to true for users and groups, and is not allowed for other requests. It must not be disabled for ownership transfers.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `transfer_ownership: bool` -- Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect. File owners can only transfer ownership of files existing on My Drive. Files existing in a shared drive are owned by the organization that owns that shared drive. Ownership transfers are not supported for files and folders in shared drives. Organizers of a shared drive can move items from that shared drive into their My Drive which transfers the ownership to them.
     * * `use_domain_admin_access: bool` -- Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
     */
    pub async fn create(
        &self,
        file_id: &str,
        email_message: &str,
        move_to_new_owners_root: bool,
        send_notification_email: bool,
        supports_all_drives: bool,
        supports_team_drives: bool,
        transfer_ownership: bool,
        use_domain_admin_access: bool,
        body: &crate::types::Permission,
    ) -> Result<crate::types::Permission> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email_message.is_empty() {
            query_args.push(("emailMessage".to_string(), email_message.to_string()));
        }
        if move_to_new_owners_root {
            query_args.push((
                "moveToNewOwnersRoot".to_string(),
                move_to_new_owners_root.to_string(),
            ));
        }
        query_args.push((
            "sendNotificationEmail".to_string(),
            send_notification_email.to_string(),
        ));
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
        if transfer_ownership {
            query_args.push((
                "transferOwnership".to_string(),
                transfer_ownership.to_string(),
            ));
        }
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/permissions?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/permissions/{permissionId}` endpoint.
     *
     * Gets a permission by ID.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `permission_id: &str` -- A link to this theme's background image.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `use_domain_admin_access: bool` -- Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
     */
    pub async fn get(
        &self,
        file_id: &str,
        permission_id: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        use_domain_admin_access: bool,
    ) -> Result<crate::types::Permission> {
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
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/permissions/{}?{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(permission_id),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/files/{fileId}/permissions/{permissionId}` endpoint.
     *
     * Deletes a permission.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `permission_id: &str` -- A link to this theme's background image.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `use_domain_admin_access: bool` -- Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
     */
    pub async fn delete(
        &self,
        file_id: &str,
        permission_id: &str,
        supports_all_drives: bool,
        supports_team_drives: bool,
        use_domain_admin_access: bool,
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
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/permissions/{}?{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(permission_id),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/files/{fileId}/permissions/{permissionId}` endpoint.
     *
     * Updates a permission with patch semantics.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `permission_id: &str` -- A link to this theme's background image.
     * * `remove_expiration: bool` -- Whether the user has installed the requesting app.
     * * `supports_all_drives: bool` -- Whether the requesting application supports both My Drives and shared drives.
     * * `supports_team_drives: bool` -- Whether the user has installed the requesting app.
     * * `transfer_ownership: bool` -- Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect. File owners can only transfer ownership of files existing on My Drive. Files existing in a shared drive are owned by the organization that owns that shared drive. Ownership transfers are not supported for files and folders in shared drives. Organizers of a shared drive can move items from that shared drive into their My Drive which transfers the ownership to them.
     * * `use_domain_admin_access: bool` -- Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs.
     */
    pub async fn update(
        &self,
        file_id: &str,
        permission_id: &str,
        remove_expiration: bool,
        supports_all_drives: bool,
        supports_team_drives: bool,
        transfer_ownership: bool,
        use_domain_admin_access: bool,
        body: &crate::types::Permission,
    ) -> Result<crate::types::Permission> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if remove_expiration {
            query_args.push((
                "removeExpiration".to_string(),
                remove_expiration.to_string(),
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
        if transfer_ownership {
            query_args.push((
                "transferOwnership".to_string(),
                transfer_ownership.to_string(),
            ));
        }
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/permissions/{}?{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(permission_id),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
