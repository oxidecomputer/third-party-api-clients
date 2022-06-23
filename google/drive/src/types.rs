//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DriveThemes {
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "backgroundImageLink"
    )]
    pub background_image_link: String,
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "colorRgb"
    )]
    pub color_rgb: String,
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

/// The user's storage quota limits and usage. All fields are measured in bytes.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct StorageQuota {
    /**
     * The user's storage quota limits and usage. All fields are measured in bytes.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub limit: i64,
    /**
     * The user's storage quota limits and usage. All fields are measured in bytes.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub usage: i64,
    /**
     * The user's storage quota limits and usage. All fields are measured in bytes.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "usageInDrive"
    )]
    pub usage_in_drive: i64,
    /**
     * The user's storage quota limits and usage. All fields are measured in bytes.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "usageInDriveTrash"
    )]
    pub usage_in_drive_trash: i64,
}

/// Information about the user, the user's Drive, and system capabilities.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct About {
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "appInstalled"
    )]
    pub app_installed: Option<bool>,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canCreateDrives"
    )]
    pub can_create_drives: Option<bool>,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canCreateTeamDrives"
    )]
    pub can_create_team_drives: Option<bool>,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "driveThemes"
    )]
    pub drive_themes: Vec<DriveThemes>,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "exportFormats"
    )]
    pub export_formats: Vec<String>,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "folderColorPalette"
    )]
    pub folder_color_palette: Vec<String>,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "importFormats"
    )]
    pub import_formats: Vec<String>,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxImportSizes"
    )]
    pub max_import_sizes: i64,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxUploadSize"
    )]
    pub max_upload_size: i64,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "storageQuota"
    )]
    pub storage_quota: Option<StorageQuota>,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "teamDriveThemes"
    )]
    pub team_drive_themes: Vec<DriveThemes>,
    /**
     * Information about the user, the user's Drive, and system capabilities.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// A change to a file or shared drive.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Change {
    /**
     * A change to a file or shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "changeType"
    )]
    pub change_type: String,
    /**
     * A change to a file or shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drive: Option<Drive>,
    /**
     * A change to a file or shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "driveId"
    )]
    pub drive_id: String,
    /**
     * A change to a file or shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    /**
     * A change to a file or shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "fileId"
    )]
    pub file_id: String,
    /**
     * A change to a file or shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A change to a file or shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed: Option<bool>,
    /**
     * A change to a file or shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teamDrive")]
    pub team_drive: Option<TeamDrive>,
    /**
     * A change to a file or shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "teamDriveId"
    )]
    pub team_drive_id: String,
    /**
     * A change to a file or shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A change to a file or shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// A list of changes for a user.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChangeList {
    /**
     * A list of changes for a user.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub changes: Vec<Change>,
    /**
     * A list of changes for a user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A list of changes for a user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "newStartPageToken"
    )]
    pub new_start_page_token: String,
    /**
     * A list of changes for a user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
}

/// An notification channel used to watch for resource changes.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Channel {
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub expiration: i64,
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub params: String,
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<bool>,
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "resourceId"
    )]
    pub resource_id: String,
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "resourceUri"
    )]
    pub resource_uri: String,
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    /**
     * An notification channel used to watch for resource changes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct QuotedFileContent {
    /**
     * The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "mimeType"
    )]
    pub mime_type: String,
    /**
     * The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// A comment on a file.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Comment {
    /**
     * A comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub anchor: String,
    /**
     * A comment on a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<User>,
    /**
     * A comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    /**
     * A comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "createdTime"
    )]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A comment on a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /**
     * A comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "htmlContent"
    )]
    pub html_content: String,
    /**
     * A comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * A comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "modifiedTime"
    )]
    pub modified_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "quotedFileContent"
    )]
    pub quoted_file_content: Option<QuotedFileContent>,
    /**
     * A comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub replies: Vec<Reply>,
    /**
     * A comment on a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
}

/// A list of comments on a file.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CommentList {
    /**
     * A list of comments on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub comments: Vec<Comment>,
    /**
     * A list of comments on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A list of comments on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
}

/// A restriction for accessing the content of the file.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContentRestriction {
    /**
     * A restriction for accessing the content of the file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readOnly")]
    pub read_only: Option<bool>,
    /**
     * A restriction for accessing the content of the file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * A restriction for accessing the content of the file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "restrictingUser"
    )]
    pub restricting_user: Option<User>,
    /**
     * A restriction for accessing the content of the file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "restrictionTime"
    )]
    pub restriction_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A restriction for accessing the content of the file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BackgroundImageFile {
    /**
     * An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub width: f64,
    /**
     * An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "xCoordinate"
    )]
    pub x_coordinate: f64,
    /**
     * An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "yCoordinate"
    )]
    pub y_coordinate: f64,
}

/// Capabilities the current user has on this shared drive.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Capabilities {
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canAddChildren"
    )]
    pub can_add_children: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeCopyRequiresWriterPermissionRestriction"
    )]
    pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeDomainUsersOnlyRestriction"
    )]
    pub can_change_domain_users_only_restriction: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeDriveBackground"
    )]
    pub can_change_drive_background: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeDriveMembersOnlyRestriction"
    )]
    pub can_change_drive_members_only_restriction: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canComment"
    )]
    pub can_comment: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canCopy")]
    pub can_copy: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canDeleteChildren"
    )]
    pub can_delete_children: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canDeleteDrive"
    )]
    pub can_delete_drive: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canDownload"
    )]
    pub can_download: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canEdit")]
    pub can_edit: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canListChildren"
    )]
    pub can_list_children: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canManageMembers"
    )]
    pub can_manage_members: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canReadRevisions"
    )]
    pub can_read_revisions: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canRename")]
    pub can_rename: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canRenameDrive"
    )]
    pub can_rename_drive: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canShare")]
    pub can_share: Option<bool>,
    /**
     * Capabilities the current user has on this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canTrashChildren"
    )]
    pub can_trash_children: Option<bool>,
}

/// A set of restrictions that apply to this shared drive or items inside this shared drive.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Restrictions {
    /**
     * A set of restrictions that apply to this shared drive or items inside this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "adminManagedRestrictions"
    )]
    pub admin_managed_restrictions: Option<bool>,
    /**
     * A set of restrictions that apply to this shared drive or items inside this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "copyRequiresWriterPermission"
    )]
    pub copy_requires_writer_permission: Option<bool>,
    /**
     * A set of restrictions that apply to this shared drive or items inside this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "domainUsersOnly"
    )]
    pub domain_users_only: Option<bool>,
    /**
     * A set of restrictions that apply to this shared drive or items inside this shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "driveMembersOnly"
    )]
    pub drive_members_only: Option<bool>,
}

/// Representation of a shared drive.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Drive {
    /**
     * Representation of a shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundImageFile"
    )]
    pub background_image_file: Option<BackgroundImageFile>,
    /**
     * Representation of a shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "backgroundImageLink"
    )]
    pub background_image_link: String,
    /**
     * Representation of a shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    /**
     * Representation of a shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "colorRgb"
    )]
    pub color_rgb: String,
    /**
     * Representation of a shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "createdTime"
    )]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Representation of a shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /**
     * Representation of a shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Representation of a shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Representation of a shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Representation of a shared drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
    /**
     * Representation of a shared drive.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "themeId"
    )]
    pub theme_id: String,
}

/// A list of shared drives.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DriveList {
    /**
     * A list of shared drives.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub drives: Vec<Drive>,
    /**
     * A list of shared drives.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A list of shared drives.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
}

/// Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FileCapabilities {
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canAddChildren"
    )]
    pub can_add_children: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canAddFolderFromAnotherDrive"
    )]
    pub can_add_folder_from_another_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canAddMyDriveParent"
    )]
    pub can_add_my_drive_parent: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeCopyRequiresWriterPermission"
    )]
    pub can_change_copy_requires_writer_permission: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeSecurityUpdateEnabled"
    )]
    pub can_change_security_update_enabled: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeViewersCanCopyContent"
    )]
    pub can_change_viewers_can_copy_content: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canComment"
    )]
    pub can_comment: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canCopy")]
    pub can_copy: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canDelete")]
    pub can_delete: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canDeleteChildren"
    )]
    pub can_delete_children: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canDownload"
    )]
    pub can_download: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canEdit")]
    pub can_edit: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canListChildren"
    )]
    pub can_list_children: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canModifyContent"
    )]
    pub can_modify_content: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canModifyContentRestriction"
    )]
    pub can_modify_content_restriction: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveChildrenOutOfDrive"
    )]
    pub can_move_children_out_of_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveChildrenOutOfTeamDrive"
    )]
    pub can_move_children_out_of_team_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveChildrenWithinDrive"
    )]
    pub can_move_children_within_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveChildrenWithinTeamDrive"
    )]
    pub can_move_children_within_team_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveItemIntoTeamDrive"
    )]
    pub can_move_item_into_team_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveItemOutOfDrive"
    )]
    pub can_move_item_out_of_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveItemOutOfTeamDrive"
    )]
    pub can_move_item_out_of_team_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveItemWithinDrive"
    )]
    pub can_move_item_within_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveItemWithinTeamDrive"
    )]
    pub can_move_item_within_team_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canMoveTeamDriveItem"
    )]
    pub can_move_team_drive_item: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canReadDrive"
    )]
    pub can_read_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canReadRevisions"
    )]
    pub can_read_revisions: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canReadTeamDrive"
    )]
    pub can_read_team_drive: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canRemoveChildren"
    )]
    pub can_remove_children: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canRemoveMyDriveParent"
    )]
    pub can_remove_my_drive_parent: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canRename")]
    pub can_rename: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canShare")]
    pub can_share: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canTrash")]
    pub can_trash: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canTrashChildren"
    )]
    pub can_trash_children: Option<bool>,
    /**
     * Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canUntrash"
    )]
    pub can_untrash: Option<bool>,
}

/// A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Thumbnail {
    /**
     * A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<bytes::Bytes>,
    /**
     * A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "mimeType"
    )]
    pub mime_type: String,
}

/// Additional information about the content of the file. These fields are never populated in responses.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContentHints {
    /**
     * Additional information about the content of the file. These fields are never populated in responses.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "indexableText"
    )]
    pub indexable_text: String,
    /**
     * Additional information about the content of the file. These fields are never populated in responses.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
}

/// Geographic location information stored in the image.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Location {
    /**
     * Geographic location information stored in the image.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub altitude: f64,
    /**
     * Geographic location information stored in the image.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub latitude: f64,
    /**
     * Geographic location information stored in the image.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub longitude: f64,
}

/// Additional metadata about image media, if available.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ImageMediaMetadata {
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub aperture: f64,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "cameraMake"
    )]
    pub camera_make: String,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "cameraModel"
    )]
    pub camera_model: String,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "colorSpace"
    )]
    pub color_space: String,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "exposureBias"
    )]
    pub exposure_bias: f64,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "exposureMode"
    )]
    pub exposure_mode: String,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "exposureTime"
    )]
    pub exposure_time: f64,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flashUsed")]
    pub flash_used: Option<bool>,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "focalLength"
    )]
    pub focal_length: f64,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub height: i64,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "isoSpeed"
    )]
    pub iso_speed: i64,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lens: String,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "maxApertureValue"
    )]
    pub max_aperture_value: f64,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "meteringMode"
    )]
    pub metering_mode: String,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub rotation: i64,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sensor: String,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "subjectDistance"
    )]
    pub subject_distance: i64,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time: String,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whiteBalance"
    )]
    pub white_balance: String,
    /**
     * Additional metadata about image media, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub width: i64,
}

/// Contains details about the link URLs that clients are using to refer to this item.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LinkShareMetadata {
    /**
     * Contains details about the link URLs that clients are using to refer to this item.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "securityUpdateEligible"
    )]
    pub security_update_eligible: Option<bool>,
    /**
     * Contains details about the link URLs that clients are using to refer to this item.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "securityUpdateEnabled"
    )]
    pub security_update_enabled: Option<bool>,
}

/// Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShortcutDetails {
    /**
     * Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "targetId"
    )]
    pub target_id: String,
    /**
     * Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "targetMimeType"
    )]
    pub target_mime_type: String,
    /**
     * Shortcut file details. Only populated for shortcut files, which have the mimeType field set to application/vnd.google-apps.shortcut.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "targetResourceKey"
    )]
    pub target_resource_key: String,
}

/// Additional metadata about video media. This may not be available immediately upon upload.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VideoMediaMetadata {
    /**
     * Additional metadata about video media. This may not be available immediately upon upload.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "durationMillis"
    )]
    pub duration_millis: i64,
    /**
     * Additional metadata about video media. This may not be available immediately upon upload.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub height: i64,
    /**
     * Additional metadata about video media. This may not be available immediately upon upload.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub width: i64,
}

/// The metadata for a file.
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct File {
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "appProperties"
    )]
    pub app_properties: String,
    /**
     * The metadata for a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<FileCapabilities>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contentHints"
    )]
    pub content_hints: Option<ContentHints>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "contentRestrictions"
    )]
    pub content_restrictions: Vec<ContentRestriction>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "copyRequiresWriterPermission"
    )]
    pub copy_requires_writer_permission: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "createdTime"
    )]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "driveId"
    )]
    pub drive_id: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "explicitlyTrashed"
    )]
    pub explicitly_trashed: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "exportLinks"
    )]
    pub export_links: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "fileExtension"
    )]
    pub file_extension: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "folderColorRgb"
    )]
    pub folder_color_rgb: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "fullFileExtension"
    )]
    pub full_file_extension: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "hasAugmentedPermissions"
    )]
    pub has_augmented_permissions: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "hasThumbnail"
    )]
    pub has_thumbnail: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "headRevisionId"
    )]
    pub head_revision_id: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "iconLink"
    )]
    pub icon_link: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "imageMediaMetadata"
    )]
    pub image_media_metadata: Option<ImageMediaMetadata>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "isAppAuthorized"
    )]
    pub is_app_authorized: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastModifyingUser"
    )]
    pub last_modifying_user: Option<User>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "linkShareMetadata"
    )]
    pub link_share_metadata: Option<LinkShareMetadata>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "md5Checksum"
    )]
    pub md_5_checksum: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "mimeType"
    )]
    pub mime_type: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "modifiedByMe"
    )]
    pub modified_by_me: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "modifiedByMeTime"
    )]
    pub modified_by_me_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "modifiedTime"
    )]
    pub modified_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "originalFilename"
    )]
    pub original_filename: String,
    /**
     * The metadata for a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownedByMe")]
    pub owned_by_me: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub owners: Vec<User>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub parents: Vec<String>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "permissionIds"
    )]
    pub permission_ids: Vec<String>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub permissions: Vec<Permission>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub properties: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "quotaBytesUsed"
    )]
    pub quota_bytes_used: i64,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "resourceKey"
    )]
    pub resource_key: String,
    /**
     * The metadata for a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "sharedWithMeTime"
    )]
    pub shared_with_me_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sharingUser"
    )]
    pub sharing_user: Option<User>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "shortcutDetails"
    )]
    pub shortcut_details: Option<ShortcutDetails>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub spaces: Vec<String>,
    /**
     * The metadata for a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "teamDriveId"
    )]
    pub team_drive_id: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "thumbnailLink"
    )]
    pub thumbnail_link: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "thumbnailVersion"
    )]
    pub thumbnail_version: i64,
    /**
     * The metadata for a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trashed: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "trashedTime"
    )]
    pub trashed_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trashingUser"
    )]
    pub trashing_user: Option<User>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub version: i64,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "videoMediaMetadata"
    )]
    pub video_media_metadata: Option<VideoMediaMetadata>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "viewedByMe"
    )]
    pub viewed_by_me: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "viewedByMeTime"
    )]
    pub viewed_by_me_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "viewersCanCopyContent"
    )]
    pub viewers_can_copy_content: Option<bool>,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "webContentLink"
    )]
    pub web_content_link: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "webViewLink"
    )]
    pub web_view_link: String,
    /**
     * The metadata for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "writersCanShare"
    )]
    pub writers_can_share: Option<bool>,
}

/// A list of files.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FileList {
    /**
     * A list of files.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub files: Vec<File>,
    /**
     * A list of files.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "incompleteSearch"
    )]
    pub incomplete_search: Option<bool>,
    /**
     * A list of files.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A list of files.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
}

/// A list of generated file IDs which can be provided in create requests.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GeneratedIds {
    /**
     * A list of generated file IDs which can be provided in create requests.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ids: Vec<String>,
    /**
     * A list of generated file IDs which can be provided in create requests.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A list of generated file IDs which can be provided in create requests.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub space: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PermissionDetails {
    /**
     * Whether the user has installed the requesting app.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "inheritedFrom"
    )]
    pub inherited_from: String,
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "permissionType"
    )]
    pub permission_type: String,
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TeamDrivePermissionDetails {
    /**
     * Whether the user has installed the requesting app.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "inheritedFrom"
    )]
    pub inherited_from: String,
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "teamDrivePermissionType"
    )]
    pub team_drive_permission_type: String,
}

/// A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Permission {
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowFileDiscovery"
    )]
    pub allow_file_discovery: Option<bool>,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "emailAddress"
    )]
    pub email_address: String,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "expirationTime"
    )]
    pub expiration_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "permissionDetails"
    )]
    pub permission_details: Vec<PermissionDetails>,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "photoLink"
    )]
    pub photo_link: String,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "teamDrivePermissionDetails"
    )]
    pub team_drive_permission_details: Vec<TeamDrivePermissionDetails>,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * A permission for a file. A permission grants a user, group, domain or the world access to a file or a folder hierarchy.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub view: String,
}

/// A list of permissions for a file.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PermissionList {
    /**
     * A list of permissions for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A list of permissions for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
    /**
     * A list of permissions for a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub permissions: Vec<Permission>,
}

/// A reply to a comment on a file.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Reply {
    /**
     * A reply to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    /**
     * A reply to a comment on a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<User>,
    /**
     * A reply to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    /**
     * A reply to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "createdTime"
    )]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A reply to a comment on a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /**
     * A reply to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "htmlContent"
    )]
    pub html_content: String,
    /**
     * A reply to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * A reply to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A reply to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "modifiedTime"
    )]
    pub modified_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// A list of replies to a comment on a file.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReplyList {
    /**
     * A list of replies to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A list of replies to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
    /**
     * A list of replies to a comment on a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub replies: Vec<Reply>,
}

/// The metadata for a revision to a file.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Revision {
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "exportLinks"
    )]
    pub export_links: String,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "keepForever"
    )]
    pub keep_forever: Option<bool>,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastModifyingUser"
    )]
    pub last_modifying_user: Option<User>,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "md5Checksum"
    )]
    pub md_5_checksum: String,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "mimeType"
    )]
    pub mime_type: String,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "modifiedTime"
    )]
    pub modified_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "originalFilename"
    )]
    pub original_filename: String,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "publishAuto"
    )]
    pub publish_auto: Option<bool>,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "publishedLink"
    )]
    pub published_link: String,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "publishedOutsideDomain"
    )]
    pub published_outside_domain: Option<bool>,
    /**
     * The metadata for a revision to a file.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
}

/// A list of revisions of a file.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RevisionList {
    /**
     * A list of revisions of a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A list of revisions of a file.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
    /**
     * A list of revisions of a file.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub revisions: Vec<Revision>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct StartPageToken {
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A link to this theme's background image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "startPageToken"
    )]
    pub start_page_token: String,
}

/// An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TeamDriveBackgroundImageFile {
    /**
     * An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub width: f64,
    /**
     * An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "xCoordinate"
    )]
    pub x_coordinate: f64,
    /**
     * An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "yCoordinate"
    )]
    pub y_coordinate: f64,
}

/// Capabilities the current user has on this Team Drive.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TeamDriveCapabilities {
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canAddChildren"
    )]
    pub can_add_children: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeCopyRequiresWriterPermissionRestriction"
    )]
    pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeDomainUsersOnlyRestriction"
    )]
    pub can_change_domain_users_only_restriction: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeTeamDriveBackground"
    )]
    pub can_change_team_drive_background: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canChangeTeamMembersOnlyRestriction"
    )]
    pub can_change_team_members_only_restriction: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canComment"
    )]
    pub can_comment: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canCopy")]
    pub can_copy: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canDeleteChildren"
    )]
    pub can_delete_children: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canDeleteTeamDrive"
    )]
    pub can_delete_team_drive: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canDownload"
    )]
    pub can_download: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canEdit")]
    pub can_edit: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canListChildren"
    )]
    pub can_list_children: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canManageMembers"
    )]
    pub can_manage_members: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canReadRevisions"
    )]
    pub can_read_revisions: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canRemoveChildren"
    )]
    pub can_remove_children: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canRename")]
    pub can_rename: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canRenameTeamDrive"
    )]
    pub can_rename_team_drive: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canShare")]
    pub can_share: Option<bool>,
    /**
     * Capabilities the current user has on this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "canTrashChildren"
    )]
    pub can_trash_children: Option<bool>,
}

/// A set of restrictions that apply to this Team Drive or items inside this Team Drive.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TeamDriveRestrictions {
    /**
     * A set of restrictions that apply to this Team Drive or items inside this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "adminManagedRestrictions"
    )]
    pub admin_managed_restrictions: Option<bool>,
    /**
     * A set of restrictions that apply to this Team Drive or items inside this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "copyRequiresWriterPermission"
    )]
    pub copy_requires_writer_permission: Option<bool>,
    /**
     * A set of restrictions that apply to this Team Drive or items inside this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "domainUsersOnly"
    )]
    pub domain_users_only: Option<bool>,
    /**
     * A set of restrictions that apply to this Team Drive or items inside this Team Drive.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "teamMembersOnly"
    )]
    pub team_members_only: Option<bool>,
}

/// Deprecated: use the drive collection instead.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TeamDrive {
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundImageFile"
    )]
    pub background_image_file: Option<TeamDriveBackgroundImageFile>,
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "backgroundImageLink"
    )]
    pub background_image_link: String,
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<TeamDriveCapabilities>,
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "colorRgb"
    )]
    pub color_rgb: String,
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "createdTime"
    )]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<TeamDriveRestrictions>,
    /**
     * Deprecated: use the drive collection instead.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "themeId"
    )]
    pub theme_id: String,
}

/// A list of Team Drives.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TeamDriveList {
    /**
     * A list of Team Drives.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * A list of Team Drives.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nextPageToken"
    )]
    pub next_page_token: String,
    /**
     * A list of Team Drives.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "teamDrives"
    )]
    pub team_drives: Vec<TeamDrive>,
}

/// Information about a Drive user.
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct User {
    /**
     * Information about a Drive user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,
    /**
     * Information about a Drive user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "emailAddress"
    )]
    pub email_address: String,
    /**
     * Information about a Drive user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Information about a Drive user.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub me: Option<bool>,
    /**
     * Information about a Drive user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "permissionId"
    )]
    pub permission_id: String,
    /**
     * Information about a Drive user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "photoLink"
    )]
    pub photo_link: String,
}

/**
 * Data format for the response.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Alt {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Alt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Alt::Json => "json",
            Alt::Noop => "",
            Alt::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Alt {
    fn default() -> Alt {
        Alt::Noop
    }
}
impl Alt {
    pub fn is_noop(&self) -> bool {
        matches!(self, Alt::Noop)
    }
}

/**
 * The source of files to list. Deprecated: use 'corpora' instead.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Corpus {
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Corpus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Corpus::Domain => "domain",
            Corpus::User => "user",
            Corpus::Noop => "",
            Corpus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Corpus {
    fn default() -> Corpus {
        Corpus::Noop
    }
}
impl Corpus {
    pub fn is_noop(&self) -> bool {
        matches!(self, Corpus::Noop)
    }
}
