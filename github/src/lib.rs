#![allow(clippy::too_many_arguments)]

pub mod auth;
#[cfg(feature = "httpcache")]
pub mod http_cache;
pub mod utils;

use anyhow::{anyhow, Error, Result};
use chrono::{DateTime, Utc};

const DEFAULT_HOST: &str = "https://api.github.com";

mod progenitor_support {
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

    const PATH_SET: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}');

    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}

pub mod types {
    use chrono::{DateTime, NaiveDate, Utc};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SimpleUser {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Owner {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Permissions {
        pub checks: Option<String>,
        pub contents: Option<String>,
        pub deployments: Option<String>,
        pub issues: Option<String>,
        pub metadata: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Integration {
        pub client_id: Option<String>,
        pub client_secret: Option<String>,
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub events: Vec<String>,
        pub external_url: String,
        pub html_url: String,
        pub id: i64,
        pub installations_count: Option<i64>,
        pub name: String,
        pub node_id: String,
        pub owner: Owner,
        pub pem: Option<String>,
        pub permissions: Permissions,
        pub slug: Option<String>,
        pub updated_at: DateTime<Utc>,
        pub webhook_secret: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct BasicError {
        pub documentation_url: Option<String>,
        pub message: Option<String>,
        pub status: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ValidationErrorSimple {
        pub documentation_url: String,
        pub errors: Option<Vec<String>>,
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct WebhookConfig {
        pub content_type: Option<String>,
        pub insecure_ssl: Option<String>,
        pub secret: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Enterprise {
        pub avatar_url: String,
        pub created_at: DateTime<Utc>,
        pub description: Option<String>,
        pub html_url: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub slug: String,
        pub updated_at: DateTime<Utc>,
        pub website_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AppPermissions {
        pub actions: Option<String>,
        pub administration: Option<String>,
        pub checks: Option<String>,
        pub content_references: Option<String>,
        pub contents: Option<String>,
        pub deployments: Option<String>,
        pub environments: Option<String>,
        pub issues: Option<String>,
        pub members: Option<String>,
        pub metadata: Option<String>,
        pub organization_administration: Option<String>,
        pub organization_hooks: Option<String>,
        pub organization_packages: Option<String>,
        pub organization_plan: Option<String>,
        pub organization_projects: Option<String>,
        pub organization_secrets: Option<String>,
        pub organization_self_hosted_runners: Option<String>,
        pub organization_user_blocking: Option<String>,
        pub packages: Option<String>,
        pub pages: Option<String>,
        pub pull_requests: Option<String>,
        pub repository_hooks: Option<String>,
        pub repository_projects: Option<String>,
        pub secret_scanning_alerts: Option<String>,
        pub secrets: Option<String>,
        pub security_events: Option<String>,
        pub single_file: Option<String>,
        pub statuses: Option<String>,
        pub team_discussions: Option<String>,
        pub vulnerability_alerts: Option<String>,
        pub workflows: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Account {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SuspendedBy {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Installation {
        pub access_tokens_url: String,
        pub account: Account,
        pub app_id: i64,
        pub app_slug: String,
        pub contact_email: Option<String>,
        pub created_at: DateTime<Utc>,
        pub events: Vec<String>,
        pub has_multiple_single_files: Option<bool>,
        pub html_url: String,
        pub id: i64,
        pub permissions: AppPermissions,
        pub repositories_url: String,
        pub repository_selection: String,
        pub single_file_name: String,
        pub single_file_paths: Option<Vec<String>>,
        pub suspended_at: DateTime<Utc>,
        pub suspended_by: SuspendedBy,
        pub target_id: i64,
        pub target_type: String,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LicenseSimple {
        pub html_url: Option<String>,
        pub key: String,
        pub name: String,
        pub node_id: String,
        pub spdx_id: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct License {
        pub html_url: Option<String>,
        pub key: String,
        pub name: String,
        pub node_id: String,
        pub spdx_id: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Organization {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RepositoryPermissions {
        pub admin: bool,
        pub maintain: Option<bool>,
        pub pull: bool,
        pub push: bool,
        pub triage: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RepositoryTemplateRepositoryTemplateRepositoryOwner {
        pub avatar_url: Option<String>,
        pub events_url: Option<String>,
        pub followers_url: Option<String>,
        pub following_url: Option<String>,
        pub gists_url: Option<String>,
        pub gravatar_id: Option<String>,
        pub html_url: Option<String>,
        pub id: Option<i64>,
        pub login: Option<String>,
        pub node_id: Option<String>,
        pub organizations_url: Option<String>,
        pub received_events_url: Option<String>,
        pub repos_url: Option<String>,
        pub site_admin: Option<bool>,
        pub starred_url: Option<String>,
        pub subscriptions_url: Option<String>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RepositoryTemplateRepositoryTemplateRepositoryPermissions {
        pub admin: Option<bool>,
        pub pull: Option<bool>,
        pub push: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TemplateRepository {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub archive_url: Option<String>,
        pub archived: Option<bool>,
        pub assignees_url: Option<String>,
        pub blobs_url: Option<String>,
        pub branches_url: Option<String>,
        pub clone_url: Option<String>,
        pub collaborators_url: Option<String>,
        pub comments_url: Option<String>,
        pub commits_url: Option<String>,
        pub compare_url: Option<String>,
        pub contents_url: Option<String>,
        pub contributors_url: Option<String>,
        pub created_at: Option<String>,
        pub default_branch: Option<String>,
        pub delete_branch_on_merge: Option<bool>,
        pub deployments_url: Option<String>,
        pub description: Option<String>,
        pub disabled: Option<bool>,
        pub downloads_url: Option<String>,
        pub events_url: Option<String>,
        pub fork: Option<bool>,
        pub forks_count: Option<i64>,
        pub forks_url: Option<String>,
        pub full_name: Option<String>,
        pub git_commits_url: Option<String>,
        pub git_refs_url: Option<String>,
        pub git_tags_url: Option<String>,
        pub git_url: Option<String>,
        pub has_downloads: Option<bool>,
        pub has_issues: Option<bool>,
        pub has_pages: Option<bool>,
        pub has_projects: Option<bool>,
        pub has_wiki: Option<bool>,
        pub homepage: Option<String>,
        pub hooks_url: Option<String>,
        pub html_url: Option<String>,
        pub id: Option<i64>,
        pub is_template: Option<bool>,
        pub issue_comment_url: Option<String>,
        pub issue_events_url: Option<String>,
        pub issues_url: Option<String>,
        pub keys_url: Option<String>,
        pub labels_url: Option<String>,
        pub language: Option<String>,
        pub languages_url: Option<String>,
        pub merges_url: Option<String>,
        pub milestones_url: Option<String>,
        pub mirror_url: Option<String>,
        pub name: Option<String>,
        pub network_count: Option<i64>,
        pub node_id: Option<String>,
        pub notifications_url: Option<String>,
        pub open_issues_count: Option<i64>,
        pub owner: Option<Owner>,
        pub permissions: Option<Permissions>,
        pub private: Option<bool>,
        pub pulls_url: Option<String>,
        pub pushed_at: Option<String>,
        pub releases_url: Option<String>,
        pub size: Option<i64>,
        pub ssh_url: Option<String>,
        pub stargazers_count: Option<i64>,
        pub stargazers_url: Option<String>,
        pub statuses_url: Option<String>,
        pub subscribers_count: Option<i64>,
        pub subscribers_url: Option<String>,
        pub subscription_url: Option<String>,
        pub svn_url: Option<String>,
        pub tags_url: Option<String>,
        pub teams_url: Option<String>,
        pub temp_clone_token: Option<String>,
        pub topics: Option<Vec<String>>,
        pub trees_url: Option<String>,
        pub updated_at: Option<String>,
        pub url: Option<String>,
        pub visibility: Option<String>,
        pub watchers_count: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Repository {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub archive_url: String,
        pub archived: bool,
        pub assignees_url: String,
        pub blobs_url: String,
        pub branches_url: String,
        pub clone_url: String,
        pub collaborators_url: String,
        pub comments_url: String,
        pub commits_url: String,
        pub compare_url: String,
        pub contents_url: String,
        pub contributors_url: String,
        pub created_at: DateTime<Utc>,
        pub default_branch: String,
        pub delete_branch_on_merge: Option<bool>,
        pub deployments_url: String,
        pub description: String,
        pub disabled: bool,
        pub downloads_url: String,
        pub events_url: String,
        pub fork: bool,
        pub forks: i64,
        pub forks_count: i64,
        pub forks_url: String,
        pub full_name: String,
        pub git_commits_url: String,
        pub git_refs_url: String,
        pub git_tags_url: String,
        pub git_url: String,
        pub has_downloads: bool,
        pub has_issues: bool,
        pub has_pages: bool,
        pub has_projects: bool,
        pub has_wiki: bool,
        pub homepage: String,
        pub hooks_url: String,
        pub html_url: String,
        pub id: i64,
        pub is_template: Option<bool>,
        pub issue_comment_url: String,
        pub issue_events_url: String,
        pub issues_url: String,
        pub keys_url: String,
        pub labels_url: String,
        pub language: String,
        pub languages_url: String,
        pub license: License,
        pub master_branch: Option<String>,
        pub merges_url: String,
        pub milestones_url: String,
        pub mirror_url: String,
        pub name: String,
        pub network_count: Option<i64>,
        pub node_id: String,
        pub notifications_url: String,
        pub open_issues: i64,
        pub open_issues_count: i64,
        pub organization: Option<Organization>,
        pub owner: Owner,
        pub permissions: Option<Permissions>,
        pub private: bool,
        pub pulls_url: String,
        pub pushed_at: DateTime<Utc>,
        pub releases_url: String,
        pub size: i64,
        pub ssh_url: String,
        pub stargazers_count: i64,
        pub stargazers_url: String,
        pub starred_at: Option<String>,
        pub statuses_url: String,
        pub subscribers_count: Option<i64>,
        pub subscribers_url: String,
        pub subscription_url: String,
        pub svn_url: String,
        pub tags_url: String,
        pub teams_url: String,
        pub temp_clone_token: Option<String>,
        pub template_repository: Option<TemplateRepository>,
        pub topics: Option<Vec<String>>,
        pub trees_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub visibility: Option<String>,
        pub watchers: i64,
        pub watchers_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct InstallationToken {
        pub expires_at: String,
        pub has_multiple_single_files: Option<bool>,
        pub permissions: Option<AppPermissions>,
        pub repositories: Option<Vec<Repository>>,
        pub repository_selection: Option<String>,
        pub single_file: Option<String>,
        pub single_file_paths: Option<Vec<String>>,
        pub token: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Errors {
        pub code: String,
        pub field: Option<String>,
        pub index: Option<i64>,
        pub message: Option<String>,
        pub resource: Option<String>,
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ValidationError {
        pub documentation_url: String,
        pub errors: Option<Vec<Errors>>,
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct App {
        pub client_id: String,
        pub name: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct User {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ApplicationGrant {
        pub app: App,
        pub created_at: DateTime<Utc>,
        pub id: i64,
        pub scopes: Vec<String>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: Option<User>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScopedInstallation {
        pub account: SimpleUser,
        pub has_multiple_single_files: Option<bool>,
        pub permissions: AppPermissions,
        pub repositories_url: String,
        pub repository_selection: String,
        pub single_file_name: String,
        pub single_file_paths: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AuthorizationInstallation {
        pub account: SimpleUser,
        pub has_multiple_single_files: Option<bool>,
        pub permissions: AppPermissions,
        pub repositories_url: String,
        pub repository_selection: String,
        pub single_file_name: String,
        pub single_file_paths: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Authorization {
        pub app: App,
        pub created_at: DateTime<Utc>,
        pub fingerprint: String,
        pub hashed_token: String,
        pub id: i64,
        pub installation: Option<Installation>,
        pub note: String,
        pub note_url: String,
        pub scopes: Vec<String>,
        pub token: String,
        pub token_last_eight: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: Option<User>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeofConduct {
        pub body: Option<String>,
        pub html_url: String,
        pub key: String,
        pub name: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ActionsEnterprisePermissions {
        pub allowed_actions: String,
        pub enabled_organizations: String,
        pub selected_actions_url: Option<String>,
        pub selected_organizations_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct OrganizationSimple {
        pub avatar_url: String,
        pub description: String,
        pub events_url: String,
        pub hooks_url: String,
        pub id: i64,
        pub issues_url: String,
        pub login: String,
        pub members_url: String,
        pub node_id: String,
        pub public_members_url: String,
        pub repos_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SelectedActions {
        pub github_owned_allowed: Option<bool>,
        pub patterns_allowed: Option<Vec<String>>,
        pub verified_allowed: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RunnerGroupsEnterprise {
        pub allows_public_repositories: bool,
        pub default: bool,
        pub id: f64,
        pub name: String,
        pub runners_url: String,
        pub selected_organizations_url: Option<String>,
        pub visibility: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Labels {
        pub id: Option<i64>,
        pub name: Option<String>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Runner {
        pub busy: bool,
        pub id: i64,
        pub labels: Vec<Labels>,
        pub name: String,
        pub os: String,
        pub status: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RunnerApplication {
        pub architecture: String,
        pub download_url: String,
        pub filename: String,
        pub os: String,
        pub sha_256_checksum: Option<String>,
        pub temp_download_token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AuthenticationTokenPermissions {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AuthenticationToken {
        pub expires_at: DateTime<Utc>,
        pub permissions: Option<Permissions>,
        pub repositories: Option<Vec<Repository>>,
        pub repository_selection: Option<String>,
        pub single_file: Option<String>,
        pub token: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ActorLocation {
        pub country_name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Data {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AuditLogEvent {
        #[serde(rename = "@timestamp")]
        pub timestamp: Option<i64>,
        pub document_id: Option<String>,
        pub action: Option<String>,
        pub active: Option<bool>,
        pub active_was: Option<bool>,
        pub actor: Option<String>,
        pub actor_id: Option<i64>,
        pub actor_location: Option<ActorLocation>,
        pub blocked_user: Option<String>,
        pub business: Option<String>,
        pub config: Option<serde_json::Value>,
        pub config_was: Option<serde_json::Value>,
        pub content_type: Option<String>,
        pub created_at: Option<i64>,
        pub data: Option<Data>,
        pub deploy_key_fingerprint: Option<String>,
        pub emoji: Option<String>,
        pub events: Option<serde_json::Value>,
        pub events_were: Option<serde_json::Value>,
        pub explanation: Option<String>,
        pub fingerprint: Option<String>,
        pub hook_id: Option<i64>,
        pub limited_availability: Option<bool>,
        pub message: Option<String>,
        pub name: Option<String>,
        pub old_user: Option<String>,
        pub openssh_public_key: Option<String>,
        pub org: Option<String>,
        pub org_id: Option<i64>,
        pub previous_visibility: Option<String>,
        pub read_only: Option<bool>,
        pub repo: Option<String>,
        pub repository: Option<String>,
        pub repository_public: Option<bool>,
        pub target_login: Option<String>,
        pub team: Option<String>,
        pub transport_protocol: Option<i64>,
        pub transport_protocol_name: Option<String>,
        pub user: Option<String>,
        pub visibility: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MinutesUsedBreakdown {
        pub macos: Option<i64>,
        pub ubuntu: Option<i64>,
        pub windows: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ActionsBillingUsage {
        pub included_minutes: i64,
        pub minutes_used_breakdown: MinutesUsedBreakdown,
        pub total_minutes_used: i64,
        pub total_paid_minutes_used: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PackagesBillingUsage {
        pub included_gigabytes_bandwidth: i64,
        pub total_gigabytes_bandwidth_used: i64,
        pub total_paid_gigabytes_bandwidth_used: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CombinedBillingUsage {
        pub days_left_in_billing_cycle: i64,
        pub estimated_paid_storage_for_month: i64,
        pub estimated_storage_for_month: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Actor {
        pub avatar_url: String,
        pub display_login: Option<String>,
        pub gravatar_id: String,
        pub id: i64,
        pub login: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Label {
        pub color: String,
        pub default: bool,
        pub description: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Creator {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Milestone {
        pub closed_at: DateTime<Utc>,
        pub closed_issues: i64,
        pub created_at: DateTime<Utc>,
        pub creator: Creator,
        pub description: String,
        pub due_on: DateTime<Utc>,
        pub html_url: String,
        pub id: i64,
        pub labels_url: String,
        pub node_id: String,
        pub number: i64,
        pub open_issues: i64,
        pub state: String,
        pub title: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Assignee {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequest {
        pub diff_url: String,
        pub html_url: String,
        pub merged_at: Option<DateTime<Utc>>,
        pub patch_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PerformedviaGithubApp {
        pub client_id: Option<String>,
        pub client_secret: Option<String>,
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub events: Vec<String>,
        pub external_url: String,
        pub html_url: String,
        pub id: i64,
        pub installations_count: Option<i64>,
        pub name: String,
        pub node_id: String,
        pub owner: Owner,
        pub pem: Option<String>,
        pub permissions: Permissions,
        pub slug: Option<String>,
        pub updated_at: DateTime<Utc>,
        pub webhook_secret: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueSimple {
        pub active_lock_reason: Option<String>,
        pub assignee: Assignee,
        pub assignees: Option<Vec<SimpleUser>>,
        pub author_association: String,
        pub body: Option<String>,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub closed_at: DateTime<Utc>,
        pub comments: i64,
        pub comments_url: String,
        pub created_at: DateTime<Utc>,
        pub events_url: String,
        pub html_url: String,
        pub id: i64,
        pub labels: Vec<Label>,
        pub labels_url: String,
        pub locked: bool,
        pub milestone: Milestone,
        pub node_id: String,
        pub number: i64,
        pub performed_via_github_app: Option<PerformedviaGithubApp>,
        pub pull_request: Option<PullRequest>,
        pub repository: Option<Repository>,
        pub repository_url: String,
        pub state: String,
        pub timeline_url: Option<String>,
        pub title: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReactionRollup {
        #[serde(rename = "+1")]
        pub plus_one: i64,
        #[serde(rename = "-1")]
        pub minus_one: i64,
        pub confused: i64,
        pub eyes: i64,
        pub heart: i64,
        pub hooray: i64,
        pub laugh: i64,
        pub rocket: i64,
        pub total_count: i64,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueComment {
        pub author_association: String,
        pub body: Option<String>,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub created_at: DateTime<Utc>,
        pub html_url: String,
        pub id: i64,
        pub issue_url: String,
        pub node_id: String,
        pub performed_via_github_app: Option<PerformedviaGithubApp>,
        pub reactions: Option<ReactionRollup>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Repo {
        pub id: i64,
        pub name: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Pages {
        pub action: Option<String>,
        pub html_url: Option<String>,
        pub page_name: Option<String>,
        pub sha: Option<String>,
        pub summary: Option<String>,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Payload {
        pub action: Option<String>,
        pub comment: Option<IssueComment>,
        pub issue: Option<IssueSimple>,
        pub pages: Option<Vec<Pages>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Event {
        pub actor: Actor,
        pub created_at: DateTime<Utc>,
        pub id: String,
        pub org: Option<Actor>,
        pub payload: Payload,
        pub public: bool,
        pub repo: Repo,
        #[serde(rename = "type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LinkWithType {
        pub href: String,
        #[serde(rename = "type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Links {
        pub current_user: Option<LinkWithType>,
        pub current_user_actor: Option<LinkWithType>,
        pub current_user_organization: Option<LinkWithType>,
        pub current_user_organizations: Option<Vec<LinkWithType>>,
        pub current_user_public: Option<LinkWithType>,
        pub security_advisories: Option<LinkWithType>,
        pub timeline: LinkWithType,
        pub user: LinkWithType,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Feed {
        pub links: Links,
        pub current_user_actor_url: Option<String>,
        pub current_user_organization_url: Option<String>,
        pub current_user_organization_urls: Option<Vec<String>>,
        pub current_user_public_url: Option<String>,
        pub current_user_url: Option<String>,
        pub security_advisories_url: Option<String>,
        pub timeline_url: String,
        pub user_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Files {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct BaseGist {
        pub comments: i64,
        pub comments_url: String,
        pub commits_url: String,
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub files: Files,
        pub forks: Option<Vec<serde_json::Value>>,
        pub forks_url: String,
        pub git_pull_url: String,
        pub git_push_url: String,
        pub history: Option<Vec<serde_json::Value>>,
        pub html_url: String,
        pub id: String,
        pub node_id: String,
        pub owner: Option<Owner>,
        pub public: bool,
        pub truncated: Option<bool>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Plan {
        pub collaborators: i64,
        pub name: String,
        pub private_repos: i64,
        pub space: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PublicUser {
        pub avatar_url: String,
        pub bio: String,
        pub blog: String,
        pub collaborators: Option<i64>,
        pub company: String,
        pub created_at: DateTime<Utc>,
        pub disk_usage: Option<i64>,
        pub email: String,
        pub events_url: String,
        pub followers: i64,
        pub followers_url: String,
        pub following: i64,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub hireable: bool,
        pub html_url: String,
        pub id: i64,
        pub location: String,
        pub login: String,
        pub name: String,
        pub node_id: String,
        pub organizations_url: String,
        pub owned_private_repos: Option<i64>,
        pub plan: Option<Plan>,
        pub private_gists: Option<i64>,
        pub public_gists: i64,
        pub public_repos: i64,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        pub suspended_at: Option<DateTime<Utc>>,
        pub total_private_repos: Option<i64>,
        pub twitter_username: Option<String>,
        #[serde(rename = "type")]
        pub type_: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ChangeStatus {
        pub additions: Option<i64>,
        pub deletions: Option<i64>,
        pub total: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GistHistory {
        pub change_status: Option<ChangeStatus>,
        pub committed_at: Option<DateTime<Utc>>,
        pub url: Option<String>,
        pub user: Option<SimpleUser>,
        pub version: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Forks {
        pub created_at: Option<DateTime<Utc>>,
        pub id: Option<String>,
        pub updated_at: Option<DateTime<Utc>>,
        pub url: Option<String>,
        pub user: Option<PublicUser>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ForkOf {
        pub comments: i64,
        pub comments_url: String,
        pub commits_url: String,
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub files: Files,
        pub forks: Option<Vec<serde_json::Value>>,
        pub forks_url: String,
        pub git_pull_url: String,
        pub git_push_url: String,
        pub history: Option<Vec<serde_json::Value>>,
        pub html_url: String,
        pub id: String,
        pub node_id: String,
        pub owner: Option<Owner>,
        pub public: bool,
        pub truncated: Option<bool>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GistSimple {
        pub comments: Option<i64>,
        pub comments_url: Option<String>,
        pub commits_url: Option<String>,
        pub created_at: Option<String>,
        pub description: Option<String>,
        pub files: Option<Files>,
        pub fork_of: Option<ForkOf>,
        pub forks: Option<Vec<Forks>>,
        pub forks_url: Option<String>,
        pub git_pull_url: Option<String>,
        pub git_push_url: Option<String>,
        pub history: Option<Vec<GistHistory>>,
        pub html_url: Option<String>,
        pub id: Option<String>,
        pub node_id: Option<String>,
        pub owner: Option<SimpleUser>,
        pub public: Option<bool>,
        pub truncated: Option<bool>,
        pub updated_at: Option<String>,
        pub url: Option<String>,
        pub user: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GistComment {
        pub author_association: String,
        pub body: String,
        pub created_at: DateTime<Utc>,
        pub id: i64,
        pub node_id: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GistCommit {
        pub change_status: ChangeStatus,
        pub committed_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
        pub version: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitignoreTemplate {
        pub name: String,
        pub source: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ClosedBy {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Issue {
        pub active_lock_reason: Option<String>,
        pub assignee: Assignee,
        pub assignees: Option<Vec<SimpleUser>>,
        pub author_association: String,
        pub body: Option<String>,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub closed_at: DateTime<Utc>,
        pub closed_by: Option<ClosedBy>,
        pub comments: i64,
        pub comments_url: String,
        pub created_at: DateTime<Utc>,
        pub events_url: String,
        pub html_url: String,
        pub id: i64,
        pub labels: Vec<Labels>,
        pub labels_url: String,
        pub locked: bool,
        pub milestone: Milestone,
        pub node_id: String,
        pub number: i64,
        pub performed_via_github_app: Option<PerformedviaGithubApp>,
        pub pull_request: Option<PullRequest>,
        pub reactions: Option<ReactionRollup>,
        pub repository: Option<Repository>,
        pub repository_url: String,
        pub state: String,
        pub timeline_url: Option<String>,
        pub title: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LicenseData {
        pub body: String,
        pub conditions: Vec<String>,
        pub description: String,
        pub featured: bool,
        pub html_url: String,
        pub implementation: String,
        pub key: String,
        pub limitations: Vec<String>,
        pub name: String,
        pub node_id: String,
        pub permissions: Vec<String>,
        pub spdx_id: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MarketplaceListingPlan {
        pub accounts_url: String,
        pub bullets: Vec<String>,
        pub description: String,
        pub has_free_trial: bool,
        pub id: i64,
        pub monthly_price_in_cents: i64,
        pub name: String,
        pub number: i64,
        pub price_model: String,
        pub state: String,
        pub unit_name: String,
        pub url: String,
        pub yearly_price_in_cents: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MarketplacePendingChange {
        pub effective_date: Option<String>,
        pub id: Option<i64>,
        pub is_installed: Option<bool>,
        pub plan: Option<MarketplaceListingPlan>,
        pub unit_count: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MarketplacePurchase {
        pub billing_cycle: Option<String>,
        pub free_trial_ends_on: Option<String>,
        pub is_installed: Option<bool>,
        pub next_billing_date: Option<String>,
        pub on_free_trial: Option<bool>,
        pub plan: Option<MarketplaceListingPlan>,
        pub unit_count: Option<i64>,
        pub updated_at: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MarketplacePurchaseData {
        pub email: Option<String>,
        pub id: i64,
        pub login: String,
        pub marketplace_pending_change: Option<MarketplacePendingChange>,
        pub marketplace_purchase: MarketplacePurchase,
        pub organization_billing_email: Option<String>,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SshKeyFingerprints {
        pub sha256_dsa: Option<String>,
        pub sha256_rsa: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ApiOverview {
        pub actions: Option<Vec<String>>,
        pub api: Option<Vec<String>>,
        pub dependabot: Option<Vec<String>>,
        pub git: Option<Vec<String>>,
        pub hooks: Option<Vec<String>>,
        pub importer: Option<Vec<String>>,
        pub packages: Option<Vec<String>>,
        pub pages: Option<Vec<String>>,
        pub ssh_key_fingerprints: Option<SshKeyFingerprints>,
        pub verifiable_password_authentication: bool,
        pub web: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MinimalRepositoryOwner {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MinimalRepositoryPermissions {
        pub admin: Option<bool>,
        pub maintain: Option<bool>,
        pub pull: Option<bool>,
        pub push: Option<bool>,
        pub triage: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MinimalRepositoryTemplateRepository {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MinimalRepositoryLicense {
        pub key: Option<String>,
        pub name: Option<String>,
        pub node_id: Option<String>,
        pub spdx_id: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MinimalRepository {
        pub archive_url: String,
        pub archived: Option<bool>,
        pub assignees_url: String,
        pub blobs_url: String,
        pub branches_url: String,
        pub clone_url: Option<String>,
        pub code_of_conduct: Option<CodeofConduct>,
        pub collaborators_url: String,
        pub comments_url: String,
        pub commits_url: String,
        pub compare_url: String,
        pub contents_url: String,
        pub contributors_url: String,
        pub created_at: Option<DateTime<Utc>>,
        pub default_branch: Option<String>,
        pub delete_branch_on_merge: Option<bool>,
        pub deployments_url: String,
        pub description: String,
        pub disabled: Option<bool>,
        pub downloads_url: String,
        pub events_url: String,
        pub fork: bool,
        pub forks: Option<i64>,
        pub forks_count: Option<i64>,
        pub forks_url: String,
        pub full_name: String,
        pub git_commits_url: String,
        pub git_refs_url: String,
        pub git_tags_url: String,
        pub git_url: Option<String>,
        pub has_downloads: Option<bool>,
        pub has_issues: Option<bool>,
        pub has_pages: Option<bool>,
        pub has_projects: Option<bool>,
        pub has_wiki: Option<bool>,
        pub homepage: Option<String>,
        pub hooks_url: String,
        pub html_url: String,
        pub id: i64,
        pub is_template: Option<bool>,
        pub issue_comment_url: String,
        pub issue_events_url: String,
        pub issues_url: String,
        pub keys_url: String,
        pub labels_url: String,
        pub language: Option<String>,
        pub languages_url: String,
        pub license: Option<License>,
        pub merges_url: String,
        pub milestones_url: String,
        pub mirror_url: Option<String>,
        pub name: String,
        pub network_count: Option<i64>,
        pub node_id: String,
        pub notifications_url: String,
        pub open_issues: Option<i64>,
        pub open_issues_count: Option<i64>,
        pub owner: Owner,
        pub permissions: Option<Permissions>,
        pub private: bool,
        pub pulls_url: String,
        pub pushed_at: Option<DateTime<Utc>>,
        pub releases_url: String,
        pub size: Option<i64>,
        pub ssh_url: Option<String>,
        pub stargazers_count: Option<i64>,
        pub stargazers_url: String,
        pub statuses_url: String,
        pub subscribers_count: Option<i64>,
        pub subscribers_url: String,
        pub subscription_url: String,
        pub svn_url: Option<String>,
        pub tags_url: String,
        pub teams_url: String,
        pub temp_clone_token: Option<String>,
        pub template_repository: Option<TemplateRepository>,
        pub topics: Option<Vec<String>>,
        pub trees_url: String,
        pub updated_at: Option<DateTime<Utc>>,
        pub url: String,
        pub visibility: Option<String>,
        pub watchers: Option<i64>,
        pub watchers_count: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Subject {
        pub latest_comment_url: String,
        pub title: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Thread {
        pub id: String,
        pub last_read_at: String,
        pub reason: String,
        pub repository: MinimalRepository,
        pub subject: Subject,
        pub subscription_url: String,
        pub unread: bool,
        pub updated_at: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ThreadSubscription {
        pub created_at: DateTime<Utc>,
        pub ignored: bool,
        pub reason: String,
        pub repository_url: Option<String>,
        pub subscribed: bool,
        pub thread_url: Option<String>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct OrganizationFullPlan {
        pub filled_seats: Option<i64>,
        pub name: String,
        pub private_repos: i64,
        pub seats: Option<i64>,
        pub space: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct OrganizationFull {
        pub avatar_url: String,
        pub billing_email: Option<String>,
        pub blog: Option<String>,
        pub collaborators: Option<i64>,
        pub company: Option<String>,
        pub created_at: DateTime<Utc>,
        pub default_repository_permission: Option<String>,
        pub description: String,
        pub disk_usage: Option<i64>,
        pub email: Option<String>,
        pub events_url: String,
        pub followers: i64,
        pub following: i64,
        pub has_organization_projects: bool,
        pub has_repository_projects: bool,
        pub hooks_url: String,
        pub html_url: String,
        pub id: i64,
        pub is_verified: Option<bool>,
        pub issues_url: String,
        pub location: Option<String>,
        pub login: String,
        pub members_allowed_repository_creation_type: Option<String>,
        pub members_can_create_internal_repositories: Option<bool>,
        pub members_can_create_pages: Option<bool>,
        pub members_can_create_private_pages: Option<bool>,
        pub members_can_create_private_repositories: Option<bool>,
        pub members_can_create_public_pages: Option<bool>,
        pub members_can_create_public_repositories: Option<bool>,
        pub members_can_create_repositories: Option<bool>,
        pub members_url: String,
        pub name: Option<String>,
        pub node_id: String,
        pub owned_private_repos: Option<i64>,
        pub plan: Option<Plan>,
        pub private_gists: Option<i64>,
        pub public_gists: i64,
        pub public_members_url: String,
        pub public_repos: i64,
        pub repos_url: String,
        pub total_private_repos: Option<i64>,
        pub twitter_username: Option<String>,
        pub two_factor_requirement_enabled: Option<bool>,
        #[serde(rename = "type")]
        pub type_: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ActionsOrganizationPermissions {
        pub allowed_actions: Option<String>,
        pub enabled_repositories: String,
        pub selected_actions_url: Option<String>,
        pub selected_repositories_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RunnerGroupsOrg {
        pub allows_public_repositories: bool,
        pub default: bool,
        pub id: f64,
        pub inherited: bool,
        pub inherited_allows_public_repositories: Option<bool>,
        pub name: String,
        pub runners_url: String,
        pub selected_repositories_url: Option<String>,
        pub visibility: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct OrganizationActionsSecret {
        pub created_at: DateTime<Utc>,
        pub name: String,
        pub selected_repositories_url: Option<String>,
        pub updated_at: DateTime<Utc>,
        pub visibility: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ActionsPublicKey {
        pub created_at: Option<String>,
        pub id: Option<i64>,
        pub key: String,
        pub key_id: String,
        pub title: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct EmptyObject {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CredentialAuthorization {
        pub authorized_credential_id: Option<i64>,
        pub authorized_credential_note: Option<String>,
        pub authorized_credential_title: Option<String>,
        pub credential_accessed_at: Option<DateTime<Utc>>,
        pub credential_authorized_at: DateTime<Utc>,
        pub credential_id: i64,
        pub credential_type: String,
        pub fingerprint: Option<String>,
        pub login: String,
        pub scopes: Option<Vec<String>>,
        pub token_last_eight: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct OrganizationInvitation {
        pub created_at: String,
        pub email: String,
        pub failed_at: Option<String>,
        pub failed_reason: Option<String>,
        pub id: i64,
        pub invitation_teams_url: String,
        pub inviter: SimpleUser,
        pub login: String,
        pub node_id: String,
        pub role: String,
        pub team_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Config {
        pub content_type: Option<String>,
        pub insecure_ssl: Option<String>,
        pub secret: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct OrgHook {
        pub active: bool,
        pub config: Config,
        pub created_at: DateTime<Utc>,
        pub events: Vec<String>,
        pub id: i64,
        pub name: String,
        pub ping_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct InteractionLimitResponse {
        pub expires_at: DateTime<Utc>,
        pub limit: String,
        pub origin: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct InteractionLimit {
        pub expiry: Option<String>,
        pub limit: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamSimple {
        pub description: String,
        pub html_url: String,
        pub id: i64,
        pub ldap_dn: Option<String>,
        pub members_url: String,
        pub name: String,
        pub node_id: String,
        pub permission: String,
        pub privacy: Option<String>,
        pub repositories_url: String,
        pub slug: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamPermissions {
        pub admin: bool,
        pub maintain: bool,
        pub pull: bool,
        pub push: bool,
        pub triage: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Parent {
        pub description: String,
        pub html_url: String,
        pub id: i64,
        pub ldap_dn: Option<String>,
        pub members_url: String,
        pub name: String,
        pub node_id: String,
        pub permission: String,
        pub privacy: Option<String>,
        pub repositories_url: String,
        pub slug: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Team {
        pub description: String,
        pub html_url: String,
        pub id: i64,
        pub members_url: String,
        pub name: String,
        pub node_id: String,
        pub parent: Parent,
        pub permission: String,
        pub permissions: Option<Permissions>,
        pub privacy: Option<String>,
        pub repositories_url: String,
        pub slug: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct OrgMembershipPermissions {
        pub can_create_repository: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct OrgMembership {
        pub organization: OrganizationSimple,
        pub organization_url: String,
        pub permissions: Option<Permissions>,
        pub role: String,
        pub state: String,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Migration {
        pub archive_url: Option<String>,
        pub created_at: DateTime<Utc>,
        pub exclude: Option<Vec<serde_json::Value>>,
        pub exclude_attachments: bool,
        pub guid: String,
        pub id: i64,
        pub lock_repositories: bool,
        pub node_id: String,
        pub owner: Owner,
        pub repositories: Vec<Repository>,
        pub state: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PackageRepository {
        pub archive_url: String,
        pub archived: Option<bool>,
        pub assignees_url: String,
        pub blobs_url: String,
        pub branches_url: String,
        pub clone_url: Option<String>,
        pub code_of_conduct: Option<CodeofConduct>,
        pub collaborators_url: String,
        pub comments_url: String,
        pub commits_url: String,
        pub compare_url: String,
        pub contents_url: String,
        pub contributors_url: String,
        pub created_at: Option<DateTime<Utc>>,
        pub default_branch: Option<String>,
        pub delete_branch_on_merge: Option<bool>,
        pub deployments_url: String,
        pub description: String,
        pub disabled: Option<bool>,
        pub downloads_url: String,
        pub events_url: String,
        pub fork: bool,
        pub forks: Option<i64>,
        pub forks_count: Option<i64>,
        pub forks_url: String,
        pub full_name: String,
        pub git_commits_url: String,
        pub git_refs_url: String,
        pub git_tags_url: String,
        pub git_url: Option<String>,
        pub has_downloads: Option<bool>,
        pub has_issues: Option<bool>,
        pub has_pages: Option<bool>,
        pub has_projects: Option<bool>,
        pub has_wiki: Option<bool>,
        pub homepage: Option<String>,
        pub hooks_url: String,
        pub html_url: String,
        pub id: i64,
        pub is_template: Option<bool>,
        pub issue_comment_url: String,
        pub issue_events_url: String,
        pub issues_url: String,
        pub keys_url: String,
        pub labels_url: String,
        pub language: Option<String>,
        pub languages_url: String,
        pub license: Option<License>,
        pub merges_url: String,
        pub milestones_url: String,
        pub mirror_url: Option<String>,
        pub name: String,
        pub network_count: Option<i64>,
        pub node_id: String,
        pub notifications_url: String,
        pub open_issues: Option<i64>,
        pub open_issues_count: Option<i64>,
        pub owner: Owner,
        pub permissions: Option<Permissions>,
        pub private: bool,
        pub pulls_url: String,
        pub pushed_at: Option<DateTime<Utc>>,
        pub releases_url: String,
        pub size: Option<i64>,
        pub ssh_url: Option<String>,
        pub stargazers_count: Option<i64>,
        pub stargazers_url: String,
        pub statuses_url: String,
        pub subscribers_count: Option<i64>,
        pub subscribers_url: String,
        pub subscription_url: String,
        pub svn_url: Option<String>,
        pub tags_url: String,
        pub teams_url: String,
        pub temp_clone_token: Option<String>,
        pub template_repository: Option<TemplateRepository>,
        pub topics: Option<Vec<String>>,
        pub trees_url: String,
        pub updated_at: Option<DateTime<Utc>>,
        pub url: String,
        pub visibility: Option<String>,
        pub watchers: Option<i64>,
        pub watchers_count: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Package {
        pub created_at: DateTime<Utc>,
        pub html_url: String,
        pub id: i64,
        pub name: String,
        pub owner: Option<Owner>,
        pub package_type: String,
        pub repository: Option<Repository>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub version_count: i64,
        pub visibility: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Container {
        pub tags: serde_json::Value,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Docker {
        pub tag: Option<serde_json::Value>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Metadata {
        pub container: Option<Container>,
        pub docker: Option<Docker>,
        pub package_type: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PackageVersion {
        pub created_at: DateTime<Utc>,
        pub deleted_at: Option<DateTime<Utc>>,
        pub description: Option<String>,
        pub html_url: Option<String>,
        pub id: i64,
        pub license: Option<String>,
        pub metadata: Option<Metadata>,
        pub name: String,
        pub package_html_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Project {
        pub body: String,
        pub columns_url: String,
        pub created_at: DateTime<Utc>,
        pub creator: Creator,
        pub html_url: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub number: i64,
        pub organization_permission: Option<String>,
        pub owner_url: String,
        pub private: Option<bool>,
        pub state: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Groups {
        pub group_description: String,
        pub group_id: String,
        pub group_name: String,
        pub status: Option<String>,
        pub synced_at: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GroupMapping {
        pub groups: Option<Vec<Groups>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamFull {
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub html_url: String,
        pub id: i64,
        pub ldap_dn: Option<String>,
        pub members_count: i64,
        pub members_url: String,
        pub name: String,
        pub node_id: String,
        pub organization: OrganizationFull,
        pub parent: Option<Parent>,
        pub permission: String,
        pub privacy: Option<String>,
        pub repos_count: i64,
        pub repositories_url: String,
        pub slug: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Author {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamDiscussion {
        pub author: Author,
        pub body: String,
        pub body_html: String,
        pub body_version: String,
        pub comments_count: i64,
        pub comments_url: String,
        pub created_at: DateTime<Utc>,
        pub html_url: String,
        pub last_edited_at: DateTime<Utc>,
        pub node_id: String,
        pub number: i64,
        pub pinned: bool,
        pub private: bool,
        pub reactions: Option<ReactionRollup>,
        pub team_url: String,
        pub title: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamDiscussionComment {
        pub author: Author,
        pub body: String,
        pub body_html: String,
        pub body_version: String,
        pub created_at: DateTime<Utc>,
        pub discussion_url: String,
        pub html_url: String,
        pub last_edited_at: DateTime<Utc>,
        pub node_id: String,
        pub number: i64,
        pub reactions: Option<ReactionRollup>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Reaction {
        pub content: String,
        pub created_at: DateTime<Utc>,
        pub id: i64,
        pub node_id: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamMembership {
        pub role: String,
        pub state: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamProjectPermissions {
        pub admin: bool,
        pub read: bool,
        pub write: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamProject {
        pub body: String,
        pub columns_url: String,
        pub created_at: String,
        pub creator: SimpleUser,
        pub html_url: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub number: i64,
        pub organization_permission: Option<String>,
        pub owner_url: String,
        pub permissions: Permissions,
        pub private: Option<bool>,
        pub state: String,
        pub updated_at: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamRepositoryPermissions {
        pub admin: bool,
        pub maintain: Option<bool>,
        pub pull: bool,
        pub push: bool,
        pub triage: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamRepositoryTemplateRepository {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TeamRepository {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub archive_url: String,
        pub archived: bool,
        pub assignees_url: String,
        pub blobs_url: String,
        pub branches_url: String,
        pub clone_url: String,
        pub collaborators_url: String,
        pub comments_url: String,
        pub commits_url: String,
        pub compare_url: String,
        pub contents_url: String,
        pub contributors_url: String,
        pub created_at: DateTime<Utc>,
        pub default_branch: String,
        pub delete_branch_on_merge: Option<bool>,
        pub deployments_url: String,
        pub description: String,
        pub disabled: bool,
        pub downloads_url: String,
        pub events_url: String,
        pub fork: bool,
        pub forks: i64,
        pub forks_count: i64,
        pub forks_url: String,
        pub full_name: String,
        pub git_commits_url: String,
        pub git_refs_url: String,
        pub git_tags_url: String,
        pub git_url: String,
        pub has_downloads: bool,
        pub has_issues: bool,
        pub has_pages: bool,
        pub has_projects: bool,
        pub has_wiki: bool,
        pub homepage: String,
        pub hooks_url: String,
        pub html_url: String,
        pub id: i64,
        pub is_template: Option<bool>,
        pub issue_comment_url: String,
        pub issue_events_url: String,
        pub issues_url: String,
        pub keys_url: String,
        pub labels_url: String,
        pub language: String,
        pub languages_url: String,
        pub license: License,
        pub master_branch: Option<String>,
        pub merges_url: String,
        pub milestones_url: String,
        pub mirror_url: String,
        pub name: String,
        pub network_count: Option<i64>,
        pub node_id: String,
        pub notifications_url: String,
        pub open_issues: i64,
        pub open_issues_count: i64,
        pub owner: Owner,
        pub permissions: Option<Permissions>,
        pub private: bool,
        pub pulls_url: String,
        pub pushed_at: DateTime<Utc>,
        pub releases_url: String,
        pub size: i64,
        pub ssh_url: String,
        pub stargazers_count: i64,
        pub stargazers_url: String,
        pub statuses_url: String,
        pub subscribers_count: Option<i64>,
        pub subscribers_url: String,
        pub subscription_url: String,
        pub svn_url: String,
        pub tags_url: String,
        pub teams_url: String,
        pub temp_clone_token: Option<String>,
        pub template_repository: Option<TemplateRepository>,
        pub topics: Option<Vec<String>>,
        pub trees_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub visibility: Option<String>,
        pub watchers: i64,
        pub watchers_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProjectCard {
        pub archived: Option<bool>,
        pub column_name: Option<String>,
        pub column_url: String,
        pub content_url: Option<String>,
        pub created_at: DateTime<Utc>,
        pub creator: Creator,
        pub id: i64,
        pub node_id: String,
        pub note: String,
        pub project_id: Option<String>,
        pub project_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProjectColumn {
        pub cards_url: String,
        pub created_at: DateTime<Utc>,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub project_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RepositoryCollaboratorPermission {
        pub permission: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RateLimit {
        pub limit: i64,
        pub remaining: i64,
        pub reset: i64,
        pub used: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Resources {
        pub code_scanning_upload: Option<RateLimit>,
        pub core: RateLimit,
        pub graphql: Option<RateLimit>,
        pub integration_manifest: Option<RateLimit>,
        pub search: RateLimit,
        pub source_import: Option<RateLimit>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RateLimitOverview {
        pub rate: RateLimit,
        pub resources: Resources,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeofConductSimple {
        pub html_url: String,
        pub key: String,
        pub name: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FullRepositoryPermissions {
        pub admin: bool,
        pub pull: bool,
        pub push: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FullRepositoryTemplateRepository {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AdvancedSecurity {
        pub status: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SecretScanning {
        pub status: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SecurityandAnalysis {
        pub advanced_security: Option<AdvancedSecurity>,
        pub secret_scanning: Option<SecretScanning>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FullRepository {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub anonymous_access_enabled: Option<bool>,
        pub archive_url: String,
        pub archived: bool,
        pub assignees_url: String,
        pub blobs_url: String,
        pub branches_url: String,
        pub clone_url: String,
        pub code_of_conduct: Option<CodeofConductSimple>,
        pub collaborators_url: String,
        pub comments_url: String,
        pub commits_url: String,
        pub compare_url: String,
        pub contents_url: String,
        pub contributors_url: String,
        pub created_at: DateTime<Utc>,
        pub default_branch: String,
        pub delete_branch_on_merge: Option<bool>,
        pub deployments_url: String,
        pub description: String,
        pub disabled: bool,
        pub downloads_url: String,
        pub events_url: String,
        pub fork: bool,
        pub forks: i64,
        pub forks_count: i64,
        pub forks_url: String,
        pub full_name: String,
        pub git_commits_url: String,
        pub git_refs_url: String,
        pub git_tags_url: String,
        pub git_url: String,
        pub has_downloads: bool,
        pub has_issues: bool,
        pub has_pages: bool,
        pub has_projects: bool,
        pub has_wiki: bool,
        pub homepage: String,
        pub hooks_url: String,
        pub html_url: String,
        pub id: i64,
        pub is_template: Option<bool>,
        pub issue_comment_url: String,
        pub issue_events_url: String,
        pub issues_url: String,
        pub keys_url: String,
        pub labels_url: String,
        pub language: String,
        pub languages_url: String,
        pub license: License,
        pub master_branch: Option<String>,
        pub merges_url: String,
        pub milestones_url: String,
        pub mirror_url: String,
        pub name: String,
        pub network_count: i64,
        pub node_id: String,
        pub notifications_url: String,
        pub open_issues: i64,
        pub open_issues_count: i64,
        pub organization: Option<Organization>,
        pub owner: SimpleUser,
        pub parent: Option<Repository>,
        pub permissions: Option<Permissions>,
        pub private: bool,
        pub pulls_url: String,
        pub pushed_at: DateTime<Utc>,
        pub releases_url: String,
        pub security_and_analysis: Option<SecurityandAnalysis>,
        pub size: i64,
        pub source: Option<Repository>,
        pub ssh_url: String,
        pub stargazers_count: i64,
        pub stargazers_url: String,
        pub statuses_url: String,
        pub subscribers_count: i64,
        pub subscribers_url: String,
        pub subscription_url: String,
        pub svn_url: String,
        pub tags_url: String,
        pub teams_url: String,
        pub temp_clone_token: Option<String>,
        pub template_repository: Option<TemplateRepository>,
        pub topics: Option<Vec<String>>,
        pub trees_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub visibility: Option<String>,
        pub watchers: i64,
        pub watchers_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Artifact {
        pub archive_download_url: String,
        pub created_at: DateTime<Utc>,
        pub expired: bool,
        pub expires_at: DateTime<Utc>,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub size_in_bytes: i64,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Steps {
        pub completed_at: Option<DateTime<Utc>>,
        pub conclusion: String,
        pub name: String,
        pub number: i64,
        pub started_at: Option<DateTime<Utc>>,
        pub status: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Job {
        pub check_run_url: String,
        pub completed_at: DateTime<Utc>,
        pub conclusion: String,
        pub head_sha: String,
        pub html_url: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub run_id: i64,
        pub run_url: String,
        pub started_at: DateTime<Utc>,
        pub status: String,
        pub steps: Option<Vec<Steps>>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ActionsRepositoryPermissions {
        pub allowed_actions: String,
        pub enabled: bool,
        pub selected_actions_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Head {
        #[serde(rename = "ref")]
        pub ref_: String,
        pub repo: Repo,
        pub sha: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Base {
        #[serde(rename = "ref")]
        pub ref_: String,
        pub repo: Repo,
        pub sha: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestMinimal {
        pub base: Base,
        pub head: Head,
        pub id: i64,
        pub number: i64,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SimpleCommitAuthor {
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Committer {
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SimpleCommit {
        pub author: Author,
        pub committer: Committer,
        pub id: String,
        pub message: String,
        pub timestamp: DateTime<Utc>,
        pub tree_id: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct HeadCommit {
        pub author: Author,
        pub committer: Committer,
        pub id: String,
        pub message: String,
        pub timestamp: DateTime<Utc>,
        pub tree_id: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct WorkflowRun {
        pub artifacts_url: String,
        pub cancel_url: String,
        pub check_suite_id: Option<i64>,
        pub check_suite_node_id: Option<String>,
        pub check_suite_url: String,
        pub conclusion: String,
        pub created_at: DateTime<Utc>,
        pub event: String,
        pub head_branch: String,
        pub head_commit: HeadCommit,
        pub head_repository: MinimalRepository,
        pub head_repository_id: Option<i64>,
        pub head_sha: String,
        pub html_url: String,
        pub id: i64,
        pub jobs_url: String,
        pub logs_url: String,
        pub name: Option<String>,
        pub node_id: String,
        pub pull_requests: Vec<PullRequestMinimal>,
        pub repository: MinimalRepository,
        pub rerun_url: String,
        pub run_number: i64,
        pub status: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub workflow_id: i64,
        pub workflow_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Environments {
        pub created_at: Option<DateTime<Utc>>,
        pub html_url: Option<String>,
        pub id: Option<i64>,
        pub name: Option<String>,
        pub node_id: Option<String>,
        pub updated_at: Option<DateTime<Utc>>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct EnvironmentApprovals {
        pub comment: String,
        pub environments: Vec<Environments>,
        pub state: String,
        pub user: SimpleUser,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Environment {
        pub html_url: Option<String>,
        pub id: Option<i64>,
        pub name: Option<String>,
        pub node_id: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Reviewer {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Reviewers {
        pub reviewer: Option<Reviewer>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PendingDeployment {
        pub current_user_can_approve: bool,
        pub environment: Environment,
        pub reviewers: Vec<Reviewers>,
        pub wait_timer: i64,
        pub wait_timer_started_at: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PayloadData {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Deployment {
        pub created_at: DateTime<Utc>,
        pub creator: Creator,
        pub description: String,
        pub environment: String,
        pub id: i64,
        pub node_id: String,
        pub original_environment: Option<String>,
        pub payload: Payload,
        pub performed_via_github_app: Option<PerformedviaGithubApp>,
        pub production_environment: Option<bool>,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub repository_url: String,
        pub sha: String,
        pub statuses_url: String,
        pub task: String,
        pub transient_environment: Option<bool>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Ubuntu {
        pub jobs: i64,
        pub total_ms: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Macos {
        pub jobs: i64,
        pub total_ms: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Windows {
        pub jobs: i64,
        pub total_ms: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Billable {
        pub macos: Option<Macos>,
        pub ubuntu: Option<Ubuntu>,
        pub windows: Option<Windows>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct WorkflowRunUsage {
        pub billable: Billable,
        pub run_duration_ms: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ActionsSecret {
        pub created_at: DateTime<Utc>,
        pub name: String,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Workflow {
        pub badge_url: String,
        pub created_at: DateTime<Utc>,
        pub deleted_at: Option<DateTime<Utc>>,
        pub html_url: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub path: String,
        pub state: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct WorkflowUsageBillableBillableUbuntu {
        pub total_ms: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct WorkflowUsageBillableBillableMacos {
        pub total_ms: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct WorkflowUsageBillableBillableWindows {
        pub total_ms: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct WorkflowUsage {
        pub billable: Billable,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProtectedBranchAdminEnforced {
        pub enabled: bool,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DismissalRestrictions {
        pub teams: Option<Vec<Team>>,
        pub teams_url: Option<String>,
        pub url: Option<String>,
        pub users: Option<Vec<SimpleUser>>,
        pub users_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProtectedBranchPullRequestReview {
        pub dismiss_stale_reviews: bool,
        pub dismissal_restrictions: Option<DismissalRestrictions>,
        pub require_code_owner_reviews: bool,
        pub required_approving_review_count: Option<i64>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Users {
        pub avatar_url: Option<String>,
        pub events_url: Option<String>,
        pub followers_url: Option<String>,
        pub following_url: Option<String>,
        pub gists_url: Option<String>,
        pub gravatar_id: Option<String>,
        pub html_url: Option<String>,
        pub id: Option<i64>,
        pub login: Option<String>,
        pub node_id: Option<String>,
        pub organizations_url: Option<String>,
        pub received_events_url: Option<String>,
        pub repos_url: Option<String>,
        pub site_admin: Option<bool>,
        pub starred_url: Option<String>,
        pub subscriptions_url: Option<String>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Teams {
        pub description: Option<String>,
        pub html_url: Option<String>,
        pub id: Option<i64>,
        pub members_url: Option<String>,
        pub name: Option<String>,
        pub node_id: Option<String>,
        pub parent: Option<String>,
        pub permission: Option<String>,
        pub privacy: Option<String>,
        pub repositories_url: Option<String>,
        pub slug: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct BranchRestrictionPolicyAppsAppsOwner {
        pub avatar_url: Option<String>,
        pub description: Option<String>,
        pub events_url: Option<String>,
        pub followers_url: Option<String>,
        pub following_url: Option<String>,
        pub gists_url: Option<String>,
        pub gravatar_id: Option<String>,
        pub hooks_url: Option<String>,
        pub html_url: Option<String>,
        pub id: Option<i64>,
        pub issues_url: Option<String>,
        pub login: Option<String>,
        pub members_url: Option<String>,
        pub node_id: Option<String>,
        pub organizations_url: Option<String>,
        pub public_members_url: Option<String>,
        pub received_events_url: Option<String>,
        pub repos_url: Option<String>,
        pub site_admin: Option<bool>,
        pub starred_url: Option<String>,
        pub subscriptions_url: Option<String>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct BranchRestrictionPolicyAppsAppsPermissions {
        pub contents: Option<String>,
        pub issues: Option<String>,
        pub metadata: Option<String>,
        pub single_file: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Apps {
        pub created_at: Option<String>,
        pub description: Option<String>,
        pub events: Option<Vec<String>>,
        pub external_url: Option<String>,
        pub html_url: Option<String>,
        pub id: Option<i64>,
        pub name: Option<String>,
        pub node_id: Option<String>,
        pub owner: Option<Owner>,
        pub permissions: Option<Permissions>,
        pub slug: Option<String>,
        pub updated_at: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct BranchRestrictionPolicy {
        pub apps: Vec<Apps>,
        pub apps_url: String,
        pub teams: Vec<Teams>,
        pub teams_url: String,
        pub url: String,
        pub users: Vec<Users>,
        pub users_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RequiredStatusChecks {
        pub contexts: Vec<String>,
        pub contexts_url: Option<String>,
        pub enforcement_level: Option<String>,
        pub strict: Option<bool>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RequiredLinearHistory {
        pub enabled: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AllowForcePushes {
        pub enabled: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AllowDeletions {
        pub enabled: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RequiredConversationResolution {
        pub enabled: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RequiredSignatures {
        pub enabled: bool,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct BranchProtection {
        pub allow_deletions: Option<AllowDeletions>,
        pub allow_force_pushes: Option<AllowForcePushes>,
        pub enabled: Option<bool>,
        pub enforce_admins: Option<ProtectedBranchAdminEnforced>,
        pub name: Option<String>,
        pub protection_url: Option<String>,
        pub required_conversation_resolution: Option<RequiredConversationResolution>,
        pub required_linear_history: Option<RequiredLinearHistory>,
        pub required_pull_request_reviews: Option<ProtectedBranchPullRequestReview>,
        pub required_signatures: Option<RequiredSignatures>,
        pub required_status_checks: Option<RequiredStatusChecks>,
        pub restrictions: Option<BranchRestrictionPolicy>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Commit {
        pub sha: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ShortBranch {
        pub commit: Commit,
        pub name: String,
        pub protected: bool,
        pub protection: Option<BranchProtection>,
        pub protection_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitUser {
        pub date: Option<String>,
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Verification {
        pub payload: String,
        pub reason: String,
        pub signature: String,
        pub verified: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitCommitCommitAuthor {
        pub date: Option<String>,
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitCommitCommitCommitter {
        pub date: Option<String>,
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Tree {
        pub sha: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitCommit {
        pub author: Author,
        pub comment_count: i64,
        pub committer: Committer,
        pub message: String,
        pub tree: Tree,
        pub url: String,
        pub verification: Option<Verification>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitCommitter {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Parents {
        pub html_url: Option<String>,
        pub sha: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Stats {
        pub additions: Option<i64>,
        pub deletions: Option<i64>,
        pub total: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitFiles {
        pub additions: Option<i64>,
        pub blob_url: Option<String>,
        pub changes: Option<i64>,
        pub contents_url: Option<String>,
        pub deletions: Option<i64>,
        pub filename: Option<String>,
        pub patch: Option<String>,
        pub previous_filename: Option<String>,
        pub raw_url: Option<String>,
        pub sha: Option<String>,
        pub status: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitData {
        pub author: Author,
        pub comments_url: String,
        pub commit: Commit,
        pub committer: Committer,
        pub files: Option<Vec<Files>>,
        pub html_url: String,
        pub node_id: String,
        pub parents: Vec<Parents>,
        pub sha: String,
        pub stats: Option<Stats>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct BranchWithProtectionLinks {
        pub html: String,
        #[serde(rename = "self")]
        pub self_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct BranchWithProtection {
        pub links: Links,
        pub commit: Commit,
        pub name: String,
        pub pattern: Option<String>,
        pub protected: bool,
        pub protection: BranchProtection,
        pub protection_url: String,
        pub required_approving_review_count: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct StatusCheckPolicy {
        pub contexts: Vec<String>,
        pub contexts_url: String,
        pub strict: bool,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProtectedBranchRequiredPullRequestReviewsRequiredPullRequestReviewsDismissalRestrictions
    {
        pub teams: Vec<Team>,
        pub teams_url: String,
        pub url: String,
        pub users: Vec<SimpleUser>,
        pub users_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RequiredPullRequestReviews {
        pub dismiss_stale_reviews: Option<bool>,
        pub dismissal_restrictions: Option<DismissalRestrictions>,
        pub require_code_owner_reviews: Option<bool>,
        pub required_approving_review_count: Option<i64>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct EnforceAdmins {
        pub enabled: bool,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProtectedBranchRequiredLinearHistory {
        pub enabled: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProtectedBranchAllowForcePushes {
        pub enabled: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProtectedBranchAllowDeletions {
        pub enabled: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProtectedBranch {
        pub allow_deletions: Option<AllowDeletions>,
        pub allow_force_pushes: Option<AllowForcePushes>,
        pub enforce_admins: Option<EnforceAdmins>,
        pub required_conversation_resolution: Option<RequiredConversationResolution>,
        pub required_linear_history: Option<RequiredLinearHistory>,
        pub required_pull_request_reviews: Option<RequiredPullRequestReviews>,
        pub required_signatures: Option<RequiredSignatures>,
        pub required_status_checks: Option<StatusCheckPolicy>,
        pub restrictions: Option<BranchRestrictionPolicy>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeploymentSimple {
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub environment: String,
        pub id: i64,
        pub node_id: String,
        pub original_environment: Option<String>,
        pub performed_via_github_app: Option<PerformedviaGithubApp>,
        pub production_environment: Option<bool>,
        pub repository_url: String,
        pub statuses_url: String,
        pub task: String,
        pub transient_environment: Option<bool>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Output {
        pub annotations_count: i64,
        pub annotations_url: String,
        pub summary: String,
        pub text: String,
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CheckSuite {
        pub id: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CheckRunApp {
        pub client_id: Option<String>,
        pub client_secret: Option<String>,
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub events: Vec<String>,
        pub external_url: String,
        pub html_url: String,
        pub id: i64,
        pub installations_count: Option<i64>,
        pub name: String,
        pub node_id: String,
        pub owner: Owner,
        pub pem: Option<String>,
        pub permissions: Permissions,
        pub slug: Option<String>,
        pub updated_at: DateTime<Utc>,
        pub webhook_secret: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CheckRun {
        pub app: App,
        pub check_suite: CheckSuite,
        pub completed_at: DateTime<Utc>,
        pub conclusion: String,
        pub deployment: Option<DeploymentSimple>,
        pub details_url: String,
        pub external_id: String,
        pub head_sha: String,
        pub html_url: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub output: Output,
        pub pull_requests: serde_json::Value,
        pub started_at: DateTime<Utc>,
        pub status: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CheckAnnotation {
        pub annotation_level: String,
        pub blob_href: String,
        pub end_column: i64,
        pub end_line: i64,
        pub message: String,
        pub path: String,
        pub raw_details: String,
        pub start_column: i64,
        pub start_line: i64,
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CheckSuiteApp {
        pub client_id: Option<String>,
        pub client_secret: Option<String>,
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub events: Vec<String>,
        pub external_url: String,
        pub html_url: String,
        pub id: i64,
        pub installations_count: Option<i64>,
        pub name: String,
        pub node_id: String,
        pub owner: Owner,
        pub pem: Option<String>,
        pub permissions: Permissions,
        pub slug: Option<String>,
        pub updated_at: DateTime<Utc>,
        pub webhook_secret: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CheckSuiteData {
        pub after: String,
        pub app: App,
        pub before: String,
        pub check_runs_url: String,
        pub conclusion: String,
        pub created_at: DateTime<Utc>,
        pub head_branch: String,
        pub head_commit: SimpleCommit,
        pub head_sha: String,
        pub id: i64,
        pub latest_check_runs_count: i64,
        pub node_id: String,
        pub pull_requests: Vec<PullRequestMinimal>,
        pub repository: MinimalRepository,
        pub status: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AutoTriggerChecks {
        pub app_id: i64,
        pub setting: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Preferences {
        pub auto_trigger_checks: Option<Vec<AutoTriggerChecks>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CheckSuitePreference {
        pub preferences: Preferences,
        pub repository: MinimalRepository,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningAlertRuleSummary {
        pub description: Option<String>,
        pub id: Option<String>,
        pub name: Option<String>,
        pub severity: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningAnalysisTool {
        pub guid: Option<String>,
        pub name: Option<String>,
        pub version: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningAlertLocation {
        pub end_column: Option<i64>,
        pub end_line: Option<i64>,
        pub path: Option<String>,
        pub start_column: Option<i64>,
        pub start_line: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Message {
        pub text: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningAlertInstance {
        pub analysis_key: Option<String>,
        pub classifications: Option<Vec<String>>,
        pub commit_sha: Option<String>,
        pub environment: Option<String>,
        pub html_url: Option<String>,
        pub location: Option<CodeScanningAlertLocation>,
        pub message: Option<Message>,
        #[serde(rename = "ref")]
        pub ref_: Option<String>,
        pub state: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningAlertItems {
        pub created_at: DateTime<Utc>,
        pub dismissed_at: DateTime<Utc>,
        pub dismissed_by: SimpleUser,
        pub dismissed_reason: serde_json::Value,
        pub html_url: String,
        pub instances_url: String,
        pub most_recent_instance: CodeScanningAlertInstance,
        pub number: i64,
        pub rule: CodeScanningAlertRuleSummary,
        pub state: String,
        pub tool: CodeScanningAnalysisTool,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningAlertRule {
        pub description: Option<String>,
        pub full_description: Option<String>,
        pub help: Option<String>,
        pub id: Option<String>,
        pub name: Option<String>,
        pub security_severity_level: Option<String>,
        pub severity: Option<String>,
        pub tags: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningAlert {
        pub created_at: DateTime<Utc>,
        pub dismissed_at: DateTime<Utc>,
        pub dismissed_by: SimpleUser,
        pub dismissed_reason: serde_json::Value,
        pub html_url: String,
        pub instances: Option<serde_json::Value>,
        pub instances_url: String,
        pub most_recent_instance: CodeScanningAlertInstance,
        pub number: i64,
        pub rule: CodeScanningAlertRule,
        pub state: String,
        pub tool: CodeScanningAnalysisTool,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningAnalysis {
        pub analysis_key: String,
        pub category: Option<String>,
        pub commit_sha: String,
        pub created_at: DateTime<Utc>,
        pub deletable: bool,
        pub environment: String,
        pub error: String,
        pub id: i64,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub results_count: i64,
        pub rules_count: i64,
        pub sarif_id: String,
        pub tool: CodeScanningAnalysisTool,
        pub tool_name: Option<String>,
        pub url: String,
        pub warning: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningAnalysisDeletion {
        pub confirm_delete_url: String,
        pub next_analysis_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimError {
        pub detail: Option<String>,
        pub documentation_url: Option<String>,
        pub message: Option<String>,
        pub schemas: Option<Vec<String>>,
        pub scim_type: Option<String>,
        pub status: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningSarifsReceipt {
        pub id: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeScanningSarifsStatus {
        pub analyses_url: Option<String>,
        pub processing_status: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CollaboratorPermissions {
        pub admin: bool,
        pub pull: bool,
        pub push: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Collaborator {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub permissions: Option<Permissions>,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Invitee {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Inviter {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RepositoryInvitation {
        pub created_at: DateTime<Utc>,
        pub expired: Option<bool>,
        pub html_url: String,
        pub id: i64,
        pub invitee: Invitee,
        pub inviter: Inviter,
        pub node_id: String,
        pub permissions: String,
        pub repository: MinimalRepository,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitComment {
        pub author_association: String,
        pub body: String,
        pub commit_id: String,
        pub created_at: DateTime<Utc>,
        pub html_url: String,
        pub id: i64,
        pub line: i64,
        pub node_id: String,
        pub path: String,
        pub position: i64,
        pub reactions: Option<ReactionRollup>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct BranchShort {
        pub commit: Commit,
        pub name: String,
        pub protected: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Link {
        pub href: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AutoMerge {
        pub commit_message: String,
        pub commit_title: String,
        pub enabled_by: SimpleUser,
        pub merge_method: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestSimpleLabels {
        pub color: Option<String>,
        pub default: Option<bool>,
        pub description: Option<String>,
        pub id: Option<i64>,
        pub name: Option<String>,
        pub node_id: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestSimpleHead {
        pub label: String,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub repo: Repository,
        pub sha: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestSimpleBase {
        pub label: String,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub repo: Repository,
        pub sha: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestSimpleLinks {
        pub comments: Link,
        pub commits: Link,
        pub html: Link,
        pub issue: Link,
        pub review_comment: Link,
        pub review_comments: Link,
        #[serde(rename = "self")]
        pub self_: Link,
        pub statuses: Link,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestSimple {
        pub links: Links,
        pub active_lock_reason: Option<String>,
        pub assignee: Assignee,
        pub assignees: Option<Vec<SimpleUser>>,
        pub author_association: String,
        pub auto_merge: AutoMerge,
        pub base: Base,
        pub body: String,
        pub closed_at: DateTime<Utc>,
        pub comments_url: String,
        pub commits_url: String,
        pub created_at: DateTime<Utc>,
        pub diff_url: String,
        pub draft: Option<bool>,
        pub head: Head,
        pub html_url: String,
        pub id: i64,
        pub issue_url: String,
        pub labels: Vec<Labels>,
        pub locked: bool,
        pub merge_commit_sha: String,
        pub merged_at: DateTime<Utc>,
        pub milestone: Milestone,
        pub node_id: String,
        pub number: i64,
        pub patch_url: String,
        pub requested_reviewers: Option<Vec<SimpleUser>>,
        pub requested_teams: Option<Vec<Team>>,
        pub review_comment_url: String,
        pub review_comments_url: String,
        pub state: String,
        pub statuses_url: String,
        pub title: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SimpleCommitStatus {
        pub avatar_url: String,
        pub context: String,
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub id: i64,
        pub node_id: String,
        pub required: Option<bool>,
        pub state: String,
        pub target_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CombinedCommitStatus {
        pub commit_url: String,
        pub repository: MinimalRepository,
        pub sha: String,
        pub state: String,
        pub statuses: Vec<SimpleCommitStatus>,
        pub total_count: i64,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Status {
        pub avatar_url: String,
        pub context: String,
        pub created_at: String,
        pub creator: SimpleUser,
        pub description: String,
        pub id: i64,
        pub node_id: String,
        pub state: String,
        pub target_url: String,
        pub updated_at: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommunityHealthFile {
        pub html_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommunityProfileFilesFilesCodeofConduct {
        pub html_url: String,
        pub key: String,
        pub name: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeofConductFile {
        pub html_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Contributing {
        pub html_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Readme {
        pub html_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueTemplate {
        pub html_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestTemplate {
        pub html_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommunityProfileFiles {
        pub code_of_conduct: CodeofConduct,
        pub code_of_conduct_file: CodeofConductFile,
        pub contributing: Contributing,
        pub issue_template: IssueTemplate,
        pub license: License,
        pub pull_request_template: PullRequestTemplate,
        pub readme: Readme,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommunityProfile {
        pub content_reports_enabled: Option<bool>,
        pub description: String,
        pub documentation: String,
        pub files: Files,
        pub health_percentage: i64,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DiffEntry {
        pub additions: i64,
        pub blob_url: String,
        pub changes: i64,
        pub contents_url: String,
        pub deletions: i64,
        pub filename: String,
        pub patch: Option<String>,
        pub previous_filename: Option<String>,
        pub raw_url: String,
        pub sha: String,
        pub status: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitComparison {
        pub ahead_by: i64,
        pub base_commit: Commit,
        pub behind_by: i64,
        pub commits: Vec<Commit>,
        pub diff_url: String,
        pub files: Option<Vec<DiffEntry>>,
        pub html_url: String,
        pub merge_base_commit: Commit,
        pub patch_url: String,
        pub permalink_url: String,
        pub status: String,
        pub total_commits: i64,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentReferenceAttachment {
        pub body: String,
        pub id: i64,
        pub node_id: Option<String>,
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentTreeEntriesEntriesLinks {
        pub git: String,
        pub html: String,
        #[serde(rename = "self")]
        pub self_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Entries {
        pub links: Links,
        pub content: Option<String>,
        pub download_url: String,
        pub git_url: String,
        pub html_url: String,
        pub name: String,
        pub path: String,
        pub sha: String,
        pub size: i64,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentTreeLinks {
        pub git: String,
        pub html: String,
        #[serde(rename = "self")]
        pub self_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentTree {
        pub links: Links,
        pub download_url: String,
        pub entries: Option<Vec<Entries>>,
        pub git_url: String,
        pub html_url: String,
        pub name: String,
        pub path: String,
        pub sha: String,
        pub size: i64,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentDirectoryLinks {
        pub git: String,
        pub html: String,
        #[serde(rename = "self")]
        pub self_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentDirectory {
        pub links: Links,
        pub content: Option<String>,
        pub download_url: String,
        pub git_url: String,
        pub html_url: String,
        pub name: String,
        pub path: String,
        pub sha: String,
        pub size: i64,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentFileLinks {
        pub git: String,
        pub html: String,
        #[serde(rename = "self")]
        pub self_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentFile {
        pub links: Links,
        pub content: String,
        pub download_url: String,
        pub encoding: String,
        pub git_url: String,
        pub html_url: String,
        pub name: String,
        pub path: String,
        pub sha: String,
        pub size: i64,
        pub submodule_git_url: Option<String>,
        pub target: Option<String>,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentSymlinkLinks {
        pub git: String,
        pub html: String,
        #[serde(rename = "self")]
        pub self_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentSymlink {
        pub links: Links,
        pub download_url: String,
        pub git_url: String,
        pub html_url: String,
        pub name: String,
        pub path: String,
        pub sha: String,
        pub size: i64,
        pub target: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentSubmoduleLinks {
        pub git: String,
        pub html: String,
        #[serde(rename = "self")]
        pub self_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentSubmodule {
        pub links: Links,
        pub download_url: String,
        pub git_url: String,
        pub html_url: String,
        pub name: String,
        pub path: String,
        pub sha: String,
        pub size: i64,
        pub submodule_git_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FileCommitContentContentLinks {
        pub git: Option<String>,
        pub html: Option<String>,
        #[serde(rename = "self")]
        pub self_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Content {
        pub links: Option<Links>,
        pub download_url: Option<String>,
        pub git_url: Option<String>,
        pub html_url: Option<String>,
        pub name: Option<String>,
        pub path: Option<String>,
        pub sha: Option<String>,
        pub size: Option<i64>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FileCommitCommitCommitAuthor {
        pub date: Option<String>,
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FileCommitCommitCommitCommitter {
        pub date: Option<String>,
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FileCommitCommitCommitTree {
        pub sha: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FileCommitCommitCommitParents {
        pub html_url: Option<String>,
        pub sha: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FileCommitCommitCommitVerification {
        pub payload: Option<String>,
        pub reason: Option<String>,
        pub signature: Option<String>,
        pub verified: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FileCommitCommit {
        pub author: Option<Author>,
        pub committer: Option<Committer>,
        pub html_url: Option<String>,
        pub message: Option<String>,
        pub node_id: Option<String>,
        pub parents: Option<Vec<Parents>>,
        pub sha: Option<String>,
        pub tree: Option<Tree>,
        pub url: Option<String>,
        pub verification: Option<Verification>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct FileCommit {
        pub commit: Commit,
        pub content: Content,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Contributor {
        pub avatar_url: Option<String>,
        pub contributions: i64,
        pub email: Option<String>,
        pub events_url: Option<String>,
        pub followers_url: Option<String>,
        pub following_url: Option<String>,
        pub gists_url: Option<String>,
        pub gravatar_id: Option<String>,
        pub html_url: Option<String>,
        pub id: Option<i64>,
        pub login: Option<String>,
        pub name: Option<String>,
        pub node_id: Option<String>,
        pub organizations_url: Option<String>,
        pub received_events_url: Option<String>,
        pub repos_url: Option<String>,
        pub site_admin: Option<bool>,
        pub starred_url: Option<String>,
        pub subscriptions_url: Option<String>,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeploymentStatus {
        pub created_at: DateTime<Utc>,
        pub creator: Creator,
        pub deployment_url: String,
        pub description: String,
        pub environment: Option<String>,
        pub environment_url: Option<String>,
        pub id: i64,
        pub log_url: Option<String>,
        pub node_id: String,
        pub performed_via_github_app: Option<PerformedviaGithubApp>,
        pub repository_url: String,
        pub state: String,
        pub target_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeploymentBranchPolicy {
        pub custom_branch_policies: bool,
        pub protected_branches: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProtectionRules {
        pub id: i64,
        pub node_id: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub wait_timer: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct EnvironmentData {
        pub created_at: DateTime<Utc>,
        pub deployment_branch_policy: Option<DeploymentBranchPolicy>,
        pub html_url: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub protection_rules: Option<Vec<ProtectionRules>>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ShortBlob {
        pub sha: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Blob {
        pub content: String,
        pub encoding: String,
        pub highlighted_content: Option<String>,
        pub node_id: String,
        pub sha: String,
        pub size: i64,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitCommitAuthor {
        pub date: DateTime<Utc>,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitCommitCommitter {
        pub date: DateTime<Utc>,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitCommitParents {
        pub html_url: String,
        pub sha: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitCommit {
        pub author: Author,
        pub committer: Committer,
        pub html_url: String,
        pub message: String,
        pub node_id: String,
        pub parents: Vec<Parents>,
        pub sha: String,
        pub tree: Tree,
        pub url: String,
        pub verification: Verification,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Object {
        pub sha: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitRef {
        pub node_id: String,
        pub object: Object,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Tagger {
        pub date: String,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitTag {
        pub message: String,
        pub node_id: String,
        pub object: Object,
        pub sha: String,
        pub tag: String,
        pub tagger: Tagger,
        pub url: String,
        pub verification: Option<Verification>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitTreeTree {
        pub mode: Option<String>,
        pub path: Option<String>,
        pub sha: Option<String>,
        pub size: Option<i64>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GitTree {
        pub sha: String,
        pub tree: Vec<Tree>,
        pub truncated: bool,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct HookResponse {
        pub code: i64,
        pub message: String,
        pub status: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct HookConfig {
        pub content_type: Option<String>,
        pub digest: Option<String>,
        pub email: Option<String>,
        pub insecure_ssl: Option<String>,
        pub password: Option<String>,
        pub room: Option<String>,
        pub secret: Option<String>,
        pub subdomain: Option<String>,
        pub token: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Hook {
        pub active: bool,
        pub config: Config,
        pub created_at: DateTime<Utc>,
        pub events: Vec<String>,
        pub id: i64,
        pub last_response: HookResponse,
        pub name: String,
        pub ping_url: String,
        pub test_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProjectChoices {
        pub human_name: Option<String>,
        pub tfvc_project: Option<String>,
        pub vcs: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Import {
        pub authors_count: Option<i64>,
        pub authors_url: String,
        pub commit_count: Option<i64>,
        pub error_message: Option<String>,
        pub failed_step: Option<String>,
        pub has_large_files: Option<bool>,
        pub html_url: String,
        pub import_percent: Option<i64>,
        pub large_files_count: Option<i64>,
        pub large_files_size: Option<i64>,
        pub message: Option<String>,
        pub project_choices: Option<Vec<ProjectChoices>>,
        pub push_percent: Option<i64>,
        pub repository_url: String,
        pub status: String,
        pub status_text: Option<String>,
        pub svc_root: Option<String>,
        pub svn_root: Option<String>,
        pub tfvc_project: Option<String>,
        pub url: String,
        pub use_lfs: Option<bool>,
        pub vcs: String,
        pub vcs_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PorterAuthor {
        pub email: String,
        pub id: i64,
        pub import_url: String,
        pub name: String,
        pub remote_id: String,
        pub remote_name: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PorterLargeFile {
        pub oid: String,
        pub path: String,
        pub ref_name: String,
        pub size: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueEventLabel {
        pub color: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueEventDismissedReview {
        pub dismissal_commit_id: Option<String>,
        pub dismissal_message: String,
        pub review_id: i64,
        pub state: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueEventMilestone {
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueEventProjectCard {
        pub column_name: String,
        pub id: i64,
        pub previous_column_name: Option<String>,
        pub project_id: i64,
        pub project_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueEventRename {
        pub from: String,
        pub to: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueEventActor {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Assigner {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReviewRequester {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RequestedReviewer {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueEvent {
        pub actor: Actor,
        pub assignee: Option<Assignee>,
        pub assigner: Option<Assigner>,
        pub author_association: Option<String>,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: DateTime<Utc>,
        pub dismissed_review: Option<IssueEventDismissedReview>,
        pub event: String,
        pub id: i64,
        pub issue: Option<IssueSimple>,
        pub label: Option<IssueEventLabel>,
        pub lock_reason: Option<String>,
        pub milestone: Option<IssueEventMilestone>,
        pub node_id: String,
        pub performed_via_github_app: Option<PerformedviaGithubApp>,
        pub project_card: Option<IssueEventProjectCard>,
        pub rename: Option<IssueEventRename>,
        pub requested_reviewer: Option<RequestedReviewer>,
        pub requested_team: Option<Team>,
        pub review_requester: Option<ReviewRequester>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LabeledIssueEventLabel {
        pub color: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LabeledIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub label: Label,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UnlabeledIssueEventLabel {
        pub color: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UnlabeledIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub label: Label,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AssignedIssueEvent {
        pub actor: SimpleUser,
        pub assignee: SimpleUser,
        pub assigner: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UnassignedIssueEvent {
        pub actor: SimpleUser,
        pub assignee: SimpleUser,
        pub assigner: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MilestonedIssueEventMilestone {
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MilestonedIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub milestone: Milestone,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DemilestonedIssueEventMilestone {
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DemilestonedIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub milestone: Milestone,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Rename {
        pub from: String,
        pub to: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RenamedIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub rename: Rename,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReviewRequestedIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub requested_reviewer: Option<SimpleUser>,
        pub requested_team: Option<Team>,
        pub review_requester: SimpleUser,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReviewRequestRemovedIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub requested_reviewer: Option<SimpleUser>,
        pub requested_team: Option<Team>,
        pub review_requester: SimpleUser,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DismissedReview {
        pub dismissal_commit_id: Option<String>,
        pub dismissal_message: String,
        pub review_id: i64,
        pub state: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReviewDismissedIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub dismissed_review: DismissedReview,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LockedIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub lock_reason: String,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddedtoProjectIssueEventProjectCard {
        pub column_name: String,
        pub id: i64,
        pub previous_column_name: Option<String>,
        pub project_id: i64,
        pub project_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddedtoProjectIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub project_card: Option<ProjectCard>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MovedColumninProjectIssueEventProjectCard {
        pub column_name: String,
        pub id: i64,
        pub previous_column_name: Option<String>,
        pub project_id: i64,
        pub project_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MovedColumninProjectIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub project_card: Option<ProjectCard>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RemovedFromProjectIssueEventProjectCard {
        pub column_name: String,
        pub id: i64,
        pub previous_column_name: Option<String>,
        pub project_id: i64,
        pub project_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RemovedFromProjectIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub project_card: Option<ProjectCard>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ConvertedNotetoIssueIssueEventProjectCard {
        pub column_name: String,
        pub id: i64,
        pub previous_column_name: Option<String>,
        pub project_id: i64,
        pub project_url: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ConvertedNotetoIssueIssueEvent {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub project_card: Option<ProjectCard>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueEventforIssue {
        pub actor: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub label: Label,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineCommentEvent {
        pub actor: SimpleUser,
        pub author_association: String,
        pub body: Option<String>,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub created_at: DateTime<Utc>,
        pub event: String,
        pub html_url: String,
        pub id: i64,
        pub issue_url: String,
        pub node_id: String,
        pub performed_via_github_app: Option<Integration>,
        pub reactions: Option<ReactionRollup>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: SimpleUser,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Source {
        pub issue: Option<IssueSimple>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineCrossReferencedEvent {
        pub actor: Option<SimpleUser>,
        pub created_at: DateTime<Utc>,
        pub event: String,
        pub source: Source,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineCommittedEventAuthor {
        pub date: DateTime<Utc>,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineCommittedEventCommitter {
        pub date: DateTime<Utc>,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineCommittedEventParents {
        pub html_url: String,
        pub sha: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineCommittedEvent {
        pub author: Author,
        pub committer: Committer,
        pub event: Option<String>,
        pub html_url: String,
        pub message: String,
        pub node_id: String,
        pub parents: Vec<Parents>,
        pub sha: String,
        pub tree: Tree,
        pub url: String,
        pub verification: Verification,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Html {
        pub href: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineReviewedEventLinksLinksPullRequest {
        pub href: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineReviewedEventLinks {
        pub html: Html,
        pub pull_request: PullRequest,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineReviewedEvent {
        pub links: Links,
        pub author_association: String,
        pub body: String,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub commit_id: String,
        pub event: String,
        pub html_url: String,
        pub id: i64,
        pub node_id: String,
        pub pull_request_url: String,
        pub state: String,
        pub submitted_at: Option<DateTime<Utc>>,
        pub user: SimpleUser,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SelfData {
        pub href: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestReviewCommentLinksLinksPullRequest {
        pub href: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestReviewCommentLinks {
        pub html: Html,
        pub pull_request: PullRequest,
        #[serde(rename = "self")]
        pub self_: SelfData,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestReviewComment {
        pub links: Links,
        pub author_association: String,
        pub body: String,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub commit_id: String,
        pub created_at: DateTime<Utc>,
        pub diff_hunk: String,
        pub html_url: String,
        pub id: i64,
        pub in_reply_to_id: Option<i64>,
        pub line: Option<i64>,
        pub node_id: String,
        pub original_commit_id: String,
        pub original_line: Option<i64>,
        pub original_position: i64,
        pub original_start_line: Option<i64>,
        pub path: String,
        pub position: i64,
        pub pull_request_review_id: i64,
        pub pull_request_url: String,
        pub reactions: Option<ReactionRollup>,
        pub side: Option<String>,
        pub start_line: Option<i64>,
        pub start_side: Option<String>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: SimpleUser,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineLineCommentedEvent {
        pub comments: Option<Vec<PullRequestReviewComment>>,
        pub event: Option<String>,
        pub node_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineCommitCommentedEvent {
        pub comments: Option<Vec<CommitComment>>,
        pub commit_id: Option<String>,
        pub event: Option<String>,
        pub node_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineAssignedIssueEvent {
        pub actor: SimpleUser,
        pub assignee: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineUnassignedIssueEvent {
        pub actor: SimpleUser,
        pub assignee: SimpleUser,
        pub commit_id: String,
        pub commit_url: String,
        pub created_at: String,
        pub event: String,
        pub id: i64,
        pub node_id: String,
        pub performed_via_github_app: Integration,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TimelineIssueEvents {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeployKey {
        pub created_at: String,
        pub id: i64,
        pub key: String,
        pub read_only: bool,
        pub title: String,
        pub url: String,
        pub verified: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Language {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LicenseContentLinks {
        pub git: String,
        pub html: String,
        #[serde(rename = "self")]
        pub self_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LicenseContent {
        pub links: Links,
        pub content: String,
        pub download_url: String,
        pub encoding: String,
        pub git_url: String,
        pub html_url: String,
        pub license: License,
        pub name: String,
        pub path: String,
        pub sha: String,
        pub size: i64,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PagesSourceHash {
        pub branch: String,
        pub path: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PagesHttpsCertificate {
        pub description: String,
        pub domains: serde_json::Value,
        pub expires_at: Option<NaiveDate>,
        pub state: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Page {
        pub cname: String,
        pub custom_404: bool,
        pub html_url: Option<String>,
        pub https_certificate: Option<PagesHttpsCertificate>,
        pub https_enforced: Option<bool>,
        pub public: bool,
        pub source: Option<PagesSourceHash>,
        pub status: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Error {
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Pusher {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PageBuild {
        pub commit: String,
        pub created_at: DateTime<Utc>,
        pub duration: i64,
        pub error: Error,
        pub pusher: Pusher,
        pub status: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PageBuildStatus {
        pub status: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Domain {
        pub caa_error: Option<String>,
        pub dns_resolves: Option<bool>,
        pub enforces_https: Option<bool>,
        pub has_cname_record: Option<bool>,
        pub has_mx_records_present: Option<bool>,
        pub host: Option<String>,
        pub https_error: Option<String>,
        pub is_a_record: Option<bool>,
        pub is_apex_domain: Option<bool>,
        pub is_cloudflare_ip: Option<bool>,
        pub is_cname_to_fastly: Option<bool>,
        pub is_cname_to_github_user_domain: Option<bool>,
        pub is_cname_to_pages_dot_github_dot_com: Option<bool>,
        pub is_fastly_ip: Option<bool>,
        pub is_https_eligible: Option<bool>,
        pub is_non_github_pages_ip_present: Option<bool>,
        pub is_old_ip_address: Option<bool>,
        pub is_pages_domain: Option<bool>,
        pub is_pointed_to_github_pages_ip: Option<bool>,
        pub is_proxied: Option<bool>,
        pub is_served_by_pages: Option<bool>,
        pub is_valid: Option<bool>,
        pub is_valid_domain: Option<bool>,
        pub nameservers: Option<String>,
        pub reason: Option<String>,
        pub responds_to_https: Option<bool>,
        pub should_be_a_record: Option<bool>,
        pub uri: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AltDomain {
        pub caa_error: Option<String>,
        pub dns_resolves: Option<bool>,
        pub enforces_https: Option<bool>,
        pub has_cname_record: Option<bool>,
        pub has_mx_records_present: Option<bool>,
        pub host: Option<String>,
        pub https_error: Option<String>,
        pub is_a_record: Option<bool>,
        pub is_apex_domain: Option<bool>,
        pub is_cloudflare_ip: Option<bool>,
        pub is_cname_to_fastly: Option<bool>,
        pub is_cname_to_github_user_domain: Option<bool>,
        pub is_cname_to_pages_dot_github_dot_com: Option<bool>,
        pub is_fastly_ip: Option<bool>,
        pub is_https_eligible: Option<bool>,
        pub is_non_github_pages_ip_present: Option<bool>,
        pub is_old_ip_address: Option<bool>,
        pub is_pages_domain: Option<bool>,
        pub is_pointed_to_github_pages_ip: Option<bool>,
        pub is_proxied: Option<bool>,
        pub is_served_by_pages: Option<bool>,
        pub is_valid: Option<bool>,
        pub is_valid_domain: Option<bool>,
        pub nameservers: Option<String>,
        pub reason: Option<String>,
        pub responds_to_https: Option<bool>,
        pub should_be_a_record: Option<bool>,
        pub uri: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PagesHealthCheck {
        pub alt_domain: Option<AltDomain>,
        pub domain: Option<Domain>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestLabels {
        pub color: Option<String>,
        pub default: Option<bool>,
        pub description: Option<String>,
        pub id: Option<i64>,
        pub name: Option<String>,
        pub node_id: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestHeadHeadRepoRepoOwner {
        pub avatar_url: String,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestHeadHeadRepoRepoPermissions {
        pub admin: bool,
        pub pull: bool,
        pub push: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestHeadHeadRepoRepoLicense {
        pub key: String,
        pub name: String,
        pub node_id: String,
        pub spdx_id: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestHeadHeadRepo {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub archive_url: String,
        pub archived: bool,
        pub assignees_url: String,
        pub blobs_url: String,
        pub branches_url: String,
        pub clone_url: String,
        pub collaborators_url: String,
        pub comments_url: String,
        pub commits_url: String,
        pub compare_url: String,
        pub contents_url: String,
        pub contributors_url: String,
        pub created_at: DateTime<Utc>,
        pub default_branch: String,
        pub deployments_url: String,
        pub description: String,
        pub disabled: bool,
        pub downloads_url: String,
        pub events_url: String,
        pub fork: bool,
        pub forks: i64,
        pub forks_count: i64,
        pub forks_url: String,
        pub full_name: String,
        pub git_commits_url: String,
        pub git_refs_url: String,
        pub git_tags_url: String,
        pub git_url: String,
        pub has_downloads: bool,
        pub has_issues: bool,
        pub has_pages: bool,
        pub has_projects: bool,
        pub has_wiki: bool,
        pub homepage: String,
        pub hooks_url: String,
        pub html_url: String,
        pub id: i64,
        pub issue_comment_url: String,
        pub issue_events_url: String,
        pub issues_url: String,
        pub keys_url: String,
        pub labels_url: String,
        pub language: String,
        pub languages_url: String,
        pub license: License,
        pub master_branch: Option<String>,
        pub merges_url: String,
        pub milestones_url: String,
        pub mirror_url: String,
        pub name: String,
        pub node_id: String,
        pub notifications_url: String,
        pub open_issues: i64,
        pub open_issues_count: i64,
        pub owner: Owner,
        pub permissions: Option<Permissions>,
        pub private: bool,
        pub pulls_url: String,
        pub pushed_at: DateTime<Utc>,
        pub releases_url: String,
        pub size: i64,
        pub ssh_url: String,
        pub stargazers_count: i64,
        pub stargazers_url: String,
        pub statuses_url: String,
        pub subscribers_url: String,
        pub subscription_url: String,
        pub svn_url: String,
        pub tags_url: String,
        pub teams_url: String,
        pub temp_clone_token: Option<String>,
        pub topics: Option<Vec<String>>,
        pub trees_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub watchers: i64,
        pub watchers_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestHeadHeadUser {
        pub avatar_url: String,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestHead {
        pub label: String,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub repo: Repo,
        pub sha: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestBaseBaseRepoRepoOwner {
        pub avatar_url: String,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestBaseBaseRepoRepoPermissions {
        pub admin: bool,
        pub pull: bool,
        pub push: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestBaseBaseRepo {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub archive_url: String,
        pub archived: bool,
        pub assignees_url: String,
        pub blobs_url: String,
        pub branches_url: String,
        pub clone_url: String,
        pub collaborators_url: String,
        pub comments_url: String,
        pub commits_url: String,
        pub compare_url: String,
        pub contents_url: String,
        pub contributors_url: String,
        pub created_at: DateTime<Utc>,
        pub default_branch: String,
        pub deployments_url: String,
        pub description: String,
        pub disabled: bool,
        pub downloads_url: String,
        pub events_url: String,
        pub fork: bool,
        pub forks: i64,
        pub forks_count: i64,
        pub forks_url: String,
        pub full_name: String,
        pub git_commits_url: String,
        pub git_refs_url: String,
        pub git_tags_url: String,
        pub git_url: String,
        pub has_downloads: bool,
        pub has_issues: bool,
        pub has_pages: bool,
        pub has_projects: bool,
        pub has_wiki: bool,
        pub homepage: String,
        pub hooks_url: String,
        pub html_url: String,
        pub id: i64,
        pub issue_comment_url: String,
        pub issue_events_url: String,
        pub issues_url: String,
        pub keys_url: String,
        pub labels_url: String,
        pub language: String,
        pub languages_url: String,
        pub license: License,
        pub master_branch: Option<String>,
        pub merges_url: String,
        pub milestones_url: String,
        pub mirror_url: String,
        pub name: String,
        pub node_id: String,
        pub notifications_url: String,
        pub open_issues: i64,
        pub open_issues_count: i64,
        pub owner: Owner,
        pub permissions: Option<Permissions>,
        pub private: bool,
        pub pulls_url: String,
        pub pushed_at: DateTime<Utc>,
        pub releases_url: String,
        pub size: i64,
        pub ssh_url: String,
        pub stargazers_count: i64,
        pub stargazers_url: String,
        pub statuses_url: String,
        pub subscribers_url: String,
        pub subscription_url: String,
        pub svn_url: String,
        pub tags_url: String,
        pub teams_url: String,
        pub temp_clone_token: Option<String>,
        pub topics: Option<Vec<String>>,
        pub trees_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub watchers: i64,
        pub watchers_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestBaseBaseUser {
        pub avatar_url: String,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestBase {
        pub label: String,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub repo: Repo,
        pub sha: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestLinks {
        pub comments: Link,
        pub commits: Link,
        pub html: Link,
        pub issue: Link,
        pub review_comment: Link,
        pub review_comments: Link,
        #[serde(rename = "self")]
        pub self_: Link,
        pub statuses: Link,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MergedBy {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestData {
        pub links: Links,
        pub active_lock_reason: Option<String>,
        pub additions: i64,
        pub assignee: Assignee,
        pub assignees: Option<Vec<SimpleUser>>,
        pub author_association: String,
        pub auto_merge: AutoMerge,
        pub base: Base,
        pub body: String,
        pub changed_files: i64,
        pub closed_at: DateTime<Utc>,
        pub comments: i64,
        pub comments_url: String,
        pub commits: i64,
        pub commits_url: String,
        pub created_at: DateTime<Utc>,
        pub deletions: i64,
        pub diff_url: String,
        pub draft: Option<bool>,
        pub head: Head,
        pub html_url: String,
        pub id: i64,
        pub issue_url: String,
        pub labels: Vec<Labels>,
        pub locked: bool,
        pub maintainer_can_modify: bool,
        pub merge_commit_sha: String,
        pub mergeable: bool,
        pub mergeable_state: String,
        pub merged: bool,
        pub merged_at: DateTime<Utc>,
        pub merged_by: MergedBy,
        pub milestone: Milestone,
        pub node_id: String,
        pub number: i64,
        pub patch_url: String,
        pub rebaseable: Option<bool>,
        pub requested_reviewers: Option<Vec<SimpleUser>>,
        pub requested_teams: Option<Vec<TeamSimple>>,
        pub review_comment_url: String,
        pub review_comments: i64,
        pub review_comments_url: String,
        pub state: String,
        pub statuses_url: String,
        pub title: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestMergeResult {
        pub merged: bool,
        pub message: String,
        pub sha: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestReviewRequest {
        pub teams: Vec<Team>,
        pub users: Vec<SimpleUser>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestReviewLinksLinksPullRequest {
        pub href: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestReviewLinks {
        pub html: Html,
        pub pull_request: PullRequest,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PullRequestReview {
        pub links: Links,
        pub author_association: String,
        pub body: String,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub commit_id: String,
        pub html_url: String,
        pub id: i64,
        pub node_id: String,
        pub pull_request_url: String,
        pub state: String,
        pub submitted_at: Option<DateTime<Utc>>,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReviewCommentLinks {
        pub html: Link,
        pub pull_request: Link,
        #[serde(rename = "self")]
        pub self_: Link,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReviewComment {
        pub links: Links,
        pub author_association: String,
        pub body: String,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub commit_id: String,
        pub created_at: DateTime<Utc>,
        pub diff_hunk: String,
        pub html_url: String,
        pub id: i64,
        pub in_reply_to_id: Option<i64>,
        pub line: Option<i64>,
        pub node_id: String,
        pub original_commit_id: String,
        pub original_line: Option<i64>,
        pub original_position: i64,
        pub original_start_line: Option<i64>,
        pub path: String,
        pub position: i64,
        pub pull_request_review_id: i64,
        pub pull_request_url: String,
        pub reactions: Option<ReactionRollup>,
        pub side: Option<String>,
        pub start_line: Option<i64>,
        pub start_side: Option<String>,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Uploader {
        pub avatar_url: String,
        pub email: Option<String>,
        pub events_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub html_url: String,
        pub id: i64,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_at: Option<String>,
        pub starred_url: String,
        pub subscriptions_url: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReleaseAsset {
        pub browser_download_url: String,
        pub content_type: String,
        pub created_at: DateTime<Utc>,
        pub download_count: i64,
        pub id: i64,
        pub label: String,
        pub name: String,
        pub node_id: String,
        pub size: i64,
        pub state: String,
        pub updated_at: DateTime<Utc>,
        pub uploader: Uploader,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Release {
        pub assets: Vec<ReleaseAsset>,
        pub assets_url: String,
        pub author: SimpleUser,
        pub body: Option<String>,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub created_at: DateTime<Utc>,
        pub discussion_url: Option<String>,
        pub draft: bool,
        pub html_url: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub prerelease: bool,
        pub published_at: DateTime<Utc>,
        pub reactions: Option<ReactionRollup>,
        pub tag_name: String,
        pub tarball_url: String,
        pub target_commitish: String,
        pub upload_url: String,
        pub url: String,
        pub zipball_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SecretScanningAlert {
        pub created_at: Option<DateTime<Utc>>,
        pub html_url: Option<String>,
        pub number: Option<i64>,
        pub resolution: Option<serde_json::Value>,
        pub resolved_at: Option<DateTime<Utc>>,
        pub resolved_by: Option<SimpleUser>,
        pub secret: Option<String>,
        pub secret_type: Option<String>,
        pub state: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Stargazer {
        pub starred_at: DateTime<Utc>,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitActivity {
        pub days: Vec<i64>,
        pub total: i64,
        pub week: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Weeks {
        pub a: Option<i64>,
        pub c: Option<i64>,
        pub d: Option<i64>,
        pub w: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContributorActivity {
        pub author: Author,
        pub total: i64,
        pub weeks: Vec<Weeks>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ParticipationStats {
        pub all: Vec<i64>,
        pub owner: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RepositorySubscription {
        pub created_at: DateTime<Utc>,
        pub ignored: bool,
        pub reason: String,
        pub repository_url: String,
        pub subscribed: bool,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Tag {
        pub commit: Commit,
        pub name: String,
        pub node_id: String,
        pub tarball_url: String,
        pub zipball_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Topic {
        pub names: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Traffic {
        pub count: i64,
        pub timestamp: DateTime<Utc>,
        pub uniques: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CloneTraffic {
        pub clones: Vec<Traffic>,
        pub count: i64,
        pub uniques: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ContentTraffic {
        pub count: i64,
        pub path: String,
        pub title: String,
        pub uniques: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReferrerTraffic {
        pub count: i64,
        pub referrer: String,
        pub uniques: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ViewTraffic {
        pub count: i64,
        pub uniques: i64,
        pub views: Vec<Traffic>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Members {
        #[serde(rename = "$ref")]
        pub ref_: Option<String>,
        pub display: Option<String>,
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Meta {
        pub created: Option<String>,
        pub last_modified: Option<String>,
        pub location: Option<String>,
        pub resource_type: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimGroupListEnterpriseResources {
        pub display_name: Option<String>,
        pub external_id: Option<String>,
        pub id: String,
        pub members: Option<Vec<Members>>,
        pub meta: Option<Meta>,
        pub schemas: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimGroupListEnterprise {
        pub resources: Vec<Resources>,
        pub items_per_page: f64,
        pub schemas: Vec<String>,
        pub start_index: f64,
        pub total_results: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimEnterpriseGroup {
        pub display_name: Option<String>,
        pub external_id: Option<String>,
        pub id: String,
        pub members: Option<Vec<Members>>,
        pub meta: Option<Meta>,
        pub schemas: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Name {
        pub family_name: Option<String>,
        pub given_name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Emails {
        pub primary: Option<bool>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimUserListEnterpriseResourcesResourcesGroups {
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimUserListEnterpriseResources {
        pub active: Option<bool>,
        pub emails: Option<Vec<Emails>>,
        pub external_id: Option<String>,
        pub groups: Option<Vec<Groups>>,
        pub id: String,
        pub meta: Option<Meta>,
        pub name: Option<Name>,
        pub schemas: Vec<String>,
        pub user_name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimUserListEnterprise {
        pub resources: Vec<Resources>,
        pub items_per_page: f64,
        pub schemas: Vec<String>,
        pub start_index: f64,
        pub total_results: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimEnterpriseUserGroups {
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimEnterpriseUser {
        pub active: Option<bool>,
        pub emails: Option<Vec<Emails>>,
        pub external_id: Option<String>,
        pub groups: Option<Vec<Groups>>,
        pub id: String,
        pub meta: Option<Meta>,
        pub name: Option<Name>,
        pub schemas: Vec<String>,
        pub user_name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimUserName {
        pub family_name: String,
        pub formatted: Option<String>,
        pub given_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimUserEmails {
        pub primary: Option<bool>,
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimUserMeta {
        pub created: Option<DateTime<Utc>>,
        pub last_modified: Option<DateTime<Utc>>,
        pub location: Option<String>,
        pub resource_type: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Operations {
        pub op: String,
        pub path: Option<String>,
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimUser {
        pub active: bool,
        pub display_name: Option<String>,
        pub emails: Vec<Emails>,
        pub external_id: String,
        pub groups: Option<Vec<serde_json::Value>>,
        pub id: String,
        pub meta: Meta,
        pub name: Name,
        pub operations: Option<Vec<Operations>>,
        pub organization_id: Option<i64>,
        pub schemas: Vec<String>,
        pub user_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ScimUserList {
        pub resources: Vec<ScimUser>,
        pub items_per_page: i64,
        pub schemas: Vec<String>,
        pub start_index: i64,
        pub total_results: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Matches {
        pub indices: Option<Vec<i64>>,
        pub text: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SearchResultTextMatches {
        pub fragment: Option<String>,
        pub matches: Option<Vec<Matches>>,
        pub object_type: Option<String>,
        pub object_url: Option<String>,
        pub property: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CodeSearchResultItem {
        pub file_size: Option<i64>,
        pub git_url: String,
        pub html_url: String,
        pub language: Option<String>,
        pub last_modified_at: Option<DateTime<Utc>>,
        pub line_numbers: Option<Vec<String>>,
        pub name: String,
        pub path: String,
        pub repository: MinimalRepository,
        pub score: f64,
        pub sha: String,
        pub text_matches: Option<SearchResultTextMatches>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitSearchResultItemCommitCommitAuthor {
        pub date: DateTime<Utc>,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitSearchResultItemCommitCommitCommitter {
        pub date: Option<String>,
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitSearchResultItemCommit {
        pub author: Author,
        pub comment_count: i64,
        pub committer: Committer,
        pub message: String,
        pub tree: Tree,
        pub url: String,
        pub verification: Option<Verification>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitSearchResultItemCommitter {
        pub date: Option<String>,
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitSearchResultItemParents {
        pub html_url: Option<String>,
        pub sha: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CommitSearchResultItem {
        pub author: Author,
        pub comments_url: String,
        pub commit: Commit,
        pub committer: Committer,
        pub html_url: String,
        pub node_id: String,
        pub parents: Vec<Parents>,
        pub repository: MinimalRepository,
        pub score: f64,
        pub sha: String,
        pub text_matches: Option<SearchResultTextMatches>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueSearchResultItemLabels {
        pub color: Option<String>,
        pub default: Option<bool>,
        pub description: Option<String>,
        pub id: Option<i64>,
        pub name: Option<String>,
        pub node_id: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct IssueSearchResultItem {
        pub active_lock_reason: Option<String>,
        pub assignee: Assignee,
        pub assignees: Option<Vec<SimpleUser>>,
        pub author_association: String,
        pub body: Option<String>,
        pub body_html: Option<String>,
        pub body_text: Option<String>,
        pub closed_at: DateTime<Utc>,
        pub comments: i64,
        pub comments_url: String,
        pub created_at: DateTime<Utc>,
        pub draft: Option<bool>,
        pub events_url: String,
        pub html_url: String,
        pub id: i64,
        pub labels: Vec<Labels>,
        pub labels_url: String,
        pub locked: bool,
        pub milestone: Milestone,
        pub node_id: String,
        pub number: i64,
        pub performed_via_github_app: Option<PerformedviaGithubApp>,
        pub pull_request: Option<PullRequest>,
        pub repository: Option<Repository>,
        pub repository_url: String,
        pub score: f64,
        pub state: String,
        pub text_matches: Option<SearchResultTextMatches>,
        pub timeline_url: Option<String>,
        pub title: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LabelSearchResultItem {
        pub color: String,
        pub default: bool,
        pub description: String,
        pub id: i64,
        pub name: String,
        pub node_id: String,
        pub score: f64,
        pub text_matches: Option<SearchResultTextMatches>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RepoSearchResultItemPermissions {
        pub admin: bool,
        pub pull: bool,
        pub push: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RepoSearchResultItem {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub archive_url: String,
        pub archived: bool,
        pub assignees_url: String,
        pub blobs_url: String,
        pub branches_url: String,
        pub clone_url: String,
        pub collaborators_url: String,
        pub comments_url: String,
        pub commits_url: String,
        pub compare_url: String,
        pub contents_url: String,
        pub contributors_url: String,
        pub created_at: DateTime<Utc>,
        pub default_branch: String,
        pub delete_branch_on_merge: Option<bool>,
        pub deployments_url: String,
        pub description: String,
        pub disabled: bool,
        pub downloads_url: String,
        pub events_url: String,
        pub fork: bool,
        pub forks: i64,
        pub forks_count: i64,
        pub forks_url: String,
        pub full_name: String,
        pub git_commits_url: String,
        pub git_refs_url: String,
        pub git_tags_url: String,
        pub git_url: String,
        pub has_downloads: bool,
        pub has_issues: bool,
        pub has_pages: bool,
        pub has_projects: bool,
        pub has_wiki: bool,
        pub homepage: String,
        pub hooks_url: String,
        pub html_url: String,
        pub id: i64,
        pub issue_comment_url: String,
        pub issue_events_url: String,
        pub issues_url: String,
        pub keys_url: String,
        pub labels_url: String,
        pub language: String,
        pub languages_url: String,
        pub license: License,
        pub master_branch: Option<String>,
        pub merges_url: String,
        pub milestones_url: String,
        pub mirror_url: String,
        pub name: String,
        pub node_id: String,
        pub notifications_url: String,
        pub open_issues: i64,
        pub open_issues_count: i64,
        pub owner: Owner,
        pub permissions: Option<Permissions>,
        pub private: bool,
        pub pulls_url: String,
        pub pushed_at: DateTime<Utc>,
        pub releases_url: String,
        pub score: f64,
        pub size: i64,
        pub ssh_url: String,
        pub stargazers_count: i64,
        pub stargazers_url: String,
        pub statuses_url: String,
        pub subscribers_url: String,
        pub subscription_url: String,
        pub svn_url: String,
        pub tags_url: String,
        pub teams_url: String,
        pub temp_clone_token: Option<String>,
        pub text_matches: Option<SearchResultTextMatches>,
        pub topics: Option<Vec<String>>,
        pub trees_url: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub watchers: i64,
        pub watchers_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TopicRelation {
        pub id: Option<i64>,
        pub name: Option<String>,
        pub relation_type: Option<String>,
        pub topic_id: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Related {
        pub topic_relation: Option<TopicRelation>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Aliases {
        pub topic_relation: Option<TopicRelation>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TopicSearchResultItem {
        pub aliases: Option<Vec<Aliases>>,
        pub created_at: DateTime<Utc>,
        pub created_by: String,
        pub curated: bool,
        pub description: String,
        pub display_name: String,
        pub featured: bool,
        pub logo_url: Option<String>,
        pub name: String,
        pub related: Option<Vec<Related>>,
        pub released: String,
        pub repository_count: Option<i64>,
        pub score: f64,
        pub short_description: String,
        pub text_matches: Option<SearchResultTextMatches>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UserSearchResultItem {
        pub avatar_url: String,
        pub bio: Option<String>,
        pub blog: Option<String>,
        pub company: Option<String>,
        pub created_at: Option<DateTime<Utc>>,
        pub email: Option<String>,
        pub events_url: String,
        pub followers: Option<i64>,
        pub followers_url: String,
        pub following: Option<i64>,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub hireable: Option<bool>,
        pub html_url: String,
        pub id: i64,
        pub location: Option<String>,
        pub login: String,
        pub name: Option<String>,
        pub node_id: String,
        pub organizations_url: String,
        pub public_gists: Option<i64>,
        pub public_repos: Option<i64>,
        pub received_events_url: String,
        pub repos_url: String,
        pub score: f64,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        pub suspended_at: Option<DateTime<Utc>>,
        pub text_matches: Option<SearchResultTextMatches>,
        #[serde(rename = "type")]
        pub type_: String,
        pub updated_at: Option<DateTime<Utc>>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PrivateUser {
        pub avatar_url: String,
        pub bio: String,
        pub blog: String,
        pub business_plus: Option<bool>,
        pub collaborators: i64,
        pub company: String,
        pub created_at: DateTime<Utc>,
        pub disk_usage: i64,
        pub email: String,
        pub events_url: String,
        pub followers: i64,
        pub followers_url: String,
        pub following: i64,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub hireable: bool,
        pub html_url: String,
        pub id: i64,
        pub ldap_dn: Option<String>,
        pub location: String,
        pub login: String,
        pub name: String,
        pub node_id: String,
        pub organizations_url: String,
        pub owned_private_repos: i64,
        pub plan: Option<Plan>,
        pub private_gists: i64,
        pub public_gists: i64,
        pub public_repos: i64,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        pub suspended_at: Option<DateTime<Utc>>,
        pub total_private_repos: i64,
        pub twitter_username: Option<String>,
        pub two_factor_authentication: bool,
        #[serde(rename = "type")]
        pub type_: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Email {
        pub email: String,
        pub primary: bool,
        pub verified: bool,
        pub visibility: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GpgKeyEmails {
        pub email: Option<String>,
        pub verified: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Subkeys {
        pub can_certify: Option<bool>,
        pub can_encrypt_comms: Option<bool>,
        pub can_encrypt_storage: Option<bool>,
        pub can_sign: Option<bool>,
        pub created_at: Option<String>,
        pub emails: Option<Vec<serde_json::Value>>,
        pub expires_at: Option<String>,
        pub id: Option<i64>,
        pub key_id: Option<String>,
        pub primary_key_id: Option<i64>,
        pub public_key: Option<String>,
        pub raw_key: Option<String>,
        pub subkeys: Option<Vec<serde_json::Value>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GpgKey {
        pub can_certify: bool,
        pub can_encrypt_comms: bool,
        pub can_encrypt_storage: bool,
        pub can_sign: bool,
        pub created_at: DateTime<Utc>,
        pub emails: Vec<Emails>,
        pub expires_at: DateTime<Utc>,
        pub id: i64,
        pub key_id: String,
        pub primary_key_id: i64,
        pub public_key: String,
        pub raw_key: String,
        pub subkeys: Vec<Subkeys>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Key {
        pub created_at: DateTime<Utc>,
        pub id: i64,
        pub key: String,
        pub read_only: bool,
        pub title: String,
        pub url: String,
        pub verified: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MarketplaceAccount {
        pub email: Option<String>,
        pub id: i64,
        pub login: String,
        pub node_id: Option<String>,
        pub organization_billing_email: Option<String>,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UserMarketplacePurchase {
        pub account: MarketplaceAccount,
        pub billing_cycle: String,
        pub free_trial_ends_on: DateTime<Utc>,
        pub next_billing_date: DateTime<Utc>,
        pub on_free_trial: bool,
        pub plan: MarketplaceListingPlan,
        pub unit_count: i64,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct StarredRepository {
        pub repo: Repository,
        pub starred_at: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Contexts {
        pub message: String,
        pub octicon: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Hovercard {
        pub contexts: Vec<Contexts>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct KeySimple {
        pub id: i64,
        pub key: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetGithubApiRootOkResponse {
        pub authorizations_url: String,
        pub code_search_url: String,
        pub commit_search_url: String,
        pub current_user_authorizations_html_url: String,
        pub current_user_repositories_url: String,
        pub current_user_url: String,
        pub emails_url: String,
        pub emojis_url: String,
        pub events_url: String,
        pub feeds_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub hub_url: String,
        pub issue_search_url: String,
        pub issues_url: String,
        pub keys_url: String,
        pub label_search_url: String,
        pub notifications_url: String,
        pub organization_repositories_url: String,
        pub organization_teams_url: String,
        pub organization_url: String,
        pub public_gists_url: String,
        pub rate_limit_url: String,
        pub repository_search_url: String,
        pub repository_url: String,
        pub starred_gists_url: String,
        pub starred_url: String,
        pub topic_search_url: Option<String>,
        pub user_organizations_url: String,
        pub user_repositories_url: String,
        pub user_search_url: String,
        pub user_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateGithubAppFromManifestRequest {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostCreateGithubAppFromManifestCreatedResponse {
        pub client_id: Option<String>,
        pub client_secret: Option<String>,
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub events: Vec<String>,
        pub external_url: String,
        pub html_url: String,
        pub id: i64,
        pub installations_count: Option<i64>,
        pub name: String,
        pub node_id: String,
        pub owner: Owner,
        pub pem: Option<String>,
        pub permissions: Permissions,
        pub slug: Option<String>,
        pub updated_at: DateTime<Utc>,
        pub webhook_secret: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateWebhookConfigurationAppRequest {
        pub content_type: Option<String>,
        pub insecure_ssl: Option<String>,
        pub secret: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateInstallationAccessTokenAppRequest {
        pub permissions: Option<AppPermissions>,
        pub repositories: Option<Vec<String>>,
        pub repository_ids: Option<Vec<i64>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteAppAuthorizationRequest {
        pub access_token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CheckTokenRequest {
        pub access_token: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteAppTokenRequest {
        pub access_token: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ResetTokenRequest {
        pub access_token: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateScopedAccessTokenRequest {
        pub access_token: String,
        pub permissions: Option<AppPermissions>,
        pub repositories: Option<Vec<String>>,
        pub repository_ids: Option<Vec<i64>>,
        pub target: Option<String>,
        pub target_id: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetCheckAuthorizationOkResponse {
        pub app: App,
        pub created_at: DateTime<Utc>,
        pub fingerprint: String,
        pub hashed_token: String,
        pub id: i64,
        pub installation: Option<Installation>,
        pub note: String,
        pub note_url: String,
        pub scopes: Vec<String>,
        pub token: String,
        pub token_last_eight: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
        pub user: Option<User>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateNewAuthorizationRequest {
        pub client_id: Option<String>,
        pub client_secret: Option<String>,
        pub fingerprint: Option<String>,
        pub note: Option<String>,
        pub note_url: Option<String>,
        pub scopes: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetorCreateAuthorizationSpecificAppRequest {
        pub client_secret: String,
        pub fingerprint: Option<String>,
        pub note: Option<String>,
        pub note_url: Option<String>,
        pub scopes: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetorCreateAuthorizationSpecificAppandFingerprintRequest {
        pub client_secret: String,
        pub note: Option<String>,
        pub note_url: Option<String>,
        pub scopes: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateExistingAuthorizationRequest {
        pub add_scopes: Option<Vec<String>>,
        pub fingerprint: Option<String>,
        pub note: Option<String>,
        pub note_url: Option<String>,
        pub remove_scopes: Option<Vec<String>>,
        pub scopes: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetEmojisOkResponse {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetGithubActionsPermissionsEnterpriseRequest {
        pub allowed_actions: Option<String>,
        pub enabled_organizations: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelectedOrganizationsEnabledGithubActionsinEnterpriseOkResponse {
        pub organizations: Vec<OrganizationSimple>,
        pub total_count: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetSelectedOrganizationsEnabledGithubActionsinEnterpriseRequest {
        pub selected_organization_ids: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelfDataHostedRunnerGroupsEnterpriseOkResponse {
        pub runner_groups: Vec<RunnerGroupsEnterprise>,
        pub total_count: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateSelfDataHostedRunnerGroupEnterpriseRequest {
        pub name: String,
        pub runners: Option<Vec<i64>>,
        pub selected_organization_ids: Option<Vec<i64>>,
        pub visibility: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateSelfDataHostedRunnerGroupEnterpriseRequest {
        pub name: Option<String>,
        pub visibility: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListOrganizationAccessSelfDataHostedRunnerGroupinEnterpriseOkResponse {
        pub organizations: Vec<OrganizationSimple>,
        pub total_count: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetOrganizationAccessSelfDataHostedRunnerGroupinEnterpriseRequest {
        pub selected_organization_ids: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelfDataHostedRunnersinGroupEnterpriseOkResponse {
        pub runners: Vec<Runner>,
        pub total_count: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetSelfDataHostedRunnersinGroupEnterpriseRequest {
        pub runners: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelfDataHostedRunnersEnterpriseOkResponse {
        pub runners: Option<Vec<Runner>>,
        pub total_count: Option<f64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateGistRequest {
        pub description: Option<String>,
        pub files: Files,
        pub public: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateGistRequest {
        pub description: Option<String>,
        pub files: Option<Files>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateGistCommentRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateGistCommentRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetCheckifGistIsStarredNotFoundResponse {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListRepositoriesAccessibleAppInstallationOkResponse {
        pub repositories: Vec<Repository>,
        pub repository_selection: Option<String>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RenderMarkdownDocumentRequest {
        pub context: Option<String>,
        pub mode: Option<String>,
        pub text: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MarkNotificationsasReadRequest {
        pub last_read_at: Option<DateTime<Utc>>,
        pub read: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PutMarkNotificationsasReadAcceptedResponse {
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetThreadSubscriptionRequest {
        pub ignored: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateOrganizationRequest {
        pub billing_email: Option<String>,
        pub blog: Option<String>,
        pub company: Option<String>,
        pub default_repository_permission: Option<String>,
        pub description: Option<String>,
        pub email: Option<String>,
        pub has_organization_projects: Option<bool>,
        pub has_repository_projects: Option<bool>,
        pub location: Option<String>,
        pub members_allowed_repository_creation_type: Option<String>,
        pub members_can_create_internal_repositories: Option<bool>,
        pub members_can_create_pages: Option<bool>,
        pub members_can_create_private_pages: Option<bool>,
        pub members_can_create_private_repositories: Option<bool>,
        pub members_can_create_public_pages: Option<bool>,
        pub members_can_create_public_repositories: Option<bool>,
        pub members_can_create_repositories: Option<bool>,
        pub name: Option<String>,
        pub twitter_username: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PatchUpdateOrganizationUnprocessableEntityResponse {
        pub documentation_url: String,
        pub errors: Option<Vec<Errors>>,
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetGithubActionsPermissionsOrganizationRequest {
        pub allowed_actions: Option<String>,
        pub enabled_repositories: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelectedRepositoriesEnabledGithubActionsinOrganizationOkResponse {
        pub repositories: Vec<Repository>,
        pub total_count: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetSelectedRepositoriesEnabledGithubActionsinOrganizationRequest {
        pub selected_repository_ids: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelfDataHostedRunnerGroupsOrganizationOkResponse {
        pub runner_groups: Vec<RunnerGroupsOrg>,
        pub total_count: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateSelfDataHostedRunnerGroupOrganizationRequest {
        pub name: String,
        pub runners: Option<Vec<i64>>,
        pub selected_repository_ids: Option<Vec<i64>>,
        pub visibility: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateSelfDataHostedRunnerGroupOrganizationRequest {
        pub name: Option<String>,
        pub visibility: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListRepositoryAccessSelfDataHostedRunnerGroupinOrganizationOkResponse {
        pub repositories: Vec<MinimalRepository>,
        pub total_count: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetRepositoryAccessSelfDataHostedRunnerGroupinOrganizationRequest {
        pub selected_repository_ids: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelfDataHostedRunnersinGroupOrganizationOkResponse {
        pub runners: Vec<Runner>,
        pub total_count: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetSelfDataHostedRunnersinGroupOrganizationRequest {
        pub runners: Vec<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelfDataHostedRunnersOrganizationOkResponse {
        pub runners: Vec<Runner>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListOrganizationSecretsOkResponse {
        pub secrets: Vec<OrganizationActionsSecret>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateOrganizationSecretRequest {
        pub encrypted_value: Option<String>,
        pub key_id: Option<String>,
        pub selected_repository_ids: Option<Vec<String>>,
        pub visibility: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelectedRepositoriesOrganizationSecretOkResponse {
        pub repositories: Vec<MinimalRepository>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetSelectedRepositoriesOrganizationSecretRequest {
        pub selected_repository_ids: Option<Vec<i64>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateOrganizationWebhookRequestConfig {
        pub content_type: Option<String>,
        pub insecure_ssl: Option<String>,
        pub password: Option<String>,
        pub secret: Option<String>,
        pub url: String,
        pub username: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateOrganizationWebhookRequest {
        pub active: Option<bool>,
        pub config: Config,
        pub events: Option<Vec<String>>,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateOrganizationWebhookRequestConfig {
        pub content_type: Option<String>,
        pub insecure_ssl: Option<String>,
        pub secret: Option<String>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateOrganizationWebhookRequest {
        pub active: Option<bool>,
        pub config: Option<Config>,
        pub events: Option<Vec<String>>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateWebhookConfigurationOrganizationRequest {
        pub content_type: Option<String>,
        pub insecure_ssl: Option<String>,
        pub secret: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListAppInstallationsOrganizationOkResponse {
        pub installations: Vec<Installation>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetInteractionRestrictionsOrganizationOkResponse {
        pub expires_at: DateTime<Utc>,
        pub limit: String,
        pub origin: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateOrganizationInvitationRequest {
        pub email: Option<String>,
        pub invitee_id: Option<i64>,
        pub role: Option<String>,
        pub team_ids: Option<Vec<i64>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetOrganizationMembershipUserRequest {
        pub role: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct StartOrganizationMigrationRequest {
        pub exclude: Option<Vec<String>>,
        pub exclude_attachments: Option<bool>,
        pub lock_repositories: Option<bool>,
        pub repositories: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PutConvertOrganizationMemberOutsideCollaboratorAcceptedResponse {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteRemoveOutsideCollaboratorFromOrganizationUnprocessableEntityResponse {
        pub documentation_url: Option<String>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateOrganizationProjectRequest {
        pub body: Option<String>,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateOrganizationRepositoryRequest {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub auto_init: Option<bool>,
        pub delete_branch_on_merge: Option<bool>,
        pub description: Option<String>,
        pub gitignore_template: Option<String>,
        pub has_issues: Option<bool>,
        pub has_projects: Option<bool>,
        pub has_wiki: Option<bool>,
        pub homepage: Option<String>,
        pub is_template: Option<bool>,
        pub license_template: Option<String>,
        pub name: String,
        pub private: Option<bool>,
        pub team_id: Option<i64>,
        pub visibility: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateTeamRequest {
        pub description: Option<String>,
        pub maintainers: Option<Vec<String>>,
        pub name: String,
        pub parent_team_id: Option<i64>,
        pub permission: Option<String>,
        pub privacy: Option<String>,
        pub repo_names: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateTeamRequest {
        pub description: Option<String>,
        pub name: Option<String>,
        pub parent_team_id: Option<i64>,
        pub permission: Option<String>,
        pub privacy: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateDiscussionRequest {
        pub body: String,
        pub private: Option<bool>,
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateDiscussionRequest {
        pub body: Option<String>,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateDiscussionCommentRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateDiscussionCommentRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReactionTeamDiscussionCommentRequest {
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReactionTeamDiscussionRequest {
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddUpdateTeamMembershipUserRequest {
        pub role: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddUpdateTeamProjectPermissionsRequest {
        pub permission: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PutAddUpdateTeamProjectPermissionsForbiddenResponse {
        pub documentation_url: Option<String>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddUpdateTeamRepositoryPermissionsRequest {
        pub permission: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateIdpGroupConnectionsRequestGroups {
        pub group_description: String,
        pub group_id: String,
        pub group_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateIdpGroupConnectionsRequest {
        pub groups: Option<Vec<Groups>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteProjectCardForbiddenResponse {
        pub documentation_url: Option<String>,
        pub errors: Option<Vec<String>>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateExistingProjectCardRequest {
        pub archived: Option<bool>,
        pub note: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MoveProjectCardRequest {
        pub column_id: Option<i64>,
        pub position: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostMoveProjectCardCreatedResponse {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostMoveProjectCardForbiddenResponseErrors {
        pub code: Option<String>,
        pub field: Option<String>,
        pub message: Option<String>,
        pub resource: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostMoveProjectCardForbiddenResponse {
        pub documentation_url: Option<String>,
        pub errors: Option<Vec<Errors>>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostMoveProjectCardServiceUnavailableResponseErrors {
        pub code: Option<String>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostMoveProjectCardServiceUnavailableResponse {
        pub code: Option<String>,
        pub documentation_url: Option<String>,
        pub errors: Option<Vec<Errors>>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateExistingProjectColumnRequest {
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateProjectCardRequest {
        pub note: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostCreateProjectCardUnprocessableEntityResponse {
        pub documentation_url: String,
        pub errors: Option<Vec<Errors>>,
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostCreateProjectCardServiceUnavailableResponseErrors {
        pub code: Option<String>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostCreateProjectCardServiceUnavailableResponse {
        pub code: Option<String>,
        pub documentation_url: Option<String>,
        pub errors: Option<Vec<Errors>>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MoveProjectColumnRequest {
        pub position: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostMoveProjectColumnCreatedResponse {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteProjectForbiddenResponse {
        pub documentation_url: Option<String>,
        pub errors: Option<Vec<String>>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateProjectRequest {
        pub body: Option<String>,
        pub name: Option<String>,
        pub organization_permission: Option<String>,
        pub private: Option<bool>,
        pub state: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PatchUpdateProjectForbiddenResponse {
        pub documentation_url: Option<String>,
        pub errors: Option<Vec<String>>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddProjectCollaboratorRequest {
        pub permission: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateProjectColumnRequest {
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteRepositoryForbiddenResponse {
        pub documentation_url: Option<String>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateRepositoryRequest {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub archived: Option<bool>,
        pub default_branch: Option<String>,
        pub delete_branch_on_merge: Option<bool>,
        pub description: Option<String>,
        pub has_issues: Option<bool>,
        pub has_projects: Option<bool>,
        pub has_wiki: Option<bool>,
        pub homepage: Option<String>,
        pub is_template: Option<bool>,
        pub name: Option<String>,
        pub private: Option<bool>,
        pub security_and_analysis: Option<SecurityandAnalysis>,
        pub visibility: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListArtifactsRepositoryOkResponse {
        pub artifacts: Vec<Artifact>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetGithubActionsPermissionsRepositoryRequest {
        pub allowed_actions: Option<String>,
        pub enabled: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListSelfDataHostedRunnersRepositoryOkResponse {
        pub runners: Vec<Runner>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListWorkflowRunsRepositoryOkResponse {
        pub total_count: i64,
        pub workflow_runs: Vec<WorkflowRun>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListWorkflowRunArtifactsOkResponse {
        pub artifacts: Vec<Artifact>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostCancelWorkflowRunAcceptedResponse {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListJobsWorkflowRunOkResponse {
        pub jobs: Vec<Job>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReviewPendingDeploymentsWorkflowRunRequest {
        pub comment: String,
        pub environment_ids: Vec<i64>,
        pub state: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostReRunWorkflowCreatedResponse {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListRepositorySecretsOkResponse {
        pub secrets: Vec<ActionsSecret>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateRepositorySecretRequest {
        pub encrypted_value: Option<String>,
        pub key_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PutCreateUpdateRepositorySecretCreatedResponse {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListRepositoryWorkflowsOkResponse {
        pub total_count: i64,
        pub workflows: Vec<Workflow>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Inputs {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateWorkflowDispatchEventRequest {
        pub inputs: Option<Inputs>,
        #[serde(rename = "ref")]
        pub ref_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListWorkflowRunsOkResponse {
        pub total_count: i64,
        pub workflow_runs: Vec<WorkflowRun>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateBranchProtectionRequestRequiredStatusChecks {
        pub contexts: Vec<String>,
        pub strict: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateBranchProtectionRequestRequiredPullRequestReviewsRequiredPullRequestReviewsDismissalRestrictions
    {
        pub teams: Option<Vec<String>>,
        pub users: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateBranchProtectionRequestRequiredPullRequestReviews {
        pub dismiss_stale_reviews: Option<bool>,
        pub dismissal_restrictions: Option<DismissalRestrictions>,
        pub require_code_owner_reviews: Option<bool>,
        pub required_approving_review_count: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Restrictions {
        pub apps: Option<Vec<String>>,
        pub teams: Vec<String>,
        pub users: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateBranchProtectionRequest {
        pub allow_deletions: Option<bool>,
        pub allow_force_pushes: Option<bool>,
        pub enforce_admins: bool,
        pub required_conversation_resolution: Option<bool>,
        pub required_linear_history: Option<bool>,
        pub required_pull_request_reviews: RequiredPullRequestReviews,
        pub required_status_checks: RequiredStatusChecks,
        pub restrictions: Restrictions,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdatePullRequestReviewProtectionRequestDismissalRestrictions {
        pub teams: Option<Vec<String>>,
        pub users: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdatePullRequestReviewProtectionRequest {
        pub dismiss_stale_reviews: Option<bool>,
        pub dismissal_restrictions: Option<DismissalRestrictions>,
        pub require_code_owner_reviews: Option<bool>,
        pub required_approving_review_count: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateStatusCheckProtectionRequest {
        pub contexts: Option<Vec<String>>,
        pub strict: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddStatusCheckContextsRequest {
        pub contexts: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetStatusCheckContextsRequest {
        pub contexts: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RemoveStatusCheckContextsRequest {
        pub contexts: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddAppAccessRestrictionsRequest {
        pub apps: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetAppAccessRestrictionsRequest {
        pub apps: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RemoveAppAccessRestrictionsRequest {
        pub apps: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddTeamAccessRestrictionsRequest {
        pub teams: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetTeamAccessRestrictionsRequest {
        pub teams: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RemoveTeamAccessRestrictionsRequest {
        pub teams: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddUserAccessRestrictionsRequest {
        pub users: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetUserAccessRestrictionsRequest {
        pub users: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RemoveUserAccessRestrictionsRequest {
        pub users: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RenameBranchRequest {
        pub new_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Annotations {
        pub annotation_level: String,
        pub end_column: Option<i64>,
        pub end_line: i64,
        pub message: String,
        pub path: String,
        pub raw_details: Option<String>,
        pub start_column: Option<i64>,
        pub start_line: i64,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Images {
        pub alt: String,
        pub caption: Option<String>,
        pub image_url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateCheckRunRequestOutput {
        pub annotations: Option<Vec<Annotations>>,
        pub images: Option<Vec<Images>>,
        pub summary: String,
        pub text: Option<String>,
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Actions {
        pub description: String,
        pub identifier: String,
        pub label: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateCheckRunRequest {
        pub actions: Option<Vec<Actions>>,
        pub completed_at: Option<DateTime<Utc>>,
        pub conclusion: Option<String>,
        pub details_url: Option<String>,
        pub external_id: Option<String>,
        pub head_sha: String,
        pub name: String,
        pub output: Option<Output>,
        pub started_at: Option<DateTime<Utc>>,
        pub status: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateCheckRunRequestOutput {
        pub annotations: Option<Vec<Annotations>>,
        pub images: Option<Vec<Images>>,
        pub summary: String,
        pub text: Option<String>,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateCheckRunRequest {
        pub actions: Option<Vec<Actions>>,
        pub completed_at: Option<DateTime<Utc>>,
        pub conclusion: Option<String>,
        pub details_url: Option<String>,
        pub external_id: Option<String>,
        pub name: Option<String>,
        pub output: Option<Output>,
        pub started_at: Option<DateTime<Utc>>,
        pub status: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateCheckSuiteRequest {
        pub head_sha: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateRepositoryPreferencesCheckSuitesRequest {
        pub auto_trigger_checks: Option<Vec<AutoTriggerChecks>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListCheckRunsinCheckSuiteOkResponse {
        pub check_runs: Vec<CheckRun>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostRerequestCheckSuiteCreatedResponse {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateCodeScanningAlertRequest {
        pub dismissed_reason: Option<serde_json::Value>,
        pub state: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UploadAnalysisasSarifDataRequest {
        pub checkout_uri: Option<String>,
        pub commit_sha: String,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub sarif: String,
        pub started_at: Option<DateTime<Utc>>,
        pub tool_name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddRepositoryCollaboratorRequest {
        pub permission: Option<String>,
        pub permissions: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateCommitCommentRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReactionCommitCommentRequest {
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateCommitCommentRequest {
        pub body: String,
        pub line: Option<i64>,
        pub path: Option<String>,
        pub position: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListCheckRunsGitReferenceOkResponse {
        pub check_runs: Vec<CheckRun>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListCheckSuitesGitReferenceOkResponse {
        pub check_suites: Vec<CheckSuite>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateContentAttachmentRequest {
        pub body: String,
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetRepositoryContentOkResponse {
        pub links: Links,
        pub content: Option<String>,
        pub download_url: String,
        pub git_url: String,
        pub html_url: String,
        pub name: String,
        pub path: String,
        pub sha: String,
        pub size: i64,
        #[serde(rename = "type")]
        pub type_: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateFileContentsRequestCommitter {
        pub date: Option<String>,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateFileContentsRequestAuthor {
        pub date: Option<String>,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateFileContentsRequest {
        pub author: Option<Author>,
        pub branch: Option<String>,
        pub committer: Option<Committer>,
        pub content: String,
        pub message: String,
        pub sha: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteFileRequestCommitter {
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteFileRequestAuthor {
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteFileRequest {
        pub author: Option<Author>,
        pub branch: Option<String>,
        pub committer: Option<Committer>,
        pub message: String,
        pub sha: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateDeploymentRequest {
        pub auto_merge: Option<bool>,
        pub description: Option<String>,
        pub environment: Option<String>,
        pub payload: Option<Payload>,
        pub production_environment: Option<bool>,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub required_contexts: Option<Vec<String>>,
        pub task: Option<String>,
        pub transient_environment: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PostCreateDeploymentAcceptedResponse {
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateDeploymentStatusRequest {
        pub auto_inactive: Option<bool>,
        pub description: Option<String>,
        pub environment: Option<String>,
        pub environment_url: Option<String>,
        pub log_url: Option<String>,
        pub state: String,
        pub target_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ClientPayload {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateRepositoryDispatchEventRequest {
        pub client_payload: Option<ClientPayload>,
        pub event_type: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetAllEnvironmentsOkResponse {
        pub environments: Option<Vec<Environment>>,
        pub total_count: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateEnvironmentRequestReviewers {
        pub id: Option<i64>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateEnvironmentRequest {
        pub deployment_branch_policy: Option<DeploymentBranchPolicy>,
        pub reviewers: Option<Vec<Reviewers>>,
        pub wait_timer: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateForkRequest {
        pub organization: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateBlobRequest {
        pub content: String,
        pub encoding: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateCommitRequestAuthor {
        pub date: Option<DateTime<Utc>>,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateCommitRequestCommitter {
        pub date: Option<DateTime<Utc>>,
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateCommitRequest {
        pub author: Option<Author>,
        pub committer: Option<Committer>,
        pub message: String,
        pub parents: Option<Vec<String>>,
        pub signature: Option<String>,
        pub tree: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReferenceRequest {
        pub key: Option<String>,
        #[serde(rename = "ref")]
        pub ref_: String,
        pub sha: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateReferenceRequest {
        pub force: Option<bool>,
        pub sha: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateTagObjectRequestTagger {
        pub date: Option<DateTime<Utc>>,
        pub email: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateTagObjectRequest {
        pub message: String,
        pub object: String,
        pub tag: String,
        pub tagger: Option<Tagger>,
        #[serde(rename = "type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateTreeRequestTree {
        pub content: Option<String>,
        pub mode: Option<String>,
        pub path: Option<String>,
        pub sha: Option<String>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateTreeRequest {
        pub base_tree: Option<String>,
        pub tree: Vec<Tree>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateRepositoryWebhookRequestConfig {
        pub content_type: Option<String>,
        pub digest: Option<String>,
        pub insecure_ssl: Option<String>,
        pub secret: Option<String>,
        pub token: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateRepositoryWebhookRequest {
        pub active: Option<bool>,
        pub config: Option<Config>,
        pub events: Option<Vec<String>>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateRepositoryWebhookRequestConfig {
        pub address: Option<String>,
        pub content_type: Option<String>,
        pub insecure_ssl: Option<String>,
        pub room: Option<String>,
        pub secret: Option<String>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateRepositoryWebhookRequest {
        pub active: Option<bool>,
        pub add_events: Option<Vec<String>>,
        pub config: Option<Config>,
        pub events: Option<Vec<String>>,
        pub remove_events: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateWebhookConfigurationRepositoryRequest {
        pub content_type: Option<String>,
        pub insecure_ssl: Option<String>,
        pub secret: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct StartImportRequest {
        pub tfvc_project: Option<String>,
        pub vcs: Option<String>,
        pub vcs_password: Option<String>,
        pub vcs_url: String,
        pub vcs_username: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateImportRequest {
        pub tfvc_project: Option<String>,
        pub vcs: Option<String>,
        pub vcs_password: Option<String>,
        pub vcs_username: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MapCommitAuthorRequest {
        pub email: Option<String>,
        pub name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateGitLfsPreferenceRequest {
        pub use_lfs: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetInteractionRestrictionsRepositoryOkResponse {
        pub expires_at: DateTime<Utc>,
        pub limit: String,
        pub origin: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateRepositoryInvitationRequest {
        pub permissions: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateIssueRequest {
        pub assignee: Option<String>,
        pub assignees: Option<Vec<String>>,
        pub body: Option<String>,
        pub labels: Option<Vec<Labels>>,
        pub milestone: Option<Milestone>,
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateIssueCommentRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReactionIssueCommentRequest {
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateIssueRequest {
        pub assignee: Option<String>,
        pub assignees: Option<Vec<String>>,
        pub body: Option<String>,
        pub labels: Option<Vec<Labels>>,
        pub milestone: Option<Milestone>,
        pub state: Option<String>,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddAssigneesIssueRequest {
        pub assignees: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RemoveAssigneesFromIssueRequest {
        pub assignees: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateIssueCommentRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddLabelsIssueRequest {
        pub labels: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetLabelsIssueRequest {
        pub labels: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct LockIssueRequest {
        pub lock_reason: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReactionIssueRequest {
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateDeployKeyRequest {
        pub key: String,
        pub read_only: Option<bool>,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateLabelRequest {
        pub color: Option<String>,
        pub description: Option<String>,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateLabelRequest {
        pub color: Option<String>,
        pub description: Option<String>,
        pub new_name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MergeBranchRequest {
        pub base: String,
        pub commit_message: Option<String>,
        pub head: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateMilestoneRequest {
        pub description: Option<String>,
        pub due_on: Option<DateTime<Utc>>,
        pub state: Option<String>,
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateMilestoneRequest {
        pub description: Option<String>,
        pub due_on: Option<DateTime<Utc>>,
        pub state: Option<String>,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MarkRepositoryNotificationsasReadRequest {
        pub last_read_at: Option<DateTime<Utc>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PutMarkRepositoryNotificationsasReadAcceptedResponse {
        pub message: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateGithubPagesSiteRequestSource {
        pub branch: String,
        pub path: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateGithubPagesSiteRequest {
        pub source: Source,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateInformationAboutGithubPagesSiteRequest {
        pub cname: Option<String>,
        pub https_enforced: Option<bool>,
        pub public: Option<bool>,
        pub source: Option<Source>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateRepositoryProjectRequest {
        pub body: Option<String>,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreatePullRequestRequest {
        pub base: String,
        pub body: Option<String>,
        pub draft: Option<bool>,
        pub head: String,
        pub issue: Option<i64>,
        pub maintainer_can_modify: Option<bool>,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateReviewCommentPullRequestRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReactionPullRequestReviewCommentRequest {
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdatePullRequestRequest {
        pub base: Option<String>,
        pub body: Option<String>,
        pub maintainer_can_modify: Option<bool>,
        pub state: Option<String>,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReviewCommentPullRequestRequest {
        pub body: String,
        pub commit_id: Option<String>,
        pub in_reply_to: Option<i64>,
        pub line: Option<i64>,
        pub path: Option<String>,
        pub position: Option<i64>,
        pub side: Option<String>,
        pub start_line: Option<i64>,
        pub start_side: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReplyReviewCommentRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct MergePullRequestRequest {
        pub commit_message: Option<String>,
        pub commit_title: Option<String>,
        pub merge_method: Option<String>,
        pub sha: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PutMergePullRequestMethodNotAllowedResponse {
        pub documentation_url: Option<String>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PutMergePullRequestConflictResponse {
        pub documentation_url: Option<String>,
        pub message: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RequestReviewersPullRequestRequest {
        pub reviewers: Option<Vec<String>>,
        pub team_reviewers: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct RemoveRequestedReviewersFromPullRequestRequest {
        pub reviewers: Vec<String>,
        pub team_reviewers: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct Comments {
        pub body: String,
        pub line: Option<i64>,
        pub path: String,
        pub position: Option<i64>,
        pub side: Option<String>,
        pub start_line: Option<i64>,
        pub start_side: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReviewPullRequestRequest {
        pub body: Option<String>,
        pub comments: Option<Vec<Comments>>,
        pub commit_id: Option<String>,
        pub event: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateReviewPullRequestRequest {
        pub body: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DismissReviewPullRequestRequest {
        pub event: Option<String>,
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SubmitReviewPullRequestRequest {
        pub body: Option<String>,
        pub event: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdatePullRequestBranchRequest {
        pub expected_head_sha: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct PutUpdatePullRequestBranchAcceptedResponse {
        pub message: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReleaseRequest {
        pub body: Option<String>,
        pub discussion_category_name: Option<String>,
        pub draft: Option<bool>,
        pub name: Option<String>,
        pub prerelease: Option<bool>,
        pub tag_name: String,
        pub target_commitish: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateReleaseAssetRequest {
        pub label: Option<String>,
        pub name: Option<String>,
        pub state: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateReleaseRequest {
        pub body: Option<String>,
        pub discussion_category_name: Option<String>,
        pub draft: Option<bool>,
        pub name: Option<String>,
        pub prerelease: Option<bool>,
        pub tag_name: Option<String>,
        pub target_commitish: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateReactionReleaseRequest {
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateSecretScanningAlertRequest {
        pub resolution: Option<serde_json::Value>,
        pub state: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateCommitStatusRequest {
        pub context: Option<String>,
        pub description: Option<String>,
        pub state: String,
        pub target_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetRepositorySubscriptionRequest {
        pub ignored: Option<bool>,
        pub subscribed: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ReplaceAllRepositoryTopicsRequest {
        pub names: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct TransferRepositoryRequest {
        pub new_owner: String,
        pub team_ids: Option<Vec<i64>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateRepositoryUsingTemplateRequest {
        pub description: Option<String>,
        pub include_all_branches: Option<bool>,
        pub name: String,
        pub owner: Option<String>,
        pub private: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListEnvironmentSecretsOkResponse {
        pub secrets: Vec<ActionsSecret>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateEnvironmentSecretRequest {
        pub encrypted_value: Option<String>,
        pub key_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProvisionScimEnterpriseGroupandInviteUsersRequestMembers {
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProvisionScimEnterpriseGroupandInviteUsersRequest {
        pub display_name: String,
        pub members: Option<Vec<Members>>,
        pub schemas: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetScimInformationProvisionedEnterpriseGroupRequestMembers {
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetScimInformationProvisionedEnterpriseGroupRequest {
        pub display_name: String,
        pub members: Option<Vec<Members>>,
        pub schemas: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateAttributeScimEnterpriseGroupRequest {
        pub operations: Vec<Operations>,
        pub schemas: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProvisionandInviteScimEnterpriseUserRequestName {
        pub family_name: String,
        pub given_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProvisionandInviteScimEnterpriseUserRequestEmails {
        pub primary: bool,
        #[serde(rename = "type")]
        pub type_: String,
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProvisionandInviteScimEnterpriseUserRequestGroups {
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProvisionandInviteScimEnterpriseUserRequest {
        pub emails: Vec<Emails>,
        pub groups: Option<Vec<Groups>>,
        pub name: Name,
        pub schemas: Vec<String>,
        pub user_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetScimInformationProvisionedEnterpriseUserRequestName {
        pub family_name: String,
        pub given_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetScimInformationProvisionedEnterpriseUserRequestEmails {
        pub primary: bool,
        #[serde(rename = "type")]
        pub type_: String,
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetScimInformationProvisionedEnterpriseUserRequestGroups {
        pub value: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetScimInformationProvisionedEnterpriseUserRequest {
        pub emails: Vec<Emails>,
        pub groups: Option<Vec<Groups>>,
        pub name: Name,
        pub schemas: Vec<String>,
        pub user_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateAttributeScimEnterpriseUserRequestOperations {}

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateAttributeScimEnterpriseUserRequest {
        pub operations: Vec<Operations>,
        pub schemas: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProvisionandInviteScimUserRequestName {
        pub family_name: String,
        pub formatted: Option<String>,
        pub given_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProvisionandInviteScimUserRequestEmails {
        pub primary: Option<bool>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ProvisionandInviteScimUserRequest {
        pub active: Option<bool>,
        pub display_name: Option<String>,
        pub emails: Vec<Emails>,
        pub external_id: Option<String>,
        pub groups: Option<Vec<String>>,
        pub name: Name,
        pub schemas: Option<Vec<String>>,
        pub user_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateProvisionedOrganizationMembershipRequestName {
        pub family_name: String,
        pub formatted: Option<String>,
        pub given_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateProvisionedOrganizationMembershipRequestEmails {
        pub primary: Option<bool>,
        #[serde(rename = "type")]
        pub type_: Option<String>,
        pub value: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateProvisionedOrganizationMembershipRequest {
        pub active: Option<bool>,
        pub display_name: Option<String>,
        pub emails: Vec<Emails>,
        pub external_id: Option<String>,
        pub groups: Option<Vec<String>>,
        pub name: Name,
        pub schemas: Option<Vec<String>>,
        pub user_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct ValueData {
        pub active: Option<bool>,
        pub external_id: Option<String>,
        pub family_name: Option<String>,
        pub given_name: Option<String>,
        pub user_name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateAttributeScimUserRequest {
        pub operations: Vec<Operations>,
        pub schemas: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetSearchCodeOkResponse {
        pub incomplete_results: bool,
        pub items: Vec<CodeSearchResultItem>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetSearchCommitsOkResponse {
        pub incomplete_results: bool,
        pub items: Vec<CommitSearchResultItem>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetSearchIssuesandPullRequestsOkResponse {
        pub incomplete_results: bool,
        pub items: Vec<IssueSearchResultItem>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetSearchLabelsOkResponse {
        pub incomplete_results: bool,
        pub items: Vec<LabelSearchResultItem>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetSearchRepositoriesOkResponse {
        pub incomplete_results: bool,
        pub items: Vec<RepoSearchResultItem>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetSearchTopicsOkResponse {
        pub incomplete_results: bool,
        pub items: Vec<TopicSearchResultItem>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetSearchUsersOkResponse {
        pub incomplete_results: bool,
        pub items: Vec<UserSearchResultItem>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateTeamRequestData {
        pub description: Option<String>,
        pub name: String,
        pub parent_team_id: Option<i64>,
        pub permission: Option<String>,
        pub privacy: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUpdateIdpGroupConnectionsRequestData {
        pub groups: Vec<Groups>,
        pub synced_at: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetOkResponse {
        pub avatar_url: String,
        pub bio: String,
        pub blog: String,
        pub business_plus: Option<bool>,
        pub collaborators: i64,
        pub company: String,
        pub created_at: DateTime<Utc>,
        pub disk_usage: i64,
        pub email: String,
        pub events_url: String,
        pub followers: i64,
        pub followers_url: String,
        pub following: i64,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub hireable: bool,
        pub html_url: String,
        pub id: i64,
        pub ldap_dn: Option<String>,
        pub location: String,
        pub login: String,
        pub name: String,
        pub node_id: String,
        pub organizations_url: String,
        pub owned_private_repos: i64,
        pub plan: Option<Plan>,
        pub private_gists: i64,
        pub public_gists: i64,
        pub public_repos: i64,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        pub suspended_at: Option<DateTime<Utc>>,
        pub total_private_repos: i64,
        pub twitter_username: Option<String>,
        pub two_factor_authentication: bool,
        #[serde(rename = "type")]
        pub type_: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateRequest {
        pub bio: Option<String>,
        pub blog: Option<String>,
        pub company: Option<String>,
        pub email: Option<String>,
        pub hireable: Option<bool>,
        pub location: Option<String>,
        pub name: Option<String>,
        pub twitter_username: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct SetPrimaryEmailVisibilityRequest {
        pub visibility: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct AddEmailAddressRequest {
        pub emails: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct DeleteEmailAddressRequest {
        pub emails: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateGpgKeyRequest {
        pub armored_public_key: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListAppInstallationsAccessibleUserAccessTokenOkResponse {
        pub installations: Vec<Installation>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetListRepositoriesAccessibleUserAccessTokenOkResponse {
        pub repositories: Vec<Repository>,
        pub repository_selection: Option<String>,
        pub total_count: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetInteractionRestrictionsPublicRepositoriesOkResponse {
        pub expires_at: DateTime<Utc>,
        pub limit: String,
        pub origin: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreatePublicSshKeyRequest {
        pub key: String,
        pub title: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct UpdateOrganizationMembershipRequest {
        pub state: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct StartUserMigrationRequest {
        pub exclude: Option<Vec<String>>,
        pub exclude_attachments: Option<bool>,
        pub lock_repositories: Option<bool>,
        pub repositories: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateUserProjectRequest {
        pub body: Option<String>,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct CreateRepositoryRequest {
        pub allow_merge_commit: Option<bool>,
        pub allow_rebase_merge: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub auto_init: Option<bool>,
        pub delete_branch_on_merge: Option<bool>,
        pub description: Option<String>,
        pub gitignore_template: Option<String>,
        pub has_downloads: Option<bool>,
        pub has_issues: Option<bool>,
        pub has_projects: Option<bool>,
        pub has_wiki: Option<bool>,
        pub homepage: Option<String>,
        pub is_template: Option<bool>,
        pub license_template: Option<String>,
        pub name: String,
        pub private: Option<bool>,
        pub team_id: Option<i64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
    pub struct GetUserOkResponse {
        pub avatar_url: String,
        pub bio: String,
        pub blog: String,
        pub business_plus: Option<bool>,
        pub collaborators: i64,
        pub company: String,
        pub created_at: DateTime<Utc>,
        pub disk_usage: i64,
        pub email: String,
        pub events_url: String,
        pub followers: i64,
        pub followers_url: String,
        pub following: i64,
        pub following_url: String,
        pub gists_url: String,
        pub gravatar_id: String,
        pub hireable: bool,
        pub html_url: String,
        pub id: i64,
        pub ldap_dn: Option<String>,
        pub location: String,
        pub login: String,
        pub name: String,
        pub node_id: String,
        pub organizations_url: String,
        pub owned_private_repos: i64,
        pub plan: Option<Plan>,
        pub private_gists: i64,
        pub public_gists: i64,
        pub public_repos: i64,
        pub received_events_url: String,
        pub repos_url: String,
        pub site_admin: bool,
        pub starred_url: String,
        pub subscriptions_url: String,
        pub suspended_at: Option<DateTime<Utc>>,
        pub total_private_repos: i64,
        pub twitter_username: Option<String>,
        pub two_factor_authentication: bool,
        #[serde(rename = "type")]
        pub type_: String,
        pub updated_at: DateTime<Utc>,
        pub url: String,
    }
}

pub struct Client {
    host: String,
    agent: String,
    client: reqwest::Client,
    credentials: Option<crate::auth::Credentials>,
    #[cfg(feature = "httpcache")]
    http_cache: crate::http_cache::BoxedHttpCache,
}

impl Client {
    pub fn new<A, C>(agent: A, credentials: C) -> Result<Self>
    where
        A: Into<String>,
        C: Into<Option<crate::auth::Credentials>>,
    {
        Self::host(DEFAULT_HOST, agent, credentials)
    }

    pub fn host<H, A, C>(host: H, agent: A, credentials: C) -> Result<Self>
    where
        H: Into<String>,
        A: Into<String>,
        C: Into<Option<crate::auth::Credentials>>,
    {
        let http = reqwest::Client::builder().build()?;
        #[cfg(feature = "httpcache")]
        {
            Ok(Self::custom(
                host,
                agent,
                credentials,
                http,
                crate::http_cache::HttpCache::noop(),
            ))
        }
        #[cfg(not(feature = "httpcache"))]
        {
            Ok(Self::custom(host, agent, credentials, http))
        }
    }

    #[cfg(feature = "httpcache")]
    pub fn custom<H, A, CR>(
        host: H,
        agent: A,
        credentials: CR,
        http: reqwest::Client,
        http_cache: crate::http_cache::BoxedHttpCache,
    ) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
            http_cache,
        }
    }

    #[cfg(not(feature = "httpcache"))]
    pub fn custom<H, A, CR>(host: H, agent: A, credentials: CR, http: Client) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
        }
    }

    pub fn set_credentials<CR>(&mut self, credentials: CR)
    where
        CR: Into<Option<crate::auth::Credentials>>,
    {
        self.credentials = credentials.into();
    }

    fn credentials(
        &self,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Option<&crate::auth::Credentials> {
        match (authentication, self.credentials.as_ref()) {
            (crate::auth::AuthenticationConstraint::Unconstrained, creds) => creds,
            (
                crate::auth::AuthenticationConstraint::JWT,
                creds @ Some(&crate::auth::Credentials::JWT(_)),
            ) => creds,
            (
                crate::auth::AuthenticationConstraint::JWT,
                Some(&crate::auth::Credentials::InstallationToken(ref apptoken)),
            ) => Some(apptoken.jwt()),
            (crate::auth::AuthenticationConstraint::JWT, creds) => {
                println!(
                    "Request needs JWT authentication but only {:?} available",
                    creds
                );
                None
            }
        }
    }

    async fn url_and_auth(
        &self,
        uri: &str,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();

        match self.credentials(authentication) {
            Some(&crate::auth::Credentials::Client(ref id, ref secret)) => parsed_url
                .map(|mut u| {
                    u.query_pairs_mut()
                        .append_pair("client_id", id)
                        .append_pair("client_secret", secret);
                    (u, None)
                })
                .map_err(Error::from),
            Some(&crate::auth::Credentials::Token(ref token)) => {
                let auth = format!("token {}", token);
                parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
            }
            Some(&crate::auth::Credentials::JWT(ref jwt)) => {
                let auth = format!("Bearer {}", jwt.token());
                parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
            }
            Some(&crate::auth::Credentials::InstallationToken(ref apptoken)) => {
                if let Some(token) = apptoken.token() {
                    let auth = format!("token {}", token);
                    parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
                } else {
                    println!("App token is stale, refreshing");
                    let token_ref = apptoken.access_key.clone();

                    let token = self
                        .apps_create_installation_access_token(
                            apptoken.installation_id as i64,
                            &types::CreateInstallationAccessTokenAppRequest {
                                permissions: Default::default(),
                                repositories: Default::default(),
                                repository_ids: Default::default(),
                            },
                        )
                        .await
                        .unwrap();
                    let auth = format!("token {}", &token.token);
                    *token_ref.lock().unwrap() = Some(token.token);
                    parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
                }
            }
            None => parsed_url.map(|u| (u, None)).map_err(Error::from),
        }
    }

    async fn request<Out>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<Vec<u8>>,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<(Option<hyperx::header::Link>, Out)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        #[cfg(feature = "httpcache")]
        let uri2 = uri.to_string();

        let (url, auth) = self.url_and_auth(uri, authentication).await?;

        let instance = <&Client>::clone(&self);

        #[cfg(not(feature = "httpcache"))]
        let mut req = instance.client.request(method, url);

        #[cfg(feature = "httpcache")]
        let mut req = {
            let mut req = instance.client.request(method.clone(), url);
            if method == http::Method::GET {
                if let Ok(etag) = instance.http_cache.lookup_etag(&uri2) {
                    req = req.header(http::header::IF_NONE_MATCH, etag);
                }
            }
            req
        };

        req = req.header(http::header::USER_AGENT, &*instance.agent);
        req = req.header(
            http::header::ACCEPT,
            &*format!(
                "{}",
                hyperx::header::qitem::<mime::Mime>(From::from(media_type))
            ),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        println!("Body: {:?}", &body);
        if let Some(body) = body {
            req = req.body(reqwest::Body::from(body));
        }
        println!("Request: {:?}", &req);
        let response = req.send().await?;

        #[cfg(feature = "httpcache")]
        let instance2 = <&Client>::clone(&self);

        #[cfg(feature = "httpcache")]
        let uri3 = uri.to_string();

        #[cfg(not(feature = "httpcache"))]
        let (remaining, reset) = crate::utils::get_header_values(response.headers());

        #[cfg(feature = "httpcache")]
        let (remaining, reset, etag) = crate::utils::get_header_values(response.headers());

        let status = response.status();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| l.parse().ok());

        let response_body = response.bytes().await?;

        if status.is_success() {
            println!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            #[cfg(feature = "httpcache")]
            {
                if let Some(etag) = etag {
                    let next_link = link.as_ref().and_then(|l| crate::utils::next_link(&l));
                    if let Err(e) = instance2.http_cache.cache_response(
                        &uri3,
                        &response_body,
                        &etag,
                        &next_link,
                    ) {
                        // failing to cache isn't fatal, so just log & swallow the error
                        println!("Failed to cache body & etag: {}", e);
                    }
                }
            }
            let parsed_response = if status == http::StatusCode::NO_CONTENT {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map(|out| (link, out)).map_err(Error::from)
        } else if status == http::StatusCode::NOT_MODIFIED {
            // only supported case is when client provides if-none-match
            // header when cargo builds with --cfg feature="httpcache"
            #[cfg(feature = "httpcache")]
            {
                let body = instance2.http_cache.lookup_body(&uri3).unwrap();
                let out = serde_json::from_str::<Out>(&body).unwrap();
                let link = match link {
                    Some(link) => Ok(Some(link)),
                    None => instance2
                        .http_cache
                        .lookup_next_link(&uri3)
                        .map(|next_link| {
                            next_link.map(|next| {
                                let next = hyperx::header::LinkValue::new(next)
                                    .push_rel(hyperx::header::RelationType::Next);
                                hyperx::header::Link::new(vec![next])
                            })
                        }),
                };
                link.map(|link| (link, out))
            }
            #[cfg(not(feature = "httpcache"))]
            {
                unreachable!("this should not be reachable without the httpcache feature enabled")
            }
        } else {
            let error = match (remaining, reset) {
                (Some(remaining), Some(reset)) if remaining == 0 => {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    anyhow!(
                        "rate limit exceeded, will reset in {} seconds",
                        u64::from(reset) - now
                    )
                }
                _ => anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    serde_json::from_slice(&response_body)?
                ),
            };
            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<Vec<u8>>,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let (_, r) = self
            .request(method, uri, body, media_type, authentication)
            .await?;
        Ok(r)
    }

    async fn get<D>(&self, uri: &str) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.get_media(uri, crate::utils::MediaType::Json).await
    }

    async fn get_media<D>(&self, uri: &str, media: crate::utils::MediaType) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::GET,
            &(self.host.clone() + uri),
            None,
            media,
            self::auth::AuthenticationConstraint::Unconstrained,
        )
        .await
    }

    /**
     * meta_root: GET /
     */
    pub async fn meta_root(&self) -> Result<types::GetGithubApiRootOkResponse> {
        let url = "".to_string();
        self.get(&url).await
    }

    /**
     * apps_get_authenticated: GET /app
     */
    pub async fn apps_get_authenticated(&self) -> Result<types::Integration> {
        let url = "/app".to_string();
        self.get(&url).await
    }

    /**
     * apps_create_from_manifest: POST /app-manifests/{code}/conversions
     */
    pub async fn apps_create_from_manifest(
        &self,
        code: &str,
        body: &types::CreateGithubAppFromManifestRequest,
    ) -> Result<types::PostCreateGithubAppFromManifestCreatedResponse> {
        let url = format!(
            "/app-manifests/{}/conversions",
            progenitor_support::encode_path(&code.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * apps_get_webhook_config_for_app: GET /app/hook/config
     */
    pub async fn apps_get_webhook_config_for_app(&self) -> Result<types::WebhookConfig> {
        let url = "/app/hook/config".to_string();
        self.get(&url).await
    }

    /**
     * apps_update_webhook_config_for_app: PATCH /app/hook/config
     */
    pub async fn apps_update_webhook_config_for_app(
        &self,
        body: &types::UpdateWebhookConfigurationAppRequest,
    ) -> Result<types::WebhookConfig> {
        let url = "/app/hook/config".to_string();
        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * apps_list_installations: GET /app/installations
     */
    pub async fn apps_list_installations(
        &self,
        per_page: i64,
        page: i64,
        since: DateTime<Utc>,
        outdated: &str,
    ) -> Result<Vec<types::Installation>> {
        let url = "/app/installations".to_string();
        self.get(&url).await
    }

    /**
     * apps_get_installation: GET /app/installations/{installation_id}
     */
    pub async fn apps_get_installation(&self, installation_id: i64) -> Result<types::Installation> {
        let url = format!(
            "/app/installations/{}",
            progenitor_support::encode_path(&installation_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * apps_delete_installation: DELETE /app/installations/{installation_id}
     */
    pub async fn apps_delete_installation(&self, installation_id: i64) -> Result<()> {
        let url = format!(
            "/app/installations/{}",
            progenitor_support::encode_path(&installation_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_create_installation_access_token: POST /app/installations/{installation_id}/access_tokens
     */
    pub async fn apps_create_installation_access_token(
        &self,
        installation_id: i64,
        body: &types::CreateInstallationAccessTokenAppRequest,
    ) -> Result<types::InstallationToken> {
        let url = format!(
            "/app/installations/{}/access_tokens",
            progenitor_support::encode_path(&installation_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * apps_suspend_installation: PUT /app/installations/{installation_id}/suspended
     */
    pub async fn apps_suspend_installation(&self, installation_id: i64) -> Result<()> {
        let url = format!(
            "/app/installations/{}/suspended",
            progenitor_support::encode_path(&installation_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_unsuspend_installation: DELETE /app/installations/{installation_id}/suspended
     */
    pub async fn apps_unsuspend_installation(&self, installation_id: i64) -> Result<()> {
        let url = format!(
            "/app/installations/{}/suspended",
            progenitor_support::encode_path(&installation_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * oauth_authorizations_list_grants: GET /applications/grants
     */
    pub async fn oauth_authorizations_list_grants(
        &self,
        per_page: i64,
        page: i64,
        client_id: &str,
    ) -> Result<Vec<types::ApplicationGrant>> {
        let url = "/applications/grants".to_string();
        self.get(&url).await
    }

    /**
     * oauth_authorizations_get_grant: GET /applications/grants/{grant_id}
     */
    pub async fn oauth_authorizations_get_grant(
        &self,
        grant_id: i64,
    ) -> Result<types::ApplicationGrant> {
        let url = format!(
            "/applications/grants/{}",
            progenitor_support::encode_path(&grant_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * oauth_authorizations_delete_grant: DELETE /applications/grants/{grant_id}
     */
    pub async fn oauth_authorizations_delete_grant(&self, grant_id: i64) -> Result<()> {
        let url = format!(
            "/applications/grants/{}",
            progenitor_support::encode_path(&grant_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_delete_authorization: DELETE /applications/{client_id}/grant
     */
    pub async fn apps_delete_authorization(
        &self,
        client_id: &str,
        body: &types::DeleteAppAuthorizationRequest,
    ) -> Result<()> {
        let url = format!(
            "/applications/{}/grant",
            progenitor_support::encode_path(&client_id.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_revoke_grant_for_application: DELETE /applications/{client_id}/grants/{access_token}
     */
    pub async fn apps_revoke_grant_for_application(
        &self,
        client_id: &str,
        access_token: &str,
    ) -> Result<()> {
        let url = format!(
            "/applications/{}/grants/{}",
            progenitor_support::encode_path(&client_id.to_string()),
            progenitor_support::encode_path(&access_token.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_check_token: POST /applications/{client_id}/token
     */
    pub async fn apps_check_token(
        &self,
        client_id: &str,
        body: &types::CheckTokenRequest,
    ) -> Result<types::Authorization> {
        let url = format!(
            "/applications/{}/token",
            progenitor_support::encode_path(&client_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * apps_delete_token: DELETE /applications/{client_id}/token
     */
    pub async fn apps_delete_token(
        &self,
        client_id: &str,
        body: &types::DeleteAppTokenRequest,
    ) -> Result<()> {
        let url = format!(
            "/applications/{}/token",
            progenitor_support::encode_path(&client_id.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_reset_token: PATCH /applications/{client_id}/token
     */
    pub async fn apps_reset_token(
        &self,
        client_id: &str,
        body: &types::ResetTokenRequest,
    ) -> Result<types::Authorization> {
        let url = format!(
            "/applications/{}/token",
            progenitor_support::encode_path(&client_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * apps_scope_token: POST /applications/{client_id}/token/scoped
     */
    pub async fn apps_scope_token(
        &self,
        client_id: &str,
        body: &types::CreateScopedAccessTokenRequest,
    ) -> Result<types::Authorization> {
        let url = format!(
            "/applications/{}/token/scoped",
            progenitor_support::encode_path(&client_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * apps_check_authorization: GET /applications/{client_id}/tokens/{access_token}
     */
    pub async fn apps_check_authorization(
        &self,
        client_id: &str,
        access_token: &str,
    ) -> Result<types::GetCheckAuthorizationOkResponse> {
        let url = format!(
            "/applications/{}/tokens/{}",
            progenitor_support::encode_path(&client_id.to_string()),
            progenitor_support::encode_path(&access_token.to_string()),
        );

        self.get(&url).await
    }

    /**
     * apps_reset_authorization: POST /applications/{client_id}/tokens/{access_token}
     */
    pub async fn apps_reset_authorization(
        &self,
        client_id: &str,
        access_token: &str,
    ) -> Result<types::Authorization> {
        let url = format!(
            "/applications/{}/tokens/{}",
            progenitor_support::encode_path(&client_id.to_string()),
            progenitor_support::encode_path(&access_token.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * apps_revoke_authorization_for_application: DELETE /applications/{client_id}/tokens/{access_token}
     */
    pub async fn apps_revoke_authorization_for_application(
        &self,
        client_id: &str,
        access_token: &str,
    ) -> Result<()> {
        let url = format!(
            "/applications/{}/tokens/{}",
            progenitor_support::encode_path(&client_id.to_string()),
            progenitor_support::encode_path(&access_token.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_get_by_slug: GET /apps/{app_slug}
     */
    pub async fn apps_get_by_slug(&self, app_slug: &str) -> Result<types::Integration> {
        let url = format!(
            "/apps/{}",
            progenitor_support::encode_path(&app_slug.to_string()),
        );

        self.get(&url).await
    }

    /**
     * oauth_authorizations_list_authorizations: GET /authorizations
     */
    pub async fn oauth_authorizations_list_authorizations(
        &self,
        per_page: i64,
        page: i64,
        client_id: &str,
    ) -> Result<Vec<types::Authorization>> {
        let url = "/authorizations".to_string();
        self.get(&url).await
    }

    /**
     * oauth_authorizations_create_authorization: POST /authorizations
     */
    pub async fn oauth_authorizations_create_authorization(
        &self,
        body: &types::CreateNewAuthorizationRequest,
    ) -> Result<types::Authorization> {
        let url = "/authorizations".to_string();
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * oauth_authorizations_get_or_create_authorization_for_app: PUT /authorizations/clients/{client_id}
     */
    pub async fn oauth_authorizations_get_or_create_authorization_for_app(
        &self,
        client_id: &str,
        body: &types::GetorCreateAuthorizationSpecificAppRequest,
    ) -> Result<types::Authorization> {
        let url = format!(
            "/authorizations/clients/{}",
            progenitor_support::encode_path(&client_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * oauth_authorizations_get_or_create_authorization_for_app_and_fingerprint: PUT /authorizations/clients/{client_id}/{fingerprint}
     */
    pub async fn oauth_authorizations_get_or_create_authorization_for_app_and_fingerprint(
        &self,
        client_id: &str,
        fingerprint: &str,
        body: &types::GetorCreateAuthorizationSpecificAppandFingerprintRequest,
    ) -> Result<types::Authorization> {
        let url = format!(
            "/authorizations/clients/{}/{}",
            progenitor_support::encode_path(&client_id.to_string()),
            progenitor_support::encode_path(&fingerprint.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * oauth_authorizations_get_authorization: GET /authorizations/{authorization_id}
     */
    pub async fn oauth_authorizations_get_authorization(
        &self,
        authorization_id: i64,
    ) -> Result<types::Authorization> {
        let url = format!(
            "/authorizations/{}",
            progenitor_support::encode_path(&authorization_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * oauth_authorizations_delete_authorization: DELETE /authorizations/{authorization_id}
     */
    pub async fn oauth_authorizations_delete_authorization(
        &self,
        authorization_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/authorizations/{}",
            progenitor_support::encode_path(&authorization_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * oauth_authorizations_update_authorization: PATCH /authorizations/{authorization_id}
     */
    pub async fn oauth_authorizations_update_authorization(
        &self,
        authorization_id: i64,
        body: &types::UpdateExistingAuthorizationRequest,
    ) -> Result<types::Authorization> {
        let url = format!(
            "/authorizations/{}",
            progenitor_support::encode_path(&authorization_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * codes_of_conduct_get_all_codes_of_conduct: GET /codes_of_conduct
     */
    pub async fn codes_of_conduct_get_all_codes_of_conduct(
        &self,
    ) -> Result<Vec<types::CodeofConduct>> {
        let url = "/codes_of_conduct".to_string();
        self.get(&url).await
    }

    /**
     * codes_of_conduct_get_conduct_code: GET /codes_of_conduct/{key}
     */
    pub async fn codes_of_conduct_get_conduct_code(
        &self,
        key: &str,
    ) -> Result<types::CodeofConduct> {
        let url = format!(
            "/codes_of_conduct/{}",
            progenitor_support::encode_path(&key.to_string()),
        );

        self.get(&url).await
    }

    /**
     * emojis_get: GET /emojis
     */
    pub async fn emojis_get(&self) -> Result<types::GetEmojisOkResponse> {
        let url = "/emojis".to_string();
        self.get(&url).await
    }

    /**
     * enterprise_admin_get_github_actions_permissions_enterprise: GET /enterprises/{enterprise}/actions/permissions
     */
    pub async fn enterprise_admin_get_github_actions_permissions_enterprise(
        &self,
        enterprise: &str,
    ) -> Result<types::ActionsEnterprisePermissions> {
        let url = format!(
            "/enterprises/{}/actions/permissions",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_set_github_actions_permissions_enterprise: PUT /enterprises/{enterprise}/actions/permissions
     */
    pub async fn enterprise_admin_set_github_actions_permissions_enterprise(
        &self,
        enterprise: &str,
        body: &types::SetGithubActionsPermissionsEnterpriseRequest,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/permissions",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_list_selected_organizations_enabled_github_actions_enterprise: GET /enterprises/{enterprise}/actions/permissions/organizations
     */
    pub async fn enterprise_admin_list_selected_organizations_enabled_github_actions_enterprise(
        &self,
        enterprise: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListSelectedOrganizationsEnabledGithubActionsinEnterpriseOkResponse> {
        let url = format!(
            "/enterprises/{}/actions/permissions/organizations",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_set_selected_organizations_enabled_github_actions_enterprise: PUT /enterprises/{enterprise}/actions/permissions/organizations
     */
    pub async fn enterprise_admin_set_selected_organizations_enabled_github_actions_enterprise(
        &self,
        enterprise: &str,
        body: &types::SetSelectedOrganizationsEnabledGithubActionsinEnterpriseRequest,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/permissions/organizations",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_enable_selected_organization_github_actions_enterprise: PUT /enterprises/{enterprise}/actions/permissions/organizations/{org_id}
     */
    pub async fn enterprise_admin_enable_selected_organization_github_actions_enterprise(
        &self,
        enterprise: &str,
        org_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/permissions/organizations/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&org_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_disable_selected_organization_github_actions_enterprise: DELETE /enterprises/{enterprise}/actions/permissions/organizations/{org_id}
     */
    pub async fn enterprise_admin_disable_selected_organization_github_actions_enterprise(
        &self,
        enterprise: &str,
        org_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/permissions/organizations/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&org_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_get_allowed_actions_enterprise: GET /enterprises/{enterprise}/actions/permissions/selected-actions
     */
    pub async fn enterprise_admin_get_allowed_actions_enterprise(
        &self,
        enterprise: &str,
    ) -> Result<types::SelectedActions> {
        let url = format!(
            "/enterprises/{}/actions/permissions/selected-actions",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_set_allowed_actions_enterprise: PUT /enterprises/{enterprise}/actions/permissions/selected-actions
     */
    pub async fn enterprise_admin_set_allowed_actions_enterprise(
        &self,
        enterprise: &str,
        body: &types::SelectedActions,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/permissions/selected-actions",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_list_self_hosted_runner_groups_for_enterprise: GET /enterprises/{enterprise}/actions/runner-groups
     */
    pub async fn enterprise_admin_list_self_hosted_runner_groups_for_enterprise(
        &self,
        enterprise: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListSelfDataHostedRunnerGroupsEnterpriseOkResponse> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_create_self_hosted_runner_group_for_enterprise: POST /enterprises/{enterprise}/actions/runner-groups
     */
    pub async fn enterprise_admin_create_self_hosted_runner_group_for_enterprise(
        &self,
        enterprise: &str,
        body: &types::CreateSelfDataHostedRunnerGroupEnterpriseRequest,
    ) -> Result<types::RunnerGroupsEnterprise> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * enterprise_admin_get_self_hosted_runner_group_for_enterprise: GET /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}
     */
    pub async fn enterprise_admin_get_self_hosted_runner_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
    ) -> Result<types::RunnerGroupsEnterprise> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_delete_self_hosted_runner_group_from_enterprise: DELETE /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}
     */
    pub async fn enterprise_admin_delete_self_hosted_runner_group_from_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_update_self_hosted_runner_group_for_enterprise: PATCH /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}
     */
    pub async fn enterprise_admin_update_self_hosted_runner_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        body: &types::UpdateSelfDataHostedRunnerGroupEnterpriseRequest,
    ) -> Result<types::RunnerGroupsEnterprise> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * enterprise_admin_list_org_access_to_self_hosted_runner_group_in_enterprise: GET /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations
     */
    pub async fn enterprise_admin_list_org_access_to_self_hosted_runner_group_in_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListOrganizationAccessSelfDataHostedRunnerGroupinEnterpriseOkResponse>
    {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}/organizations",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_set_org_access_to_self_hosted_runner_group_in_enterprise: PUT /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations
     */
    pub async fn enterprise_admin_set_org_access_to_self_hosted_runner_group_in_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        body: &types::SetOrganizationAccessSelfDataHostedRunnerGroupinEnterpriseRequest,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}/organizations",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_add_org_access_to_self_hosted_runner_group_in_enterprise: PUT /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations/{org_id}
     */
    pub async fn enterprise_admin_add_org_access_to_self_hosted_runner_group_in_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        org_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}/organizations/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
            progenitor_support::encode_path(&org_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_remove_org_access_to_self_hosted_runner_group_in_enterprise: DELETE /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations/{org_id}
     */
    pub async fn enterprise_admin_remove_org_access_to_self_hosted_runner_group_in_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        org_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}/organizations/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
            progenitor_support::encode_path(&org_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_list_self_hosted_runners_in_group_for_enterprise: GET /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners
     */
    pub async fn enterprise_admin_list_self_hosted_runners_in_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListSelfDataHostedRunnersinGroupEnterpriseOkResponse> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}/runners",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_set_self_hosted_runners_in_group_for_enterprise: PUT /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners
     */
    pub async fn enterprise_admin_set_self_hosted_runners_in_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        body: &types::SetSelfDataHostedRunnersinGroupEnterpriseRequest,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}/runners",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_add_self_hosted_runner_to_group_for_enterprise: PUT /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners/{runner_id}
     */
    pub async fn enterprise_admin_add_self_hosted_runner_to_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        runner_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}/runners/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_remove_self_hosted_runner_from_group_for_enterprise: DELETE /enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners/{runner_id}
     */
    pub async fn enterprise_admin_remove_self_hosted_runner_from_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        runner_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/runner-groups/{}/runners/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_list_self_hosted_runners_for_enterprise: GET /enterprises/{enterprise}/actions/runners
     */
    pub async fn enterprise_admin_list_self_hosted_runners_for_enterprise(
        &self,
        enterprise: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListSelfDataHostedRunnersEnterpriseOkResponse> {
        let url = format!(
            "/enterprises/{}/actions/runners",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_list_runner_applications_for_enterprise: GET /enterprises/{enterprise}/actions/runners/downloads
     */
    pub async fn enterprise_admin_list_runner_applications_for_enterprise(
        &self,
        enterprise: &str,
    ) -> Result<Vec<types::RunnerApplication>> {
        let url = format!(
            "/enterprises/{}/actions/runners/downloads",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_create_registration_token_for_enterprise: POST /enterprises/{enterprise}/actions/runners/registration-token
     */
    pub async fn enterprise_admin_create_registration_token_for_enterprise(
        &self,
        enterprise: &str,
    ) -> Result<types::AuthenticationToken> {
        let url = format!(
            "/enterprises/{}/actions/runners/registration-token",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * enterprise_admin_create_remove_token_for_enterprise: POST /enterprises/{enterprise}/actions/runners/remove-token
     */
    pub async fn enterprise_admin_create_remove_token_for_enterprise(
        &self,
        enterprise: &str,
    ) -> Result<types::AuthenticationToken> {
        let url = format!(
            "/enterprises/{}/actions/runners/remove-token",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * enterprise_admin_get_self_hosted_runner_for_enterprise: GET /enterprises/{enterprise}/actions/runners/{runner_id}
     */
    pub async fn enterprise_admin_get_self_hosted_runner_for_enterprise(
        &self,
        enterprise: &str,
        runner_id: i64,
    ) -> Result<types::Runner> {
        let url = format!(
            "/enterprises/{}/actions/runners/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_delete_self_hosted_runner_from_enterprise: DELETE /enterprises/{enterprise}/actions/runners/{runner_id}
     */
    pub async fn enterprise_admin_delete_self_hosted_runner_from_enterprise(
        &self,
        enterprise: &str,
        runner_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/enterprises/{}/actions/runners/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_get_audit_log: GET /enterprises/{enterprise}/audit-log
     */
    pub async fn enterprise_admin_get_audit_log(
        &self,
        enterprise: &str,
        phrase: &str,
        include: &str,
        after: &str,
        before: &str,
        order: &str,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<types::AuditLogEvent>> {
        let url = format!(
            "/enterprises/{}/audit-log",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * billing_get_github_actions_billing_ghe: GET /enterprises/{enterprise}/settings/billing/actions
     */
    pub async fn billing_get_github_actions_billing_ghe(
        &self,
        enterprise: &str,
    ) -> Result<types::ActionsBillingUsage> {
        let url = format!(
            "/enterprises/{}/settings/billing/actions",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * billing_get_github_packages_billing_ghe: GET /enterprises/{enterprise}/settings/billing/packages
     */
    pub async fn billing_get_github_packages_billing_ghe(
        &self,
        enterprise: &str,
    ) -> Result<types::PackagesBillingUsage> {
        let url = format!(
            "/enterprises/{}/settings/billing/packages",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * billing_get_shared_storage_billing_ghe: GET /enterprises/{enterprise}/settings/billing/shared-storage
     */
    pub async fn billing_get_shared_storage_billing_ghe(
        &self,
        enterprise: &str,
    ) -> Result<types::CombinedBillingUsage> {
        let url = format!(
            "/enterprises/{}/settings/billing/shared-storage",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_public_events: GET /events
     */
    pub async fn activity_list_public_events(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Event>> {
        let url = "/events".to_string();
        self.get(&url).await
    }

    /**
     * activity_get_feeds: GET /feeds
     */
    pub async fn activity_get_feeds(&self) -> Result<types::Feed> {
        let url = "/feeds".to_string();
        self.get(&url).await
    }

    /**
     * gists_list: GET /gists
     */
    pub async fn gists_list(
        &self,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::BaseGist>> {
        let url = "/gists".to_string();
        self.get(&url).await
    }

    /**
     * gists_create: POST /gists
     */
    pub async fn gists_create(&self, body: &types::CreateGistRequest) -> Result<types::GistSimple> {
        let url = "/gists".to_string();
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * gists_list_public: GET /gists/public
     */
    pub async fn gists_list_public(
        &self,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::BaseGist>> {
        let url = "/gists/public".to_string();
        self.get(&url).await
    }

    /**
     * gists_list_starred: GET /gists/starred
     */
    pub async fn gists_list_starred(
        &self,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::BaseGist>> {
        let url = "/gists/starred".to_string();
        self.get(&url).await
    }

    /**
     * gists_get: GET /gists/{gist_id}
     */
    pub async fn gists_get(&self, gist_id: &str) -> Result<types::GistSimple> {
        let url = format!(
            "/gists/{}",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * gists_delete: DELETE /gists/{gist_id}
     */
    pub async fn gists_delete(&self, gist_id: &str) -> Result<()> {
        let url = format!(
            "/gists/{}",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * gists_update: PATCH /gists/{gist_id}
     */
    pub async fn gists_update(
        &self,
        gist_id: &str,
        body: &types::UpdateGistRequest,
    ) -> Result<types::GistSimple> {
        let url = format!(
            "/gists/{}",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * gists_list_comments: GET /gists/{gist_id}/comments
     */
    pub async fn gists_list_comments(
        &self,
        gist_id: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::GistComment>> {
        let url = format!(
            "/gists/{}/comments",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * gists_create_comment: POST /gists/{gist_id}/comments
     */
    pub async fn gists_create_comment(
        &self,
        gist_id: &str,
        body: &types::CreateGistCommentRequest,
    ) -> Result<types::GistComment> {
        let url = format!(
            "/gists/{}/comments",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * gists_get_comment: GET /gists/{gist_id}/comments/{comment_id}
     */
    pub async fn gists_get_comment(
        &self,
        gist_id: &str,
        comment_id: i64,
    ) -> Result<types::GistComment> {
        let url = format!(
            "/gists/{}/comments/{}",
            progenitor_support::encode_path(&gist_id.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * gists_delete_comment: DELETE /gists/{gist_id}/comments/{comment_id}
     */
    pub async fn gists_delete_comment(&self, gist_id: &str, comment_id: i64) -> Result<()> {
        let url = format!(
            "/gists/{}/comments/{}",
            progenitor_support::encode_path(&gist_id.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * gists_update_comment: PATCH /gists/{gist_id}/comments/{comment_id}
     */
    pub async fn gists_update_comment(
        &self,
        gist_id: &str,
        comment_id: i64,
        body: &types::UpdateGistCommentRequest,
    ) -> Result<types::GistComment> {
        let url = format!(
            "/gists/{}/comments/{}",
            progenitor_support::encode_path(&gist_id.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * gists_list_commits: GET /gists/{gist_id}/commits
     */
    pub async fn gists_list_commits(
        &self,
        gist_id: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::GistCommit>> {
        let url = format!(
            "/gists/{}/commits",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * gists_list_forks: GET /gists/{gist_id}/forks
     */
    pub async fn gists_list_forks(
        &self,
        gist_id: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::GistSimple>> {
        let url = format!(
            "/gists/{}/forks",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * gists_fork: POST /gists/{gist_id}/forks
     */
    pub async fn gists_fork(&self, gist_id: &str) -> Result<types::BaseGist> {
        let url = format!(
            "/gists/{}/forks",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * gists_check_is_starred: GET /gists/{gist_id}/star
     */
    pub async fn gists_check_is_starred(&self, gist_id: &str) -> Result<()> {
        let url = format!(
            "/gists/{}/star",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * gists_star: PUT /gists/{gist_id}/star
     */
    pub async fn gists_star(&self, gist_id: &str) -> Result<()> {
        let url = format!(
            "/gists/{}/star",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * gists_unstar: DELETE /gists/{gist_id}/star
     */
    pub async fn gists_unstar(&self, gist_id: &str) -> Result<()> {
        let url = format!(
            "/gists/{}/star",
            progenitor_support::encode_path(&gist_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * gists_get_revision: GET /gists/{gist_id}/{sha}
     */
    pub async fn gists_get_revision(&self, gist_id: &str, sha: &str) -> Result<types::GistSimple> {
        let url = format!(
            "/gists/{}/{}",
            progenitor_support::encode_path(&gist_id.to_string()),
            progenitor_support::encode_path(&sha.to_string()),
        );

        self.get(&url).await
    }

    /**
     * gitignore_get_all_templates: GET /gitignore/templates
     */
    pub async fn gitignore_get_all_templates(&self) -> Result<Vec<String>> {
        let url = "/gitignore/templates".to_string();
        self.get(&url).await
    }

    /**
     * gitignore_get_template: GET /gitignore/templates/{name}
     */
    pub async fn gitignore_get_template(&self, name: &str) -> Result<types::GitignoreTemplate> {
        let url = format!(
            "/gitignore/templates/{}",
            progenitor_support::encode_path(&name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * apps_list_repos_accessible_to_installation: GET /installation/repositories
     */
    pub async fn apps_list_repos_accessible_to_installation(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListRepositoriesAccessibleAppInstallationOkResponse> {
        let url = "/installation/repositories".to_string();
        self.get(&url).await
    }

    /**
     * apps_revoke_installation_access_token: DELETE /installation/token
     */
    pub async fn apps_revoke_installation_access_token(&self) -> Result<()> {
        let url = "/installation/token".to_string();
        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_list: GET /issues
     */
    pub async fn issues_list(
        &self,
        filter: &str,
        state: &str,
        labels: &str,
        sort: &str,
        direction: &str,
        since: DateTime<Utc>,
        collab: bool,
        orgs: bool,
        owned: bool,
        pulls: bool,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Issue>> {
        let url = "/issues".to_string();
        self.get(&url).await
    }

    /**
     * licenses_get_all_commonly_used: GET /licenses
     */
    pub async fn licenses_get_all_commonly_used(
        &self,
        featured: bool,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::LicenseSimple>> {
        let url = "/licenses".to_string();
        self.get(&url).await
    }

    /**
     * licenses_get: GET /licenses/{license}
     */
    pub async fn licenses_get(&self, license: &str) -> Result<types::License> {
        let url = format!(
            "/licenses/{}",
            progenitor_support::encode_path(&license.to_string()),
        );

        self.get(&url).await
    }

    /**
     * markdown_render: POST /markdown
     */
    pub async fn markdown_render(
        &self,
        body: &types::RenderMarkdownDocumentRequest,
    ) -> Result<String> {
        let url = "/markdown".to_string();
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.text().await?)
    }

    /**
     * markdown_render_raw: POST /markdown/raw
     */
    pub async fn markdown_render_raw<T: Into<reqwest::Body>>(&self, body: T) -> Result<String> {
        let url = "/markdown/raw".to_string();
        let res = self
            .client
            .post(url)
            .body(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.text().await?)
    }

    /**
     * apps_get_subscription_plan_for_account: GET /marketplace_listing/accounts/{account_id}
     */
    pub async fn apps_get_subscription_plan_for_account(
        &self,
        account_id: i64,
    ) -> Result<types::MarketplacePurchase> {
        let url = format!(
            "/marketplace_listing/accounts/{}",
            progenitor_support::encode_path(&account_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * apps_list_plans: GET /marketplace_listing/plans
     */
    pub async fn apps_list_plans(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MarketplaceListingPlan>> {
        let url = "/marketplace_listing/plans".to_string();
        self.get(&url).await
    }

    /**
     * apps_list_accounts_for_plan: GET /marketplace_listing/plans/{plan_id}/accounts
     */
    pub async fn apps_list_accounts_for_plan(
        &self,
        plan_id: i64,
        sort: &str,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MarketplacePurchase>> {
        let url = format!(
            "/marketplace_listing/plans/{}/accounts",
            progenitor_support::encode_path(&plan_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * apps_get_subscription_plan_for_account_stubbed: GET /marketplace_listing/stubbed/accounts/{account_id}
     */
    pub async fn apps_get_subscription_plan_for_account_stubbed(
        &self,
        account_id: i64,
    ) -> Result<types::MarketplacePurchase> {
        let url = format!(
            "/marketplace_listing/stubbed/accounts/{}",
            progenitor_support::encode_path(&account_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * apps_list_plans_stubbed: GET /marketplace_listing/stubbed/plans
     */
    pub async fn apps_list_plans_stubbed(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MarketplaceListingPlan>> {
        let url = "/marketplace_listing/stubbed/plans".to_string();
        self.get(&url).await
    }

    /**
     * apps_list_accounts_for_plan_stubbed: GET /marketplace_listing/stubbed/plans/{plan_id}/accounts
     */
    pub async fn apps_list_accounts_for_plan_stubbed(
        &self,
        plan_id: i64,
        sort: &str,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MarketplacePurchase>> {
        let url = format!(
            "/marketplace_listing/stubbed/plans/{}/accounts",
            progenitor_support::encode_path(&plan_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * meta_get: GET /meta
     */
    pub async fn meta_get(&self) -> Result<types::ApiOverview> {
        let url = "/meta".to_string();
        self.get(&url).await
    }

    /**
     * activity_list_public_events_for_repo_network: GET /networks/{owner}/{repo}/events
     */
    pub async fn activity_list_public_events_for_repo_network(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Event>> {
        let url = format!(
            "/networks/{}/{}/events",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_notifications_for_authenticated_user: GET /notifications
     */
    pub async fn activity_list_notifications_for_authenticated_user(
        &self,
        all: bool,
        participating: bool,
        since: DateTime<Utc>,
        before: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Thread>> {
        let url = "/notifications".to_string();
        self.get(&url).await
    }

    /**
     * activity_mark_notifications_as_read: PUT /notifications
     */
    pub async fn activity_mark_notifications_as_read(
        &self,
        body: &types::MarkNotificationsasReadRequest,
    ) -> Result<types::PutMarkNotificationsasReadAcceptedResponse> {
        let url = "/notifications".to_string();
        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * activity_get_thread: GET /notifications/threads/{thread_id}
     */
    pub async fn activity_get_thread(&self, thread_id: i64) -> Result<types::Thread> {
        let url = format!(
            "/notifications/threads/{}",
            progenitor_support::encode_path(&thread_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_mark_thread_as_read: PATCH /notifications/threads/{thread_id}
     */
    pub async fn activity_mark_thread_as_read(&self, thread_id: i64) -> Result<()> {
        let url = format!(
            "/notifications/threads/{}",
            progenitor_support::encode_path(&thread_id.to_string()),
        );

        let res = self.client.patch(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * activity_get_thread_subscription_for_authenticated_user: GET /notifications/threads/{thread_id}/subscription
     */
    pub async fn activity_get_thread_subscription_for_authenticated_user(
        &self,
        thread_id: i64,
    ) -> Result<types::ThreadSubscription> {
        let url = format!(
            "/notifications/threads/{}/subscription",
            progenitor_support::encode_path(&thread_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_set_thread_subscription: PUT /notifications/threads/{thread_id}/subscription
     */
    pub async fn activity_set_thread_subscription(
        &self,
        thread_id: i64,
        body: &types::SetThreadSubscriptionRequest,
    ) -> Result<types::ThreadSubscription> {
        let url = format!(
            "/notifications/threads/{}/subscription",
            progenitor_support::encode_path(&thread_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * activity_delete_thread_subscription: DELETE /notifications/threads/{thread_id}/subscription
     */
    pub async fn activity_delete_thread_subscription(&self, thread_id: i64) -> Result<()> {
        let url = format!(
            "/notifications/threads/{}/subscription",
            progenitor_support::encode_path(&thread_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * meta_get_octocat: GET /octocat
     */
    pub async fn meta_get_octocat(&self, s: &str) -> Result<String> {
        let url = "/octocat".to_string();
        self.get(&url).await
    }

    /**
     * orgs_list: GET /organizations
     */
    pub async fn orgs_list(
        &self,
        since: i64,
        per_page: i64,
    ) -> Result<Vec<types::OrganizationSimple>> {
        let url = "/organizations".to_string();
        self.get(&url).await
    }

    /**
     * orgs_get: GET /orgs/{org}
     */
    pub async fn orgs_get(&self, org: &str) -> Result<types::OrganizationFull> {
        let url = format!(
            "/orgs/{}",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_update: PATCH /orgs/{org}
     */
    pub async fn orgs_update(
        &self,
        org: &str,
        body: &types::UpdateOrganizationRequest,
    ) -> Result<types::OrganizationFull> {
        let url = format!(
            "/orgs/{}",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_get_github_actions_permissions_organization: GET /orgs/{org}/actions/permissions
     */
    pub async fn actions_get_github_actions_permissions_organization(
        &self,
        org: &str,
    ) -> Result<types::ActionsOrganizationPermissions> {
        let url = format!(
            "/orgs/{}/actions/permissions",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_set_github_actions_permissions_organization: PUT /orgs/{org}/actions/permissions
     */
    pub async fn actions_set_github_actions_permissions_organization(
        &self,
        org: &str,
        body: &types::SetGithubActionsPermissionsOrganizationRequest,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/permissions",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_selected_repositories_enabled_github_actions_organization: GET /orgs/{org}/actions/permissions/repositories
     */
    pub async fn actions_list_selected_repositories_enabled_github_actions_organization(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListSelectedRepositoriesEnabledGithubActionsinOrganizationOkResponse>
    {
        let url = format!(
            "/orgs/{}/actions/permissions/repositories",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_set_selected_repositories_enabled_github_actions_organization: PUT /orgs/{org}/actions/permissions/repositories
     */
    pub async fn actions_set_selected_repositories_enabled_github_actions_organization(
        &self,
        org: &str,
        body: &types::SetSelectedRepositoriesEnabledGithubActionsinOrganizationRequest,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/permissions/repositories",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_enable_selected_repository_github_actions_organization: PUT /orgs/{org}/actions/permissions/repositories/{repository_id}
     */
    pub async fn actions_enable_selected_repository_github_actions_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/permissions/repositories/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&repository_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_disable_selected_repository_github_actions_organization: DELETE /orgs/{org}/actions/permissions/repositories/{repository_id}
     */
    pub async fn actions_disable_selected_repository_github_actions_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/permissions/repositories/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&repository_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_get_allowed_actions_organization: GET /orgs/{org}/actions/permissions/selected-actions
     */
    pub async fn actions_get_allowed_actions_organization(
        &self,
        org: &str,
    ) -> Result<types::SelectedActions> {
        let url = format!(
            "/orgs/{}/actions/permissions/selected-actions",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_set_allowed_actions_organization: PUT /orgs/{org}/actions/permissions/selected-actions
     */
    pub async fn actions_set_allowed_actions_organization(
        &self,
        org: &str,
        body: &types::SelectedActions,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/permissions/selected-actions",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_self_hosted_runner_groups_for_org: GET /orgs/{org}/actions/runner-groups
     */
    pub async fn actions_list_self_hosted_runner_groups_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListSelfDataHostedRunnerGroupsOrganizationOkResponse> {
        let url = format!(
            "/orgs/{}/actions/runner-groups",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_create_self_hosted_runner_group_for_org: POST /orgs/{org}/actions/runner-groups
     */
    pub async fn actions_create_self_hosted_runner_group_for_org(
        &self,
        org: &str,
        body: &types::CreateSelfDataHostedRunnerGroupOrganizationRequest,
    ) -> Result<types::RunnerGroupsOrg> {
        let url = format!(
            "/orgs/{}/actions/runner-groups",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_get_self_hosted_runner_group_for_org: GET /orgs/{org}/actions/runner-groups/{runner_group_id}
     */
    pub async fn actions_get_self_hosted_runner_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
    ) -> Result<types::RunnerGroupsOrg> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_delete_self_hosted_runner_group_from_org: DELETE /orgs/{org}/actions/runner-groups/{runner_group_id}
     */
    pub async fn actions_delete_self_hosted_runner_group_from_org(
        &self,
        org: &str,
        runner_group_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_update_self_hosted_runner_group_for_org: PATCH /orgs/{org}/actions/runner-groups/{runner_group_id}
     */
    pub async fn actions_update_self_hosted_runner_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        body: &types::UpdateSelfDataHostedRunnerGroupOrganizationRequest,
    ) -> Result<types::RunnerGroupsOrg> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_list_repo_access_to_self_hosted_runner_group_in_org: GET /orgs/{org}/actions/runner-groups/{runner_group_id}/repositories
     */
    pub async fn actions_list_repo_access_to_self_hosted_runner_group_in_org(
        &self,
        org: &str,
        runner_group_id: i64,
        page: i64,
        per_page: i64,
    ) -> Result<types::GetListRepositoryAccessSelfDataHostedRunnerGroupinOrganizationOkResponse>
    {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}/repositories",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_set_repo_access_to_self_hosted_runner_group_in_org: PUT /orgs/{org}/actions/runner-groups/{runner_group_id}/repositories
     */
    pub async fn actions_set_repo_access_to_self_hosted_runner_group_in_org(
        &self,
        org: &str,
        runner_group_id: i64,
        body: &types::SetRepositoryAccessSelfDataHostedRunnerGroupinOrganizationRequest,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}/repositories",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_add_repo_access_to_self_hosted_runner_group_in_org: PUT /orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id}
     */
    pub async fn actions_add_repo_access_to_self_hosted_runner_group_in_org(
        &self,
        org: &str,
        runner_group_id: i64,
        repository_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}/repositories/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
            progenitor_support::encode_path(&repository_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_remove_repo_access_to_self_hosted_runner_group_in_org: DELETE /orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id}
     */
    pub async fn actions_remove_repo_access_to_self_hosted_runner_group_in_org(
        &self,
        org: &str,
        runner_group_id: i64,
        repository_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}/repositories/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
            progenitor_support::encode_path(&repository_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_self_hosted_runners_in_group_for_org: GET /orgs/{org}/actions/runner-groups/{runner_group_id}/runners
     */
    pub async fn actions_list_self_hosted_runners_in_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListSelfDataHostedRunnersinGroupOrganizationOkResponse> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}/runners",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_set_self_hosted_runners_in_group_for_org: PUT /orgs/{org}/actions/runner-groups/{runner_group_id}/runners
     */
    pub async fn actions_set_self_hosted_runners_in_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        body: &types::SetSelfDataHostedRunnersinGroupOrganizationRequest,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}/runners",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_add_self_hosted_runner_to_group_for_org: PUT /orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id}
     */
    pub async fn actions_add_self_hosted_runner_to_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        runner_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}/runners/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_remove_self_hosted_runner_from_group_for_org: DELETE /orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id}
     */
    pub async fn actions_remove_self_hosted_runner_from_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        runner_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/runner-groups/{}/runners/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_group_id.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_self_hosted_runners_for_org: GET /orgs/{org}/actions/runners
     */
    pub async fn actions_list_self_hosted_runners_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListSelfDataHostedRunnersOrganizationOkResponse> {
        let url = format!(
            "/orgs/{}/actions/runners",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_list_runner_applications_for_org: GET /orgs/{org}/actions/runners/downloads
     */
    pub async fn actions_list_runner_applications_for_org(
        &self,
        org: &str,
    ) -> Result<Vec<types::RunnerApplication>> {
        let url = format!(
            "/orgs/{}/actions/runners/downloads",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_create_registration_token_for_org: POST /orgs/{org}/actions/runners/registration-token
     */
    pub async fn actions_create_registration_token_for_org(
        &self,
        org: &str,
    ) -> Result<types::AuthenticationToken> {
        let url = format!(
            "/orgs/{}/actions/runners/registration-token",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_create_remove_token_for_org: POST /orgs/{org}/actions/runners/remove-token
     */
    pub async fn actions_create_remove_token_for_org(
        &self,
        org: &str,
    ) -> Result<types::AuthenticationToken> {
        let url = format!(
            "/orgs/{}/actions/runners/remove-token",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_get_self_hosted_runner_for_org: GET /orgs/{org}/actions/runners/{runner_id}
     */
    pub async fn actions_get_self_hosted_runner_for_org(
        &self,
        org: &str,
        runner_id: i64,
    ) -> Result<types::Runner> {
        let url = format!(
            "/orgs/{}/actions/runners/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_delete_self_hosted_runner_from_org: DELETE /orgs/{org}/actions/runners/{runner_id}
     */
    pub async fn actions_delete_self_hosted_runner_from_org(
        &self,
        org: &str,
        runner_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/runners/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_org_secrets: GET /orgs/{org}/actions/secrets
     */
    pub async fn actions_list_org_secrets(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListOrganizationSecretsOkResponse> {
        let url = format!(
            "/orgs/{}/actions/secrets",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_org_public_key: GET /orgs/{org}/actions/secrets/public-key
     */
    pub async fn actions_get_org_public_key(&self, org: &str) -> Result<types::ActionsPublicKey> {
        let url = format!(
            "/orgs/{}/actions/secrets/public-key",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_org_secret: GET /orgs/{org}/actions/secrets/{secret_name}
     */
    pub async fn actions_get_org_secret(
        &self,
        org: &str,
        secret_name: &str,
    ) -> Result<types::OrganizationActionsSecret> {
        let url = format!(
            "/orgs/{}/actions/secrets/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_create_or_update_org_secret: PUT /orgs/{org}/actions/secrets/{secret_name}
     */
    pub async fn actions_create_or_update_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        body: &types::CreateUpdateOrganizationSecretRequest,
    ) -> Result<types::EmptyObject> {
        let url = format!(
            "/orgs/{}/actions/secrets/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_delete_org_secret: DELETE /orgs/{org}/actions/secrets/{secret_name}
     */
    pub async fn actions_delete_org_secret(&self, org: &str, secret_name: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/secrets/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_selected_repos_for_org_secret: GET /orgs/{org}/actions/secrets/{secret_name}/repositories
     */
    pub async fn actions_list_selected_repos_for_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        page: i64,
        per_page: i64,
    ) -> Result<types::GetListSelectedRepositoriesOrganizationSecretOkResponse> {
        let url = format!(
            "/orgs/{}/actions/secrets/{}/repositories",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_set_selected_repos_for_org_secret: PUT /orgs/{org}/actions/secrets/{secret_name}/repositories
     */
    pub async fn actions_set_selected_repos_for_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        body: &types::SetSelectedRepositoriesOrganizationSecretRequest,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/secrets/{}/repositories",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_add_selected_repo_to_org_secret: PUT /orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}
     */
    pub async fn actions_add_selected_repo_to_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        repository_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/secrets/{}/repositories/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
            progenitor_support::encode_path(&repository_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_remove_selected_repo_from_org_secret: DELETE /orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}
     */
    pub async fn actions_remove_selected_repo_from_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        repository_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/actions/secrets/{}/repositories/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
            progenitor_support::encode_path(&repository_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * orgs_get_audit_log: GET /orgs/{org}/audit-log
     */
    pub async fn orgs_get_audit_log(
        &self,
        org: &str,
        phrase: &str,
        include: &str,
        after: &str,
        before: &str,
        order: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::AuditLogEvent>> {
        let url = format!(
            "/orgs/{}/audit-log",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_list_blocked_users: GET /orgs/{org}/blocks
     */
    pub async fn orgs_list_blocked_users(&self, org: &str) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/orgs/{}/blocks",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_check_blocked_user: GET /orgs/{org}/blocks/{username}
     */
    pub async fn orgs_check_blocked_user(&self, org: &str, username: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/blocks/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_block_user: PUT /orgs/{org}/blocks/{username}
     */
    pub async fn orgs_block_user(&self, org: &str, username: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/blocks/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * orgs_unblock_user: DELETE /orgs/{org}/blocks/{username}
     */
    pub async fn orgs_unblock_user(&self, org: &str, username: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/blocks/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * orgs_list_saml_sso_authorizations: GET /orgs/{org}/credential-authorizations
     */
    pub async fn orgs_list_saml_sso_authorizations(
        &self,
        org: &str,
    ) -> Result<Vec<types::CredentialAuthorization>> {
        let url = format!(
            "/orgs/{}/credential-authorizations",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_remove_saml_sso_authorization: DELETE /orgs/{org}/credential-authorizations/{credential_id}
     */
    pub async fn orgs_remove_saml_sso_authorization(
        &self,
        org: &str,
        credential_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/credential-authorizations/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&credential_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * activity_list_public_org_events: GET /orgs/{org}/events
     */
    pub async fn activity_list_public_org_events(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Event>> {
        let url = format!(
            "/orgs/{}/events",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_list_failed_invitations: GET /orgs/{org}/failed_invitations
     */
    pub async fn orgs_list_failed_invitations(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::OrganizationInvitation>> {
        let url = format!(
            "/orgs/{}/failed_invitations",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_list_webhooks: GET /orgs/{org}/hooks
     */
    pub async fn orgs_list_webhooks(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::OrgHook>> {
        let url = format!(
            "/orgs/{}/hooks",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_create_webhook: POST /orgs/{org}/hooks
     */
    pub async fn orgs_create_webhook(
        &self,
        org: &str,
        body: &types::CreateOrganizationWebhookRequest,
    ) -> Result<types::OrgHook> {
        let url = format!(
            "/orgs/{}/hooks",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * orgs_get_webhook: GET /orgs/{org}/hooks/{hook_id}
     */
    pub async fn orgs_get_webhook(&self, org: &str, hook_id: i64) -> Result<types::OrgHook> {
        let url = format!(
            "/orgs/{}/hooks/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_delete_webhook: DELETE /orgs/{org}/hooks/{hook_id}
     */
    pub async fn orgs_delete_webhook(&self, org: &str, hook_id: i64) -> Result<()> {
        let url = format!(
            "/orgs/{}/hooks/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * orgs_update_webhook: PATCH /orgs/{org}/hooks/{hook_id}
     */
    pub async fn orgs_update_webhook(
        &self,
        org: &str,
        hook_id: i64,
        body: &types::UpdateOrganizationWebhookRequest,
    ) -> Result<types::OrgHook> {
        let url = format!(
            "/orgs/{}/hooks/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * orgs_get_webhook_config_for_org: GET /orgs/{org}/hooks/{hook_id}/config
     */
    pub async fn orgs_get_webhook_config_for_org(
        &self,
        org: &str,
        hook_id: i64,
    ) -> Result<types::WebhookConfig> {
        let url = format!(
            "/orgs/{}/hooks/{}/config",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_update_webhook_config_for_org: PATCH /orgs/{org}/hooks/{hook_id}/config
     */
    pub async fn orgs_update_webhook_config_for_org(
        &self,
        org: &str,
        hook_id: i64,
        body: &types::UpdateWebhookConfigurationOrganizationRequest,
    ) -> Result<types::WebhookConfig> {
        let url = format!(
            "/orgs/{}/hooks/{}/config",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * orgs_ping_webhook: POST /orgs/{org}/hooks/{hook_id}/pings
     */
    pub async fn orgs_ping_webhook(&self, org: &str, hook_id: i64) -> Result<()> {
        let url = format!(
            "/orgs/{}/hooks/{}/pings",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_get_org_installation: GET /orgs/{org}/installation
     */
    pub async fn apps_get_org_installation(&self, org: &str) -> Result<types::Installation> {
        let url = format!(
            "/orgs/{}/installation",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_list_app_installations: GET /orgs/{org}/installations
     */
    pub async fn orgs_list_app_installations(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListAppInstallationsOrganizationOkResponse> {
        let url = format!(
            "/orgs/{}/installations",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * interactions_get_restrictions_for_org: GET /orgs/{org}/interaction-limits
     */
    pub async fn interactions_get_restrictions_for_org(
        &self,
        org: &str,
    ) -> Result<types::GetInteractionRestrictionsOrganizationOkResponse> {
        let url = format!(
            "/orgs/{}/interaction-limits",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * interactions_set_restrictions_for_org: PUT /orgs/{org}/interaction-limits
     */
    pub async fn interactions_set_restrictions_for_org(
        &self,
        org: &str,
        body: &types::InteractionLimit,
    ) -> Result<types::InteractionLimitResponse> {
        let url = format!(
            "/orgs/{}/interaction-limits",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * interactions_remove_restrictions_for_org: DELETE /orgs/{org}/interaction-limits
     */
    pub async fn interactions_remove_restrictions_for_org(&self, org: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/interaction-limits",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * orgs_list_pending_invitations: GET /orgs/{org}/invitations
     */
    pub async fn orgs_list_pending_invitations(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::OrganizationInvitation>> {
        let url = format!(
            "/orgs/{}/invitations",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_create_invitation: POST /orgs/{org}/invitations
     */
    pub async fn orgs_create_invitation(
        &self,
        org: &str,
        body: &types::CreateOrganizationInvitationRequest,
    ) -> Result<types::OrganizationInvitation> {
        let url = format!(
            "/orgs/{}/invitations",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * orgs_cancel_invitation: DELETE /orgs/{org}/invitations/{invitation_id}
     */
    pub async fn orgs_cancel_invitation(&self, org: &str, invitation_id: i64) -> Result<()> {
        let url = format!(
            "/orgs/{}/invitations/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&invitation_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * orgs_list_invitation_teams: GET /orgs/{org}/invitations/{invitation_id}/teams
     */
    pub async fn orgs_list_invitation_teams(
        &self,
        org: &str,
        invitation_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Team>> {
        let url = format!(
            "/orgs/{}/invitations/{}/teams",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&invitation_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_list_for_org: GET /orgs/{org}/issues
     */
    pub async fn issues_list_for_org(
        &self,
        org: &str,
        filter: &str,
        state: &str,
        labels: &str,
        sort: &str,
        direction: &str,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Issue>> {
        let url = format!(
            "/orgs/{}/issues",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_list_members: GET /orgs/{org}/members
     */
    pub async fn orgs_list_members(
        &self,
        org: &str,
        filter: &str,
        role: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/orgs/{}/members",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_check_membership_for_user: GET /orgs/{org}/members/{username}
     */
    pub async fn orgs_check_membership_for_user(&self, org: &str, username: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/members/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_remove_member: DELETE /orgs/{org}/members/{username}
     */
    pub async fn orgs_remove_member(&self, org: &str, username: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/members/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * orgs_get_membership_for_user: GET /orgs/{org}/memberships/{username}
     */
    pub async fn orgs_get_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> Result<types::OrgMembership> {
        let url = format!(
            "/orgs/{}/memberships/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_set_membership_for_user: PUT /orgs/{org}/memberships/{username}
     */
    pub async fn orgs_set_membership_for_user(
        &self,
        org: &str,
        username: &str,
        body: &types::SetOrganizationMembershipUserRequest,
    ) -> Result<types::OrgMembership> {
        let url = format!(
            "/orgs/{}/memberships/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * orgs_remove_membership_for_user: DELETE /orgs/{org}/memberships/{username}
     */
    pub async fn orgs_remove_membership_for_user(&self, org: &str, username: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/memberships/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * migrations_list_for_org: GET /orgs/{org}/migrations
     */
    pub async fn migrations_list_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        exclude: &[String],
    ) -> Result<Vec<types::Migration>> {
        let url = format!(
            "/orgs/{}/migrations",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * migrations_start_for_org: POST /orgs/{org}/migrations
     */
    pub async fn migrations_start_for_org(
        &self,
        org: &str,
        body: &types::StartOrganizationMigrationRequest,
    ) -> Result<types::Migration> {
        let url = format!(
            "/orgs/{}/migrations",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * migrations_get_status_for_org: GET /orgs/{org}/migrations/{migration_id}
     */
    pub async fn migrations_get_status_for_org(
        &self,
        org: &str,
        migration_id: i64,
        exclude: &[String],
    ) -> Result<types::Migration> {
        let url = format!(
            "/orgs/{}/migrations/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&migration_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * migrations_download_archive_for_org: GET /orgs/{org}/migrations/{migration_id}/archive
     */
    pub async fn migrations_download_archive_for_org(
        &self,
        org: &str,
        migration_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/migrations/{}/archive",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&migration_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * migrations_delete_archive_for_org: DELETE /orgs/{org}/migrations/{migration_id}/archive
     */
    pub async fn migrations_delete_archive_for_org(
        &self,
        org: &str,
        migration_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/migrations/{}/archive",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&migration_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * migrations_unlock_repo_for_org: DELETE /orgs/{org}/migrations/{migration_id}/repos/{repo_name}/lock
     */
    pub async fn migrations_unlock_repo_for_org(
        &self,
        org: &str,
        migration_id: i64,
        repo_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/migrations/{}/repos/{}/lock",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&migration_id.to_string()),
            progenitor_support::encode_path(&repo_name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * migrations_list_repos_for_org: GET /orgs/{org}/migrations/{migration_id}/repositories
     */
    pub async fn migrations_list_repos_for_org(
        &self,
        org: &str,
        migration_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MinimalRepository>> {
        let url = format!(
            "/orgs/{}/migrations/{}/repositories",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&migration_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_list_outside_collaborators: GET /orgs/{org}/outside_collaborators
     */
    pub async fn orgs_list_outside_collaborators(
        &self,
        org: &str,
        filter: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/orgs/{}/outside_collaborators",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_convert_member_to_outside_collaborator: PUT /orgs/{org}/outside_collaborators/{username}
     */
    pub async fn orgs_convert_member_to_outside_collaborator(
        &self,
        org: &str,
        username: &str,
    ) -> Result<types::PutConvertOrganizationMemberOutsideCollaboratorAcceptedResponse> {
        let url = format!(
            "/orgs/{}/outside_collaborators/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * orgs_remove_outside_collaborator: DELETE /orgs/{org}/outside_collaborators/{username}
     */
    pub async fn orgs_remove_outside_collaborator(&self, org: &str, username: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/outside_collaborators/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * packages_get_package_for_organization: GET /orgs/{org}/packages/{package_type}/{package_name}
     */
    pub async fn packages_get_package_for_organization(
        &self,
        package_type: &str,
        package_name: &str,
        org: &str,
    ) -> Result<types::Package> {
        let url = format!(
            "/orgs/{}/packages/{}/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * packages_delete_package_for_org: DELETE /orgs/{org}/packages/{package_type}/{package_name}
     */
    pub async fn packages_delete_package_for_org(
        &self,
        package_type: &str,
        package_name: &str,
        org: &str,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/packages/{}/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * packages_restore_package_for_org: POST /orgs/{org}/packages/{package_type}/{package_name}/restore
     */
    pub async fn packages_restore_package_for_org(
        &self,
        package_type: &str,
        package_name: &str,
        org: &str,
        token: &str,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/packages/{}/{}/restore",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        let res = self
            .client
            .post(url)
            .query(&[("token", token.to_string())])
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * packages_get_all_package_versions_for_package_owned_by_org: GET /orgs/{org}/packages/{package_type}/{package_name}/versions
     */
    pub async fn packages_get_all_package_versions_for_package_owned_by_org(
        &self,
        package_type: &str,
        package_name: &str,
        org: &str,
        page: i64,
        per_page: i64,
        state: &str,
    ) -> Result<Vec<types::PackageVersion>> {
        let url = format!(
            "/orgs/{}/packages/{}/{}/versions",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * packages_get_package_version_for_organization: GET /orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}
     */
    pub async fn packages_get_package_version_for_organization(
        &self,
        package_type: &str,
        package_name: &str,
        org: &str,
        package_version_id: i64,
    ) -> Result<types::PackageVersion> {
        let url = format!(
            "/orgs/{}/packages/{}/{}/versions/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
            progenitor_support::encode_path(&package_version_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * packages_delete_package_version_for_org: DELETE /orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}
     */
    pub async fn packages_delete_package_version_for_org(
        &self,
        package_type: &str,
        package_name: &str,
        org: &str,
        package_version_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/packages/{}/{}/versions/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
            progenitor_support::encode_path(&package_version_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * packages_restore_package_version_for_org: POST /orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore
     */
    pub async fn packages_restore_package_version_for_org(
        &self,
        package_type: &str,
        package_name: &str,
        org: &str,
        package_version_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/packages/{}/{}/versions/{}/restore",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
            progenitor_support::encode_path(&package_version_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * projects_list_for_org: GET /orgs/{org}/projects
     */
    pub async fn projects_list_for_org(
        &self,
        org: &str,
        state: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Project>> {
        let url = format!(
            "/orgs/{}/projects",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_create_for_org: POST /orgs/{org}/projects
     */
    pub async fn projects_create_for_org(
        &self,
        org: &str,
        body: &types::CreateOrganizationProjectRequest,
    ) -> Result<types::Project> {
        let url = format!(
            "/orgs/{}/projects",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * orgs_list_public_members: GET /orgs/{org}/public_members
     */
    pub async fn orgs_list_public_members(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/orgs/{}/public_members",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_check_public_membership_for_user: GET /orgs/{org}/public_members/{username}
     */
    pub async fn orgs_check_public_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/public_members/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_set_public_membership_for_authenticated_user: PUT /orgs/{org}/public_members/{username}
     */
    pub async fn orgs_set_public_membership_for_authenticated_user(
        &self,
        org: &str,
        username: &str,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/public_members/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * orgs_remove_public_membership_for_authenticated_user: DELETE /orgs/{org}/public_members/{username}
     */
    pub async fn orgs_remove_public_membership_for_authenticated_user(
        &self,
        org: &str,
        username: &str,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/public_members/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_list_for_org: GET /orgs/{org}/repos
     */
    pub async fn repos_list_for_org(
        &self,
        org: &str,
        type_: &&str,
        sort: &str,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MinimalRepository>> {
        let url = format!(
            "/orgs/{}/repos",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_in_org: POST /orgs/{org}/repos
     */
    pub async fn repos_create_in_org(
        &self,
        org: &str,
        body: &types::CreateOrganizationRepositoryRequest,
    ) -> Result<types::Repository> {
        let url = format!(
            "/orgs/{}/repos",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * billing_get_github_actions_billing_org: GET /orgs/{org}/settings/billing/actions
     */
    pub async fn billing_get_github_actions_billing_org(
        &self,
        org: &str,
    ) -> Result<types::ActionsBillingUsage> {
        let url = format!(
            "/orgs/{}/settings/billing/actions",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * billing_get_github_packages_billing_org: GET /orgs/{org}/settings/billing/packages
     */
    pub async fn billing_get_github_packages_billing_org(
        &self,
        org: &str,
    ) -> Result<types::PackagesBillingUsage> {
        let url = format!(
            "/orgs/{}/settings/billing/packages",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * billing_get_shared_storage_billing_org: GET /orgs/{org}/settings/billing/shared-storage
     */
    pub async fn billing_get_shared_storage_billing_org(
        &self,
        org: &str,
    ) -> Result<types::CombinedBillingUsage> {
        let url = format!(
            "/orgs/{}/settings/billing/shared-storage",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_list_idp_groups_for_org: GET /orgs/{org}/team-sync/groups
     */
    pub async fn teams_list_idp_groups_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: &str,
    ) -> Result<types::GroupMapping> {
        let url = format!(
            "/orgs/{}/team-sync/groups",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_list: GET /orgs/{org}/teams
     */
    pub async fn teams_list(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Team>> {
        let url = format!(
            "/orgs/{}/teams",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_create: POST /orgs/{org}/teams
     */
    pub async fn teams_create(
        &self,
        org: &str,
        body: &types::CreateTeamRequest,
    ) -> Result<types::TeamFull> {
        let url = format!(
            "/orgs/{}/teams",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_get_by_name: GET /orgs/{org}/teams/{team_slug}
     */
    pub async fn teams_get_by_name(&self, org: &str, team_slug: &str) -> Result<types::TeamFull> {
        let url = format!(
            "/orgs/{}/teams/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_delete_in_org: DELETE /orgs/{org}/teams/{team_slug}
     */
    pub async fn teams_delete_in_org(&self, org: &str, team_slug: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_update_in_org: PATCH /orgs/{org}/teams/{team_slug}
     */
    pub async fn teams_update_in_org(
        &self,
        org: &str,
        team_slug: &str,
        body: &types::UpdateTeamRequest,
    ) -> Result<types::TeamFull> {
        let url = format!(
            "/orgs/{}/teams/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_list_discussions_in_org: GET /orgs/{org}/teams/{team_slug}/discussions
     */
    pub async fn teams_list_discussions_in_org(
        &self,
        org: &str,
        team_slug: &str,
        direction: &str,
        per_page: i64,
        page: i64,
        pinned: &str,
    ) -> Result<Vec<types::TeamDiscussion>> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_create_discussion_in_org: POST /orgs/{org}/teams/{team_slug}/discussions
     */
    pub async fn teams_create_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        body: &types::CreateDiscussionRequest,
    ) -> Result<types::TeamDiscussion> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_get_discussion_in_org: GET /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}
     */
    pub async fn teams_get_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
    ) -> Result<types::TeamDiscussion> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_delete_discussion_in_org: DELETE /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}
     */
    pub async fn teams_delete_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_update_discussion_in_org: PATCH /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}
     */
    pub async fn teams_update_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        body: &types::UpdateDiscussionRequest,
    ) -> Result<types::TeamDiscussion> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_list_discussion_comments_in_org: GET /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments
     */
    pub async fn teams_list_discussion_comments_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::TeamDiscussionComment>> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/comments",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_create_discussion_comment_in_org: POST /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments
     */
    pub async fn teams_create_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        body: &types::CreateDiscussionCommentRequest,
    ) -> Result<types::TeamDiscussionComment> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/comments",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_get_discussion_comment_in_org: GET /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}
     */
    pub async fn teams_get_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
    ) -> Result<types::TeamDiscussionComment> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/comments/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_delete_discussion_comment_in_org: DELETE /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}
     */
    pub async fn teams_delete_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/comments/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_update_discussion_comment_in_org: PATCH /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}
     */
    pub async fn teams_update_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
        body: &types::UpdateDiscussionCommentRequest,
    ) -> Result<types::TeamDiscussionComment> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/comments/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_list_for_team_discussion_comment_in_org: GET /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions
     */
    pub async fn reactions_list_for_team_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
        content: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Reaction>> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/comments/{}/reactions",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * reactions_create_for_team_discussion_comment_in_org: POST /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions
     */
    pub async fn reactions_create_for_team_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
        body: &types::CreateReactionTeamDiscussionCommentRequest,
    ) -> Result<types::Reaction> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/comments/{}/reactions",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_delete_for_team_discussion_comment: DELETE /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions/{reaction_id}
     */
    pub async fn reactions_delete_for_team_discussion_comment(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
        reaction_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/comments/{}/reactions/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
            progenitor_support::encode_path(&reaction_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * reactions_list_for_team_discussion_in_org: GET /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions
     */
    pub async fn reactions_list_for_team_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        content: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Reaction>> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/reactions",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * reactions_create_for_team_discussion_in_org: POST /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions
     */
    pub async fn reactions_create_for_team_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        body: &types::CreateReactionTeamDiscussionRequest,
    ) -> Result<types::Reaction> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/reactions",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_delete_for_team_discussion: DELETE /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions/{reaction_id}
     */
    pub async fn reactions_delete_for_team_discussion(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        reaction_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}/discussions/{}/reactions/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&reaction_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_list_pending_invitations_in_org: GET /orgs/{org}/teams/{team_slug}/invitations
     */
    pub async fn teams_list_pending_invitations_in_org(
        &self,
        org: &str,
        team_slug: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::OrganizationInvitation>> {
        let url = format!(
            "/orgs/{}/teams/{}/invitations",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_list_members_in_org: GET /orgs/{org}/teams/{team_slug}/members
     */
    pub async fn teams_list_members_in_org(
        &self,
        org: &str,
        team_slug: &str,
        role: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/orgs/{}/teams/{}/members",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_get_membership_for_user_in_org: GET /orgs/{org}/teams/{team_slug}/memberships/{username}
     */
    pub async fn teams_get_membership_for_user_in_org(
        &self,
        org: &str,
        team_slug: &str,
        username: &str,
    ) -> Result<types::TeamMembership> {
        let url = format!(
            "/orgs/{}/teams/{}/memberships/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_add_or_update_membership_for_user_in_org: PUT /orgs/{org}/teams/{team_slug}/memberships/{username}
     */
    pub async fn teams_add_or_update_membership_for_user_in_org(
        &self,
        org: &str,
        team_slug: &str,
        username: &str,
        body: &types::AddUpdateTeamMembershipUserRequest,
    ) -> Result<types::TeamMembership> {
        let url = format!(
            "/orgs/{}/teams/{}/memberships/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_remove_membership_for_user_in_org: DELETE /orgs/{org}/teams/{team_slug}/memberships/{username}
     */
    pub async fn teams_remove_membership_for_user_in_org(
        &self,
        org: &str,
        team_slug: &str,
        username: &str,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}/memberships/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_list_projects_in_org: GET /orgs/{org}/teams/{team_slug}/projects
     */
    pub async fn teams_list_projects_in_org(
        &self,
        org: &str,
        team_slug: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::TeamProject>> {
        let url = format!(
            "/orgs/{}/teams/{}/projects",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_check_permissions_for_project_in_org: GET /orgs/{org}/teams/{team_slug}/projects/{project_id}
     */
    pub async fn teams_check_permissions_for_project_in_org(
        &self,
        org: &str,
        team_slug: &str,
        project_id: i64,
    ) -> Result<types::TeamProject> {
        let url = format!(
            "/orgs/{}/teams/{}/projects/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&project_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_add_or_update_project_permissions_in_org: PUT /orgs/{org}/teams/{team_slug}/projects/{project_id}
     */
    pub async fn teams_add_or_update_project_permissions_in_org(
        &self,
        org: &str,
        team_slug: &str,
        project_id: i64,
        body: &types::AddUpdateTeamProjectPermissionsRequest,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}/projects/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&project_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_remove_project_in_org: DELETE /orgs/{org}/teams/{team_slug}/projects/{project_id}
     */
    pub async fn teams_remove_project_in_org(
        &self,
        org: &str,
        team_slug: &str,
        project_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}/projects/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&project_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_list_repos_in_org: GET /orgs/{org}/teams/{team_slug}/repos
     */
    pub async fn teams_list_repos_in_org(
        &self,
        org: &str,
        team_slug: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MinimalRepository>> {
        let url = format!(
            "/orgs/{}/teams/{}/repos",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_check_permissions_for_repo_in_org: GET /orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}
     */
    pub async fn teams_check_permissions_for_repo_in_org(
        &self,
        org: &str,
        team_slug: &str,
        owner: &str,
        repo: &str,
    ) -> Result<types::TeamRepository> {
        let url = format!(
            "/orgs/{}/teams/{}/repos/{}/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_add_or_update_repo_permissions_in_org: PUT /orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}
     */
    pub async fn teams_add_or_update_repo_permissions_in_org(
        &self,
        org: &str,
        team_slug: &str,
        owner: &str,
        repo: &str,
        body: &types::AddUpdateTeamRepositoryPermissionsRequest,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}/repos/{}/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_remove_repo_in_org: DELETE /orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}
     */
    pub async fn teams_remove_repo_in_org(
        &self,
        org: &str,
        team_slug: &str,
        owner: &str,
        repo: &str,
    ) -> Result<()> {
        let url = format!(
            "/orgs/{}/teams/{}/repos/{}/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_list_idp_groups_in_org: GET /orgs/{org}/teams/{team_slug}/team-sync/group-mappings
     */
    pub async fn teams_list_idp_groups_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> Result<types::GroupMapping> {
        let url = format!(
            "/orgs/{}/teams/{}/team-sync/group-mappings",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_create_or_update_idp_group_connections_in_org: PATCH /orgs/{org}/teams/{team_slug}/team-sync/group-mappings
     */
    pub async fn teams_create_or_update_idp_group_connections_in_org(
        &self,
        org: &str,
        team_slug: &str,
        body: &types::CreateUpdateIdpGroupConnectionsRequest,
    ) -> Result<types::GroupMapping> {
        let url = format!(
            "/orgs/{}/teams/{}/team-sync/group-mappings",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_list_child_in_org: GET /orgs/{org}/teams/{team_slug}/teams
     */
    pub async fn teams_list_child_in_org(
        &self,
        org: &str,
        team_slug: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Team>> {
        let url = format!(
            "/orgs/{}/teams/{}/teams",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&team_slug.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_get_card: GET /projects/columns/cards/{card_id}
     */
    pub async fn projects_get_card(&self, card_id: i64) -> Result<types::ProjectCard> {
        let url = format!(
            "/projects/columns/cards/{}",
            progenitor_support::encode_path(&card_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_delete_card: DELETE /projects/columns/cards/{card_id}
     */
    pub async fn projects_delete_card(&self, card_id: i64) -> Result<()> {
        let url = format!(
            "/projects/columns/cards/{}",
            progenitor_support::encode_path(&card_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * projects_update_card: PATCH /projects/columns/cards/{card_id}
     */
    pub async fn projects_update_card(
        &self,
        card_id: i64,
        body: &types::UpdateExistingProjectCardRequest,
    ) -> Result<types::ProjectCard> {
        let url = format!(
            "/projects/columns/cards/{}",
            progenitor_support::encode_path(&card_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * projects_move_card: POST /projects/columns/cards/{card_id}/moves
     */
    pub async fn projects_move_card(
        &self,
        card_id: i64,
        body: &types::MoveProjectCardRequest,
    ) -> Result<types::PostMoveProjectCardCreatedResponse> {
        let url = format!(
            "/projects/columns/cards/{}/moves",
            progenitor_support::encode_path(&card_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * projects_get_column: GET /projects/columns/{column_id}
     */
    pub async fn projects_get_column(&self, column_id: i64) -> Result<types::ProjectColumn> {
        let url = format!(
            "/projects/columns/{}",
            progenitor_support::encode_path(&column_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_delete_column: DELETE /projects/columns/{column_id}
     */
    pub async fn projects_delete_column(&self, column_id: i64) -> Result<()> {
        let url = format!(
            "/projects/columns/{}",
            progenitor_support::encode_path(&column_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * projects_update_column: PATCH /projects/columns/{column_id}
     */
    pub async fn projects_update_column(
        &self,
        column_id: i64,
        body: &types::UpdateExistingProjectColumnRequest,
    ) -> Result<types::ProjectColumn> {
        let url = format!(
            "/projects/columns/{}",
            progenitor_support::encode_path(&column_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * projects_list_cards: GET /projects/columns/{column_id}/cards
     */
    pub async fn projects_list_cards(
        &self,
        column_id: i64,
        archived_state: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::ProjectCard>> {
        let url = format!(
            "/projects/columns/{}/cards",
            progenitor_support::encode_path(&column_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_create_card: POST /projects/columns/{column_id}/cards
     */
    pub async fn projects_create_card(
        &self,
        column_id: i64,
        body: &types::CreateProjectCardRequest,
    ) -> Result<types::ProjectCard> {
        let url = format!(
            "/projects/columns/{}/cards",
            progenitor_support::encode_path(&column_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * projects_move_column: POST /projects/columns/{column_id}/moves
     */
    pub async fn projects_move_column(
        &self,
        column_id: i64,
        body: &types::MoveProjectColumnRequest,
    ) -> Result<types::PostMoveProjectColumnCreatedResponse> {
        let url = format!(
            "/projects/columns/{}/moves",
            progenitor_support::encode_path(&column_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * projects_get: GET /projects/{project_id}
     */
    pub async fn projects_get(&self, project_id: i64) -> Result<types::Project> {
        let url = format!(
            "/projects/{}",
            progenitor_support::encode_path(&project_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_delete: DELETE /projects/{project_id}
     */
    pub async fn projects_delete(&self, project_id: i64) -> Result<()> {
        let url = format!(
            "/projects/{}",
            progenitor_support::encode_path(&project_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * projects_update: PATCH /projects/{project_id}
     */
    pub async fn projects_update(
        &self,
        project_id: i64,
        body: &types::UpdateProjectRequest,
    ) -> Result<types::Project> {
        let url = format!(
            "/projects/{}",
            progenitor_support::encode_path(&project_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * projects_list_collaborators: GET /projects/{project_id}/collaborators
     */
    pub async fn projects_list_collaborators(
        &self,
        project_id: i64,
        affiliation: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/projects/{}/collaborators",
            progenitor_support::encode_path(&project_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_add_collaborator: PUT /projects/{project_id}/collaborators/{username}
     */
    pub async fn projects_add_collaborator(
        &self,
        project_id: i64,
        username: &str,
        body: &types::AddProjectCollaboratorRequest,
    ) -> Result<()> {
        let url = format!(
            "/projects/{}/collaborators/{}",
            progenitor_support::encode_path(&project_id.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * projects_remove_collaborator: DELETE /projects/{project_id}/collaborators/{username}
     */
    pub async fn projects_remove_collaborator(
        &self,
        project_id: i64,
        username: &str,
    ) -> Result<()> {
        let url = format!(
            "/projects/{}/collaborators/{}",
            progenitor_support::encode_path(&project_id.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * projects_get_permission_for_user: GET /projects/{project_id}/collaborators/{username}/permission
     */
    pub async fn projects_get_permission_for_user(
        &self,
        project_id: i64,
        username: &str,
    ) -> Result<types::RepositoryCollaboratorPermission> {
        let url = format!(
            "/projects/{}/collaborators/{}/permission",
            progenitor_support::encode_path(&project_id.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_list_columns: GET /projects/{project_id}/columns
     */
    pub async fn projects_list_columns(
        &self,
        project_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::ProjectColumn>> {
        let url = format!(
            "/projects/{}/columns",
            progenitor_support::encode_path(&project_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_create_column: POST /projects/{project_id}/columns
     */
    pub async fn projects_create_column(
        &self,
        project_id: i64,
        body: &types::CreateProjectColumnRequest,
    ) -> Result<types::ProjectColumn> {
        let url = format!(
            "/projects/{}/columns",
            progenitor_support::encode_path(&project_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * rate_limit_get: GET /rate_limit
     */
    pub async fn rate_limit_get(&self) -> Result<types::RateLimitOverview> {
        let url = "/rate_limit".to_string();
        self.get(&url).await
    }

    /**
     * reactions_delete_legacy: DELETE /reactions/{reaction_id}
     */
    pub async fn reactions_delete_legacy(&self, reaction_id: i64) -> Result<()> {
        let url = format!(
            "/reactions/{}",
            progenitor_support::encode_path(&reaction_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_get: GET /repos/{owner}/{repo}
     */
    pub async fn repos_get(&self, owner: &str, repo: &str) -> Result<types::FullRepository> {
        let url = format!(
            "/repos/{}/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete: DELETE /repos/{owner}/{repo}
     */
    pub async fn repos_delete(&self, owner: &str, repo: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_update: PATCH /repos/{owner}/{repo}
     */
    pub async fn repos_update(
        &self,
        owner: &str,
        repo: &str,
        body: &types::UpdateRepositoryRequest,
    ) -> Result<types::FullRepository> {
        let url = format!(
            "/repos/{}/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_list_artifacts_for_repo: GET /repos/{owner}/{repo}/actions/artifacts
     */
    pub async fn actions_list_artifacts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListArtifactsRepositoryOkResponse> {
        let url = format!(
            "/repos/{}/{}/actions/artifacts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_artifact: GET /repos/{owner}/{repo}/actions/artifacts/{artifact_id}
     */
    pub async fn actions_get_artifact(
        &self,
        owner: &str,
        repo: &str,
        artifact_id: i64,
    ) -> Result<types::Artifact> {
        let url = format!(
            "/repos/{}/{}/actions/artifacts/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&artifact_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_delete_artifact: DELETE /repos/{owner}/{repo}/actions/artifacts/{artifact_id}
     */
    pub async fn actions_delete_artifact(
        &self,
        owner: &str,
        repo: &str,
        artifact_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/artifacts/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&artifact_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_download_artifact: GET /repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}
     */
    pub async fn actions_download_artifact(
        &self,
        owner: &str,
        repo: &str,
        artifact_id: i64,
        archive_format: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/artifacts/{}/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&artifact_id.to_string()),
            progenitor_support::encode_path(&archive_format.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_job_for_workflow_run: GET /repos/{owner}/{repo}/actions/jobs/{job_id}
     */
    pub async fn actions_get_job_for_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        job_id: i64,
    ) -> Result<types::Job> {
        let url = format!(
            "/repos/{}/{}/actions/jobs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&job_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_download_job_logs_for_workflow_run: GET /repos/{owner}/{repo}/actions/jobs/{job_id}/logs
     */
    pub async fn actions_download_job_logs_for_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        job_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/jobs/{}/logs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&job_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_github_actions_permissions_repository: GET /repos/{owner}/{repo}/actions/permissions
     */
    pub async fn actions_get_github_actions_permissions_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::ActionsRepositoryPermissions> {
        let url = format!(
            "/repos/{}/{}/actions/permissions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_set_github_actions_permissions_repository: PUT /repos/{owner}/{repo}/actions/permissions
     */
    pub async fn actions_set_github_actions_permissions_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &types::SetGithubActionsPermissionsRepositoryRequest,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/permissions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_get_allowed_actions_repository: GET /repos/{owner}/{repo}/actions/permissions/selected-actions
     */
    pub async fn actions_get_allowed_actions_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::SelectedActions> {
        let url = format!(
            "/repos/{}/{}/actions/permissions/selected-actions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_set_allowed_actions_repository: PUT /repos/{owner}/{repo}/actions/permissions/selected-actions
     */
    pub async fn actions_set_allowed_actions_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &types::SelectedActions,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/permissions/selected-actions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_self_hosted_runners_for_repo: GET /repos/{owner}/{repo}/actions/runners
     */
    pub async fn actions_list_self_hosted_runners_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListSelfDataHostedRunnersRepositoryOkResponse> {
        let url = format!(
            "/repos/{}/{}/actions/runners",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_list_runner_applications_for_repo: GET /repos/{owner}/{repo}/actions/runners/downloads
     */
    pub async fn actions_list_runner_applications_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<types::RunnerApplication>> {
        let url = format!(
            "/repos/{}/{}/actions/runners/downloads",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_create_registration_token_for_repo: POST /repos/{owner}/{repo}/actions/runners/registration-token
     */
    pub async fn actions_create_registration_token_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::AuthenticationToken> {
        let url = format!(
            "/repos/{}/{}/actions/runners/registration-token",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_create_remove_token_for_repo: POST /repos/{owner}/{repo}/actions/runners/remove-token
     */
    pub async fn actions_create_remove_token_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::AuthenticationToken> {
        let url = format!(
            "/repos/{}/{}/actions/runners/remove-token",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_get_self_hosted_runner_for_repo: GET /repos/{owner}/{repo}/actions/runners/{runner_id}
     */
    pub async fn actions_get_self_hosted_runner_for_repo(
        &self,
        owner: &str,
        repo: &str,
        runner_id: i64,
    ) -> Result<types::Runner> {
        let url = format!(
            "/repos/{}/{}/actions/runners/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_delete_self_hosted_runner_from_repo: DELETE /repos/{owner}/{repo}/actions/runners/{runner_id}
     */
    pub async fn actions_delete_self_hosted_runner_from_repo(
        &self,
        owner: &str,
        repo: &str,
        runner_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/runners/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&runner_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_workflow_runs_for_repo: GET /repos/{owner}/{repo}/actions/runs
     */
    pub async fn actions_list_workflow_runs_for_repo(
        &self,
        owner: &str,
        repo: &str,
        actor: &str,
        branch: &str,
        event: &str,
        status: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListWorkflowRunsRepositoryOkResponse> {
        let url = format!(
            "/repos/{}/{}/actions/runs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_workflow_run: GET /repos/{owner}/{repo}/actions/runs/{run_id}
     */
    pub async fn actions_get_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<types::WorkflowRun> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_delete_workflow_run: DELETE /repos/{owner}/{repo}/actions/runs/{run_id}
     */
    pub async fn actions_delete_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_get_reviews_for_run: GET /repos/{owner}/{repo}/actions/runs/{run_id}/approvals
     */
    pub async fn actions_get_reviews_for_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<Vec<types::EnvironmentApprovals>> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/approvals",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_approve_workflow_run: POST /repos/{owner}/{repo}/actions/runs/{run_id}/approve
     */
    pub async fn actions_approve_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<types::EmptyObject> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/approve",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_list_workflow_run_artifacts: GET /repos/{owner}/{repo}/actions/runs/{run_id}/artifacts
     */
    pub async fn actions_list_workflow_run_artifacts(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListWorkflowRunArtifactsOkResponse> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/artifacts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_cancel_workflow_run: POST /repos/{owner}/{repo}/actions/runs/{run_id}/cancel
     */
    pub async fn actions_cancel_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<types::PostCancelWorkflowRunAcceptedResponse> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/cancel",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_list_jobs_for_workflow_run: GET /repos/{owner}/{repo}/actions/runs/{run_id}/jobs
     */
    pub async fn actions_list_jobs_for_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        filter: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListJobsWorkflowRunOkResponse> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/jobs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_download_workflow_run_logs: GET /repos/{owner}/{repo}/actions/runs/{run_id}/logs
     */
    pub async fn actions_download_workflow_run_logs(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/logs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_delete_workflow_run_logs: DELETE /repos/{owner}/{repo}/actions/runs/{run_id}/logs
     */
    pub async fn actions_delete_workflow_run_logs(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/logs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_get_pending_deployments_for_run: GET /repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments
     */
    pub async fn actions_get_pending_deployments_for_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<Vec<types::PendingDeployment>> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/pending_deployments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_review_pending_deployments_for_run: POST /repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments
     */
    pub async fn actions_review_pending_deployments_for_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        body: &types::ReviewPendingDeploymentsWorkflowRunRequest,
    ) -> Result<Vec<types::Deployment>> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/pending_deployments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_re_run_workflow: POST /repos/{owner}/{repo}/actions/runs/{run_id}/rerun
     */
    pub async fn actions_re_run_workflow(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<types::PostReRunWorkflowCreatedResponse> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/rerun",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_get_workflow_run_usage: GET /repos/{owner}/{repo}/actions/runs/{run_id}/timing
     */
    pub async fn actions_get_workflow_run_usage(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> Result<types::WorkflowRunUsage> {
        let url = format!(
            "/repos/{}/{}/actions/runs/{}/timing",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&run_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_list_repo_secrets: GET /repos/{owner}/{repo}/actions/secrets
     */
    pub async fn actions_list_repo_secrets(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListRepositorySecretsOkResponse> {
        let url = format!(
            "/repos/{}/{}/actions/secrets",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_repo_public_key: GET /repos/{owner}/{repo}/actions/secrets/public-key
     */
    pub async fn actions_get_repo_public_key(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::ActionsPublicKey> {
        let url = format!(
            "/repos/{}/{}/actions/secrets/public-key",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_repo_secret: GET /repos/{owner}/{repo}/actions/secrets/{secret_name}
     */
    pub async fn actions_get_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secret_name: &str,
    ) -> Result<types::ActionsSecret> {
        let url = format!(
            "/repos/{}/{}/actions/secrets/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_create_or_update_repo_secret: PUT /repos/{owner}/{repo}/actions/secrets/{secret_name}
     */
    pub async fn actions_create_or_update_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secret_name: &str,
        body: &types::CreateUpdateRepositorySecretRequest,
    ) -> Result<types::PutCreateUpdateRepositorySecretCreatedResponse> {
        let url = format!(
            "/repos/{}/{}/actions/secrets/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_delete_repo_secret: DELETE /repos/{owner}/{repo}/actions/secrets/{secret_name}
     */
    pub async fn actions_delete_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secret_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/secrets/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_repo_workflows: GET /repos/{owner}/{repo}/actions/workflows
     */
    pub async fn actions_list_repo_workflows(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListRepositoryWorkflowsOkResponse> {
        let url = format!(
            "/repos/{}/{}/actions/workflows",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_workflow: GET /repos/{owner}/{repo}/actions/workflows/{workflow_id}
     */
    pub async fn actions_get_workflow(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
    ) -> Result<types::Workflow> {
        let url = format!(
            "/repos/{}/{}/actions/workflows/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&workflow_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_disable_workflow: PUT /repos/{owner}/{repo}/actions/workflows/{workflow_id}/disable
     */
    pub async fn actions_disable_workflow(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/workflows/{}/disable",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&workflow_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_create_workflow_dispatch: POST /repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches
     */
    pub async fn actions_create_workflow_dispatch(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
        body: &types::CreateWorkflowDispatchEventRequest,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/workflows/{}/dispatches",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&workflow_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_enable_workflow: PUT /repos/{owner}/{repo}/actions/workflows/{workflow_id}/enable
     */
    pub async fn actions_enable_workflow(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/actions/workflows/{}/enable",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&workflow_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * actions_list_workflow_runs: GET /repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs
     */
    pub async fn actions_list_workflow_runs(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
        actor: &str,
        branch: &str,
        event: &str,
        status: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListWorkflowRunsOkResponse> {
        let url = format!(
            "/repos/{}/{}/actions/workflows/{}/runs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&workflow_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_workflow_usage: GET /repos/{owner}/{repo}/actions/workflows/{workflow_id}/timing
     */
    pub async fn actions_get_workflow_usage(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
    ) -> Result<types::WorkflowUsage> {
        let url = format!(
            "/repos/{}/{}/actions/workflows/{}/timing",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&workflow_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_list_assignees: GET /repos/{owner}/{repo}/assignees
     */
    pub async fn issues_list_assignees(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/repos/{}/{}/assignees",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_check_user_can_be_assigned: GET /repos/{owner}/{repo}/assignees/{assignee}
     */
    pub async fn issues_check_user_can_be_assigned(
        &self,
        owner: &str,
        repo: &str,
        assignee: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/assignees/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&assignee.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_enable_automated_security_fixes: PUT /repos/{owner}/{repo}/automated-security-fixes
     */
    pub async fn repos_enable_automated_security_fixes(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/automated-security-fixes",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_disable_automated_security_fixes: DELETE /repos/{owner}/{repo}/automated-security-fixes
     */
    pub async fn repos_disable_automated_security_fixes(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/automated-security-fixes",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_list_branches: GET /repos/{owner}/{repo}/branches
     */
    pub async fn repos_list_branches(
        &self,
        owner: &str,
        repo: &str,
        protected: bool,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::ShortBranch>> {
        let url = format!(
            "/repos/{}/{}/branches",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_branch: GET /repos/{owner}/{repo}/branches/{branch}
     */
    pub async fn repos_get_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<types::BranchWithProtection> {
        let url = format!(
            "/repos/{}/{}/branches/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_branch_protection: GET /repos/{owner}/{repo}/branches/{branch}/protection
     */
    pub async fn repos_get_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<types::BranchProtection> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_update_branch_protection: PUT /repos/{owner}/{repo}/branches/{branch}/protection
     */
    pub async fn repos_update_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::UpdateBranchProtectionRequest,
    ) -> Result<types::ProtectedBranch> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_delete_branch_protection: DELETE /repos/{owner}/{repo}/branches/{branch}/protection
     */
    pub async fn repos_delete_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_get_admin_branch_protection: GET /repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins
     */
    pub async fn repos_get_admin_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<types::ProtectedBranchAdminEnforced> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/enforce_admins",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_set_admin_branch_protection: POST /repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins
     */
    pub async fn repos_set_admin_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<types::ProtectedBranchAdminEnforced> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/enforce_admins",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_delete_admin_branch_protection: DELETE /repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins
     */
    pub async fn repos_delete_admin_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/enforce_admins",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_get_pull_request_review_protection: GET /repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews
     */
    pub async fn repos_get_pull_request_review_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<types::ProtectedBranchPullRequestReview> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_pull_request_reviews",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete_pull_request_review_protection: DELETE /repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews
     */
    pub async fn repos_delete_pull_request_review_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_pull_request_reviews",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_update_pull_request_review_protection: PATCH /repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews
     */
    pub async fn repos_update_pull_request_review_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::UpdatePullRequestReviewProtectionRequest,
    ) -> Result<types::ProtectedBranchPullRequestReview> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_pull_request_reviews",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_commit_signature_protection: GET /repos/{owner}/{repo}/branches/{branch}/protection/required_signatures
     */
    pub async fn repos_get_commit_signature_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<types::ProtectedBranchAdminEnforced> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_signatures",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_commit_signature_protection: POST /repos/{owner}/{repo}/branches/{branch}/protection/required_signatures
     */
    pub async fn repos_create_commit_signature_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<types::ProtectedBranchAdminEnforced> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_signatures",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_delete_commit_signature_protection: DELETE /repos/{owner}/{repo}/branches/{branch}/protection/required_signatures
     */
    pub async fn repos_delete_commit_signature_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_signatures",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_get_status_checks_protection: GET /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks
     */
    pub async fn repos_get_status_checks_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<types::StatusCheckPolicy> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_status_checks",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_remove_status_check_protection: DELETE /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks
     */
    pub async fn repos_remove_status_check_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_status_checks",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_update_status_check_protection: PATCH /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks
     */
    pub async fn repos_update_status_check_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::UpdateStatusCheckProtectionRequest,
    ) -> Result<types::StatusCheckPolicy> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_status_checks",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_all_status_check_contexts: GET /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts
     */
    pub async fn repos_get_all_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<Vec<String>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_set_status_check_contexts: PUT /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts
     */
    pub async fn repos_set_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::SetStatusCheckContextsRequest,
    ) -> Result<Vec<String>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_add_status_check_contexts: POST /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts
     */
    pub async fn repos_add_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::AddStatusCheckContextsRequest,
    ) -> Result<Vec<String>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_remove_status_check_contexts: DELETE /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts
     */
    pub async fn repos_remove_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::RemoveStatusCheckContextsRequest,
    ) -> Result<Vec<String>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_access_restrictions: GET /repos/{owner}/{repo}/branches/{branch}/protection/restrictions
     */
    pub async fn repos_get_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<types::BranchRestrictionPolicy> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete_access_restrictions: DELETE /repos/{owner}/{repo}/branches/{branch}/protection/restrictions
     */
    pub async fn repos_delete_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_get_apps_with_access_to_protected_branch: GET /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps
     */
    pub async fn repos_get_apps_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<Vec<types::Integration>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/apps",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_set_app_access_restrictions: PUT /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps
     */
    pub async fn repos_set_app_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::SetAppAccessRestrictionsRequest,
    ) -> Result<Vec<types::Integration>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/apps",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_add_app_access_restrictions: POST /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps
     */
    pub async fn repos_add_app_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::AddAppAccessRestrictionsRequest,
    ) -> Result<Vec<types::Integration>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/apps",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_remove_app_access_restrictions: DELETE /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps
     */
    pub async fn repos_remove_app_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::RemoveAppAccessRestrictionsRequest,
    ) -> Result<Vec<types::Integration>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/apps",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_teams_with_access_to_protected_branch: GET /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams
     */
    pub async fn repos_get_teams_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<Vec<types::Team>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/teams",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_set_team_access_restrictions: PUT /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams
     */
    pub async fn repos_set_team_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::SetTeamAccessRestrictionsRequest,
    ) -> Result<Vec<types::Team>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/teams",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_add_team_access_restrictions: POST /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams
     */
    pub async fn repos_add_team_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::AddTeamAccessRestrictionsRequest,
    ) -> Result<Vec<types::Team>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/teams",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_remove_team_access_restrictions: DELETE /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams
     */
    pub async fn repos_remove_team_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::RemoveTeamAccessRestrictionsRequest,
    ) -> Result<Vec<types::Team>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/teams",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_users_with_access_to_protected_branch: GET /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users
     */
    pub async fn repos_get_users_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/users",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_set_user_access_restrictions: PUT /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users
     */
    pub async fn repos_set_user_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::SetUserAccessRestrictionsRequest,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/users",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_add_user_access_restrictions: POST /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users
     */
    pub async fn repos_add_user_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::AddUserAccessRestrictionsRequest,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/users",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_remove_user_access_restrictions: DELETE /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users
     */
    pub async fn repos_remove_user_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::RemoveUserAccessRestrictionsRequest,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/repos/{}/{}/branches/{}/protection/restrictions/users",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_rename_branch: POST /repos/{owner}/{repo}/branches/{branch}/rename
     */
    pub async fn repos_rename_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &types::RenameBranchRequest,
    ) -> Result<types::BranchWithProtection> {
        let url = format!(
            "/repos/{}/{}/branches/{}/rename",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&branch.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * checks_create: POST /repos/{owner}/{repo}/check-runs
     */
    pub async fn checks_create(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateCheckRunRequest,
    ) -> Result<types::CheckRun> {
        let url = format!(
            "/repos/{}/{}/check-runs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * checks_get: GET /repos/{owner}/{repo}/check-runs/{check_run_id}
     */
    pub async fn checks_get(
        &self,
        owner: &str,
        repo: &str,
        check_run_id: i64,
    ) -> Result<types::CheckRun> {
        let url = format!(
            "/repos/{}/{}/check-runs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&check_run_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * checks_update: PATCH /repos/{owner}/{repo}/check-runs/{check_run_id}
     */
    pub async fn checks_update(
        &self,
        owner: &str,
        repo: &str,
        check_run_id: i64,
        body: &types::UpdateCheckRunRequest,
    ) -> Result<types::CheckRun> {
        let url = format!(
            "/repos/{}/{}/check-runs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&check_run_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * checks_list_annotations: GET /repos/{owner}/{repo}/check-runs/{check_run_id}/annotations
     */
    pub async fn checks_list_annotations(
        &self,
        owner: &str,
        repo: &str,
        check_run_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::CheckAnnotation>> {
        let url = format!(
            "/repos/{}/{}/check-runs/{}/annotations",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&check_run_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * checks_create_suite: POST /repos/{owner}/{repo}/check-suites
     */
    pub async fn checks_create_suite(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateCheckSuiteRequest,
    ) -> Result<types::CheckSuite> {
        let url = format!(
            "/repos/{}/{}/check-suites",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * checks_set_suites_preferences: PATCH /repos/{owner}/{repo}/check-suites/preferences
     */
    pub async fn checks_set_suites_preferences(
        &self,
        owner: &str,
        repo: &str,
        body: &types::UpdateRepositoryPreferencesCheckSuitesRequest,
    ) -> Result<types::CheckSuitePreference> {
        let url = format!(
            "/repos/{}/{}/check-suites/preferences",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * checks_get_suite: GET /repos/{owner}/{repo}/check-suites/{check_suite_id}
     */
    pub async fn checks_get_suite(
        &self,
        owner: &str,
        repo: &str,
        check_suite_id: i64,
    ) -> Result<types::CheckSuite> {
        let url = format!(
            "/repos/{}/{}/check-suites/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&check_suite_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * checks_list_for_suite: GET /repos/{owner}/{repo}/check-suites/{check_suite_id}/check-runs
     */
    pub async fn checks_list_for_suite(
        &self,
        owner: &str,
        repo: &str,
        check_suite_id: i64,
        check_name: &str,
        status: &str,
        filter: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListCheckRunsinCheckSuiteOkResponse> {
        let url = format!(
            "/repos/{}/{}/check-suites/{}/check-runs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&check_suite_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * checks_rerequest_suite: POST /repos/{owner}/{repo}/check-suites/{check_suite_id}/rerequest
     */
    pub async fn checks_rerequest_suite(
        &self,
        owner: &str,
        repo: &str,
        check_suite_id: i64,
    ) -> Result<types::PostRerequestCheckSuiteCreatedResponse> {
        let url = format!(
            "/repos/{}/{}/check-suites/{}/rerequest",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&check_suite_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * code_scanning_list_alerts_for_repo: GET /repos/{owner}/{repo}/code-scanning/alerts
     */
    pub async fn code_scanning_list_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        tool_name: &str,
        tool_guid: &str,
        page: i64,
        per_page: i64,
        ref_: &&str,
        state: &str,
    ) -> Result<Vec<types::CodeScanningAlertItems>> {
        let url = format!(
            "/repos/{}/{}/code-scanning/alerts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * code_scanning_get_alert: GET /repos/{owner}/{repo}/code-scanning/alerts/{alert_number}
     */
    pub async fn code_scanning_get_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: &str,
    ) -> Result<types::CodeScanningAlert> {
        let url = format!(
            "/repos/{}/{}/code-scanning/alerts/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&alert_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * code_scanning_update_alert: PATCH /repos/{owner}/{repo}/code-scanning/alerts/{alert_number}
     */
    pub async fn code_scanning_update_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: &str,
        body: &types::UpdateCodeScanningAlertRequest,
    ) -> Result<types::CodeScanningAlert> {
        let url = format!(
            "/repos/{}/{}/code-scanning/alerts/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&alert_number.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * code_scanning_list_alert_instances: GET /repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/instances
     */
    pub async fn code_scanning_list_alert_instances(
        &self,
        owner: &str,
        repo: &str,
        alert_number: &str,
        page: i64,
        per_page: i64,
        ref_: &&str,
    ) -> Result<Vec<types::CodeScanningAlertInstance>> {
        let url = format!(
            "/repos/{}/{}/code-scanning/alerts/{}/instances",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&alert_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * code_scanning_list_recent_analyses: GET /repos/{owner}/{repo}/code-scanning/analyses
     */
    pub async fn code_scanning_list_recent_analyses(
        &self,
        owner: &str,
        repo: &str,
        tool_name: &str,
        tool_guid: &str,
        page: i64,
        per_page: i64,
        ref_: &&str,
        sarif_id: &str,
    ) -> Result<Vec<types::CodeScanningAnalysis>> {
        let url = format!(
            "/repos/{}/{}/code-scanning/analyses",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * code_scanning_get_analysis: GET /repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}
     */
    pub async fn code_scanning_get_analysis(
        &self,
        owner: &str,
        repo: &str,
        analysis_id: i64,
    ) -> Result<types::CodeScanningAnalysis> {
        let url = format!(
            "/repos/{}/{}/code-scanning/analyses/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&analysis_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * code_scanning_delete_analysis: DELETE /repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}
     */
    pub async fn code_scanning_delete_analysis(
        &self,
        owner: &str,
        repo: &str,
        analysis_id: i64,
        confirm_delete: &str,
    ) -> Result<types::CodeScanningAnalysisDeletion> {
        let url = format!(
            "/repos/{}/{}/code-scanning/analyses/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&analysis_id.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .query(&[("confirm_delete", confirm_delete.to_string())])
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * code_scanning_upload_sarif: POST /repos/{owner}/{repo}/code-scanning/sarifs
     */
    pub async fn code_scanning_upload_sarif(
        &self,
        owner: &str,
        repo: &str,
        body: &types::UploadAnalysisasSarifDataRequest,
    ) -> Result<types::CodeScanningSarifsReceipt> {
        let url = format!(
            "/repos/{}/{}/code-scanning/sarifs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * code_scanning_get_sarif: GET /repos/{owner}/{repo}/code-scanning/sarifs/{sarif_id}
     */
    pub async fn code_scanning_get_sarif(
        &self,
        owner: &str,
        repo: &str,
        sarif_id: &str,
    ) -> Result<types::CodeScanningSarifsStatus> {
        let url = format!(
            "/repos/{}/{}/code-scanning/sarifs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&sarif_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_collaborators: GET /repos/{owner}/{repo}/collaborators
     */
    pub async fn repos_list_collaborators(
        &self,
        owner: &str,
        repo: &str,
        affiliation: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Collaborator>> {
        let url = format!(
            "/repos/{}/{}/collaborators",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_check_collaborator: GET /repos/{owner}/{repo}/collaborators/{username}
     */
    pub async fn repos_check_collaborator(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/collaborators/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_add_collaborator: PUT /repos/{owner}/{repo}/collaborators/{username}
     */
    pub async fn repos_add_collaborator(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
        body: &types::AddRepositoryCollaboratorRequest,
    ) -> Result<types::RepositoryInvitation> {
        let url = format!(
            "/repos/{}/{}/collaborators/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_remove_collaborator: DELETE /repos/{owner}/{repo}/collaborators/{username}
     */
    pub async fn repos_remove_collaborator(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/collaborators/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_get_collaborator_permission_level: GET /repos/{owner}/{repo}/collaborators/{username}/permission
     */
    pub async fn repos_get_collaborator_permission_level(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> Result<types::RepositoryCollaboratorPermission> {
        let url = format!(
            "/repos/{}/{}/collaborators/{}/permission",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_commit_comments_for_repo: GET /repos/{owner}/{repo}/comments
     */
    pub async fn repos_list_commit_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::CommitComment>> {
        let url = format!(
            "/repos/{}/{}/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_commit_comment: GET /repos/{owner}/{repo}/comments/{comment_id}
     */
    pub async fn repos_get_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> Result<types::CommitComment> {
        let url = format!(
            "/repos/{}/{}/comments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete_commit_comment: DELETE /repos/{owner}/{repo}/comments/{comment_id}
     */
    pub async fn repos_delete_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/comments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_update_commit_comment: PATCH /repos/{owner}/{repo}/comments/{comment_id}
     */
    pub async fn repos_update_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &types::UpdateCommitCommentRequest,
    ) -> Result<types::CommitComment> {
        let url = format!(
            "/repos/{}/{}/comments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_list_for_commit_comment: GET /repos/{owner}/{repo}/comments/{comment_id}/reactions
     */
    pub async fn reactions_list_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Reaction>> {
        let url = format!(
            "/repos/{}/{}/comments/{}/reactions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * reactions_create_for_commit_comment: POST /repos/{owner}/{repo}/comments/{comment_id}/reactions
     */
    pub async fn reactions_create_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &types::CreateReactionCommitCommentRequest,
    ) -> Result<types::Reaction> {
        let url = format!(
            "/repos/{}/{}/comments/{}/reactions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_delete_for_commit_comment: DELETE /repos/{owner}/{repo}/comments/{comment_id}/reactions/{reaction_id}
     */
    pub async fn reactions_delete_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/comments/{}/reactions/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
            progenitor_support::encode_path(&reaction_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_list_commits: GET /repos/{owner}/{repo}/commits
     */
    pub async fn repos_list_commits(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        path: &str,
        author: &str,
        since: DateTime<Utc>,
        until: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Commit>> {
        let url = format!(
            "/repos/{}/{}/commits",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_branches_for_head_commit: GET /repos/{owner}/{repo}/commits/{commit_sha}/branches-where-head
     */
    pub async fn repos_list_branches_for_head_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> Result<Vec<types::BranchShort>> {
        let url = format!(
            "/repos/{}/{}/commits/{}/branches-where-head",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&commit_sha.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_comments_for_commit: GET /repos/{owner}/{repo}/commits/{commit_sha}/comments
     */
    pub async fn repos_list_comments_for_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::CommitComment>> {
        let url = format!(
            "/repos/{}/{}/commits/{}/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&commit_sha.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_commit_comment: POST /repos/{owner}/{repo}/commits/{commit_sha}/comments
     */
    pub async fn repos_create_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        body: &types::CreateCommitCommentRequest,
    ) -> Result<types::CommitComment> {
        let url = format!(
            "/repos/{}/{}/commits/{}/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&commit_sha.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_list_pull_requests_associated_with_commit: GET /repos/{owner}/{repo}/commits/{commit_sha}/pulls
     */
    pub async fn repos_list_pull_requests_associated_with_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::PullRequestSimple>> {
        let url = format!(
            "/repos/{}/{}/commits/{}/pulls",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&commit_sha.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_commit: GET /repos/{owner}/{repo}/commits/{ref}
     */
    pub async fn repos_get_commit(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
        per_page: i64,
        ref_: &str,
    ) -> Result<types::Commit> {
        let url = format!(
            "/repos/{}/{}/commits/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        self.get(&url).await
    }

    /**
     * checks_list_for_ref: GET /repos/{owner}/{repo}/commits/{ref}/check-runs
     */
    pub async fn checks_list_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        check_name: &str,
        status: &str,
        filter: &str,
        per_page: i64,
        page: i64,
        app_id: i64,
    ) -> Result<types::GetListCheckRunsGitReferenceOkResponse> {
        let url = format!(
            "/repos/{}/{}/commits/{}/check-runs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        self.get(&url).await
    }

    /**
     * checks_list_suites_for_ref: GET /repos/{owner}/{repo}/commits/{ref}/check-suites
     */
    pub async fn checks_list_suites_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        app_id: i64,
        check_name: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListCheckSuitesGitReferenceOkResponse> {
        let url = format!(
            "/repos/{}/{}/commits/{}/check-suites",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_combined_status_for_ref: GET /repos/{owner}/{repo}/commits/{ref}/status
     */
    pub async fn repos_get_combined_status_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::CombinedCommitStatus> {
        let url = format!(
            "/repos/{}/{}/commits/{}/status",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_commit_statuses_for_ref: GET /repos/{owner}/{repo}/commits/{ref}/statuses
     */
    pub async fn repos_list_commit_statuses_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Status>> {
        let url = format!(
            "/repos/{}/{}/commits/{}/statuses",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        self.get(&url).await
    }

    /**
     * codes_of_conduct_get_for_repo: GET /repos/{owner}/{repo}/community/code_of_conduct
     */
    pub async fn codes_of_conduct_get_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::CodeofConduct> {
        let url = format!(
            "/repos/{}/{}/community/code_of_conduct",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_community_profile_metrics: GET /repos/{owner}/{repo}/community/profile
     */
    pub async fn repos_get_community_profile_metrics(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::CommunityProfile> {
        let url = format!(
            "/repos/{}/{}/community/profile",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_compare_commits: GET /repos/{owner}/{repo}/compare/{basehead}
     */
    pub async fn repos_compare_commits(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
        per_page: i64,
        basehead: &str,
    ) -> Result<types::CommitComparison> {
        let url = format!(
            "/repos/{}/{}/compare/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&basehead.to_string()),
        );

        self.get(&url).await
    }

    /**
     * apps_create_content_attachment: POST /repos/{owner}/{repo}/content_references/{content_reference_id}/attachments
     */
    pub async fn apps_create_content_attachment(
        &self,
        owner: &str,
        repo: &str,
        content_reference_id: i64,
        body: &types::CreateContentAttachmentRequest,
    ) -> Result<types::ContentReferenceAttachment> {
        let url = format!(
            "/repos/{}/{}/content_references/{}/attachments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&content_reference_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_content: GET /repos/{owner}/{repo}/contents/{path}
     */
    pub async fn repos_get_content(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &&str,
    ) -> Result<types::GetRepositoryContentOkResponse> {
        let url = format!(
            "/repos/{}/{}/contents/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&path.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_or_update_file_contents: PUT /repos/{owner}/{repo}/contents/{path}
     */
    pub async fn repos_create_or_update_file_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        body: &types::CreateUpdateFileContentsRequest,
    ) -> Result<types::FileCommit> {
        let url = format!(
            "/repos/{}/{}/contents/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&path.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_delete_file: DELETE /repos/{owner}/{repo}/contents/{path}
     */
    pub async fn repos_delete_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        body: &types::DeleteFileRequest,
    ) -> Result<types::FileCommit> {
        let url = format!(
            "/repos/{}/{}/contents/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&path.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_list_contributors: GET /repos/{owner}/{repo}/contributors
     */
    pub async fn repos_list_contributors(
        &self,
        owner: &str,
        repo: &str,
        anon: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Contributor>> {
        let url = format!(
            "/repos/{}/{}/contributors",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_deployments: GET /repos/{owner}/{repo}/deployments
     */
    pub async fn repos_list_deployments(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        ref_: &&str,
        task: &str,
        environment: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Deployment>> {
        let url = format!(
            "/repos/{}/{}/deployments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_deployment: POST /repos/{owner}/{repo}/deployments
     */
    pub async fn repos_create_deployment(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateDeploymentRequest,
    ) -> Result<types::Deployment> {
        let url = format!(
            "/repos/{}/{}/deployments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_deployment: GET /repos/{owner}/{repo}/deployments/{deployment_id}
     */
    pub async fn repos_get_deployment(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
    ) -> Result<types::Deployment> {
        let url = format!(
            "/repos/{}/{}/deployments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&deployment_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete_deployment: DELETE /repos/{owner}/{repo}/deployments/{deployment_id}
     */
    pub async fn repos_delete_deployment(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/deployments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&deployment_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_list_deployment_statuses: GET /repos/{owner}/{repo}/deployments/{deployment_id}/statuses
     */
    pub async fn repos_list_deployment_statuses(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::DeploymentStatus>> {
        let url = format!(
            "/repos/{}/{}/deployments/{}/statuses",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&deployment_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_deployment_status: POST /repos/{owner}/{repo}/deployments/{deployment_id}/statuses
     */
    pub async fn repos_create_deployment_status(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
        body: &types::CreateDeploymentStatusRequest,
    ) -> Result<types::DeploymentStatus> {
        let url = format!(
            "/repos/{}/{}/deployments/{}/statuses",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&deployment_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_deployment_status: GET /repos/{owner}/{repo}/deployments/{deployment_id}/statuses/{status_id}
     */
    pub async fn repos_get_deployment_status(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
        status_id: i64,
    ) -> Result<types::DeploymentStatus> {
        let url = format!(
            "/repos/{}/{}/deployments/{}/statuses/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&deployment_id.to_string()),
            progenitor_support::encode_path(&status_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_dispatch_event: POST /repos/{owner}/{repo}/dispatches
     */
    pub async fn repos_create_dispatch_event(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateRepositoryDispatchEventRequest,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/dispatches",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_get_all_environments: GET /repos/{owner}/{repo}/environments
     */
    pub async fn repos_get_all_environments(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::GetAllEnvironmentsOkResponse> {
        let url = format!(
            "/repos/{}/{}/environments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_environment: GET /repos/{owner}/{repo}/environments/{environment_name}
     */
    pub async fn repos_get_environment(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
    ) -> Result<types::Environment> {
        let url = format!(
            "/repos/{}/{}/environments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&environment_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_or_update_environment: PUT /repos/{owner}/{repo}/environments/{environment_name}
     */
    pub async fn repos_create_or_update_environment(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        body: &types::CreateUpdateEnvironmentRequest,
    ) -> Result<types::Environment> {
        let url = format!(
            "/repos/{}/{}/environments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&environment_name.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_delete_an_environment: DELETE /repos/{owner}/{repo}/environments/{environment_name}
     */
    pub async fn repos_delete_an_environment(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/environments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&environment_name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * activity_list_repo_events: GET /repos/{owner}/{repo}/events
     */
    pub async fn activity_list_repo_events(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Event>> {
        let url = format!(
            "/repos/{}/{}/events",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_forks: GET /repos/{owner}/{repo}/forks
     */
    pub async fn repos_list_forks(
        &self,
        owner: &str,
        repo: &str,
        sort: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MinimalRepository>> {
        let url = format!(
            "/repos/{}/{}/forks",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_fork: POST /repos/{owner}/{repo}/forks
     */
    pub async fn repos_create_fork(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateForkRequest,
    ) -> Result<types::FullRepository> {
        let url = format!(
            "/repos/{}/{}/forks",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * git_create_blob: POST /repos/{owner}/{repo}/git/blobs
     */
    pub async fn git_create_blob(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateBlobRequest,
    ) -> Result<types::ShortBlob> {
        let url = format!(
            "/repos/{}/{}/git/blobs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * git_get_blob: GET /repos/{owner}/{repo}/git/blobs/{file_sha}
     */
    pub async fn git_get_blob(
        &self,
        owner: &str,
        repo: &str,
        file_sha: &str,
    ) -> Result<types::Blob> {
        let url = format!(
            "/repos/{}/{}/git/blobs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&file_sha.to_string()),
        );

        self.get(&url).await
    }

    /**
     * git_create_commit: POST /repos/{owner}/{repo}/git/commits
     */
    pub async fn git_create_commit(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateCommitRequest,
    ) -> Result<types::GitCommit> {
        let url = format!(
            "/repos/{}/{}/git/commits",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * git_get_commit: GET /repos/{owner}/{repo}/git/commits/{commit_sha}
     */
    pub async fn git_get_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> Result<types::GitCommit> {
        let url = format!(
            "/repos/{}/{}/git/commits/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&commit_sha.to_string()),
        );

        self.get(&url).await
    }

    /**
     * git_list_matching_refs: GET /repos/{owner}/{repo}/git/matching-refs/{ref}
     */
    pub async fn git_list_matching_refs(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::GitRef>> {
        let url = format!(
            "/repos/{}/{}/git/matching-refs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        self.get(&url).await
    }

    /**
     * git_get_ref: GET /repos/{owner}/{repo}/git/ref/{ref}
     */
    pub async fn git_get_ref(&self, owner: &str, repo: &str, ref_: &str) -> Result<types::GitRef> {
        let url = format!(
            "/repos/{}/{}/git/ref/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        self.get(&url).await
    }

    /**
     * git_create_ref: POST /repos/{owner}/{repo}/git/refs
     */
    pub async fn git_create_ref(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateReferenceRequest,
    ) -> Result<types::GitRef> {
        let url = format!(
            "/repos/{}/{}/git/refs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * git_delete_ref: DELETE /repos/{owner}/{repo}/git/refs/{ref}
     */
    pub async fn git_delete_ref(&self, owner: &str, repo: &str, ref_: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/git/refs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * git_update_ref: PATCH /repos/{owner}/{repo}/git/refs/{ref}
     */
    pub async fn git_update_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        body: &types::UpdateReferenceRequest,
    ) -> Result<types::GitRef> {
        let url = format!(
            "/repos/{}/{}/git/refs/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * git_create_tag: POST /repos/{owner}/{repo}/git/tags
     */
    pub async fn git_create_tag(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateTagObjectRequest,
    ) -> Result<types::GitTag> {
        let url = format!(
            "/repos/{}/{}/git/tags",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * git_get_tag: GET /repos/{owner}/{repo}/git/tags/{tag_sha}
     */
    pub async fn git_get_tag(
        &self,
        owner: &str,
        repo: &str,
        tag_sha: &str,
    ) -> Result<types::GitTag> {
        let url = format!(
            "/repos/{}/{}/git/tags/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&tag_sha.to_string()),
        );

        self.get(&url).await
    }

    /**
     * git_create_tree: POST /repos/{owner}/{repo}/git/trees
     */
    pub async fn git_create_tree(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateTreeRequest,
    ) -> Result<types::GitTree> {
        let url = format!(
            "/repos/{}/{}/git/trees",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * git_get_tree: GET /repos/{owner}/{repo}/git/trees/{tree_sha}
     */
    pub async fn git_get_tree(
        &self,
        owner: &str,
        repo: &str,
        tree_sha: &str,
        recursive: &str,
    ) -> Result<types::GitTree> {
        let url = format!(
            "/repos/{}/{}/git/trees/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&tree_sha.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_webhooks: GET /repos/{owner}/{repo}/hooks
     */
    pub async fn repos_list_webhooks(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Hook>> {
        let url = format!(
            "/repos/{}/{}/hooks",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_webhook: POST /repos/{owner}/{repo}/hooks
     */
    pub async fn repos_create_webhook(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateRepositoryWebhookRequest,
    ) -> Result<types::Hook> {
        let url = format!(
            "/repos/{}/{}/hooks",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_webhook: GET /repos/{owner}/{repo}/hooks/{hook_id}
     */
    pub async fn repos_get_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> Result<types::Hook> {
        let url = format!(
            "/repos/{}/{}/hooks/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete_webhook: DELETE /repos/{owner}/{repo}/hooks/{hook_id}
     */
    pub async fn repos_delete_webhook(&self, owner: &str, repo: &str, hook_id: i64) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/hooks/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_update_webhook: PATCH /repos/{owner}/{repo}/hooks/{hook_id}
     */
    pub async fn repos_update_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        body: &types::UpdateRepositoryWebhookRequest,
    ) -> Result<types::Hook> {
        let url = format!(
            "/repos/{}/{}/hooks/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_webhook_config_for_repo: GET /repos/{owner}/{repo}/hooks/{hook_id}/config
     */
    pub async fn repos_get_webhook_config_for_repo(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> Result<types::WebhookConfig> {
        let url = format!(
            "/repos/{}/{}/hooks/{}/config",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_update_webhook_config_for_repo: PATCH /repos/{owner}/{repo}/hooks/{hook_id}/config
     */
    pub async fn repos_update_webhook_config_for_repo(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        body: &types::UpdateWebhookConfigurationRepositoryRequest,
    ) -> Result<types::WebhookConfig> {
        let url = format!(
            "/repos/{}/{}/hooks/{}/config",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_ping_webhook: POST /repos/{owner}/{repo}/hooks/{hook_id}/pings
     */
    pub async fn repos_ping_webhook(&self, owner: &str, repo: &str, hook_id: i64) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/hooks/{}/pings",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_test_push_webhook: POST /repos/{owner}/{repo}/hooks/{hook_id}/tests
     */
    pub async fn repos_test_push_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/hooks/{}/tests",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&hook_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * migrations_get_import_status: GET /repos/{owner}/{repo}/import
     */
    pub async fn migrations_get_import_status(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::Import> {
        let url = format!(
            "/repos/{}/{}/import",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * migrations_start_import: PUT /repos/{owner}/{repo}/import
     */
    pub async fn migrations_start_import(
        &self,
        owner: &str,
        repo: &str,
        body: &types::StartImportRequest,
    ) -> Result<types::Import> {
        let url = format!(
            "/repos/{}/{}/import",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * migrations_cancel_import: DELETE /repos/{owner}/{repo}/import
     */
    pub async fn migrations_cancel_import(&self, owner: &str, repo: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/import",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * migrations_update_import: PATCH /repos/{owner}/{repo}/import
     */
    pub async fn migrations_update_import(
        &self,
        owner: &str,
        repo: &str,
        body: &types::UpdateImportRequest,
    ) -> Result<types::Import> {
        let url = format!(
            "/repos/{}/{}/import",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * migrations_get_commit_authors: GET /repos/{owner}/{repo}/import/authors
     */
    pub async fn migrations_get_commit_authors(
        &self,
        owner: &str,
        repo: &str,
        since: i64,
    ) -> Result<Vec<types::PorterAuthor>> {
        let url = format!(
            "/repos/{}/{}/import/authors",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * migrations_map_commit_author: PATCH /repos/{owner}/{repo}/import/authors/{author_id}
     */
    pub async fn migrations_map_commit_author(
        &self,
        owner: &str,
        repo: &str,
        author_id: i64,
        body: &types::MapCommitAuthorRequest,
    ) -> Result<types::PorterAuthor> {
        let url = format!(
            "/repos/{}/{}/import/authors/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&author_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * migrations_get_large_files: GET /repos/{owner}/{repo}/import/large_files
     */
    pub async fn migrations_get_large_files(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<types::PorterLargeFile>> {
        let url = format!(
            "/repos/{}/{}/import/large_files",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * migrations_set_lfs_preference: PATCH /repos/{owner}/{repo}/import/lfs
     */
    pub async fn migrations_set_lfs_preference(
        &self,
        owner: &str,
        repo: &str,
        body: &types::UpdateGitLfsPreferenceRequest,
    ) -> Result<types::Import> {
        let url = format!(
            "/repos/{}/{}/import/lfs",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * apps_get_repo_installation: GET /repos/{owner}/{repo}/installation
     */
    pub async fn apps_get_repo_installation(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::Installation> {
        let url = format!(
            "/repos/{}/{}/installation",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * interactions_get_restrictions_for_repo: GET /repos/{owner}/{repo}/interaction-limits
     */
    pub async fn interactions_get_restrictions_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::GetInteractionRestrictionsRepositoryOkResponse> {
        let url = format!(
            "/repos/{}/{}/interaction-limits",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * interactions_set_restrictions_for_repo: PUT /repos/{owner}/{repo}/interaction-limits
     */
    pub async fn interactions_set_restrictions_for_repo(
        &self,
        owner: &str,
        repo: &str,
        body: &types::InteractionLimit,
    ) -> Result<types::InteractionLimitResponse> {
        let url = format!(
            "/repos/{}/{}/interaction-limits",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * interactions_remove_restrictions_for_repo: DELETE /repos/{owner}/{repo}/interaction-limits
     */
    pub async fn interactions_remove_restrictions_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/interaction-limits",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_list_invitations: GET /repos/{owner}/{repo}/invitations
     */
    pub async fn repos_list_invitations(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::RepositoryInvitation>> {
        let url = format!(
            "/repos/{}/{}/invitations",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete_invitation: DELETE /repos/{owner}/{repo}/invitations/{invitation_id}
     */
    pub async fn repos_delete_invitation(
        &self,
        owner: &str,
        repo: &str,
        invitation_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/invitations/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&invitation_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_update_invitation: PATCH /repos/{owner}/{repo}/invitations/{invitation_id}
     */
    pub async fn repos_update_invitation(
        &self,
        owner: &str,
        repo: &str,
        invitation_id: i64,
        body: &types::UpdateRepositoryInvitationRequest,
    ) -> Result<types::RepositoryInvitation> {
        let url = format!(
            "/repos/{}/{}/invitations/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&invitation_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_list_for_repo: GET /repos/{owner}/{repo}/issues
     */
    pub async fn issues_list_for_repo(
        &self,
        owner: &str,
        repo: &str,
        milestone: &str,
        state: &str,
        assignee: &str,
        creator: &str,
        mentioned: &str,
        labels: &str,
        sort: &str,
        direction: &str,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::IssueSimple>> {
        let url = format!(
            "/repos/{}/{}/issues",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_create: POST /repos/{owner}/{repo}/issues
     */
    pub async fn issues_create(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateIssueRequest,
    ) -> Result<types::Issue> {
        let url = format!(
            "/repos/{}/{}/issues",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_list_comments_for_repo: GET /repos/{owner}/{repo}/issues/comments
     */
    pub async fn issues_list_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        sort: &str,
        direction: &str,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::IssueComment>> {
        let url = format!(
            "/repos/{}/{}/issues/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_get_comment: GET /repos/{owner}/{repo}/issues/comments/{comment_id}
     */
    pub async fn issues_get_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> Result<types::IssueComment> {
        let url = format!(
            "/repos/{}/{}/issues/comments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_delete_comment: DELETE /repos/{owner}/{repo}/issues/comments/{comment_id}
     */
    pub async fn issues_delete_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/comments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_update_comment: PATCH /repos/{owner}/{repo}/issues/comments/{comment_id}
     */
    pub async fn issues_update_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &types::UpdateIssueCommentRequest,
    ) -> Result<types::IssueComment> {
        let url = format!(
            "/repos/{}/{}/issues/comments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_list_for_issue_comment: GET /repos/{owner}/{repo}/issues/comments/{comment_id}/reactions
     */
    pub async fn reactions_list_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Reaction>> {
        let url = format!(
            "/repos/{}/{}/issues/comments/{}/reactions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * reactions_create_for_issue_comment: POST /repos/{owner}/{repo}/issues/comments/{comment_id}/reactions
     */
    pub async fn reactions_create_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &types::CreateReactionIssueCommentRequest,
    ) -> Result<types::Reaction> {
        let url = format!(
            "/repos/{}/{}/issues/comments/{}/reactions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_delete_for_issue_comment: DELETE /repos/{owner}/{repo}/issues/comments/{comment_id}/reactions/{reaction_id}
     */
    pub async fn reactions_delete_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/comments/{}/reactions/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
            progenitor_support::encode_path(&reaction_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_list_events_for_repo: GET /repos/{owner}/{repo}/issues/events
     */
    pub async fn issues_list_events_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::IssueEvent>> {
        let url = format!(
            "/repos/{}/{}/issues/events",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_get_event: GET /repos/{owner}/{repo}/issues/events/{event_id}
     */
    pub async fn issues_get_event(
        &self,
        owner: &str,
        repo: &str,
        event_id: i64,
    ) -> Result<types::IssueEvent> {
        let url = format!(
            "/repos/{}/{}/issues/events/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&event_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_get: GET /repos/{owner}/{repo}/issues/{issue_number}
     */
    pub async fn issues_get(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> Result<types::Issue> {
        let url = format!(
            "/repos/{}/{}/issues/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_update: PATCH /repos/{owner}/{repo}/issues/{issue_number}
     */
    pub async fn issues_update(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &types::UpdateIssueRequest,
    ) -> Result<types::Issue> {
        let url = format!(
            "/repos/{}/{}/issues/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_add_assignees: POST /repos/{owner}/{repo}/issues/{issue_number}/assignees
     */
    pub async fn issues_add_assignees(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &types::AddAssigneesIssueRequest,
    ) -> Result<types::IssueSimple> {
        let url = format!(
            "/repos/{}/{}/issues/{}/assignees",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_remove_assignees: DELETE /repos/{owner}/{repo}/issues/{issue_number}/assignees
     */
    pub async fn issues_remove_assignees(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &types::RemoveAssigneesFromIssueRequest,
    ) -> Result<types::IssueSimple> {
        let url = format!(
            "/repos/{}/{}/issues/{}/assignees",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_list_comments: GET /repos/{owner}/{repo}/issues/{issue_number}/comments
     */
    pub async fn issues_list_comments(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::IssueComment>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_create_comment: POST /repos/{owner}/{repo}/issues/{issue_number}/comments
     */
    pub async fn issues_create_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &types::CreateIssueCommentRequest,
    ) -> Result<types::IssueComment> {
        let url = format!(
            "/repos/{}/{}/issues/{}/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_list_events: GET /repos/{owner}/{repo}/issues/{issue_number}/events
     */
    pub async fn issues_list_events(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::IssueEventforIssue>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/events",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_list_labels_on_issue: GET /repos/{owner}/{repo}/issues/{issue_number}/labels
     */
    pub async fn issues_list_labels_on_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Label>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_set_labels: PUT /repos/{owner}/{repo}/issues/{issue_number}/labels
     */
    pub async fn issues_set_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &types::SetLabelsIssueRequest,
    ) -> Result<Vec<types::Label>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_add_labels: POST /repos/{owner}/{repo}/issues/{issue_number}/labels
     */
    pub async fn issues_add_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &types::AddLabelsIssueRequest,
    ) -> Result<Vec<types::Label>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_remove_all_labels: DELETE /repos/{owner}/{repo}/issues/{issue_number}/labels
     */
    pub async fn issues_remove_all_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_remove_label: DELETE /repos/{owner}/{repo}/issues/{issue_number}/labels/{name}
     */
    pub async fn issues_remove_label(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        name: &str,
    ) -> Result<Vec<types::Label>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/labels/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
            progenitor_support::encode_path(&name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_lock: PUT /repos/{owner}/{repo}/issues/{issue_number}/lock
     */
    pub async fn issues_lock(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &types::LockIssueRequest,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/{}/lock",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_unlock: DELETE /repos/{owner}/{repo}/issues/{issue_number}/lock
     */
    pub async fn issues_unlock(&self, owner: &str, repo: &str, issue_number: i64) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/{}/lock",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * reactions_list_for_issue: GET /repos/{owner}/{repo}/issues/{issue_number}/reactions
     */
    pub async fn reactions_list_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        content: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Reaction>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/reactions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * reactions_create_for_issue: POST /repos/{owner}/{repo}/issues/{issue_number}/reactions
     */
    pub async fn reactions_create_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &types::CreateReactionIssueRequest,
    ) -> Result<types::Reaction> {
        let url = format!(
            "/repos/{}/{}/issues/{}/reactions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_delete_for_issue: DELETE /repos/{owner}/{repo}/issues/{issue_number}/reactions/{reaction_id}
     */
    pub async fn reactions_delete_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        reaction_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/issues/{}/reactions/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
            progenitor_support::encode_path(&reaction_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_list_events_for_timeline: GET /repos/{owner}/{repo}/issues/{issue_number}/timeline
     */
    pub async fn issues_list_events_for_timeline(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::TimelineIssueEvents>> {
        let url = format!(
            "/repos/{}/{}/issues/{}/timeline",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&issue_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_deploy_keys: GET /repos/{owner}/{repo}/keys
     */
    pub async fn repos_list_deploy_keys(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::DeployKey>> {
        let url = format!(
            "/repos/{}/{}/keys",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_deploy_key: POST /repos/{owner}/{repo}/keys
     */
    pub async fn repos_create_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateDeployKeyRequest,
    ) -> Result<types::DeployKey> {
        let url = format!(
            "/repos/{}/{}/keys",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_deploy_key: GET /repos/{owner}/{repo}/keys/{key_id}
     */
    pub async fn repos_get_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        key_id: i64,
    ) -> Result<types::DeployKey> {
        let url = format!(
            "/repos/{}/{}/keys/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&key_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete_deploy_key: DELETE /repos/{owner}/{repo}/keys/{key_id}
     */
    pub async fn repos_delete_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        key_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/keys/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&key_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_list_labels_for_repo: GET /repos/{owner}/{repo}/labels
     */
    pub async fn issues_list_labels_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Label>> {
        let url = format!(
            "/repos/{}/{}/labels",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_create_label: POST /repos/{owner}/{repo}/labels
     */
    pub async fn issues_create_label(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateLabelRequest,
    ) -> Result<types::Label> {
        let url = format!(
            "/repos/{}/{}/labels",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_get_label: GET /repos/{owner}/{repo}/labels/{name}
     */
    pub async fn issues_get_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> Result<types::Label> {
        let url = format!(
            "/repos/{}/{}/labels/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_delete_label: DELETE /repos/{owner}/{repo}/labels/{name}
     */
    pub async fn issues_delete_label(&self, owner: &str, repo: &str, name: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/labels/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_update_label: PATCH /repos/{owner}/{repo}/labels/{name}
     */
    pub async fn issues_update_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        body: &types::UpdateLabelRequest,
    ) -> Result<types::Label> {
        let url = format!(
            "/repos/{}/{}/labels/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&name.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_list_languages: GET /repos/{owner}/{repo}/languages
     */
    pub async fn repos_list_languages(&self, owner: &str, repo: &str) -> Result<types::Language> {
        let url = format!(
            "/repos/{}/{}/languages",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * licenses_get_for_repo: GET /repos/{owner}/{repo}/license
     */
    pub async fn licenses_get_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::LicenseContent> {
        let url = format!(
            "/repos/{}/{}/license",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_merge: POST /repos/{owner}/{repo}/merges
     */
    pub async fn repos_merge(
        &self,
        owner: &str,
        repo: &str,
        body: &types::MergeBranchRequest,
    ) -> Result<types::Commit> {
        let url = format!(
            "/repos/{}/{}/merges",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_list_milestones: GET /repos/{owner}/{repo}/milestones
     */
    pub async fn issues_list_milestones(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
        sort: &str,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Milestone>> {
        let url = format!(
            "/repos/{}/{}/milestones",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_create_milestone: POST /repos/{owner}/{repo}/milestones
     */
    pub async fn issues_create_milestone(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateMilestoneRequest,
    ) -> Result<types::Milestone> {
        let url = format!(
            "/repos/{}/{}/milestones",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_get_milestone: GET /repos/{owner}/{repo}/milestones/{milestone_number}
     */
    pub async fn issues_get_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
    ) -> Result<types::Milestone> {
        let url = format!(
            "/repos/{}/{}/milestones/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&milestone_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * issues_delete_milestone: DELETE /repos/{owner}/{repo}/milestones/{milestone_number}
     */
    pub async fn issues_delete_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/milestones/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&milestone_number.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_update_milestone: PATCH /repos/{owner}/{repo}/milestones/{milestone_number}
     */
    pub async fn issues_update_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
        body: &types::UpdateMilestoneRequest,
    ) -> Result<types::Milestone> {
        let url = format!(
            "/repos/{}/{}/milestones/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&milestone_number.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * issues_list_labels_for_milestone: GET /repos/{owner}/{repo}/milestones/{milestone_number}/labels
     */
    pub async fn issues_list_labels_for_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Label>> {
        let url = format!(
            "/repos/{}/{}/milestones/{}/labels",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&milestone_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_repo_notifications_for_authenticated_user: GET /repos/{owner}/{repo}/notifications
     */
    pub async fn activity_list_repo_notifications_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
        all: bool,
        participating: bool,
        since: DateTime<Utc>,
        before: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Thread>> {
        let url = format!(
            "/repos/{}/{}/notifications",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_mark_repo_notifications_as_read: PUT /repos/{owner}/{repo}/notifications
     */
    pub async fn activity_mark_repo_notifications_as_read(
        &self,
        owner: &str,
        repo: &str,
        body: &types::MarkRepositoryNotificationsasReadRequest,
    ) -> Result<types::PutMarkRepositoryNotificationsasReadAcceptedResponse> {
        let url = format!(
            "/repos/{}/{}/notifications",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_pages: GET /repos/{owner}/{repo}/pages
     */
    pub async fn repos_get_pages(&self, owner: &str, repo: &str) -> Result<types::Page> {
        let url = format!(
            "/repos/{}/{}/pages",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_update_information_about_pages_site: PUT /repos/{owner}/{repo}/pages
     */
    pub async fn repos_update_information_about_pages_site(
        &self,
        owner: &str,
        repo: &str,
        body: &types::UpdateInformationAboutGithubPagesSiteRequest,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/pages",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_create_pages_site: POST /repos/{owner}/{repo}/pages
     */
    pub async fn repos_create_pages_site(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateGithubPagesSiteRequest,
    ) -> Result<types::Page> {
        let url = format!(
            "/repos/{}/{}/pages",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_delete_pages_site: DELETE /repos/{owner}/{repo}/pages
     */
    pub async fn repos_delete_pages_site(&self, owner: &str, repo: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/pages",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_list_pages_builds: GET /repos/{owner}/{repo}/pages/builds
     */
    pub async fn repos_list_pages_builds(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::PageBuild>> {
        let url = format!(
            "/repos/{}/{}/pages/builds",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_request_pages_build: POST /repos/{owner}/{repo}/pages/builds
     */
    pub async fn repos_request_pages_build(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::PageBuildStatus> {
        let url = format!(
            "/repos/{}/{}/pages/builds",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_latest_pages_build: GET /repos/{owner}/{repo}/pages/builds/latest
     */
    pub async fn repos_get_latest_pages_build(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::PageBuild> {
        let url = format!(
            "/repos/{}/{}/pages/builds/latest",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_pages_build: GET /repos/{owner}/{repo}/pages/builds/{build_id}
     */
    pub async fn repos_get_pages_build(
        &self,
        owner: &str,
        repo: &str,
        build_id: i64,
    ) -> Result<types::PageBuild> {
        let url = format!(
            "/repos/{}/{}/pages/builds/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&build_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_pages_health_check: GET /repos/{owner}/{repo}/pages/health
     */
    pub async fn repos_get_pages_health_check(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::PagesHealthCheck> {
        let url = format!(
            "/repos/{}/{}/pages/health",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_list_for_repo: GET /repos/{owner}/{repo}/projects
     */
    pub async fn projects_list_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Project>> {
        let url = format!(
            "/repos/{}/{}/projects",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_create_for_repo: POST /repos/{owner}/{repo}/projects
     */
    pub async fn projects_create_for_repo(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateRepositoryProjectRequest,
    ) -> Result<types::Project> {
        let url = format!(
            "/repos/{}/{}/projects",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_list: GET /repos/{owner}/{repo}/pulls
     */
    pub async fn pulls_list(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
        head: &str,
        base: &str,
        sort: &str,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::PullRequestSimple>> {
        let url = format!(
            "/repos/{}/{}/pulls",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_create: POST /repos/{owner}/{repo}/pulls
     */
    pub async fn pulls_create(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreatePullRequestRequest,
    ) -> Result<types::PullRequest> {
        let url = format!(
            "/repos/{}/{}/pulls",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_list_review_comments_for_repo: GET /repos/{owner}/{repo}/pulls/comments
     */
    pub async fn pulls_list_review_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        sort: &str,
        direction: &str,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::PullRequestReviewComment>> {
        let url = format!(
            "/repos/{}/{}/pulls/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_get_review_comment: GET /repos/{owner}/{repo}/pulls/comments/{comment_id}
     */
    pub async fn pulls_get_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> Result<types::PullRequestReviewComment> {
        let url = format!(
            "/repos/{}/{}/pulls/comments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_delete_review_comment: DELETE /repos/{owner}/{repo}/pulls/comments/{comment_id}
     */
    pub async fn pulls_delete_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/pulls/comments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * pulls_update_review_comment: PATCH /repos/{owner}/{repo}/pulls/comments/{comment_id}
     */
    pub async fn pulls_update_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &types::UpdateReviewCommentPullRequestRequest,
    ) -> Result<types::PullRequestReviewComment> {
        let url = format!(
            "/repos/{}/{}/pulls/comments/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_list_for_pull_request_review_comment: GET /repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions
     */
    pub async fn reactions_list_for_pull_request_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Reaction>> {
        let url = format!(
            "/repos/{}/{}/pulls/comments/{}/reactions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * reactions_create_for_pull_request_review_comment: POST /repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions
     */
    pub async fn reactions_create_for_pull_request_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &types::CreateReactionPullRequestReviewCommentRequest,
    ) -> Result<types::Reaction> {
        let url = format!(
            "/repos/{}/{}/pulls/comments/{}/reactions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_delete_for_pull_request_comment: DELETE /repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions/{reaction_id}
     */
    pub async fn reactions_delete_for_pull_request_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/pulls/comments/{}/reactions/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
            progenitor_support::encode_path(&reaction_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * pulls_get: GET /repos/{owner}/{repo}/pulls/{pull_number}
     */
    pub async fn pulls_get(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
    ) -> Result<types::PullRequest> {
        let url = format!(
            "/repos/{}/{}/pulls/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_update: PATCH /repos/{owner}/{repo}/pulls/{pull_number}
     */
    pub async fn pulls_update(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &types::UpdatePullRequestRequest,
    ) -> Result<types::PullRequest> {
        let url = format!(
            "/repos/{}/{}/pulls/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_list_review_comments: GET /repos/{owner}/{repo}/pulls/{pull_number}/comments
     */
    pub async fn pulls_list_review_comments(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        sort: &str,
        direction: &str,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::PullRequestReviewComment>> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_create_review_comment: POST /repos/{owner}/{repo}/pulls/{pull_number}/comments
     */
    pub async fn pulls_create_review_comment(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &types::CreateReviewCommentPullRequestRequest,
    ) -> Result<types::PullRequestReviewComment> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_create_reply_for_review_comment: POST /repos/{owner}/{repo}/pulls/{pull_number}/comments/{comment_id}/replies
     */
    pub async fn pulls_create_reply_for_review_comment(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        comment_id: i64,
        body: &types::CreateReplyReviewCommentRequest,
    ) -> Result<types::PullRequestReviewComment> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/comments/{}/replies",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
            progenitor_support::encode_path(&comment_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_list_commits: GET /repos/{owner}/{repo}/pulls/{pull_number}/commits
     */
    pub async fn pulls_list_commits(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Commit>> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/commits",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_list_files: GET /repos/{owner}/{repo}/pulls/{pull_number}/files
     */
    pub async fn pulls_list_files(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::DiffEntry>> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/files",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_check_if_merged: GET /repos/{owner}/{repo}/pulls/{pull_number}/merge
     */
    pub async fn pulls_check_if_merged(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/merge",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_merge: PUT /repos/{owner}/{repo}/pulls/{pull_number}/merge
     */
    pub async fn pulls_merge(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &types::MergePullRequestRequest,
    ) -> Result<types::PullRequestMergeResult> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/merge",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_list_requested_reviewers: GET /repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers
     */
    pub async fn pulls_list_requested_reviewers(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<types::PullRequestReviewRequest> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/requested_reviewers",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_request_reviewers: POST /repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers
     */
    pub async fn pulls_request_reviewers(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &types::RequestReviewersPullRequestRequest,
    ) -> Result<types::PullRequestSimple> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/requested_reviewers",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_remove_requested_reviewers: DELETE /repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers
     */
    pub async fn pulls_remove_requested_reviewers(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &types::RemoveRequestedReviewersFromPullRequestRequest,
    ) -> Result<types::PullRequestSimple> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/requested_reviewers",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_list_reviews: GET /repos/{owner}/{repo}/pulls/{pull_number}/reviews
     */
    pub async fn pulls_list_reviews(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::PullRequestReview>> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/reviews",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_create_review: POST /repos/{owner}/{repo}/pulls/{pull_number}/reviews
     */
    pub async fn pulls_create_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &types::CreateReviewPullRequestRequest,
    ) -> Result<types::PullRequestReview> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/reviews",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_get_review: GET /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}
     */
    pub async fn pulls_get_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
    ) -> Result<types::PullRequestReview> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
            progenitor_support::encode_path(&review_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_update_review: PUT /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}
     */
    pub async fn pulls_update_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
        body: &types::UpdateReviewPullRequestRequest,
    ) -> Result<types::PullRequestReview> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
            progenitor_support::encode_path(&review_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_delete_pending_review: DELETE /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}
     */
    pub async fn pulls_delete_pending_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
    ) -> Result<types::PullRequestReview> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
            progenitor_support::encode_path(&review_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_list_comments_for_review: GET /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/comments
     */
    pub async fn pulls_list_comments_for_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::ReviewComment>> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}/comments",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
            progenitor_support::encode_path(&review_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * pulls_dismiss_review: PUT /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/dismissals
     */
    pub async fn pulls_dismiss_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
        body: &types::DismissReviewPullRequestRequest,
    ) -> Result<types::PullRequestReview> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}/dismissals",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
            progenitor_support::encode_path(&review_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_submit_review: POST /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/events
     */
    pub async fn pulls_submit_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
        body: &types::SubmitReviewPullRequestRequest,
    ) -> Result<types::PullRequestReview> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}/events",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
            progenitor_support::encode_path(&review_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * pulls_update_branch: PUT /repos/{owner}/{repo}/pulls/{pull_number}/update-branch
     */
    pub async fn pulls_update_branch(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &types::UpdatePullRequestBranchRequest,
    ) -> Result<types::PutUpdatePullRequestBranchAcceptedResponse> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/update-branch",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&pull_number.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_readme: GET /repos/{owner}/{repo}/readme
     */
    pub async fn repos_get_readme(
        &self,
        owner: &str,
        repo: &str,
        ref_: &&str,
    ) -> Result<types::ContentFile> {
        let url = format!(
            "/repos/{}/{}/readme",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_readme_in_directory: GET /repos/{owner}/{repo}/readme/{dir}
     */
    pub async fn repos_get_readme_in_directory(
        &self,
        owner: &str,
        repo: &str,
        dir: &str,
        ref_: &&str,
    ) -> Result<types::ContentFile> {
        let url = format!(
            "/repos/{}/{}/readme/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&dir.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_releases: GET /repos/{owner}/{repo}/releases
     */
    pub async fn repos_list_releases(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Release>> {
        let url = format!(
            "/repos/{}/{}/releases",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_release: POST /repos/{owner}/{repo}/releases
     */
    pub async fn repos_create_release(
        &self,
        owner: &str,
        repo: &str,
        body: &types::CreateReleaseRequest,
    ) -> Result<types::Release> {
        let url = format!(
            "/repos/{}/{}/releases",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_release_asset: GET /repos/{owner}/{repo}/releases/assets/{asset_id}
     */
    pub async fn repos_get_release_asset(
        &self,
        owner: &str,
        repo: &str,
        asset_id: i64,
    ) -> Result<types::ReleaseAsset> {
        let url = format!(
            "/repos/{}/{}/releases/assets/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&asset_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete_release_asset: DELETE /repos/{owner}/{repo}/releases/assets/{asset_id}
     */
    pub async fn repos_delete_release_asset(
        &self,
        owner: &str,
        repo: &str,
        asset_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/releases/assets/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&asset_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_update_release_asset: PATCH /repos/{owner}/{repo}/releases/assets/{asset_id}
     */
    pub async fn repos_update_release_asset(
        &self,
        owner: &str,
        repo: &str,
        asset_id: i64,
        body: &types::UpdateReleaseAssetRequest,
    ) -> Result<types::ReleaseAsset> {
        let url = format!(
            "/repos/{}/{}/releases/assets/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&asset_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_latest_release: GET /repos/{owner}/{repo}/releases/latest
     */
    pub async fn repos_get_latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::Release> {
        let url = format!(
            "/repos/{}/{}/releases/latest",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_release_by_tag: GET /repos/{owner}/{repo}/releases/tags/{tag}
     */
    pub async fn repos_get_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<types::Release> {
        let url = format!(
            "/repos/{}/{}/releases/tags/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&tag.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_release: GET /repos/{owner}/{repo}/releases/{release_id}
     */
    pub async fn repos_get_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
    ) -> Result<types::Release> {
        let url = format!(
            "/repos/{}/{}/releases/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&release_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_delete_release: DELETE /repos/{owner}/{repo}/releases/{release_id}
     */
    pub async fn repos_delete_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/releases/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&release_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_update_release: PATCH /repos/{owner}/{repo}/releases/{release_id}
     */
    pub async fn repos_update_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        body: &types::UpdateReleaseRequest,
    ) -> Result<types::Release> {
        let url = format!(
            "/repos/{}/{}/releases/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&release_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_list_release_assets: GET /repos/{owner}/{repo}/releases/{release_id}/assets
     */
    pub async fn repos_list_release_assets(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::ReleaseAsset>> {
        let url = format!(
            "/repos/{}/{}/releases/{}/assets",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&release_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_upload_release_asset: POST /repos/{owner}/{repo}/releases/{release_id}/assets
     */
    pub async fn repos_upload_release_asset<T: Into<reqwest::Body>>(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        name: &str,
        label: &str,
        body: T,
    ) -> Result<types::ReleaseAsset> {
        let url = format!(
            "/repos/{}/{}/releases/{}/assets",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&release_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .query(&[("name", name.to_string()), ("label", label.to_string())])
            .body(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_create_for_release: POST /repos/{owner}/{repo}/releases/{release_id}/reactions
     */
    pub async fn reactions_create_for_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        body: &types::CreateReactionReleaseRequest,
    ) -> Result<types::Reaction> {
        let url = format!(
            "/repos/{}/{}/releases/{}/reactions",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&release_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * secret_scanning_list_alerts_for_repo: GET /repos/{owner}/{repo}/secret-scanning/alerts
     */
    pub async fn secret_scanning_list_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
        secret_type: &str,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<types::SecretScanningAlert>> {
        let url = format!(
            "/repos/{}/{}/secret-scanning/alerts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * secret_scanning_get_alert: GET /repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}
     */
    pub async fn secret_scanning_get_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: &str,
    ) -> Result<types::SecretScanningAlert> {
        let url = format!(
            "/repos/{}/{}/secret-scanning/alerts/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&alert_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * secret_scanning_update_alert: PATCH /repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}
     */
    pub async fn secret_scanning_update_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: &str,
        body: &types::UpdateSecretScanningAlertRequest,
    ) -> Result<types::SecretScanningAlert> {
        let url = format!(
            "/repos/{}/{}/secret-scanning/alerts/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&alert_number.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * activity_list_stargazers_for_repo: GET /repos/{owner}/{repo}/stargazers
     */
    pub async fn activity_list_stargazers_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/repos/{}/{}/stargazers",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_code_frequency_stats: GET /repos/{owner}/{repo}/stats/code_frequency
     */
    pub async fn repos_get_code_frequency_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<Vec<i64>>> {
        let url = format!(
            "/repos/{}/{}/stats/code_frequency",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_commit_activity_stats: GET /repos/{owner}/{repo}/stats/commit_activity
     */
    pub async fn repos_get_commit_activity_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<types::CommitActivity>> {
        let url = format!(
            "/repos/{}/{}/stats/commit_activity",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_contributors_stats: GET /repos/{owner}/{repo}/stats/contributors
     */
    pub async fn repos_get_contributors_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<types::ContributorActivity>> {
        let url = format!(
            "/repos/{}/{}/stats/contributors",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_participation_stats: GET /repos/{owner}/{repo}/stats/participation
     */
    pub async fn repos_get_participation_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::ParticipationStats> {
        let url = format!(
            "/repos/{}/{}/stats/participation",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_punch_card_stats: GET /repos/{owner}/{repo}/stats/punch_card
     */
    pub async fn repos_get_punch_card_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<Vec<i64>>> {
        let url = format!(
            "/repos/{}/{}/stats/punch_card",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_commit_status: POST /repos/{owner}/{repo}/statuses/{sha}
     */
    pub async fn repos_create_commit_status(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        body: &types::CreateCommitStatusRequest,
    ) -> Result<types::Status> {
        let url = format!(
            "/repos/{}/{}/statuses/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&sha.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * activity_list_watchers_for_repo: GET /repos/{owner}/{repo}/subscribers
     */
    pub async fn activity_list_watchers_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/repos/{}/{}/subscribers",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_get_repo_subscription: GET /repos/{owner}/{repo}/subscription
     */
    pub async fn activity_get_repo_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<types::RepositorySubscription> {
        let url = format!(
            "/repos/{}/{}/subscription",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_set_repo_subscription: PUT /repos/{owner}/{repo}/subscription
     */
    pub async fn activity_set_repo_subscription(
        &self,
        owner: &str,
        repo: &str,
        body: &types::SetRepositorySubscriptionRequest,
    ) -> Result<types::RepositorySubscription> {
        let url = format!(
            "/repos/{}/{}/subscription",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * activity_delete_repo_subscription: DELETE /repos/{owner}/{repo}/subscription
     */
    pub async fn activity_delete_repo_subscription(&self, owner: &str, repo: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/subscription",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_list_tags: GET /repos/{owner}/{repo}/tags
     */
    pub async fn repos_list_tags(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Tag>> {
        let url = format!(
            "/repos/{}/{}/tags",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_download_tarball_archive: GET /repos/{owner}/{repo}/tarball/{ref}
     */
    pub async fn repos_download_tarball_archive(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/tarball/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_teams: GET /repos/{owner}/{repo}/teams
     */
    pub async fn repos_list_teams(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Team>> {
        let url = format!(
            "/repos/{}/{}/teams",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_all_topics: GET /repos/{owner}/{repo}/topics
     */
    pub async fn repos_get_all_topics(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
        per_page: i64,
    ) -> Result<types::Topic> {
        let url = format!(
            "/repos/{}/{}/topics",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_replace_all_topics: PUT /repos/{owner}/{repo}/topics
     */
    pub async fn repos_replace_all_topics(
        &self,
        owner: &str,
        repo: &str,
        body: &types::ReplaceAllRepositoryTopicsRequest,
    ) -> Result<types::Topic> {
        let url = format!(
            "/repos/{}/{}/topics",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_get_clones: GET /repos/{owner}/{repo}/traffic/clones
     */
    pub async fn repos_get_clones(
        &self,
        owner: &str,
        repo: &str,
        per: &str,
    ) -> Result<types::CloneTraffic> {
        let url = format!(
            "/repos/{}/{}/traffic/clones",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_top_paths: GET /repos/{owner}/{repo}/traffic/popular/paths
     */
    pub async fn repos_get_top_paths(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<types::ContentTraffic>> {
        let url = format!(
            "/repos/{}/{}/traffic/popular/paths",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_top_referrers: GET /repos/{owner}/{repo}/traffic/popular/referrers
     */
    pub async fn repos_get_top_referrers(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<types::ReferrerTraffic>> {
        let url = format!(
            "/repos/{}/{}/traffic/popular/referrers",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_get_views: GET /repos/{owner}/{repo}/traffic/views
     */
    pub async fn repos_get_views(
        &self,
        owner: &str,
        repo: &str,
        per: &str,
    ) -> Result<types::ViewTraffic> {
        let url = format!(
            "/repos/{}/{}/traffic/views",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_transfer: POST /repos/{owner}/{repo}/transfer
     */
    pub async fn repos_transfer(
        &self,
        owner: &str,
        repo: &str,
        body: &types::TransferRepositoryRequest,
    ) -> Result<types::MinimalRepository> {
        let url = format!(
            "/repos/{}/{}/transfer",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_check_vulnerability_alerts: GET /repos/{owner}/{repo}/vulnerability-alerts
     */
    pub async fn repos_check_vulnerability_alerts(&self, owner: &str, repo: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/vulnerability-alerts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_enable_vulnerability_alerts: PUT /repos/{owner}/{repo}/vulnerability-alerts
     */
    pub async fn repos_enable_vulnerability_alerts(&self, owner: &str, repo: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/vulnerability-alerts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_disable_vulnerability_alerts: DELETE /repos/{owner}/{repo}/vulnerability-alerts
     */
    pub async fn repos_disable_vulnerability_alerts(&self, owner: &str, repo: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/vulnerability-alerts",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_download_zipball_archive: GET /repos/{owner}/{repo}/zipball/{ref}
     */
    pub async fn repos_download_zipball_archive(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/zipball/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
            progenitor_support::encode_path(&ref_.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_create_using_template: POST /repos/{template_owner}/{template_repo}/generate
     */
    pub async fn repos_create_using_template(
        &self,
        template_owner: &str,
        template_repo: &str,
        body: &types::CreateRepositoryUsingTemplateRequest,
    ) -> Result<types::Repository> {
        let url = format!(
            "/repos/{}/{}/generate",
            progenitor_support::encode_path(&template_owner.to_string()),
            progenitor_support::encode_path(&template_repo.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_list_public: GET /repositories
     */
    pub async fn repos_list_public(&self, since: i64) -> Result<Vec<types::MinimalRepository>> {
        let url = "/repositories".to_string();
        self.get(&url).await
    }

    /**
     * actions_list_environment_secrets: GET /repositories/{repository_id}/environments/{environment_name}/secrets
     */
    pub async fn actions_list_environment_secrets(
        &self,
        repository_id: i64,
        environment_name: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListEnvironmentSecretsOkResponse> {
        let url = format!(
            "/repositories/{}/environments/{}/secrets",
            progenitor_support::encode_path(&repository_id.to_string()),
            progenitor_support::encode_path(&environment_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_environment_public_key: GET /repositories/{repository_id}/environments/{environment_name}/secrets/public-key
     */
    pub async fn actions_get_environment_public_key(
        &self,
        repository_id: i64,
        environment_name: &str,
    ) -> Result<types::ActionsPublicKey> {
        let url = format!(
            "/repositories/{}/environments/{}/secrets/public-key",
            progenitor_support::encode_path(&repository_id.to_string()),
            progenitor_support::encode_path(&environment_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_get_environment_secret: GET /repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}
     */
    pub async fn actions_get_environment_secret(
        &self,
        repository_id: i64,
        environment_name: &str,
        secret_name: &str,
    ) -> Result<types::ActionsSecret> {
        let url = format!(
            "/repositories/{}/environments/{}/secrets/{}",
            progenitor_support::encode_path(&repository_id.to_string()),
            progenitor_support::encode_path(&environment_name.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * actions_create_or_update_environment_secret: PUT /repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}
     */
    pub async fn actions_create_or_update_environment_secret(
        &self,
        repository_id: i64,
        environment_name: &str,
        secret_name: &str,
        body: &types::CreateUpdateEnvironmentSecretRequest,
    ) -> Result<types::EmptyObject> {
        let url = format!(
            "/repositories/{}/environments/{}/secrets/{}",
            progenitor_support::encode_path(&repository_id.to_string()),
            progenitor_support::encode_path(&environment_name.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * actions_delete_environment_secret: DELETE /repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}
     */
    pub async fn actions_delete_environment_secret(
        &self,
        repository_id: i64,
        environment_name: &str,
        secret_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/repositories/{}/environments/{}/secrets/{}",
            progenitor_support::encode_path(&repository_id.to_string()),
            progenitor_support::encode_path(&environment_name.to_string()),
            progenitor_support::encode_path(&secret_name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_list_provisioned_groups_enterprise: GET /scim/v2/enterprises/{enterprise}/Groups
     */
    pub async fn enterprise_admin_list_provisioned_groups_enterprise(
        &self,
        enterprise: &str,
        start_index: i64,
        count: i64,
        filter: &str,
        excluded_attributes: &str,
    ) -> Result<types::ScimGroupListEnterprise> {
        let url = format!(
            "/scim/v2/enterprises/{}/Groups",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_provision_and_invite_enterprise_group: POST /scim/v2/enterprises/{enterprise}/Groups
     */
    pub async fn enterprise_admin_provision_and_invite_enterprise_group(
        &self,
        enterprise: &str,
        body: &types::ProvisionScimEnterpriseGroupandInviteUsersRequest,
    ) -> Result<types::ScimEnterpriseGroup> {
        let url = format!(
            "/scim/v2/enterprises/{}/Groups",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * enterprise_admin_get_provisioning_information_for_enterprise_group: GET /scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}
     */
    pub async fn enterprise_admin_get_provisioning_information_for_enterprise_group(
        &self,
        enterprise: &str,
        scim_group_id: &str,
        excluded_attributes: &str,
    ) -> Result<types::ScimEnterpriseGroup> {
        let url = format!(
            "/scim/v2/enterprises/{}/Groups/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&scim_group_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_set_information_for_provisioned_enterprise_group: PUT /scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}
     */
    pub async fn enterprise_admin_set_information_for_provisioned_enterprise_group(
        &self,
        enterprise: &str,
        scim_group_id: &str,
        body: &types::SetScimInformationProvisionedEnterpriseGroupRequest,
    ) -> Result<types::ScimEnterpriseGroup> {
        let url = format!(
            "/scim/v2/enterprises/{}/Groups/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&scim_group_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * enterprise_admin_delete_scim_group_from_enterprise: DELETE /scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}
     */
    pub async fn enterprise_admin_delete_scim_group_from_enterprise(
        &self,
        enterprise: &str,
        scim_group_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/scim/v2/enterprises/{}/Groups/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&scim_group_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_update_attribute_for_enterprise_group: PATCH /scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}
     */
    pub async fn enterprise_admin_update_attribute_for_enterprise_group(
        &self,
        enterprise: &str,
        scim_group_id: &str,
        body: &types::UpdateAttributeScimEnterpriseGroupRequest,
    ) -> Result<types::ScimEnterpriseGroup> {
        let url = format!(
            "/scim/v2/enterprises/{}/Groups/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&scim_group_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * enterprise_admin_list_provisioned_identities_enterprise: GET /scim/v2/enterprises/{enterprise}/Users
     */
    pub async fn enterprise_admin_list_provisioned_identities_enterprise(
        &self,
        enterprise: &str,
        start_index: i64,
        count: i64,
        filter: &str,
    ) -> Result<types::ScimUserListEnterprise> {
        let url = format!(
            "/scim/v2/enterprises/{}/Users",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_provision_and_invite_enterprise_user: POST /scim/v2/enterprises/{enterprise}/Users
     */
    pub async fn enterprise_admin_provision_and_invite_enterprise_user(
        &self,
        enterprise: &str,
        body: &types::ProvisionandInviteScimEnterpriseUserRequest,
    ) -> Result<types::ScimEnterpriseUser> {
        let url = format!(
            "/scim/v2/enterprises/{}/Users",
            progenitor_support::encode_path(&enterprise.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * enterprise_admin_get_provisioning_information_for_enterprise_user: GET /scim/v2/enterprises/{enterprise}/Users/{scim_user_id}
     */
    pub async fn enterprise_admin_get_provisioning_information_for_enterprise_user(
        &self,
        enterprise: &str,
        scim_user_id: &str,
    ) -> Result<types::ScimEnterpriseUser> {
        let url = format!(
            "/scim/v2/enterprises/{}/Users/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&scim_user_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * enterprise_admin_set_information_for_provisioned_enterprise_user: PUT /scim/v2/enterprises/{enterprise}/Users/{scim_user_id}
     */
    pub async fn enterprise_admin_set_information_for_provisioned_enterprise_user(
        &self,
        enterprise: &str,
        scim_user_id: &str,
        body: &types::SetScimInformationProvisionedEnterpriseUserRequest,
    ) -> Result<types::ScimEnterpriseUser> {
        let url = format!(
            "/scim/v2/enterprises/{}/Users/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&scim_user_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * enterprise_admin_delete_user_from_enterprise: DELETE /scim/v2/enterprises/{enterprise}/Users/{scim_user_id}
     */
    pub async fn enterprise_admin_delete_user_from_enterprise(
        &self,
        enterprise: &str,
        scim_user_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/scim/v2/enterprises/{}/Users/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&scim_user_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * enterprise_admin_update_attribute_for_enterprise_user: PATCH /scim/v2/enterprises/{enterprise}/Users/{scim_user_id}
     */
    pub async fn enterprise_admin_update_attribute_for_enterprise_user(
        &self,
        enterprise: &str,
        scim_user_id: &str,
        body: &types::UpdateAttributeScimEnterpriseUserRequest,
    ) -> Result<types::ScimEnterpriseUser> {
        let url = format!(
            "/scim/v2/enterprises/{}/Users/{}",
            progenitor_support::encode_path(&enterprise.to_string()),
            progenitor_support::encode_path(&scim_user_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * scim_list_provisioned_identities: GET /scim/v2/organizations/{org}/Users
     */
    pub async fn scim_list_provisioned_identities(
        &self,
        org: &str,
        start_index: i64,
        count: i64,
        filter: &str,
    ) -> Result<types::ScimUserList> {
        let url = format!(
            "/scim/v2/organizations/{}/Users",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * scim_provision_and_invite_user: POST /scim/v2/organizations/{org}/Users
     */
    pub async fn scim_provision_and_invite_user(
        &self,
        org: &str,
        body: &types::ProvisionandInviteScimUserRequest,
    ) -> Result<types::ScimUser> {
        let url = format!(
            "/scim/v2/organizations/{}/Users",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * scim_get_provisioning_information_for_user: GET /scim/v2/organizations/{org}/Users/{scim_user_id}
     */
    pub async fn scim_get_provisioning_information_for_user(
        &self,
        org: &str,
        scim_user_id: &str,
    ) -> Result<types::ScimUser> {
        let url = format!(
            "/scim/v2/organizations/{}/Users/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&scim_user_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * scim_set_information_for_provisioned_user: PUT /scim/v2/organizations/{org}/Users/{scim_user_id}
     */
    pub async fn scim_set_information_for_provisioned_user(
        &self,
        org: &str,
        scim_user_id: &str,
        body: &types::UpdateProvisionedOrganizationMembershipRequest,
    ) -> Result<types::ScimUser> {
        let url = format!(
            "/scim/v2/organizations/{}/Users/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&scim_user_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * scim_delete_user_from_org: DELETE /scim/v2/organizations/{org}/Users/{scim_user_id}
     */
    pub async fn scim_delete_user_from_org(&self, org: &str, scim_user_id: &str) -> Result<()> {
        let url = format!(
            "/scim/v2/organizations/{}/Users/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&scim_user_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * scim_update_attribute_for_user: PATCH /scim/v2/organizations/{org}/Users/{scim_user_id}
     */
    pub async fn scim_update_attribute_for_user(
        &self,
        org: &str,
        scim_user_id: &str,
        body: &types::UpdateAttributeScimUserRequest,
    ) -> Result<types::ScimUser> {
        let url = format!(
            "/scim/v2/organizations/{}/Users/{}",
            progenitor_support::encode_path(&org.to_string()),
            progenitor_support::encode_path(&scim_user_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * search_code: GET /search/code
     */
    pub async fn search_code(
        &self,
        q: &str,
        sort: &str,
        order: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetSearchCodeOkResponse> {
        let url = "/search/code".to_string();
        self.get(&url).await
    }

    /**
     * search_commits: GET /search/commits
     */
    pub async fn search_commits(
        &self,
        q: &str,
        sort: &str,
        order: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetSearchCommitsOkResponse> {
        let url = "/search/commits".to_string();
        self.get(&url).await
    }

    /**
     * search_issues_and_pull_requests: GET /search/issues
     */
    pub async fn search_issues_and_pull_requests(
        &self,
        q: &str,
        sort: &str,
        order: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetSearchIssuesandPullRequestsOkResponse> {
        let url = "/search/issues".to_string();
        self.get(&url).await
    }

    /**
     * search_labels: GET /search/labels
     */
    pub async fn search_labels(
        &self,
        repository_id: i64,
        q: &str,
        sort: &str,
        order: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetSearchLabelsOkResponse> {
        let url = "/search/labels".to_string();
        self.get(&url).await
    }

    /**
     * search_repos: GET /search/repositories
     */
    pub async fn search_repos(
        &self,
        q: &str,
        sort: &str,
        order: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetSearchRepositoriesOkResponse> {
        let url = "/search/repositories".to_string();
        self.get(&url).await
    }

    /**
     * search_topics: GET /search/topics
     */
    pub async fn search_topics(
        &self,
        q: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetSearchTopicsOkResponse> {
        let url = "/search/topics".to_string();
        self.get(&url).await
    }

    /**
     * search_users: GET /search/users
     */
    pub async fn search_users(
        &self,
        q: &str,
        sort: &str,
        order: &str,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetSearchUsersOkResponse> {
        let url = "/search/users".to_string();
        self.get(&url).await
    }

    /**
     * teams_get_legacy: GET /teams/{team_id}
     */
    pub async fn teams_get_legacy(&self, team_id: i64) -> Result<types::TeamFull> {
        let url = format!(
            "/teams/{}",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_delete_legacy: DELETE /teams/{team_id}
     */
    pub async fn teams_delete_legacy(&self, team_id: i64) -> Result<()> {
        let url = format!(
            "/teams/{}",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_update_legacy: PATCH /teams/{team_id}
     */
    pub async fn teams_update_legacy(
        &self,
        team_id: i64,
        body: &types::UpdateTeamRequest,
    ) -> Result<types::TeamFull> {
        let url = format!(
            "/teams/{}",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_list_discussions_legacy: GET /teams/{team_id}/discussions
     */
    pub async fn teams_list_discussions_legacy(
        &self,
        team_id: i64,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::TeamDiscussion>> {
        let url = format!(
            "/teams/{}/discussions",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_create_discussion_legacy: POST /teams/{team_id}/discussions
     */
    pub async fn teams_create_discussion_legacy(
        &self,
        team_id: i64,
        body: &types::CreateDiscussionRequest,
    ) -> Result<types::TeamDiscussion> {
        let url = format!(
            "/teams/{}/discussions",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_get_discussion_legacy: GET /teams/{team_id}/discussions/{discussion_number}
     */
    pub async fn teams_get_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
    ) -> Result<types::TeamDiscussion> {
        let url = format!(
            "/teams/{}/discussions/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_delete_discussion_legacy: DELETE /teams/{team_id}/discussions/{discussion_number}
     */
    pub async fn teams_delete_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
    ) -> Result<()> {
        let url = format!(
            "/teams/{}/discussions/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_update_discussion_legacy: PATCH /teams/{team_id}/discussions/{discussion_number}
     */
    pub async fn teams_update_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        body: &types::UpdateDiscussionRequest,
    ) -> Result<types::TeamDiscussion> {
        let url = format!(
            "/teams/{}/discussions/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_list_discussion_comments_legacy: GET /teams/{team_id}/discussions/{discussion_number}/comments
     */
    pub async fn teams_list_discussion_comments_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::TeamDiscussionComment>> {
        let url = format!(
            "/teams/{}/discussions/{}/comments",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_create_discussion_comment_legacy: POST /teams/{team_id}/discussions/{discussion_number}/comments
     */
    pub async fn teams_create_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        body: &types::CreateDiscussionCommentRequest,
    ) -> Result<types::TeamDiscussionComment> {
        let url = format!(
            "/teams/{}/discussions/{}/comments",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_get_discussion_comment_legacy: GET /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}
     */
    pub async fn teams_get_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
    ) -> Result<types::TeamDiscussionComment> {
        let url = format!(
            "/teams/{}/discussions/{}/comments/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_delete_discussion_comment_legacy: DELETE /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}
     */
    pub async fn teams_delete_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
    ) -> Result<()> {
        let url = format!(
            "/teams/{}/discussions/{}/comments/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_update_discussion_comment_legacy: PATCH /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}
     */
    pub async fn teams_update_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
        body: &types::UpdateDiscussionCommentRequest,
    ) -> Result<types::TeamDiscussionComment> {
        let url = format!(
            "/teams/{}/discussions/{}/comments/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_list_for_team_discussion_comment_legacy: GET /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions
     */
    pub async fn reactions_list_for_team_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
        content: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Reaction>> {
        let url = format!(
            "/teams/{}/discussions/{}/comments/{}/reactions",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * reactions_create_for_team_discussion_comment_legacy: POST /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions
     */
    pub async fn reactions_create_for_team_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
        body: &types::CreateReactionTeamDiscussionCommentRequest,
    ) -> Result<types::Reaction> {
        let url = format!(
            "/teams/{}/discussions/{}/comments/{}/reactions",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
            progenitor_support::encode_path(&comment_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * reactions_list_for_team_discussion_legacy: GET /teams/{team_id}/discussions/{discussion_number}/reactions
     */
    pub async fn reactions_list_for_team_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        content: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Reaction>> {
        let url = format!(
            "/teams/{}/discussions/{}/reactions",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        self.get(&url).await
    }

    /**
     * reactions_create_for_team_discussion_legacy: POST /teams/{team_id}/discussions/{discussion_number}/reactions
     */
    pub async fn reactions_create_for_team_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        body: &types::CreateReactionTeamDiscussionRequest,
    ) -> Result<types::Reaction> {
        let url = format!(
            "/teams/{}/discussions/{}/reactions",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&discussion_number.to_string()),
        );

        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_list_pending_invitations_legacy: GET /teams/{team_id}/invitations
     */
    pub async fn teams_list_pending_invitations_legacy(
        &self,
        team_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::OrganizationInvitation>> {
        let url = format!(
            "/teams/{}/invitations",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_list_members_legacy: GET /teams/{team_id}/members
     */
    pub async fn teams_list_members_legacy(
        &self,
        team_id: i64,
        role: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/teams/{}/members",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_get_member_legacy: GET /teams/{team_id}/members/{username}
     */
    pub async fn teams_get_member_legacy(&self, team_id: i64, username: &str) -> Result<()> {
        let url = format!(
            "/teams/{}/members/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_add_member_legacy: PUT /teams/{team_id}/members/{username}
     */
    pub async fn teams_add_member_legacy(&self, team_id: i64, username: &str) -> Result<()> {
        let url = format!(
            "/teams/{}/members/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_remove_member_legacy: DELETE /teams/{team_id}/members/{username}
     */
    pub async fn teams_remove_member_legacy(&self, team_id: i64, username: &str) -> Result<()> {
        let url = format!(
            "/teams/{}/members/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_get_membership_for_user_legacy: GET /teams/{team_id}/memberships/{username}
     */
    pub async fn teams_get_membership_for_user_legacy(
        &self,
        team_id: i64,
        username: &str,
    ) -> Result<types::TeamMembership> {
        let url = format!(
            "/teams/{}/memberships/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_add_or_update_membership_for_user_legacy: PUT /teams/{team_id}/memberships/{username}
     */
    pub async fn teams_add_or_update_membership_for_user_legacy(
        &self,
        team_id: i64,
        username: &str,
        body: &types::AddUpdateTeamMembershipUserRequest,
    ) -> Result<types::TeamMembership> {
        let url = format!(
            "/teams/{}/memberships/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_remove_membership_for_user_legacy: DELETE /teams/{team_id}/memberships/{username}
     */
    pub async fn teams_remove_membership_for_user_legacy(
        &self,
        team_id: i64,
        username: &str,
    ) -> Result<()> {
        let url = format!(
            "/teams/{}/memberships/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_list_projects_legacy: GET /teams/{team_id}/projects
     */
    pub async fn teams_list_projects_legacy(
        &self,
        team_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::TeamProject>> {
        let url = format!(
            "/teams/{}/projects",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_check_permissions_for_project_legacy: GET /teams/{team_id}/projects/{project_id}
     */
    pub async fn teams_check_permissions_for_project_legacy(
        &self,
        team_id: i64,
        project_id: i64,
    ) -> Result<types::TeamProject> {
        let url = format!(
            "/teams/{}/projects/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&project_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_add_or_update_project_permissions_legacy: PUT /teams/{team_id}/projects/{project_id}
     */
    pub async fn teams_add_or_update_project_permissions_legacy(
        &self,
        team_id: i64,
        project_id: i64,
        body: &types::AddUpdateTeamProjectPermissionsRequest,
    ) -> Result<()> {
        let url = format!(
            "/teams/{}/projects/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&project_id.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_remove_project_legacy: DELETE /teams/{team_id}/projects/{project_id}
     */
    pub async fn teams_remove_project_legacy(&self, team_id: i64, project_id: i64) -> Result<()> {
        let url = format!(
            "/teams/{}/projects/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&project_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_list_repos_legacy: GET /teams/{team_id}/repos
     */
    pub async fn teams_list_repos_legacy(
        &self,
        team_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MinimalRepository>> {
        let url = format!(
            "/teams/{}/repos",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_check_permissions_for_repo_legacy: GET /teams/{team_id}/repos/{owner}/{repo}
     */
    pub async fn teams_check_permissions_for_repo_legacy(
        &self,
        team_id: i64,
        owner: &str,
        repo: &str,
    ) -> Result<types::TeamRepository> {
        let url = format!(
            "/teams/{}/repos/{}/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_add_or_update_repo_permissions_legacy: PUT /teams/{team_id}/repos/{owner}/{repo}
     */
    pub async fn teams_add_or_update_repo_permissions_legacy(
        &self,
        team_id: i64,
        owner: &str,
        repo: &str,
        body: &types::AddUpdateTeamRepositoryPermissionsRequest,
    ) -> Result<()> {
        let url = format!(
            "/teams/{}/repos/{}/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_remove_repo_legacy: DELETE /teams/{team_id}/repos/{owner}/{repo}
     */
    pub async fn teams_remove_repo_legacy(
        &self,
        team_id: i64,
        owner: &str,
        repo: &str,
    ) -> Result<()> {
        let url = format!(
            "/teams/{}/repos/{}/{}",
            progenitor_support::encode_path(&team_id.to_string()),
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * teams_list_idp_groups_for_legacy: GET /teams/{team_id}/team-sync/group-mappings
     */
    pub async fn teams_list_idp_groups_for_legacy(
        &self,
        team_id: i64,
    ) -> Result<types::GroupMapping> {
        let url = format!(
            "/teams/{}/team-sync/group-mappings",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * teams_create_or_update_idp_group_connections_legacy: PATCH /teams/{team_id}/team-sync/group-mappings
     */
    pub async fn teams_create_or_update_idp_group_connections_legacy(
        &self,
        team_id: i64,
        body: &types::CreateUpdateIdpGroupConnectionsRequest,
    ) -> Result<types::GroupMapping> {
        let url = format!(
            "/teams/{}/team-sync/group-mappings",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * teams_list_child_legacy: GET /teams/{team_id}/teams
     */
    pub async fn teams_list_child_legacy(
        &self,
        team_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Team>> {
        let url = format!(
            "/teams/{}/teams",
            progenitor_support::encode_path(&team_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_get_authenticated: GET /user
     */
    pub async fn users_get_authenticated(&self) -> Result<types::GetOkResponse> {
        let url = "/user".to_string();
        self.get(&url).await
    }

    /**
     * users_update_authenticated: PATCH /user
     */
    pub async fn users_update_authenticated(
        &self,
        body: &types::UpdateRequest,
    ) -> Result<types::PrivateUser> {
        let url = "/user".to_string();
        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * users_list_blocked_by_authenticated: GET /user/blocks
     */
    pub async fn users_list_blocked_by_authenticated(&self) -> Result<Vec<types::SimpleUser>> {
        let url = "/user/blocks".to_string();
        self.get(&url).await
    }

    /**
     * users_check_blocked: GET /user/blocks/{username}
     */
    pub async fn users_check_blocked(&self, username: &str) -> Result<()> {
        let url = format!(
            "/user/blocks/{}",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_block: PUT /user/blocks/{username}
     */
    pub async fn users_block(&self, username: &str) -> Result<()> {
        let url = format!(
            "/user/blocks/{}",
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * users_unblock: DELETE /user/blocks/{username}
     */
    pub async fn users_unblock(&self, username: &str) -> Result<()> {
        let url = format!(
            "/user/blocks/{}",
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * users_set_primary_email_visibility_for_authenticated: PATCH /user/email/visibility
     */
    pub async fn users_set_primary_email_visibility_for_authenticated(
        &self,
        body: &types::SetPrimaryEmailVisibilityRequest,
    ) -> Result<Vec<types::Email>> {
        let url = "/user/email/visibility".to_string();
        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * users_list_emails_for_authenticated: GET /user/emails
     */
    pub async fn users_list_emails_for_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Email>> {
        let url = "/user/emails".to_string();
        self.get(&url).await
    }

    /**
     * users_add_email_for_authenticated: POST /user/emails
     */
    pub async fn users_add_email_for_authenticated(
        &self,
        body: &types::AddEmailAddressRequest,
    ) -> Result<Vec<types::Email>> {
        let url = "/user/emails".to_string();
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * users_delete_email_for_authenticated: DELETE /user/emails
     */
    pub async fn users_delete_email_for_authenticated(
        &self,
        body: &types::DeleteEmailAddressRequest,
    ) -> Result<()> {
        let url = "/user/emails".to_string();
        let res = self
            .client
            .delete(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * users_list_followers_for_authenticated_user: GET /user/followers
     */
    pub async fn users_list_followers_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = "/user/followers".to_string();
        self.get(&url).await
    }

    /**
     * users_list_followed_by_authenticated: GET /user/following
     */
    pub async fn users_list_followed_by_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = "/user/following".to_string();
        self.get(&url).await
    }

    /**
     * users_check_person_is_followed_by_authenticated: GET /user/following/{username}
     */
    pub async fn users_check_person_is_followed_by_authenticated(
        &self,
        username: &str,
    ) -> Result<()> {
        let url = format!(
            "/user/following/{}",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_follow: PUT /user/following/{username}
     */
    pub async fn users_follow(&self, username: &str) -> Result<()> {
        let url = format!(
            "/user/following/{}",
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * users_unfollow: DELETE /user/following/{username}
     */
    pub async fn users_unfollow(&self, username: &str) -> Result<()> {
        let url = format!(
            "/user/following/{}",
            progenitor_support::encode_path(&username.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * users_list_gpg_keys_for_authenticated: GET /user/gpg_keys
     */
    pub async fn users_list_gpg_keys_for_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::GpgKey>> {
        let url = "/user/gpg_keys".to_string();
        self.get(&url).await
    }

    /**
     * users_create_gpg_key_for_authenticated: POST /user/gpg_keys
     */
    pub async fn users_create_gpg_key_for_authenticated(
        &self,
        body: &types::CreateGpgKeyRequest,
    ) -> Result<types::GpgKey> {
        let url = "/user/gpg_keys".to_string();
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * users_get_gpg_key_for_authenticated: GET /user/gpg_keys/{gpg_key_id}
     */
    pub async fn users_get_gpg_key_for_authenticated(
        &self,
        gpg_key_id: i64,
    ) -> Result<types::GpgKey> {
        let url = format!(
            "/user/gpg_keys/{}",
            progenitor_support::encode_path(&gpg_key_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_delete_gpg_key_for_authenticated: DELETE /user/gpg_keys/{gpg_key_id}
     */
    pub async fn users_delete_gpg_key_for_authenticated(&self, gpg_key_id: i64) -> Result<()> {
        let url = format!(
            "/user/gpg_keys/{}",
            progenitor_support::encode_path(&gpg_key_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_list_installations_for_authenticated_user: GET /user/installations
     */
    pub async fn apps_list_installations_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListAppInstallationsAccessibleUserAccessTokenOkResponse> {
        let url = "/user/installations".to_string();
        self.get(&url).await
    }

    /**
     * apps_list_installation_repos_for_authenticated_user: GET /user/installations/{installation_id}/repositories
     */
    pub async fn apps_list_installation_repos_for_authenticated_user(
        &self,
        installation_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<types::GetListRepositoriesAccessibleUserAccessTokenOkResponse> {
        let url = format!(
            "/user/installations/{}/repositories",
            progenitor_support::encode_path(&installation_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * apps_add_repo_to_installation: PUT /user/installations/{installation_id}/repositories/{repository_id}
     */
    pub async fn apps_add_repo_to_installation(
        &self,
        installation_id: i64,
        repository_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/user/installations/{}/repositories/{}",
            progenitor_support::encode_path(&installation_id.to_string()),
            progenitor_support::encode_path(&repository_id.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_remove_repo_from_installation: DELETE /user/installations/{installation_id}/repositories/{repository_id}
     */
    pub async fn apps_remove_repo_from_installation(
        &self,
        installation_id: i64,
        repository_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/user/installations/{}/repositories/{}",
            progenitor_support::encode_path(&installation_id.to_string()),
            progenitor_support::encode_path(&repository_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * interactions_get_restrictions_for_authenticated_user: GET /user/interaction-limits
     */
    pub async fn interactions_get_restrictions_for_authenticated_user(
        &self,
    ) -> Result<types::GetInteractionRestrictionsPublicRepositoriesOkResponse> {
        let url = "/user/interaction-limits".to_string();
        self.get(&url).await
    }

    /**
     * interactions_set_restrictions_for_authenticated_user: PUT /user/interaction-limits
     */
    pub async fn interactions_set_restrictions_for_authenticated_user(
        &self,
        body: &types::InteractionLimit,
    ) -> Result<types::InteractionLimitResponse> {
        let url = "/user/interaction-limits".to_string();
        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * interactions_remove_restrictions_for_authenticated_user: DELETE /user/interaction-limits
     */
    pub async fn interactions_remove_restrictions_for_authenticated_user(&self) -> Result<()> {
        let url = "/user/interaction-limits".to_string();
        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * issues_list_for_authenticated_user: GET /user/issues
     */
    pub async fn issues_list_for_authenticated_user(
        &self,
        filter: &str,
        state: &str,
        labels: &str,
        sort: &str,
        direction: &str,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Issue>> {
        let url = "/user/issues".to_string();
        self.get(&url).await
    }

    /**
     * users_list_public_ssh_keys_for_authenticated: GET /user/keys
     */
    pub async fn users_list_public_ssh_keys_for_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Key>> {
        let url = "/user/keys".to_string();
        self.get(&url).await
    }

    /**
     * users_create_public_ssh_key_for_authenticated: POST /user/keys
     */
    pub async fn users_create_public_ssh_key_for_authenticated(
        &self,
        body: &types::CreatePublicSshKeyRequest,
    ) -> Result<types::Key> {
        let url = "/user/keys".to_string();
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * users_get_public_ssh_key_for_authenticated: GET /user/keys/{key_id}
     */
    pub async fn users_get_public_ssh_key_for_authenticated(
        &self,
        key_id: i64,
    ) -> Result<types::Key> {
        let url = format!(
            "/user/keys/{}",
            progenitor_support::encode_path(&key_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_delete_public_ssh_key_for_authenticated: DELETE /user/keys/{key_id}
     */
    pub async fn users_delete_public_ssh_key_for_authenticated(&self, key_id: i64) -> Result<()> {
        let url = format!(
            "/user/keys/{}",
            progenitor_support::encode_path(&key_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * apps_list_subscriptions_for_authenticated_user: GET /user/marketplace_purchases
     */
    pub async fn apps_list_subscriptions_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::UserMarketplacePurchase>> {
        let url = "/user/marketplace_purchases".to_string();
        self.get(&url).await
    }

    /**
     * apps_list_subscriptions_for_authenticated_user_stubbed: GET /user/marketplace_purchases/stubbed
     */
    pub async fn apps_list_subscriptions_for_authenticated_user_stubbed(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::UserMarketplacePurchase>> {
        let url = "/user/marketplace_purchases/stubbed".to_string();
        self.get(&url).await
    }

    /**
     * orgs_list_memberships_for_authenticated_user: GET /user/memberships/orgs
     */
    pub async fn orgs_list_memberships_for_authenticated_user(
        &self,
        state: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::OrgMembership>> {
        let url = "/user/memberships/orgs".to_string();
        self.get(&url).await
    }

    /**
     * orgs_get_membership_for_authenticated_user: GET /user/memberships/orgs/{org}
     */
    pub async fn orgs_get_membership_for_authenticated_user(
        &self,
        org: &str,
    ) -> Result<types::OrgMembership> {
        let url = format!(
            "/user/memberships/orgs/{}",
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_update_membership_for_authenticated_user: PATCH /user/memberships/orgs/{org}
     */
    pub async fn orgs_update_membership_for_authenticated_user(
        &self,
        org: &str,
        body: &types::UpdateOrganizationMembershipRequest,
    ) -> Result<types::OrgMembership> {
        let url = format!(
            "/user/memberships/orgs/{}",
            progenitor_support::encode_path(&org.to_string()),
        );

        let res = self
            .client
            .patch(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * migrations_list_for_authenticated_user: GET /user/migrations
     */
    pub async fn migrations_list_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Migration>> {
        let url = "/user/migrations".to_string();
        self.get(&url).await
    }

    /**
     * migrations_start_for_authenticated_user: POST /user/migrations
     */
    pub async fn migrations_start_for_authenticated_user(
        &self,
        body: &types::StartUserMigrationRequest,
    ) -> Result<types::Migration> {
        let url = "/user/migrations".to_string();
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * migrations_get_status_for_authenticated_user: GET /user/migrations/{migration_id}
     */
    pub async fn migrations_get_status_for_authenticated_user(
        &self,
        migration_id: i64,
        exclude: &[String],
    ) -> Result<types::Migration> {
        let url = format!(
            "/user/migrations/{}",
            progenitor_support::encode_path(&migration_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * migrations_get_archive_for_authenticated_user: GET /user/migrations/{migration_id}/archive
     */
    pub async fn migrations_get_archive_for_authenticated_user(
        &self,
        migration_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/user/migrations/{}/archive",
            progenitor_support::encode_path(&migration_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * migrations_delete_archive_for_authenticated_user: DELETE /user/migrations/{migration_id}/archive
     */
    pub async fn migrations_delete_archive_for_authenticated_user(
        &self,
        migration_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/user/migrations/{}/archive",
            progenitor_support::encode_path(&migration_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * migrations_unlock_repo_for_authenticated_user: DELETE /user/migrations/{migration_id}/repos/{repo_name}/lock
     */
    pub async fn migrations_unlock_repo_for_authenticated_user(
        &self,
        migration_id: i64,
        repo_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/user/migrations/{}/repos/{}/lock",
            progenitor_support::encode_path(&migration_id.to_string()),
            progenitor_support::encode_path(&repo_name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * migrations_list_repos_for_user: GET /user/migrations/{migration_id}/repositories
     */
    pub async fn migrations_list_repos_for_user(
        &self,
        migration_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MinimalRepository>> {
        let url = format!(
            "/user/migrations/{}/repositories",
            progenitor_support::encode_path(&migration_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_list_for_authenticated_user: GET /user/orgs
     */
    pub async fn orgs_list_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::OrganizationSimple>> {
        let url = "/user/orgs".to_string();
        self.get(&url).await
    }

    /**
     * packages_get_package_for_authenticated_user: GET /user/packages/{package_type}/{package_name}
     */
    pub async fn packages_get_package_for_authenticated_user(
        &self,
        package_type: &str,
        package_name: &str,
    ) -> Result<types::Package> {
        let url = format!(
            "/user/packages/{}/{}",
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * packages_delete_package_for_authenticated_user: DELETE /user/packages/{package_type}/{package_name}
     */
    pub async fn packages_delete_package_for_authenticated_user(
        &self,
        package_type: &str,
        package_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/user/packages/{}/{}",
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * packages_restore_package_for_authenticated_user: POST /user/packages/{package_type}/{package_name}/restore
     */
    pub async fn packages_restore_package_for_authenticated_user(
        &self,
        package_type: &str,
        package_name: &str,
        token: &str,
    ) -> Result<()> {
        let url = format!(
            "/user/packages/{}/{}/restore",
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        let res = self
            .client
            .post(url)
            .query(&[("token", token.to_string())])
            .send()
            .await?
            .error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * packages_get_all_package_versions_for_package_owned_by_authenticated_user: GET /user/packages/{package_type}/{package_name}/versions
     */
    pub async fn packages_get_all_package_versions_for_package_owned_by_authenticated_user(
        &self,
        package_type: &str,
        package_name: &str,
        page: i64,
        per_page: i64,
        state: &str,
    ) -> Result<Vec<types::PackageVersion>> {
        let url = format!(
            "/user/packages/{}/{}/versions",
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * packages_get_package_version_for_authenticated_user: GET /user/packages/{package_type}/{package_name}/versions/{package_version_id}
     */
    pub async fn packages_get_package_version_for_authenticated_user(
        &self,
        package_type: &str,
        package_name: &str,
        package_version_id: i64,
    ) -> Result<types::PackageVersion> {
        let url = format!(
            "/user/packages/{}/{}/versions/{}",
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
            progenitor_support::encode_path(&package_version_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * packages_delete_package_version_for_authenticated_user: DELETE /user/packages/{package_type}/{package_name}/versions/{package_version_id}
     */
    pub async fn packages_delete_package_version_for_authenticated_user(
        &self,
        package_type: &str,
        package_name: &str,
        package_version_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/user/packages/{}/{}/versions/{}",
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
            progenitor_support::encode_path(&package_version_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * packages_restore_package_version_for_authenticated_user: POST /user/packages/{package_type}/{package_name}/versions/{package_version_id}/restore
     */
    pub async fn packages_restore_package_version_for_authenticated_user(
        &self,
        package_type: &str,
        package_name: &str,
        package_version_id: i64,
    ) -> Result<()> {
        let url = format!(
            "/user/packages/{}/{}/versions/{}/restore",
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
            progenitor_support::encode_path(&package_version_id.to_string()),
        );

        let res = self.client.post(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * projects_create_for_authenticated_user: POST /user/projects
     */
    pub async fn projects_create_for_authenticated_user(
        &self,
        body: &types::CreateUserProjectRequest,
    ) -> Result<types::Project> {
        let url = "/user/projects".to_string();
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * users_list_public_emails_for_authenticated: GET /user/public_emails
     */
    pub async fn users_list_public_emails_for_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Email>> {
        let url = "/user/public_emails".to_string();
        self.get(&url).await
    }

    /**
     * repos_list_for_authenticated_user: GET /user/repos
     */
    pub async fn repos_list_for_authenticated_user(
        &self,
        visibility: &str,
        affiliation: &str,
        type_: &&str,
        sort: &str,
        direction: &str,
        per_page: i64,
        page: i64,
        since: DateTime<Utc>,
        before: DateTime<Utc>,
    ) -> Result<Vec<types::Repository>> {
        let url = "/user/repos".to_string();
        self.get(&url).await
    }

    /**
     * repos_create_for_authenticated_user: POST /user/repos
     */
    pub async fn repos_create_for_authenticated_user(
        &self,
        body: &types::CreateRepositoryRequest,
    ) -> Result<types::Repository> {
        let url = "/user/repos".to_string();
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        Ok(res.json().await?)
    }

    /**
     * repos_list_invitations_for_authenticated_user: GET /user/repository_invitations
     */
    pub async fn repos_list_invitations_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::RepositoryInvitation>> {
        let url = "/user/repository_invitations".to_string();
        self.get(&url).await
    }

    /**
     * repos_decline_invitation: DELETE /user/repository_invitations/{invitation_id}
     */
    pub async fn repos_decline_invitation(&self, invitation_id: i64) -> Result<()> {
        let url = format!(
            "/user/repository_invitations/{}",
            progenitor_support::encode_path(&invitation_id.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * repos_accept_invitation: PATCH /user/repository_invitations/{invitation_id}
     */
    pub async fn repos_accept_invitation(&self, invitation_id: i64) -> Result<()> {
        let url = format!(
            "/user/repository_invitations/{}",
            progenitor_support::encode_path(&invitation_id.to_string()),
        );

        let res = self.client.patch(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * activity_list_repos_starred_by_authenticated_user: GET /user/starred
     */
    pub async fn activity_list_repos_starred_by_authenticated_user(
        &self,
        sort: &str,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Repository>> {
        let url = "/user/starred".to_string();
        self.get(&url).await
    }

    /**
     * activity_check_repo_is_starred_by_authenticated_user: GET /user/starred/{owner}/{repo}
     */
    pub async fn activity_check_repo_is_starred_by_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<()> {
        let url = format!(
            "/user/starred/{}/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_star_repo_for_authenticated_user: PUT /user/starred/{owner}/{repo}
     */
    pub async fn activity_star_repo_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<()> {
        let url = format!(
            "/user/starred/{}/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.put(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * activity_unstar_repo_for_authenticated_user: DELETE /user/starred/{owner}/{repo}
     */
    pub async fn activity_unstar_repo_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<()> {
        let url = format!(
            "/user/starred/{}/{}",
            progenitor_support::encode_path(&owner.to_string()),
            progenitor_support::encode_path(&repo.to_string()),
        );

        let res = self.client.delete(url).send().await?.error_for_status()?;

        let _ = res.text().await?;
        Ok(())
    }

    /**
     * activity_list_watched_repos_for_authenticated_user: GET /user/subscriptions
     */
    pub async fn activity_list_watched_repos_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MinimalRepository>> {
        let url = "/user/subscriptions".to_string();
        self.get(&url).await
    }

    /**
     * teams_list_for_authenticated_user: GET /user/teams
     */
    pub async fn teams_list_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::TeamFull>> {
        let url = "/user/teams".to_string();
        self.get(&url).await
    }

    /**
     * users_list: GET /users
     */
    pub async fn users_list(&self, since: i64, per_page: i64) -> Result<Vec<types::SimpleUser>> {
        let url = "/users".to_string();
        self.get(&url).await
    }

    /**
     * users_get_by_username: GET /users/{username}
     */
    pub async fn users_get_by_username(&self, username: &str) -> Result<types::GetUserOkResponse> {
        let url = format!(
            "/users/{}",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_events_for_authenticated_user: GET /users/{username}/events
     */
    pub async fn activity_list_events_for_authenticated_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Event>> {
        let url = format!(
            "/users/{}/events",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_org_events_for_authenticated_user: GET /users/{username}/events/orgs/{org}
     */
    pub async fn activity_list_org_events_for_authenticated_user(
        &self,
        username: &str,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Event>> {
        let url = format!(
            "/users/{}/events/orgs/{}",
            progenitor_support::encode_path(&username.to_string()),
            progenitor_support::encode_path(&org.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_public_events_for_user: GET /users/{username}/events/public
     */
    pub async fn activity_list_public_events_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Event>> {
        let url = format!(
            "/users/{}/events/public",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_list_followers_for_user: GET /users/{username}/followers
     */
    pub async fn users_list_followers_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/users/{}/followers",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_list_following_for_user: GET /users/{username}/following
     */
    pub async fn users_list_following_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::SimpleUser>> {
        let url = format!(
            "/users/{}/following",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_check_following_for_user: GET /users/{username}/following/{target_user}
     */
    pub async fn users_check_following_for_user(
        &self,
        username: &str,
        target_user: &str,
    ) -> Result<()> {
        let url = format!(
            "/users/{}/following/{}",
            progenitor_support::encode_path(&username.to_string()),
            progenitor_support::encode_path(&target_user.to_string()),
        );

        self.get(&url).await
    }

    /**
     * gists_list_for_user: GET /users/{username}/gists
     */
    pub async fn gists_list_for_user(
        &self,
        username: &str,
        since: DateTime<Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::BaseGist>> {
        let url = format!(
            "/users/{}/gists",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_list_gpg_keys_for_user: GET /users/{username}/gpg_keys
     */
    pub async fn users_list_gpg_keys_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::GpgKey>> {
        let url = format!(
            "/users/{}/gpg_keys",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_get_context_for_user: GET /users/{username}/hovercard
     */
    pub async fn users_get_context_for_user(
        &self,
        username: &str,
        subject_type: &str,
        subject_id: &str,
    ) -> Result<types::Hovercard> {
        let url = format!(
            "/users/{}/hovercard",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * apps_get_user_installation: GET /users/{username}/installation
     */
    pub async fn apps_get_user_installation(&self, username: &str) -> Result<types::Installation> {
        let url = format!(
            "/users/{}/installation",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * users_list_public_keys_for_user: GET /users/{username}/keys
     */
    pub async fn users_list_public_keys_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::KeySimple>> {
        let url = format!(
            "/users/{}/keys",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * orgs_list_for_user: GET /users/{username}/orgs
     */
    pub async fn orgs_list_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::OrganizationSimple>> {
        let url = format!(
            "/users/{}/orgs",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * packages_get_package_for_user: GET /users/{username}/packages/{package_type}/{package_name}
     */
    pub async fn packages_get_package_for_user(
        &self,
        package_type: &str,
        package_name: &str,
        username: &str,
    ) -> Result<types::Package> {
        let url = format!(
            "/users/{}/packages/{}/{}",
            progenitor_support::encode_path(&username.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * packages_get_all_package_versions_for_package_owned_by_user: GET /users/{username}/packages/{package_type}/{package_name}/versions
     */
    pub async fn packages_get_all_package_versions_for_package_owned_by_user(
        &self,
        package_type: &str,
        package_name: &str,
        username: &str,
    ) -> Result<Vec<types::PackageVersion>> {
        let url = format!(
            "/users/{}/packages/{}/{}/versions",
            progenitor_support::encode_path(&username.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
        );

        self.get(&url).await
    }

    /**
     * packages_get_package_version_for_user: GET /users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}
     */
    pub async fn packages_get_package_version_for_user(
        &self,
        package_type: &str,
        package_name: &str,
        package_version_id: i64,
        username: &str,
    ) -> Result<types::PackageVersion> {
        let url = format!(
            "/users/{}/packages/{}/{}/versions/{}",
            progenitor_support::encode_path(&username.to_string()),
            progenitor_support::encode_path(&package_type.to_string()),
            progenitor_support::encode_path(&package_name.to_string()),
            progenitor_support::encode_path(&package_version_id.to_string()),
        );

        self.get(&url).await
    }

    /**
     * projects_list_for_user: GET /users/{username}/projects
     */
    pub async fn projects_list_for_user(
        &self,
        username: &str,
        state: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Project>> {
        let url = format!(
            "/users/{}/projects",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_received_events_for_user: GET /users/{username}/received_events
     */
    pub async fn activity_list_received_events_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Event>> {
        let url = format!(
            "/users/{}/received_events",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_received_public_events_for_user: GET /users/{username}/received_events/public
     */
    pub async fn activity_list_received_public_events_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::Event>> {
        let url = format!(
            "/users/{}/received_events/public",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * repos_list_for_user: GET /users/{username}/repos
     */
    pub async fn repos_list_for_user(
        &self,
        username: &str,
        type_: &&str,
        sort: &str,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MinimalRepository>> {
        let url = format!(
            "/users/{}/repos",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * billing_get_github_actions_billing_user: GET /users/{username}/settings/billing/actions
     */
    pub async fn billing_get_github_actions_billing_user(
        &self,
        username: &str,
    ) -> Result<types::ActionsBillingUsage> {
        let url = format!(
            "/users/{}/settings/billing/actions",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * billing_get_github_packages_billing_user: GET /users/{username}/settings/billing/packages
     */
    pub async fn billing_get_github_packages_billing_user(
        &self,
        username: &str,
    ) -> Result<types::PackagesBillingUsage> {
        let url = format!(
            "/users/{}/settings/billing/packages",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * billing_get_shared_storage_billing_user: GET /users/{username}/settings/billing/shared-storage
     */
    pub async fn billing_get_shared_storage_billing_user(
        &self,
        username: &str,
    ) -> Result<types::CombinedBillingUsage> {
        let url = format!(
            "/users/{}/settings/billing/shared-storage",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_repos_starred_by_user: GET /users/{username}/starred
     */
    pub async fn activity_list_repos_starred_by_user(
        &self,
        username: &str,
        sort: &str,
        direction: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::StarredRepository>> {
        let url = format!(
            "/users/{}/starred",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * activity_list_repos_watched_by_user: GET /users/{username}/subscriptions
     */
    pub async fn activity_list_repos_watched_by_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<types::MinimalRepository>> {
        let url = format!(
            "/users/{}/subscriptions",
            progenitor_support::encode_path(&username.to_string()),
        );

        self.get(&url).await
    }

    /**
     * meta_get_zen: GET /zen
     */
    pub async fn meta_get_zen(&self) -> Result<String> {
        let url = "/zen".to_string();
        self.get(&url).await
    }
}
