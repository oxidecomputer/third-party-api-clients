//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AcsEndpoint {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub index: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ActivateFactorRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub attestation: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "clientData"
    )]
    pub client_data: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "passCode"
    )]
    pub pass_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "registrationData"
    )]
    pub registration_data: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "stateToken"
    )]
    pub state_token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Type {
    #[serde(rename = "APP")]
    App,
    #[serde(rename = "APP_TYPE")]
    AppType,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::App => "APP",
            Type::AppType => "APP_TYPE",
            Type::Noop => "",
            Type::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::Noop
    }
}
impl Type {
    pub fn is_noop(&self) -> bool {
        matches!(self, Type::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AppInstanceConditionEvaluatorOr {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AppInstancePolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub exclude: Vec<AppInstanceConditionEvaluatorOr>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<AppInstanceConditionEvaluatorOr>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserCondition {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub exclude: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AppLink {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "appAssignmentId"
    )]
    pub app_assignment_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "appInstanceId"
    )]
    pub app_instance_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "appName"
    )]
    pub app_name: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "credentialsSetup"
    )]
    pub credentials_setup: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub hidden: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "linkUrl"
    )]
    pub link_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "logoUrl"
    )]
    pub logo_url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sortOrder"
    )]
    pub sort_order: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Links {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AppUser {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<AppUserCredentials>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "externalId"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastSync"
    )]
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "passwordChanged"
    )]
    pub password_changed: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scope: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "statusChanged"
    )]
    pub status_changed: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "syncState"
    )]
    pub sync_state: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AppUserCredentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<AppUserPasswordCredential>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "userName"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AppUserPasswordCredential {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Active => "ACTIVE",
            Status::Deleted => "DELETED",
            Status::Inactive => "INACTIVE",
            Status::Noop => "",
            Status::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Status {
    fn default() -> Status {
        Status::Noop
    }
}
impl Status {
    pub fn is_noop(&self) -> bool {
        matches!(self, Status::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Application {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<ApplicationAccessibility>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ApplicationCredentials>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub features: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub licensing: Option<ApplicationLicensing>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<ApplicationSettings>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "signOnMode"
    )]
    pub sign_on_mode: Option<ApplicationSignOnMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ApplicationVisibility>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationAccessibility {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "errorRedirectUrl"
    )]
    pub error_redirect_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "loginRedirectUrl"
    )]
    pub login_redirect_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "selfService"
    )]
    pub self_service: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationCredentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing: Option<ApplicationCredentialsSigning>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userNameTemplate"
    )]
    pub user_name_template: Option<ApplicationCredentialsUsernameTemplate>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationCredentialsOAuthClient {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "autoKeyRotation"
    )]
    pub auto_key_rotation: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<OAuthEndpointAuthenticationMethod>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ApplicationCredentialsScheme {
    #[serde(rename = "ADMIN_SETS_CREDENTIALS")]
    AdminSetsCredentials,
    #[serde(rename = "EDIT_PASSWORD_ONLY")]
    EditPasswordOnly,
    #[serde(rename = "EDIT_USERNAME_AND_PASSWORD")]
    EditUsernameAndPassword,
    #[serde(rename = "EXTERNAL_PASSWORD_SYNC")]
    ExternalPasswordSync,
    #[serde(rename = "SHARED_USERNAME_AND_PASSWORD")]
    SharedUsernameAndPassword,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ApplicationCredentialsScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationCredentialsScheme::AdminSetsCredentials => "ADMIN_SETS_CREDENTIALS",
            ApplicationCredentialsScheme::EditPasswordOnly => "EDIT_PASSWORD_ONLY",
            ApplicationCredentialsScheme::EditUsernameAndPassword => "EDIT_USERNAME_AND_PASSWORD",
            ApplicationCredentialsScheme::ExternalPasswordSync => "EXTERNAL_PASSWORD_SYNC",
            ApplicationCredentialsScheme::SharedUsernameAndPassword => {
                "SHARED_USERNAME_AND_PASSWORD"
            }
            ApplicationCredentialsScheme::Noop => "",
            ApplicationCredentialsScheme::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ApplicationCredentialsScheme {
    fn default() -> ApplicationCredentialsScheme {
        ApplicationCredentialsScheme::Noop
    }
}
impl ApplicationCredentialsScheme {
    pub fn is_noop(&self) -> bool {
        matches!(self, ApplicationCredentialsScheme::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationCredentialsSigning {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kid: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastRotated"
    )]
    pub last_rotated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "nextRotation"
    )]
    pub next_rotation: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "rotationMode"
    )]
    pub rotation_mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "use")]
    pub use_: Option<Use>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Use {
    #[serde(rename = "sig")]
    Sig,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Use {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Use::Sig => "sig",
            Use::Noop => "",
            Use::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Use {
    fn default() -> Use {
        Use::Noop
    }
}
impl Use {
    pub fn is_noop(&self) -> bool {
        matches!(self, Use::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationCredentialsUsernameTemplate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub suffix: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub template: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationGroupAssignment {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Links>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationLicensing {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "seatCount"
    )]
    pub seat_count: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<serde_json::Value>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "implicitAssignment"
    )]
    pub implicit_assignment: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "inlineHookId"
    )]
    pub inline_hook_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<ApplicationSettingsNotifications>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationSettingsNotifications {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpn: Option<ApplicationSettingsNotificationsVpn>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationSettingsNotificationsVpn {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "helpUrl"
    )]
    pub help_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<ApplicationSettingsNotificationsVpnNetwork>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationSettingsNotificationsVpnNetwork {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub connection: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub exclude: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ApplicationSignOnMode {
    #[serde(rename = "AUTO_LOGIN")]
    AutoLogin,
    #[serde(rename = "BASIC_AUTH")]
    BasicAuth,
    #[serde(rename = "BOOKMARK")]
    Bookmark,
    #[serde(rename = "BROWSER_PLUGIN")]
    BrowserPlugin,
    #[serde(rename = "OPENID_CONNECT")]
    OpenidConnect,
    #[serde(rename = "SAML_1_1")]
    Saml1,
    #[serde(rename = "SAML_2_0")]
    Saml20,
    #[serde(rename = "SECURE_PASSWORD_STORE")]
    SecurePasswordStore,
    #[serde(rename = "WS_FEDERATION")]
    WsFederation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ApplicationSignOnMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationSignOnMode::AutoLogin => "AUTO_LOGIN",
            ApplicationSignOnMode::BasicAuth => "BASIC_AUTH",
            ApplicationSignOnMode::Bookmark => "BOOKMARK",
            ApplicationSignOnMode::BrowserPlugin => "BROWSER_PLUGIN",
            ApplicationSignOnMode::OpenidConnect => "OPENID_CONNECT",
            ApplicationSignOnMode::Saml1 => "SAML_1_1",
            ApplicationSignOnMode::Saml20 => "SAML_2_0",
            ApplicationSignOnMode::SecurePasswordStore => "SECURE_PASSWORD_STORE",
            ApplicationSignOnMode::WsFederation => "WS_FEDERATION",
            ApplicationSignOnMode::Noop => "",
            ApplicationSignOnMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ApplicationSignOnMode {
    fn default() -> ApplicationSignOnMode {
        ApplicationSignOnMode::Noop
    }
}
impl ApplicationSignOnMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, ApplicationSignOnMode::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationVisibility {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "appLinks"
    )]
    pub app_links: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "autoSubmitToolbar"
    )]
    pub auto_submit_toolbar: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide: Option<ApplicationVisibilityHide>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApplicationVisibilityHide {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "iOS"
    )]
    pub i_os: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub web: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AssignRoleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<RoleType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthenticationProvider {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<AuthenticationProviderType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AuthenticationProviderType {
    #[serde(rename = "ACTIVE_DIRECTORY")]
    ActiveDirectory,
    #[serde(rename = "FEDERATION")]
    Federation,
    #[serde(rename = "IMPORT")]
    Import,
    #[serde(rename = "LDAP")]
    Ldap,
    #[serde(rename = "OKTA")]
    Okta,
    #[serde(rename = "SOCIAL")]
    Social,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AuthenticationProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationProviderType::ActiveDirectory => "ACTIVE_DIRECTORY",
            AuthenticationProviderType::Federation => "FEDERATION",
            AuthenticationProviderType::Import => "IMPORT",
            AuthenticationProviderType::Ldap => "LDAP",
            AuthenticationProviderType::Okta => "OKTA",
            AuthenticationProviderType::Social => "SOCIAL",
            AuthenticationProviderType::Noop => "",
            AuthenticationProviderType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AuthenticationProviderType {
    fn default() -> AuthenticationProviderType {
        AuthenticationProviderType::Noop
    }
}
impl AuthenticationProviderType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AuthenticationProviderType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum IssuerMode {
    #[serde(rename = "CUSTOM_URL")]
    CustomUrl,
    #[serde(rename = "ORG_URL")]
    OrgUrl,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IssuerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssuerMode::CustomUrl => "CUSTOM_URL",
            IssuerMode::OrgUrl => "ORG_URL",
            IssuerMode::Noop => "",
            IssuerMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IssuerMode {
    fn default() -> IssuerMode {
        IssuerMode::Noop
    }
}
impl IssuerMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, IssuerMode::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum RoleStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RoleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleStatus::Active => "ACTIVE",
            RoleStatus::Inactive => "INACTIVE",
            RoleStatus::Noop => "",
            RoleStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RoleStatus {
    fn default() -> RoleStatus {
        RoleStatus::Noop
    }
}
impl RoleStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, RoleStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthorizationServer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub audiences: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<AuthorizationServerCredentials>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issuer: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "issuerMode"
    )]
    pub issuer_mode: Option<IssuerMode>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthorizationServerCredentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing: Option<AuthorizationServerCredentialsSigningConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AuthorizationServerCredentialsRotationMode {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AuthorizationServerCredentialsRotationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorizationServerCredentialsRotationMode::Auto => "AUTO",
            AuthorizationServerCredentialsRotationMode::Manual => "MANUAL",
            AuthorizationServerCredentialsRotationMode::Noop => "",
            AuthorizationServerCredentialsRotationMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AuthorizationServerCredentialsRotationMode {
    fn default() -> AuthorizationServerCredentialsRotationMode {
        AuthorizationServerCredentialsRotationMode::Noop
    }
}
impl AuthorizationServerCredentialsRotationMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, AuthorizationServerCredentialsRotationMode::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthorizationServerCredentialsSigningConfig {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kid: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastRotated"
    )]
    pub last_rotated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "nextRotation"
    )]
    pub next_rotation: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "rotationMode"
    )]
    pub rotation_mode: Option<AuthorizationServerCredentialsRotationMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "use")]
    pub use_: Option<Use>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthorizationServerPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<PolicyRuleConditions>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub system: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<PolicyType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AuthorizationServerPolicyRuleType {
    #[serde(rename = "RESOURCE_ACCESS")]
    ResourceAccess,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AuthorizationServerPolicyRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorizationServerPolicyRuleType::ResourceAccess => "RESOURCE_ACCESS",
            AuthorizationServerPolicyRuleType::Noop => "",
            AuthorizationServerPolicyRuleType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AuthorizationServerPolicyRuleType {
    fn default() -> AuthorizationServerPolicyRuleType {
        AuthorizationServerPolicyRuleType::Noop
    }
}
impl AuthorizationServerPolicyRuleType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AuthorizationServerPolicyRuleType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthorizationServerPolicyRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<AuthorizationServerPolicyRuleActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<AuthorizationServerPolicyRuleConditions>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub system: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<AuthorizationServerPolicyRuleType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthorizationServerPolicyRuleActions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<TokenAuthorizationServerPolicyRuleAction>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthorizationServerPolicyRuleConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clients: Option<ClientPolicyCondition>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "grantTypes"
    )]
    pub grant_types: Option<ClientPolicyCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub people: Option<PolicyPeopleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<OAuth2ScopesMediationPolicyRuleCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutoLoginApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SchemeApplicationCredentials>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AutoLoginApplicationSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutoLoginApplicationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signOn")]
    pub sign_on: Option<AutoLoginApplicationSettingsSignOn>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutoLoginApplicationSettingsSignOn {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "loginUrl"
    )]
    pub login_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "redirectUrl"
    )]
    pub redirect_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BasicApplicationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<BasicApplicationSettingsData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BasicApplicationSettingsData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "authURL"
    )]
    pub auth_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BasicAuthApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SchemeApplicationCredentials>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<BasicApplicationSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BeforeScheduledActionPolicyRuleCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lifecycleAction"
    )]
    pub lifecycle_action: Option<ScheduledUserLifecycleAction>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BookmarkApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<BookmarkApplicationSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BookmarkApplicationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<BookmarkApplicationSettingsData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BookmarkApplicationSettingsData {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "requestIntegration"
    )]
    pub request_integration: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BrowserPluginApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SchemeApplicationCredentials>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CallUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<CallUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CallUserFactorProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "phoneExtension"
    )]
    pub phone_extension: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "phoneNumber"
    )]
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CatalogApplication {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub features: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "signOnModes"
    )]
    pub sign_on_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "verificationStatus"
    )]
    pub verification_status: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub website: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChangePasswordRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "newPassword"
    )]
    pub new_password: Option<PasswordCredential>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "oldPassword"
    )]
    pub old_password: Option<PasswordCredential>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ClientPolicyCondition {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContextPolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expression: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreateSessionRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "sessionToken"
    )]
    pub session_token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreateUserRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<UserCredentials>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "groupIds"
    )]
    pub group_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<UserProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<UserType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Csr {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub csr: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kty: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CsrMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<CsrMetadataSubject>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subjectAltNames"
    )]
    pub subject_alt_names: Option<CsrMetadataSubjectAltNames>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CsrMetadataSubject {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "commonName"
    )]
    pub common_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "countryName"
    )]
    pub country_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "localityName"
    )]
    pub locality_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "organizationName"
    )]
    pub organization_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "organizationalUnitName"
    )]
    pub organizational_unit_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "stateOrProvinceName"
    )]
    pub state_or_province_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CsrMetadataSubjectAltNames {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dnsNames"
    )]
    pub dns_names: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CustomHotpUserFactor {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "factorProfileId"
    )]
    pub factor_profile_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<CustomHotpUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CustomHotpUserFactorProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "sharedSecret"
    )]
    pub shared_secret: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DnsRecord {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expiration: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fqdn: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "recordType"
    )]
    pub record_type: Option<DnsRecordType>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DnsRecordType {
    #[serde(rename = "CNAME")]
    Cname,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsRecordType::Cname => "CNAME",
            DnsRecordType::Txt => "TXT",
            DnsRecordType::Noop => "",
            DnsRecordType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DnsRecordType {
    fn default() -> DnsRecordType {
        DnsRecordType::Noop
    }
}
impl DnsRecordType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DnsRecordType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TrustLevel {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "TRUSTED")]
    Trusted,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrustLevel::Any => "ANY",
            TrustLevel::Trusted => "TRUSTED",
            TrustLevel::Noop => "",
            TrustLevel::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TrustLevel {
    fn default() -> TrustLevel {
        TrustLevel::Noop
    }
}
impl TrustLevel {
    pub fn is_noop(&self) -> bool {
        matches!(self, TrustLevel::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DevicePolicyRuleCondition {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub migrated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<DevicePolicyRuleConditionPlatform>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub rooted: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trustLevel"
    )]
    pub trust_level: Option<TrustLevel>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SupportedMdmFrameworks {
    #[serde(rename = "AFW")]
    Afw,
    #[serde(rename = "NATIVE")]
    Native,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SupportedMdmFrameworks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedMdmFrameworks::Afw => "AFW",
            SupportedMdmFrameworks::Native => "NATIVE",
            SupportedMdmFrameworks::Safe => "SAFE",
            SupportedMdmFrameworks::Noop => "",
            SupportedMdmFrameworks::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SupportedMdmFrameworks {
    fn default() -> SupportedMdmFrameworks {
        SupportedMdmFrameworks::Noop
    }
}
impl SupportedMdmFrameworks {
    pub fn is_noop(&self) -> bool {
        matches!(self, SupportedMdmFrameworks::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Types {
    #[serde(rename = "ANDROID")]
    Android,
    #[serde(rename = "IOS")]
    Ios,
    #[serde(rename = "OSX")]
    Osx,
    #[serde(rename = "WINDOWS")]
    Windows,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Android => "ANDROID",
            Types::Ios => "IOS",
            Types::Osx => "OSX",
            Types::Windows => "WINDOWS",
            Types::Noop => "",
            Types::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Types {
    fn default() -> Types {
        Types::Noop
    }
}
impl Types {
    pub fn is_noop(&self) -> bool {
        matches!(self, Types::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DevicePolicyRuleConditionPlatform {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "supportedMDMFrameworks"
    )]
    pub supported_mdm_frameworks: Vec<SupportedMdmFrameworks>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub types: Vec<Types>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Domain {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "certificateSourceType"
    )]
    pub certificate_source_type: Option<DomainCertificateSourceType>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dnsRecords"
    )]
    pub dns_records: Vec<DnsRecord>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "publicCertificate"
    )]
    pub public_certificate: Option<DomainCertificateMetadata>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "validationStatus"
    )]
    pub validation_status: Option<DomainValidationStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DomainCertificate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub certificate: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "certificateChain"
    )]
    pub certificate_chain: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "privateKey"
    )]
    pub private_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<DomainCertificateType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DomainCertificateMetadata {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expiration: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DomainCertificateSourceType {
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DomainCertificateSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainCertificateSourceType::Manual => "MANUAL",
            DomainCertificateSourceType::Noop => "",
            DomainCertificateSourceType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DomainCertificateSourceType {
    fn default() -> DomainCertificateSourceType {
        DomainCertificateSourceType::Noop
    }
}
impl DomainCertificateSourceType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DomainCertificateSourceType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DomainCertificateType {
    #[serde(rename = "PEM")]
    Pem,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DomainCertificateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainCertificateType::Pem => "PEM",
            DomainCertificateType::Noop => "",
            DomainCertificateType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DomainCertificateType {
    fn default() -> DomainCertificateType {
        DomainCertificateType::Noop
    }
}
impl DomainCertificateType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DomainCertificateType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DomainListResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub domains: Vec<Domain>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DomainValidationStatus {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DomainValidationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainValidationStatus::Completed => "COMPLETED",
            DomainValidationStatus::InProgress => "IN_PROGRESS",
            DomainValidationStatus::NotStarted => "NOT_STARTED",
            DomainValidationStatus::Verified => "VERIFIED",
            DomainValidationStatus::Noop => "",
            DomainValidationStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DomainValidationStatus {
    fn default() -> DomainValidationStatus {
        DomainValidationStatus::Noop
    }
}
impl DomainValidationStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, DomainValidationStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Duration {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unit: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<EmailUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailUserFactorProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EnabledStatus {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EnabledStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnabledStatus::Disabled => "DISABLED",
            EnabledStatus::Enabled => "ENABLED",
            EnabledStatus::Noop => "",
            EnabledStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EnabledStatus {
    fn default() -> EnabledStatus {
        EnabledStatus::Noop
    }
}
impl EnabledStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, EnabledStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum VerificationStatus {
    #[serde(rename = "UNVERIFIED")]
    Unverified,
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationStatus::Unverified => "UNVERIFIED",
            VerificationStatus::Verified => "VERIFIED",
            VerificationStatus::Noop => "",
            VerificationStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for VerificationStatus {
    fn default() -> VerificationStatus {
        VerificationStatus::Noop
    }
}
impl VerificationStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, VerificationStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventHook {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<EventHookChannel>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "createdBy"
    )]
    pub created_by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<EventSubscriptions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verificationStatus"
    )]
    pub verification_status: Option<VerificationStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EventHookChannelType {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EventHookChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventHookChannelType::Http => "HTTP",
            EventHookChannelType::Noop => "",
            EventHookChannelType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EventHookChannelType {
    fn default() -> EventHookChannelType {
        EventHookChannelType::Noop
    }
}
impl EventHookChannelType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EventHookChannelType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventHookChannel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<EventHookChannelConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<EventHookChannelType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventHookChannelConfig {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authScheme"
    )]
    pub auth_scheme: Option<EventHookChannelConfigAuthScheme>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub headers: Vec<EventHookChannelConfigHeader>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uri: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventHookChannelConfigAuthScheme {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<EventHookChannelConfigAuthSchemeType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EventHookChannelConfigAuthSchemeType {
    #[serde(rename = "HEADER")]
    Header,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EventHookChannelConfigAuthSchemeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventHookChannelConfigAuthSchemeType::Header => "HEADER",
            EventHookChannelConfigAuthSchemeType::Noop => "",
            EventHookChannelConfigAuthSchemeType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EventHookChannelConfigAuthSchemeType {
    fn default() -> EventHookChannelConfigAuthSchemeType {
        EventHookChannelConfigAuthSchemeType::Noop
    }
}
impl EventHookChannelConfigAuthSchemeType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EventHookChannelConfigAuthSchemeType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventHookChannelConfigHeader {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EventSubscriptionsType {
    #[serde(rename = "EVENT_TYPE")]
    EventType,
    #[serde(rename = "FLOW_EVENT")]
    FlowEvent,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EventSubscriptionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventSubscriptionsType::EventType => "EVENT_TYPE",
            EventSubscriptionsType::FlowEvent => "FLOW_EVENT",
            EventSubscriptionsType::Noop => "",
            EventSubscriptionsType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EventSubscriptionsType {
    fn default() -> EventSubscriptionsType {
        EventSubscriptionsType::Noop
    }
}
impl EventSubscriptionsType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EventSubscriptionsType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventSubscriptions {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<EventSubscriptionsType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FactorProvider {
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "DUO")]
    Duo,
    #[serde(rename = "FIDO")]
    Fido,
    #[serde(rename = "GOOGLE")]
    Google,
    #[serde(rename = "OKTA")]
    Okta,
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "SYMANTEC")]
    Symantec,
    #[serde(rename = "YUBICO")]
    Yubico,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FactorProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorProvider::Custom => "CUSTOM",
            FactorProvider::Duo => "DUO",
            FactorProvider::Fido => "FIDO",
            FactorProvider::Google => "GOOGLE",
            FactorProvider::Okta => "OKTA",
            FactorProvider::Rsa => "RSA",
            FactorProvider::Symantec => "SYMANTEC",
            FactorProvider::Yubico => "YUBICO",
            FactorProvider::Noop => "",
            FactorProvider::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FactorProvider {
    fn default() -> FactorProvider {
        FactorProvider::Noop
    }
}
impl FactorProvider {
    pub fn is_noop(&self) -> bool {
        matches!(self, FactorProvider::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FactorResultType {
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "CHALLENGE")]
    Challenge,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PASSCODE_REPLAYED")]
    PasscodeReplayed,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "TIMEOUT")]
    Timeout,
    #[serde(rename = "TIME_WINDOW_EXCEEDED")]
    TimeWindowExceeded,
    #[serde(rename = "WAITING")]
    Waiting,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FactorResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorResultType::Cancelled => "CANCELLED",
            FactorResultType::Challenge => "CHALLENGE",
            FactorResultType::Error => "ERROR",
            FactorResultType::Failed => "FAILED",
            FactorResultType::PasscodeReplayed => "PASSCODE_REPLAYED",
            FactorResultType::Rejected => "REJECTED",
            FactorResultType::Success => "SUCCESS",
            FactorResultType::Timeout => "TIMEOUT",
            FactorResultType::TimeWindowExceeded => "TIME_WINDOW_EXCEEDED",
            FactorResultType::Waiting => "WAITING",
            FactorResultType::Noop => "",
            FactorResultType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FactorResultType {
    fn default() -> FactorResultType {
        FactorResultType::Noop
    }
}
impl FactorResultType {
    pub fn is_noop(&self) -> bool {
        matches!(self, FactorResultType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FactorStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ENROLLED")]
    Enrolled,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "NOT_SETUP")]
    NotSetup,
    #[serde(rename = "PENDING_ACTIVATION")]
    PendingActivation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FactorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorStatus::Active => "ACTIVE",
            FactorStatus::Disabled => "DISABLED",
            FactorStatus::Enrolled => "ENROLLED",
            FactorStatus::Expired => "EXPIRED",
            FactorStatus::Inactive => "INACTIVE",
            FactorStatus::NotSetup => "NOT_SETUP",
            FactorStatus::PendingActivation => "PENDING_ACTIVATION",
            FactorStatus::Noop => "",
            FactorStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FactorStatus {
    fn default() -> FactorStatus {
        FactorStatus::Noop
    }
}
impl FactorStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, FactorStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FactorType {
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "hotp")]
    Hotp,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "question")]
    Question,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "token:hardware")]
    TokenHardware,
    #[serde(rename = "token:hotp")]
    TokenHotp,
    #[serde(rename = "token:software:totp")]
    TokenSoftwareTotp,
    #[serde(rename = "u2f")]
    U2F,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "webauthn")]
    Webauthn,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FactorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorType::Call => "call",
            FactorType::Email => "email",
            FactorType::Hotp => "hotp",
            FactorType::Push => "push",
            FactorType::Question => "question",
            FactorType::Sms => "sms",
            FactorType::Token => "token",
            FactorType::TokenHardware => "token:hardware",
            FactorType::TokenHotp => "token:hotp",
            FactorType::TokenSoftwareTotp => "token:software:totp",
            FactorType::U2F => "u2f",
            FactorType::Web => "web",
            FactorType::Webauthn => "webauthn",
            FactorType::Noop => "",
            FactorType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FactorType {
    fn default() -> FactorType {
        FactorType::Noop
    }
}
impl FactorType {
    pub fn is_noop(&self) -> bool {
        matches!(self, FactorType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Feature {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<FeatureStage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnabledStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<FeatureType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FeatureStage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<FeatureStageState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<FeatureStageValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FeatureStageState {
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FeatureStageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureStageState::Closed => "CLOSED",
            FeatureStageState::Open => "OPEN",
            FeatureStageState::Noop => "",
            FeatureStageState::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FeatureStageState {
    fn default() -> FeatureStageState {
        FeatureStageState::Noop
    }
}
impl FeatureStageState {
    pub fn is_noop(&self) -> bool {
        matches!(self, FeatureStageState::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FeatureStageValue {
    #[serde(rename = "BETA")]
    Beta,
    #[serde(rename = "EA")]
    Ea,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FeatureStageValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureStageValue::Beta => "BETA",
            FeatureStageValue::Ea => "EA",
            FeatureStageValue::Noop => "",
            FeatureStageValue::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FeatureStageValue {
    fn default() -> FeatureStageValue {
        FeatureStageValue::Noop
    }
}
impl FeatureStageValue {
    pub fn is_noop(&self) -> bool {
        matches!(self, FeatureStageValue::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FeatureType {
    #[serde(rename = "self-service")]
    SelfService,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FeatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureType::SelfService => "self-service",
            FeatureType::Noop => "",
            FeatureType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FeatureType {
    fn default() -> FeatureType {
        FeatureType::Noop
    }
}
impl FeatureType {
    pub fn is_noop(&self) -> bool {
        matches!(self, FeatureType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ResetPasswordToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "resetPasswordUrl"
    )]
    pub reset_password_url: String,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Group {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastMembershipUpdated"
    )]
    pub last_membership_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "objectClass"
    )]
    pub object_class: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<GroupProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<GroupType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<GroupRuleAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<GroupRuleConditions>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GroupRuleStatus>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupRuleAction {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "assignUserToGroups"
    )]
    pub assign_user_to_groups: Option<GroupRuleAssignment>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupRuleConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<GroupRuleExpression>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub people: Option<GroupRulePeopleCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupRuleExpression {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupRuleAssignment {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "groupIds"
    )]
    pub group_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub exclude: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupRulePeopleCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<GroupRuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<GroupRuleCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GroupRuleStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GroupRuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupRuleStatus::Active => "ACTIVE",
            GroupRuleStatus::Inactive => "INACTIVE",
            GroupRuleStatus::Invalid => "INVALID",
            GroupRuleStatus::Noop => "",
            GroupRuleStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GroupRuleStatus {
    fn default() -> GroupRuleStatus {
        GroupRuleStatus::Noop
    }
}
impl GroupRuleStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, GroupRuleStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GroupType {
    #[serde(rename = "APP_GROUP")]
    AppGroup,
    #[serde(rename = "BUILT_IN")]
    BuiltIn,
    #[serde(rename = "OKTA_GROUP")]
    OktaGroup,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupType::AppGroup => "APP_GROUP",
            GroupType::BuiltIn => "BUILT_IN",
            GroupType::OktaGroup => "OKTA_GROUP",
            GroupType::Noop => "",
            GroupType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GroupType {
    fn default() -> GroupType {
        GroupType::Noop
    }
}
impl GroupType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GroupType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct HardwareUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<WebUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebUserFactorProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "credentialId"
    )]
    pub credential_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum IdentityProviderIssuerMode {
    #[serde(rename = "CUSTOM_URL_DOMAIN")]
    CustomUrlDomain,
    #[serde(rename = "ORG_URL")]
    OrgUrl,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IdentityProviderIssuerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentityProviderIssuerMode::CustomUrlDomain => "CUSTOM_URL_DOMAIN",
            IdentityProviderIssuerMode::OrgUrl => "ORG_URL",
            IdentityProviderIssuerMode::Noop => "",
            IdentityProviderIssuerMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IdentityProviderIssuerMode {
    fn default() -> IdentityProviderIssuerMode {
        IdentityProviderIssuerMode::Noop
    }
}
impl IdentityProviderIssuerMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, IdentityProviderIssuerMode::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum IdentityProviderType {
    #[serde(rename = "AgentlessDSSO")]
    AgentlessDsso,
    #[serde(rename = "FACEBOOK")]
    Facebook,
    #[serde(rename = "GOOGLE")]
    Google,
    #[serde(rename = "IWA")]
    Iwa,
    #[serde(rename = "LINKEDIN")]
    Linkedin,
    #[serde(rename = "MICROSOFT")]
    Microsoft,
    #[serde(rename = "OIDC")]
    Oidc,
    #[serde(rename = "OKTA")]
    Okta,
    #[serde(rename = "SAML2")]
    Saml2,
    #[serde(rename = "X509")]
    X509,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IdentityProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentityProviderType::AgentlessDsso => "AgentlessDSSO",
            IdentityProviderType::Facebook => "FACEBOOK",
            IdentityProviderType::Google => "GOOGLE",
            IdentityProviderType::Iwa => "IWA",
            IdentityProviderType::Linkedin => "LINKEDIN",
            IdentityProviderType::Microsoft => "MICROSOFT",
            IdentityProviderType::Oidc => "OIDC",
            IdentityProviderType::Okta => "OKTA",
            IdentityProviderType::Saml2 => "SAML2",
            IdentityProviderType::X509 => "X509",
            IdentityProviderType::Noop => "",
            IdentityProviderType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IdentityProviderType {
    fn default() -> IdentityProviderType {
        IdentityProviderType::Noop
    }
}
impl IdentityProviderType {
    pub fn is_noop(&self) -> bool {
        matches!(self, IdentityProviderType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IdentityProvider {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "issuerMode"
    )]
    pub issuer_mode: Option<IdentityProviderIssuerMode>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<IdentityProviderPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<IdentityProviderType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IdentityProviderApplicationUser {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "externalId"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Links>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IdentityProviderCredentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<IdentityProviderCredentialsClient>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing: Option<IdentityProviderCredentialsSigning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust: Option<IdentityProviderCredentialsTrust>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IdentityProviderCredentialsClient {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IdentityProviderCredentialsSigning {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kid: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Revocation {
    #[serde(rename = "CRL")]
    Crl,
    #[serde(rename = "DELTA_CRL")]
    DeltaCrl,
    #[serde(rename = "OCSP")]
    Ocsp,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Revocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Revocation::Crl => "CRL",
            Revocation::DeltaCrl => "DELTA_CRL",
            Revocation::Ocsp => "OCSP",
            Revocation::Noop => "",
            Revocation::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Revocation {
    fn default() -> Revocation {
        Revocation::Noop
    }
}
impl Revocation {
    pub fn is_noop(&self) -> bool {
        matches!(self, Revocation::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IdentityProviderCredentialsTrust {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub audience: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issuer: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation: Option<Revocation>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "revocationCacheLifetime"
    )]
    pub revocation_cache_lifetime: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IdentityProviderPolicy {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "accountLink"
    )]
    pub account_link: Option<PolicyAccountLink>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxClockSkew"
    )]
    pub max_clock_skew: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning: Option<Provisioning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<PolicySubject>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Provider {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "OKTA")]
    Okta,
    #[serde(rename = "SPECIFIC_IDP")]
    SpecificIdp,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Any => "ANY",
            Provider::Okta => "OKTA",
            Provider::SpecificIdp => "SPECIFIC_IDP",
            Provider::Noop => "",
            Provider::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Provider {
    fn default() -> Provider {
        Provider::Noop
    }
}
impl Provider {
    pub fn is_noop(&self) -> bool {
        matches!(self, Provider::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IdentityProviderPolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "idpIds"
    )]
    pub idp_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InlineHook {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<InlineHookChannel>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<InlineHookType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InlineHookChannel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<InlineHookChannelConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<EventHookChannelType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InlineHookChannelConfig {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authScheme"
    )]
    pub auth_scheme: Option<InlineHookChannelConfigAuthScheme>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub headers: Vec<InlineHookChannelConfigHeaders>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub method: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uri: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InlineHookChannelConfigAuthScheme {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InlineHookChannelConfigHeaders {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InlineHookResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub commands: Vec<InlineHookResponseCommands>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InlineHookResponseCommandValue {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub op: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InlineHookResponseCommands {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub value: Vec<InlineHookResponseCommandValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum InlineHookType {
    #[serde(rename = "com.okta.import.transform")]
    ComOktaImportTransform,
    #[serde(rename = "com.okta.oauth2.tokens.transform")]
    ComOktaOauth2TokensTransform,
    #[serde(rename = "com.okta.saml.tokens.transform")]
    ComOktaSamlTokensTransform,
    #[serde(rename = "com.okta.user.credential.password.import")]
    ComOktaUserCredentialPasswordImport,
    #[serde(rename = "com.okta.user.pre-registration")]
    ComOktaUserPreRegistration,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for InlineHookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineHookType::ComOktaImportTransform => "com.okta.import.transform",
            InlineHookType::ComOktaOauth2TokensTransform => "com.okta.oauth2.tokens.transform",
            InlineHookType::ComOktaSamlTokensTransform => "com.okta.saml.tokens.transform",
            InlineHookType::ComOktaUserCredentialPasswordImport => {
                "com.okta.user.credential.password.import"
            }
            InlineHookType::ComOktaUserPreRegistration => "com.okta.user.pre-registration",
            InlineHookType::Noop => "",
            InlineHookType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for InlineHookType {
    fn default() -> InlineHookType {
        InlineHookType::Noop
    }
}
impl InlineHookType {
    pub fn is_noop(&self) -> bool {
        matches!(self, InlineHookType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IonField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<IonForm>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub mutable: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub required: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub secret: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Links>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub visible: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IonForm {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub accepts: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub href: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub method: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub produces: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub refresh: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub rel: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "relatesTo"
    )]
    pub relates_to: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub value: Vec<IonField>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct JsonWebKey {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub alg: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub e: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "expiresAt"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub key_ops: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kid: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kty: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub n: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "use"
    )]
    pub use_: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "x5c"
    )]
    pub x_5c: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "x5t"
    )]
    pub x_5t: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "x5t#S256"
    )]
    pub x_5t_s256: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "x5u"
    )]
    pub x_5u: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct JwkUse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "use")]
    pub use_: Option<Use>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LifecycleExpirationPolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "lifecycleStatus"
    )]
    pub lifecycle_status: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unit: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LinkedObject {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associated: Option<LinkedObjectDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<LinkedObjectDetails>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LinkedObjectDetails {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<LinkedObjectDetailsType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LinkedObjectDetailsType {
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LinkedObjectDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkedObjectDetailsType::User => "USER",
            LinkedObjectDetailsType::Noop => "",
            LinkedObjectDetailsType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LinkedObjectDetailsType {
    fn default() -> LinkedObjectDetailsType {
        LinkedObjectDetailsType::Noop
    }
}
impl LinkedObjectDetailsType {
    pub fn is_noop(&self) -> bool {
        matches!(self, LinkedObjectDetailsType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogActor {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "alternateId"
    )]
    pub alternate_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogAuthenticationContext {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticationProvider"
    )]
    pub authentication_provider: Option<LogAuthenticationProvider>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "authenticationStep"
    )]
    pub authentication_step: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "credentialProvider"
    )]
    pub credential_provider: Option<LogCredentialProvider>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "credentialType"
    )]
    pub credential_type: Option<LogCredentialType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "externalSessionId"
    )]
    pub external_session_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub interface: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<LogIssuer>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LogAuthenticationProvider {
    #[serde(rename = "ACTIVE_DIRECTORY")]
    ActiveDirectory,
    #[serde(rename = "FACTOR_PROVIDER")]
    FactorProvider,
    #[serde(rename = "FEDERATION")]
    Federation,
    #[serde(rename = "LDAP")]
    Ldap,
    #[serde(rename = "OKTA_AUTHENTICATION_PROVIDER")]
    OktaAuthenticationProvider,
    #[serde(rename = "SOCIAL")]
    Social,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LogAuthenticationProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogAuthenticationProvider::ActiveDirectory => "ACTIVE_DIRECTORY",
            LogAuthenticationProvider::FactorProvider => "FACTOR_PROVIDER",
            LogAuthenticationProvider::Federation => "FEDERATION",
            LogAuthenticationProvider::Ldap => "LDAP",
            LogAuthenticationProvider::OktaAuthenticationProvider => "OKTA_AUTHENTICATION_PROVIDER",
            LogAuthenticationProvider::Social => "SOCIAL",
            LogAuthenticationProvider::Noop => "",
            LogAuthenticationProvider::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LogAuthenticationProvider {
    fn default() -> LogAuthenticationProvider {
        LogAuthenticationProvider::Noop
    }
}
impl LogAuthenticationProvider {
    pub fn is_noop(&self) -> bool {
        matches!(self, LogAuthenticationProvider::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogClient {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub device: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "geographicalContext"
    )]
    pub geographical_context: Option<LogGeographicalContext>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ipAddress"
    )]
    pub ip_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userAgent")]
    pub user_agent: Option<LogUserAgent>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zone: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LogCredentialProvider {
    #[serde(rename = "DUO")]
    Duo,
    #[serde(rename = "GOOGLE")]
    Google,
    #[serde(rename = "OKTA_AUTHENTICATION_PROVIDER")]
    OktaAuthenticationProvider,
    #[serde(rename = "OKTA_CREDENTIAL_PROVIDER")]
    OktaCredentialProvider,
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "SYMANTEC")]
    Symantec,
    #[serde(rename = "YUBIKEY")]
    Yubikey,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LogCredentialProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogCredentialProvider::Duo => "DUO",
            LogCredentialProvider::Google => "GOOGLE",
            LogCredentialProvider::OktaAuthenticationProvider => "OKTA_AUTHENTICATION_PROVIDER",
            LogCredentialProvider::OktaCredentialProvider => "OKTA_CREDENTIAL_PROVIDER",
            LogCredentialProvider::Rsa => "RSA",
            LogCredentialProvider::Symantec => "SYMANTEC",
            LogCredentialProvider::Yubikey => "YUBIKEY",
            LogCredentialProvider::Noop => "",
            LogCredentialProvider::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LogCredentialProvider {
    fn default() -> LogCredentialProvider {
        LogCredentialProvider::Noop
    }
}
impl LogCredentialProvider {
    pub fn is_noop(&self) -> bool {
        matches!(self, LogCredentialProvider::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LogCredentialType {
    #[serde(rename = "ASSERTION")]
    Assertion,
    #[serde(rename = "EMAIL")]
    Email,
    #[serde(rename = "IWA")]
    Iwa,
    #[serde(rename = "JWT")]
    Jwt,
    #[serde(rename = "OAUTH2")]
    Oauth2,
    #[serde(rename = "OTP")]
    Otp,
    #[serde(rename = "PASSWORD")]
    Password,
    #[serde(rename = "SMS")]
    Sms,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LogCredentialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogCredentialType::Assertion => "ASSERTION",
            LogCredentialType::Email => "EMAIL",
            LogCredentialType::Iwa => "IWA",
            LogCredentialType::Jwt => "JWT",
            LogCredentialType::Oauth2 => "OAUTH2",
            LogCredentialType::Otp => "OTP",
            LogCredentialType::Password => "PASSWORD",
            LogCredentialType::Sms => "SMS",
            LogCredentialType::Noop => "",
            LogCredentialType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LogCredentialType {
    fn default() -> LogCredentialType {
        LogCredentialType::Noop
    }
}
impl LogCredentialType {
    pub fn is_noop(&self) -> bool {
        matches!(self, LogCredentialType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogDebugContext {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "debugData")]
    pub debug_data: Option<Links>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<LogActor>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticationContext"
    )]
    pub authentication_context: Option<LogAuthenticationContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<LogClient>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "debugContext"
    )]
    pub debug_context: Option<LogDebugContext>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "displayMessage"
    )]
    pub display_message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "eventType"
    )]
    pub event_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "legacyEventType"
    )]
    pub legacy_event_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<LogOutcome>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub published: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<LogRequest>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "securityContext"
    )]
    pub security_context: Option<LogSecurityContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<LogSeverity>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub target: Vec<LogTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<LogTransaction>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogGeographicalContext {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geolocation: Option<LogGeolocation>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "postalCode"
    )]
    pub postal_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogGeolocation {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub lat: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub lon: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogIpAddress {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "geographicalContext"
    )]
    pub geographical_context: Option<LogGeographicalContext>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub source: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogIssuer {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogOutcome {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub result: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogRequest {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "ipChain"
    )]
    pub ip_chain: Vec<LogIpAddress>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogSecurityContext {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "asNumber"
    )]
    pub as_number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "asOrg"
    )]
    pub as_org: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "isProxy"
    )]
    pub is_proxy: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub isp: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LogSeverity {
    #[serde(rename = "DEBUG")]
    Debug,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "WARN")]
    Warn,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LogSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogSeverity::Debug => "DEBUG",
            LogSeverity::Error => "ERROR",
            LogSeverity::Info => "INFO",
            LogSeverity::Warn => "WARN",
            LogSeverity::Noop => "",
            LogSeverity::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LogSeverity {
    fn default() -> LogSeverity {
        LogSeverity::Noop
    }
}
impl LogSeverity {
    pub fn is_noop(&self) -> bool {
        matches!(self, LogSeverity::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogTarget {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "alternateId"
    )]
    pub alternate_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "detailEntry"
    )]
    pub detail_entry: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogTransaction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LogUserAgent {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub browser: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub os: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "rawUserAgent"
    )]
    pub raw_user_agent: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Enrollment {
    #[serde(rename = "ANY_OR_NONE")]
    AnyOrNone,
    #[serde(rename = "OMM")]
    Omm,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Enrollment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Enrollment::AnyOrNone => "ANY_OR_NONE",
            Enrollment::Omm => "OMM",
            Enrollment::Noop => "",
            Enrollment::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Enrollment {
    fn default() -> Enrollment {
        Enrollment::Noop
    }
}
impl Enrollment {
    pub fn is_noop(&self) -> bool {
        matches!(self, Enrollment::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MdmEnrollmentPolicyRuleCondition {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "blockNonSafeAndroid"
    )]
    pub block_non_safe_android: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enrollment: Option<Enrollment>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NetworkZone {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub asns: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub gateways: Vec<NetworkZoneAddress>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub locations: Vec<NetworkZoneLocation>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub proxies: Vec<NetworkZoneAddress>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "proxyType"
    )]
    pub proxy_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub system: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<NetworkZoneType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<NetworkZoneUsage>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NetworkZoneAddress {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<NetworkZoneAddressType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NetworkZoneAddressType {
    #[serde(rename = "CIDR")]
    Cidr,
    #[serde(rename = "RANGE")]
    Range,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NetworkZoneAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkZoneAddressType::Cidr => "CIDR",
            NetworkZoneAddressType::Range => "RANGE",
            NetworkZoneAddressType::Noop => "",
            NetworkZoneAddressType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NetworkZoneAddressType {
    fn default() -> NetworkZoneAddressType {
        NetworkZoneAddressType::Noop
    }
}
impl NetworkZoneAddressType {
    pub fn is_noop(&self) -> bool {
        matches!(self, NetworkZoneAddressType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NetworkZoneLocation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub region: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NetworkZoneType {
    #[serde(rename = "DYNAMIC")]
    Dynamic,
    #[serde(rename = "IP")]
    Ip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NetworkZoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkZoneType::Dynamic => "DYNAMIC",
            NetworkZoneType::Ip => "IP",
            NetworkZoneType::Noop => "",
            NetworkZoneType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NetworkZoneType {
    fn default() -> NetworkZoneType {
        NetworkZoneType::Noop
    }
}
impl NetworkZoneType {
    pub fn is_noop(&self) -> bool {
        matches!(self, NetworkZoneType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NetworkZoneUsage {
    #[serde(rename = "BLOCKLIST")]
    Blocklist,
    #[serde(rename = "POLICY")]
    Policy,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NetworkZoneUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkZoneUsage::Blocklist => "BLOCKLIST",
            NetworkZoneUsage::Policy => "POLICY",
            NetworkZoneUsage::Noop => "",
            NetworkZoneUsage::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NetworkZoneUsage {
    fn default() -> NetworkZoneUsage {
        NetworkZoneUsage::Noop
    }
}
impl NetworkZoneUsage {
    pub fn is_noop(&self) -> bool {
        matches!(self, NetworkZoneUsage::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuth2Actor {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ClaimType {
    #[serde(rename = "IDENTITY")]
    Identity,
    #[serde(rename = "RESOURCE")]
    Resource,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ClaimType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClaimType::Identity => "IDENTITY",
            ClaimType::Resource => "RESOURCE",
            ClaimType::Noop => "",
            ClaimType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ClaimType {
    fn default() -> ClaimType {
        ClaimType::Noop
    }
}
impl ClaimType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ClaimType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GroupFilterType {
    #[serde(rename = "CONTAINS")]
    Contains,
    #[serde(rename = "EQUALS")]
    Equals,
    #[serde(rename = "REGEX")]
    Regex,
    #[serde(rename = "STARTS_WITH")]
    StartsWith,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GroupFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupFilterType::Contains => "CONTAINS",
            GroupFilterType::Equals => "EQUALS",
            GroupFilterType::Regex => "REGEX",
            GroupFilterType::StartsWith => "STARTS_WITH",
            GroupFilterType::Noop => "",
            GroupFilterType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GroupFilterType {
    fn default() -> GroupFilterType {
        GroupFilterType::Noop
    }
}
impl GroupFilterType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GroupFilterType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ValueType {
    #[serde(rename = "EXPRESSION")]
    Expression,
    #[serde(rename = "GROUPS")]
    Groups,
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Expression => "EXPRESSION",
            ValueType::Groups => "GROUPS",
            ValueType::System => "SYSTEM",
            ValueType::Noop => "",
            ValueType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ValueType {
    fn default() -> ValueType {
        ValueType::Noop
    }
}
impl ValueType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ValueType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuth2Claim {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "alwaysIncludeInToken"
    )]
    pub always_include_in_token: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "claimType")]
    pub claim_type: Option<ClaimType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<OAuth2ClaimConditions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_filter_type: Option<GroupFilterType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub system: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueType")]
    pub value_type: Option<ValueType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuth2ClaimConditions {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuth2Client {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_uri: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub logo_uri: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OAuth2RefreshTokenStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OAuth2RefreshTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuth2RefreshTokenStatus::Active => "ACTIVE",
            OAuth2RefreshTokenStatus::Revoked => "REVOKED",
            OAuth2RefreshTokenStatus::Noop => "",
            OAuth2RefreshTokenStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OAuth2RefreshTokenStatus {
    fn default() -> OAuth2RefreshTokenStatus {
        OAuth2RefreshTokenStatus::Noop
    }
}
impl OAuth2RefreshTokenStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, OAuth2RefreshTokenStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuth2RefreshToken {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "clientId"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdBy")]
    pub created_by: Option<OAuth2Actor>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "expiresAt"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issuer: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OAuth2RefreshTokenStatus>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "userId"
    )]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Consent {
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "IMPLICIT")]
    Implicit,
    #[serde(rename = "REQUIRED")]
    Required,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Consent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Consent::Admin => "ADMIN",
            Consent::Implicit => "IMPLICIT",
            Consent::Required => "REQUIRED",
            Consent::Noop => "",
            Consent::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Consent {
    fn default() -> Consent {
        Consent::Noop
    }
}
impl Consent {
    pub fn is_noop(&self) -> bool {
        matches!(self, Consent::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MetadataPublish {
    #[serde(rename = "ALL_CLIENTS")]
    AllClients,
    #[serde(rename = "NO_CLIENTS")]
    NoClients,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MetadataPublish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetadataPublish::AllClients => "ALL_CLIENTS",
            MetadataPublish::NoClients => "NO_CLIENTS",
            MetadataPublish::Noop => "",
            MetadataPublish::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MetadataPublish {
    fn default() -> MetadataPublish {
        MetadataPublish::Noop
    }
}
impl MetadataPublish {
    pub fn is_noop(&self) -> bool {
        matches!(self, MetadataPublish::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuth2Scope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent: Option<Consent>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "metadataPublish"
    )]
    pub metadata_publish: Option<MetadataPublish>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub system: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuth2ScopeConsentGrant {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "clientId"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdBy")]
    pub created_by: Option<OAuth2Actor>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issuer: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "scopeId"
    )]
    pub scope_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<OAuth2ScopeConsentGrantSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OAuth2RefreshTokenStatus>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "userId"
    )]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OAuth2ScopeConsentGrantSource {
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "END_USER")]
    EndUser,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OAuth2ScopeConsentGrantSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuth2ScopeConsentGrantSource::Admin => "ADMIN",
            OAuth2ScopeConsentGrantSource::EndUser => "END_USER",
            OAuth2ScopeConsentGrantSource::Noop => "",
            OAuth2ScopeConsentGrantSource::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OAuth2ScopeConsentGrantSource {
    fn default() -> OAuth2ScopeConsentGrantSource {
        OAuth2ScopeConsentGrantSource::Noop
    }
}
impl OAuth2ScopeConsentGrantSource {
    pub fn is_noop(&self) -> bool {
        matches!(self, OAuth2ScopeConsentGrantSource::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuth2ScopesMediationPolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuth2Token {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "clientId"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "expiresAt"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issuer: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OAuth2RefreshTokenStatus>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "userId"
    )]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OAuthApplicationCredentials {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "oauthClient"
    )]
    pub oauth_client: Option<ApplicationCredentialsOAuthClient>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OAuthEndpointAuthenticationMethod {
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
    #[serde(rename = "client_secret_jwt")]
    ClientSecretJwt,
    #[serde(rename = "client_secret_post")]
    ClientSecretPost,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "private_key_jwt")]
    PrivateKeyJwt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OAuthEndpointAuthenticationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthEndpointAuthenticationMethod::ClientSecretBasic => "client_secret_basic",
            OAuthEndpointAuthenticationMethod::ClientSecretJwt => "client_secret_jwt",
            OAuthEndpointAuthenticationMethod::ClientSecretPost => "client_secret_post",
            OAuthEndpointAuthenticationMethod::None => "none",
            OAuthEndpointAuthenticationMethod::PrivateKeyJwt => "private_key_jwt",
            OAuthEndpointAuthenticationMethod::Noop => "",
            OAuthEndpointAuthenticationMethod::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OAuthEndpointAuthenticationMethod {
    fn default() -> OAuthEndpointAuthenticationMethod {
        OAuthEndpointAuthenticationMethod::Noop
    }
}
impl OAuthEndpointAuthenticationMethod {
    pub fn is_noop(&self) -> bool {
        matches!(self, OAuthEndpointAuthenticationMethod::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OAuthGrantType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "client_credentials")]
    ClientCredentials,
    #[serde(rename = "implicit")]
    Implicit,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "refresh_token")]
    RefreshToken,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OAuthGrantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthGrantType::AuthorizationCode => "authorization_code",
            OAuthGrantType::ClientCredentials => "client_credentials",
            OAuthGrantType::Implicit => "implicit",
            OAuthGrantType::Password => "password",
            OAuthGrantType::RefreshToken => "refresh_token",
            OAuthGrantType::Noop => "",
            OAuthGrantType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OAuthGrantType {
    fn default() -> OAuthGrantType {
        OAuthGrantType::Noop
    }
}
impl OAuthGrantType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OAuthGrantType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OAuthResponseType {
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "id_token")]
    IdToken,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OAuthResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthResponseType::Code => "code",
            OAuthResponseType::IdToken => "id_token",
            OAuthResponseType::Token => "token",
            OAuthResponseType::Noop => "",
            OAuthResponseType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OAuthResponseType {
    fn default() -> OAuthResponseType {
        OAuthResponseType::Noop
    }
}
impl OAuthResponseType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OAuthResponseType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OktaSignOnPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<OktaSignOnPolicyConditions>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OktaSignOnPolicyConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub people: Option<PolicyPeopleCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OktaSignOnPolicyRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<OktaSignOnPolicyRuleActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<OktaSignOnPolicyRuleConditions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OktaSignOnPolicyRuleActions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signon: Option<OktaSignOnPolicyRuleSignonActions>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OktaSignOnPolicyRuleConditions {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authContext"
    )]
    pub auth_context: Option<PolicyRuleAuthContextCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<PolicyNetworkCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub people: Option<PolicyPeopleCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Access {
    #[serde(rename = "ALLOW")]
    Allow,
    #[serde(rename = "DENY")]
    Deny,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Access::Allow => "ALLOW",
            Access::Deny => "DENY",
            Access::Noop => "",
            Access::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Access {
    fn default() -> Access {
        Access::Noop
    }
}
impl Access {
    pub fn is_noop(&self) -> bool {
        matches!(self, Access::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FactorPromptMode {
    #[serde(rename = "ALWAYS")]
    Always,
    #[serde(rename = "DEVICE")]
    Device,
    #[serde(rename = "SESSION")]
    Session,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FactorPromptMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorPromptMode::Always => "ALWAYS",
            FactorPromptMode::Device => "DEVICE",
            FactorPromptMode::Session => "SESSION",
            FactorPromptMode::Noop => "",
            FactorPromptMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FactorPromptMode {
    fn default() -> FactorPromptMode {
        FactorPromptMode::Noop
    }
}
impl FactorPromptMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, FactorPromptMode::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OktaSignOnPolicyRuleSignonActions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<Access>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "factorLifetime"
    )]
    pub factor_lifetime: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "factorPromptMode"
    )]
    pub factor_prompt_mode: Option<FactorPromptMode>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "rememberDeviceByDefault"
    )]
    pub remember_device_by_default: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "requireFactor"
    )]
    pub require_factor: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<OktaSignOnPolicyRuleSignonSessionActions>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OktaSignOnPolicyRuleSignonSessionActions {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxSessionIdleMinutes"
    )]
    pub max_session_idle_minutes: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxSessionLifetimeMinutes"
    )]
    pub max_session_lifetime_minutes: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "usePersistentCookie"
    )]
    pub use_persistent_cookie: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenConnectApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<OAuthApplicationCredentials>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<OpenConnectApplicationSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OpenConnectApplicationConsentMethod {
    #[serde(rename = "REQUIRED")]
    Required,
    #[serde(rename = "TRUSTED")]
    Trusted,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OpenConnectApplicationConsentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenConnectApplicationConsentMethod::Required => "REQUIRED",
            OpenConnectApplicationConsentMethod::Trusted => "TRUSTED",
            OpenConnectApplicationConsentMethod::Noop => "",
            OpenConnectApplicationConsentMethod::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OpenConnectApplicationConsentMethod {
    fn default() -> OpenConnectApplicationConsentMethod {
        OpenConnectApplicationConsentMethod::Noop
    }
}
impl OpenConnectApplicationConsentMethod {
    pub fn is_noop(&self) -> bool {
        matches!(self, OpenConnectApplicationConsentMethod::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenConnectApplicationIdpInitiatedLogin {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub default_scope: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mode: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenConnectApplicationSettings {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "oauthClient"
    )]
    pub oauth_client: Option<OpenConnectApplicationSettingsClient>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenConnectApplicationSettingsClient {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<OpenConnectApplicationType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_method: Option<OpenConnectApplicationConsentMethod>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub grant_types: Vec<OAuthGrantType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idp_initiated_login: Option<OpenConnectApplicationIdpInitiatedLogin>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub initiate_login_uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer_mode: Option<IssuerMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwks: Option<OpenConnectApplicationSettingsClientKeys>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub logo_uri: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub policy_uri: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub post_logout_redirect_uris: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub redirect_uris: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<OpenConnectApplicationSettingsRefreshToken>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub response_types: Vec<OAuthResponseType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tos_uri: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub wildcard_redirect: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenConnectApplicationSettingsClientKeys {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub keys: Vec<JsonWebKey>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenConnectApplicationSettingsRefreshToken {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub leeway: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_type: Option<OpenConnectRefreshTokenRotationType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OpenConnectApplicationType {
    #[serde(rename = "browser")]
    Browser,
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OpenConnectApplicationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenConnectApplicationType::Browser => "browser",
            OpenConnectApplicationType::Native => "native",
            OpenConnectApplicationType::Service => "service",
            OpenConnectApplicationType::Web => "web",
            OpenConnectApplicationType::Noop => "",
            OpenConnectApplicationType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OpenConnectApplicationType {
    fn default() -> OpenConnectApplicationType {
        OpenConnectApplicationType::Noop
    }
}
impl OpenConnectApplicationType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OpenConnectApplicationType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OpenConnectRefreshTokenRotationType {
    #[serde(rename = "rotate")]
    Rotate,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OpenConnectRefreshTokenRotationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenConnectRefreshTokenRotationType::Rotate => "rotate",
            OpenConnectRefreshTokenRotationType::Static => "static",
            OpenConnectRefreshTokenRotationType::Noop => "",
            OpenConnectRefreshTokenRotationType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OpenConnectRefreshTokenRotationType {
    fn default() -> OpenConnectRefreshTokenRotationType {
        OpenConnectRefreshTokenRotationType::Noop
    }
}
impl OpenConnectRefreshTokenRotationType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OpenConnectRefreshTokenRotationType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<PasswordCredentialHash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hook: Option<PasswordCredentialHook>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordCredentialHash {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<PasswordCredentialHashAlgorithm>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub salt: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "saltOrder"
    )]
    pub salt_order: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "workFactor"
    )]
    pub work_factor: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PasswordCredentialHashAlgorithm {
    #[serde(rename = "BCRYPT")]
    Bcrypt,
    #[serde(rename = "MD5")]
    Md5,
    #[serde(rename = "SHA-1")]
    ShAminusOne,
    #[serde(rename = "SHA-256")]
    Sha256,
    #[serde(rename = "SHA-512")]
    Sha512,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PasswordCredentialHashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordCredentialHashAlgorithm::Bcrypt => "BCRYPT",
            PasswordCredentialHashAlgorithm::Md5 => "MD5",
            PasswordCredentialHashAlgorithm::ShAminusOne => "SHA-1",
            PasswordCredentialHashAlgorithm::Sha256 => "SHA-256",
            PasswordCredentialHashAlgorithm::Sha512 => "SHA-512",
            PasswordCredentialHashAlgorithm::Noop => "",
            PasswordCredentialHashAlgorithm::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PasswordCredentialHashAlgorithm {
    fn default() -> PasswordCredentialHashAlgorithm {
        PasswordCredentialHashAlgorithm::Noop
    }
}
impl PasswordCredentialHashAlgorithm {
    pub fn is_noop(&self) -> bool {
        matches!(self, PasswordCredentialHashAlgorithm::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordCredentialHook {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordDictionary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common: Option<PasswordDictionaryCommon>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordDictionaryCommon {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub exclude: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<PasswordPolicyConditions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<PasswordPolicySettingsData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PasswordPolicyAuthenticationProviderCondition {
    #[serde(rename = "ACTIVE_DIRECTORY")]
    ActiveDirectory,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "LDAP")]
    Ldap,
    #[serde(rename = "OKTA")]
    Okta,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PasswordPolicyAuthenticationProviderCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordPolicyAuthenticationProviderCondition::ActiveDirectory => "ACTIVE_DIRECTORY",
            PasswordPolicyAuthenticationProviderCondition::Any => "ANY",
            PasswordPolicyAuthenticationProviderCondition::Ldap => "LDAP",
            PasswordPolicyAuthenticationProviderCondition::Okta => "OKTA",
            PasswordPolicyAuthenticationProviderCondition::Noop => "",
            PasswordPolicyAuthenticationProviderCondition::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PasswordPolicyAuthenticationProviderCondition {
    fn default() -> PasswordPolicyAuthenticationProviderCondition {
        PasswordPolicyAuthenticationProviderCondition::Noop
    }
}
impl PasswordPolicyAuthenticationProviderCondition {
    pub fn is_noop(&self) -> bool {
        matches!(self, PasswordPolicyAuthenticationProviderCondition::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyAuthenticationProviderConditionData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<PasswordPolicyAuthenticationProviderCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyConditions {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authProvider"
    )]
    pub auth_provider: Option<PasswordPolicyAuthenticationProviderConditionData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub people: Option<PolicyPeopleCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyDelegationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<PasswordPolicyDelegationSettingsOptions>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyDelegationSettingsOptions {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "skipUnlock"
    )]
    pub skip_unlock: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicySettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<PasswordPolicySettingsAge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complexity: Option<PasswordPolicySettingsComplexity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lockout: Option<PasswordPolicySettingsLockout>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicySettingsAge {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "expireWarnDays"
    )]
    pub expire_warn_days: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "historyCount"
    )]
    pub history_count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxAgeDays"
    )]
    pub max_age_days: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "minAgeMinutes"
    )]
    pub min_age_minutes: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicySettingsComplexity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dictionary: Option<PasswordDictionary>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "excludeAttributes"
    )]
    pub exclude_attributes: Vec<String>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "excludeUsername"
    )]
    pub exclude_username: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "minLength"
    )]
    pub min_length: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "minLowerCase"
    )]
    pub min_lower_case: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "minNumber"
    )]
    pub min_number: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "minSymbol"
    )]
    pub min_symbol: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "minUpperCase"
    )]
    pub min_upper_case: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicySettingsLockout {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "autoUnlockMinutes"
    )]
    pub auto_unlock_minutes: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxAttempts"
    )]
    pub max_attempts: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "showLockoutFailures"
    )]
    pub show_lockout_failures: bool,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "userLockoutNotificationChannels"
    )]
    pub user_lockout_notification_channels: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRecoveryEmail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PasswordPolicyRecoveryEmailProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRecoveryEmailProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "recoveryToken"
    )]
    pub recovery_token: Option<PasswordPolicyRecoveryEmailToken>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRecoveryEmailToken {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "tokenLifetimeMinutes"
    )]
    pub token_lifetime_minutes: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRecoveryFactorSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRecoveryFactors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okta_call: Option<PasswordPolicyRecoveryFactorSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okta_email: Option<PasswordPolicyRecoveryEmail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okta_sms: Option<PasswordPolicyRecoveryFactorSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_question: Option<PasswordPolicyRecoveryQuestion>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRecoveryQuestion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PasswordPolicyRecoveryQuestionProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRecoveryQuestionComplexity {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "minLength"
    )]
    pub min_length: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRecoveryQuestionProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complexity: Option<PasswordPolicyRecoveryQuestionComplexity>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRecoverySettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factors: Option<PasswordPolicyRecoveryFactors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<PasswordPolicyRuleActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<PasswordPolicyRuleConditions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRuleAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<Access>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRuleActions {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "passwordChange"
    )]
    pub password_change: Option<PasswordPolicyRuleAction>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "selfServicePasswordReset"
    )]
    pub self_service_password_reset: Option<PasswordPolicyRuleAction>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "selfServiceUnlock"
    )]
    pub self_service_unlock: Option<PasswordPolicyRuleAction>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicyRuleConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<PolicyNetworkCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub people: Option<PolicyPeopleCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasswordPolicySettingsData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegation: Option<PasswordPolicyDelegationSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordPolicySettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<PasswordPolicyRecoverySettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PlatformConditionEvaluatorType {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "DESKTOP")]
    Desktop,
    #[serde(rename = "MOBILE")]
    Mobile,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PlatformConditionEvaluatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformConditionEvaluatorType::Any => "ANY",
            PlatformConditionEvaluatorType::Desktop => "DESKTOP",
            PlatformConditionEvaluatorType::Mobile => "MOBILE",
            PlatformConditionEvaluatorType::Other => "OTHER",
            PlatformConditionEvaluatorType::Noop => "",
            PlatformConditionEvaluatorType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PlatformConditionEvaluatorType {
    fn default() -> PlatformConditionEvaluatorType {
        PlatformConditionEvaluatorType::Noop
    }
}
impl PlatformConditionEvaluatorType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PlatformConditionEvaluatorType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PlatformConditionEvaluator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<PlatformConditionEvaluatorOperatingSystem>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<PlatformConditionEvaluatorType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PlatformConditionEvaluatorOperatingSystemType {
    #[serde(rename = "ANDROID")]
    Android,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "IOS")]
    Ios,
    #[serde(rename = "OSX")]
    Osx,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "WINDOWS")]
    Windows,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PlatformConditionEvaluatorOperatingSystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformConditionEvaluatorOperatingSystemType::Android => "ANDROID",
            PlatformConditionEvaluatorOperatingSystemType::Any => "ANY",
            PlatformConditionEvaluatorOperatingSystemType::Ios => "IOS",
            PlatformConditionEvaluatorOperatingSystemType::Osx => "OSX",
            PlatformConditionEvaluatorOperatingSystemType::Other => "OTHER",
            PlatformConditionEvaluatorOperatingSystemType::Windows => "WINDOWS",
            PlatformConditionEvaluatorOperatingSystemType::Noop => "",
            PlatformConditionEvaluatorOperatingSystemType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PlatformConditionEvaluatorOperatingSystemType {
    fn default() -> PlatformConditionEvaluatorOperatingSystemType {
        PlatformConditionEvaluatorOperatingSystemType::Noop
    }
}
impl PlatformConditionEvaluatorOperatingSystemType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PlatformConditionEvaluatorOperatingSystemType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PlatformConditionEvaluatorOperatingSystem {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expression: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<PlatformConditionEvaluatorOperatingSystemType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<PlatformConditionEvaluatorOperatingSystemVersion>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MatchType {
    #[serde(rename = "EXPRESSION")]
    Expression,
    #[serde(rename = "SEMVER")]
    Semver,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchType::Expression => "EXPRESSION",
            MatchType::Semver => "SEMVER",
            MatchType::Noop => "",
            MatchType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MatchType {
    fn default() -> MatchType {
        MatchType::Noop
    }
}
impl MatchType {
    pub fn is_noop(&self) -> bool {
        matches!(self, MatchType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PlatformConditionEvaluatorOperatingSystemVersion {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchType")]
    pub match_type: Option<MatchType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PlatformPolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub exclude: Vec<PlatformConditionEvaluator>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<PlatformConditionEvaluator>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Policy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<PolicyRuleConditions>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub system: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<PolicyType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Action {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Auto => "AUTO",
            Action::Disabled => "DISABLED",
            Action::Noop => "",
            Action::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Action {
    fn default() -> Action {
        Action::Noop
    }
}
impl Action {
    pub fn is_noop(&self) -> bool {
        matches!(self, Action::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyAccountLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<PolicyAccountLinkFilter>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyAccountLinkFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<ClientPolicyCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Connection {
    #[serde(rename = "ANYWHERE")]
    Anywhere,
    #[serde(rename = "ZONE")]
    Zone,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connection::Anywhere => "ANYWHERE",
            Connection::Zone => "ZONE",
            Connection::Noop => "",
            Connection::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Connection {
    fn default() -> Connection {
        Connection::Noop
    }
}
impl Connection {
    pub fn is_noop(&self) -> bool {
        matches!(self, Connection::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyNetworkCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub exclude: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyPeopleCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<UserCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<UserCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PolicyRuleType {
    #[serde(rename = "PASSWORD")]
    Password,
    #[serde(rename = "SIGN_ON")]
    SignOn,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PolicyRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyRuleType::Password => "PASSWORD",
            PolicyRuleType::SignOn => "SIGN_ON",
            PolicyRuleType::Noop => "",
            PolicyRuleType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PolicyRuleType {
    fn default() -> PolicyRuleType {
        PolicyRuleType::Noop
    }
}
impl PolicyRuleType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PolicyRuleType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<PolicyRuleActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<PolicyRuleConditions>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub system: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<PolicyRuleType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyRuleActions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enroll: Option<PolicyRuleActionsEnroll>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "passwordChange"
    )]
    pub password_change: Option<PasswordPolicyRuleAction>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "selfServicePasswordReset"
    )]
    pub self_service_password_reset: Option<PasswordPolicyRuleAction>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "selfServiceUnlock"
    )]
    pub self_service_unlock: Option<PasswordPolicyRuleAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signon: Option<OktaSignOnPolicyRuleSignonActions>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyRuleActionsEnroll {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "self")]
    pub self_: Option<PolicyRuleActionsEnrollSelf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PolicyRuleActionsEnrollSelf {
    #[serde(rename = "CHALLENGE")]
    Challenge,
    #[serde(rename = "LOGIN")]
    Login,
    #[serde(rename = "NEVER")]
    Never,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PolicyRuleActionsEnrollSelf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyRuleActionsEnrollSelf::Challenge => "CHALLENGE",
            PolicyRuleActionsEnrollSelf::Login => "LOGIN",
            PolicyRuleActionsEnrollSelf::Never => "NEVER",
            PolicyRuleActionsEnrollSelf::Noop => "",
            PolicyRuleActionsEnrollSelf::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PolicyRuleActionsEnrollSelf {
    fn default() -> PolicyRuleActionsEnrollSelf {
        PolicyRuleActionsEnrollSelf::Noop
    }
}
impl PolicyRuleActionsEnrollSelf {
    pub fn is_noop(&self) -> bool {
        matches!(self, PolicyRuleActionsEnrollSelf::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AuthType {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "RADIUS")]
    Radius,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AuthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthType::Any => "ANY",
            AuthType::Radius => "RADIUS",
            AuthType::Noop => "",
            AuthType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AuthType {
    fn default() -> AuthType {
        AuthType::Noop
    }
}
impl AuthType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AuthType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyRuleAuthContextCondition {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authType")]
    pub auth_type: Option<AuthType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyRuleConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<AppInstancePolicyRuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apps: Option<UserCondition>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authContext"
    )]
    pub auth_context: Option<PolicyRuleAuthContextCondition>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authProvider"
    )]
    pub auth_provider: Option<PasswordPolicyAuthenticationProviderConditionData>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "beforeScheduledAction"
    )]
    pub before_scheduled_action: Option<BeforeScheduledActionPolicyRuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clients: Option<ClientPolicyCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<ContextPolicyRuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<DevicePolicyRuleCondition>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "grantTypes"
    )]
    pub grant_types: Option<ClientPolicyCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<UserCondition>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "identityProvider"
    )]
    pub identity_provider: Option<IdentityProviderPolicyRuleCondition>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mdmEnrollment"
    )]
    pub mdm_enrollment: Option<MdmEnrollmentPolicyRuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<PolicyNetworkCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub people: Option<PolicyPeopleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformPolicyRuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk: Option<RiskPolicyRuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "riskScore")]
    pub risk_score: Option<RiskScorePolicyRuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<OAuth2ScopesMediationPolicyRuleCondition>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userIdentifier"
    )]
    pub user_identifier: Option<UserIdentifierPolicyRuleCondition>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userStatus"
    )]
    pub user_status: Option<UserStatusPolicyRuleCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<UserPolicyRuleCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicySubject {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filter: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub format: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "matchAttribute"
    )]
    pub match_attribute: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchType")]
    pub match_type: Option<PolicySubjectMatchType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userNameTemplate"
    )]
    pub user_name_template: Option<PolicyUserNameTemplate>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PolicySubjectMatchType {
    #[serde(rename = "CUSTOM_ATTRIBUTE")]
    CustomAttribute,
    #[serde(rename = "EMAIL")]
    Email,
    #[serde(rename = "USERNAME")]
    Username,
    #[serde(rename = "USERNAME_OR_EMAIL")]
    UsernameOrEmail,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PolicySubjectMatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicySubjectMatchType::CustomAttribute => "CUSTOM_ATTRIBUTE",
            PolicySubjectMatchType::Email => "EMAIL",
            PolicySubjectMatchType::Username => "USERNAME",
            PolicySubjectMatchType::UsernameOrEmail => "USERNAME_OR_EMAIL",
            PolicySubjectMatchType::Noop => "",
            PolicySubjectMatchType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PolicySubjectMatchType {
    fn default() -> PolicySubjectMatchType {
        PolicySubjectMatchType::Noop
    }
}
impl PolicySubjectMatchType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PolicySubjectMatchType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PolicyType {
    #[serde(rename = "IDP_DISCOVERY")]
    IdpDiscovery,
    #[serde(rename = "OAUTH_AUTHORIZATION_POLICY")]
    OauthAuthorizationPolicy,
    #[serde(rename = "OKTA_SIGN_ON")]
    OktaSignOn,
    #[serde(rename = "PASSWORD")]
    Password,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PolicyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyType::IdpDiscovery => "IDP_DISCOVERY",
            PolicyType::OauthAuthorizationPolicy => "OAUTH_AUTHORIZATION_POLICY",
            PolicyType::OktaSignOn => "OKTA_SIGN_ON",
            PolicyType::Password => "PASSWORD",
            PolicyType::Noop => "",
            PolicyType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PolicyType {
    fn default() -> PolicyType {
        PolicyType::Noop
    }
}
impl PolicyType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PolicyType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PolicyUserNameTemplate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub template: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProfileMapping {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileMappingProperty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ProfileMappingSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<ProfileMappingSource>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProfileMappingProperty {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expression: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pushStatus"
    )]
    pub push_status: Option<Links>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProfileMappingSource {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProtocolType {
    #[serde(rename = "MTLS")]
    Mtls,
    #[serde(rename = "OAUTH2")]
    Oauth2,
    #[serde(rename = "OIDC")]
    Oidc,
    #[serde(rename = "SAML2")]
    Saml2,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProtocolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolType::Mtls => "MTLS",
            ProtocolType::Oauth2 => "OAUTH2",
            ProtocolType::Oidc => "OIDC",
            ProtocolType::Saml2 => "SAML2",
            ProtocolType::Noop => "",
            ProtocolType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProtocolType {
    fn default() -> ProtocolType {
        ProtocolType::Noop
    }
}
impl ProtocolType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProtocolType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Protocol {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithms: Option<ProtocolAlgorithms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<IdentityProviderCredentials>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<ProtocolEndpoints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<ProtocolEndpoint>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "relayState"
    )]
    pub relay_state: Option<ProtocolRelayState>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<ProtocolSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<ProtocolType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProtocolAlgorithmType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<ProtocolAlgorithmTypeSignature>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Scope {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "REQUEST")]
    Request,
    #[serde(rename = "RESPONSE")]
    Response,
    #[serde(rename = "TOKEN")]
    Token,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scope::Any => "ANY",
            Scope::None => "NONE",
            Scope::Request => "REQUEST",
            Scope::Response => "RESPONSE",
            Scope::Token => "TOKEN",
            Scope::Noop => "",
            Scope::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Noop
    }
}
impl Scope {
    pub fn is_noop(&self) -> bool {
        matches!(self, Scope::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProtocolAlgorithmTypeSignature {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub algorithm: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProtocolAlgorithms {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<ProtocolAlgorithmType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<ProtocolAlgorithmType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Binding {
    #[serde(rename = "HTTP-POST")]
    HttpPost,
    #[serde(rename = "HTTP-REDIRECT")]
    HttpRedirect,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binding::HttpPost => "HTTP-POST",
            Binding::HttpRedirect => "HTTP-REDIRECT",
            Binding::Noop => "",
            Binding::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Binding {
    fn default() -> Binding {
        Binding::Noop
    }
}
impl Binding {
    pub fn is_noop(&self) -> bool {
        matches!(self, Binding::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProtocolEndpointType {
    #[serde(rename = "INSTANCE")]
    Instance,
    #[serde(rename = "ORG")]
    Org,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProtocolEndpointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolEndpointType::Instance => "INSTANCE",
            ProtocolEndpointType::Org => "ORG",
            ProtocolEndpointType::Noop => "",
            ProtocolEndpointType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProtocolEndpointType {
    fn default() -> ProtocolEndpointType {
        ProtocolEndpointType::Noop
    }
}
impl ProtocolEndpointType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProtocolEndpointType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProtocolEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<Binding>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub destination: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<ProtocolEndpointType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProtocolEndpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acs: Option<ProtocolEndpoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<ProtocolEndpoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwks: Option<ProtocolEndpoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ProtocolEndpoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slo: Option<ProtocolEndpoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sso: Option<ProtocolEndpoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<ProtocolEndpoint>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userInfo")]
    pub user_info: Option<ProtocolEndpoint>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProtocolRelayState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ProtocolRelayStateFormat>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProtocolRelayStateFormat {
    #[serde(rename = "FROM_URL")]
    FromUrl,
    #[serde(rename = "OPAQUE")]
    Opaque,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProtocolRelayStateFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolRelayStateFormat::FromUrl => "FROM_URL",
            ProtocolRelayStateFormat::Opaque => "OPAQUE",
            ProtocolRelayStateFormat::Noop => "",
            ProtocolRelayStateFormat::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProtocolRelayStateFormat {
    fn default() -> ProtocolRelayStateFormat {
        ProtocolRelayStateFormat::Noop
    }
}
impl ProtocolRelayStateFormat {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProtocolRelayStateFormat::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProtocolSettings {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nameFormat"
    )]
    pub name_format: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProvisioningAction {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "CALLOUT")]
    Callout,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProvisioningAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvisioningAction::Auto => "AUTO",
            ProvisioningAction::Callout => "CALLOUT",
            ProvisioningAction::Disabled => "DISABLED",
            ProvisioningAction::Noop => "",
            ProvisioningAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProvisioningAction {
    fn default() -> ProvisioningAction {
        ProvisioningAction::Noop
    }
}
impl ProvisioningAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProvisioningAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Provisioning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ProvisioningAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<ProvisioningConditions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<ProvisioningGroups>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "profileMaster"
    )]
    pub profile_master: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProvisioningConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprovisioned: Option<ProvisioningDeprovisionedCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended: Option<ProvisioningSuspendedCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProvisioningDeprovisionedConditionAction {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "REACTIVATE")]
    Reactivate,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProvisioningDeprovisionedConditionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvisioningDeprovisionedConditionAction::None => "NONE",
            ProvisioningDeprovisionedConditionAction::Reactivate => "REACTIVATE",
            ProvisioningDeprovisionedConditionAction::Noop => "",
            ProvisioningDeprovisionedConditionAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProvisioningDeprovisionedConditionAction {
    fn default() -> ProvisioningDeprovisionedConditionAction {
        ProvisioningDeprovisionedConditionAction::Noop
    }
}
impl ProvisioningDeprovisionedConditionAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProvisioningDeprovisionedConditionAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProvisioningDeprovisionedCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ProvisioningDeprovisionedConditionAction>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProvisioningGroupsAction {
    #[serde(rename = "APPEND")]
    Append,
    #[serde(rename = "ASSIGN")]
    Assign,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SYNC")]
    Sync,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProvisioningGroupsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvisioningGroupsAction::Append => "APPEND",
            ProvisioningGroupsAction::Assign => "ASSIGN",
            ProvisioningGroupsAction::None => "NONE",
            ProvisioningGroupsAction::Sync => "SYNC",
            ProvisioningGroupsAction::Noop => "",
            ProvisioningGroupsAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProvisioningGroupsAction {
    fn default() -> ProvisioningGroupsAction {
        ProvisioningGroupsAction::Noop
    }
}
impl ProvisioningGroupsAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProvisioningGroupsAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProvisioningGroups {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ProvisioningGroupsAction>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub assignments: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub filter: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "sourceAttributeName"
    )]
    pub source_attribute_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProvisioningSuspendedConditionAction {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "UNSUSPEND")]
    Unsuspend,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProvisioningSuspendedConditionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvisioningSuspendedConditionAction::None => "NONE",
            ProvisioningSuspendedConditionAction::Unsuspend => "UNSUSPEND",
            ProvisioningSuspendedConditionAction::Noop => "",
            ProvisioningSuspendedConditionAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProvisioningSuspendedConditionAction {
    fn default() -> ProvisioningSuspendedConditionAction {
        ProvisioningSuspendedConditionAction::Noop
    }
}
impl ProvisioningSuspendedConditionAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProvisioningSuspendedConditionAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProvisioningSuspendedCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ProvisioningSuspendedConditionAction>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PushUserFactor {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "expiresAt"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "factorResult"
    )]
    pub factor_result: Option<FactorResultType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<PushUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PushUserFactorProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "credentialId"
    )]
    pub credential_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "deviceToken"
    )]
    pub device_token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "deviceType"
    )]
    pub device_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub platform: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RecoveryQuestionCredential {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub answer: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub question: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RiskPolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub behaviors: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RiskScorePolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub level: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Role {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "assignmentType"
    )]
    pub assignment_type: Option<RoleAssignmentType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<RoleType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum RoleAssignmentType {
    #[serde(rename = "GROUP")]
    Group,
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RoleAssignmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleAssignmentType::Group => "GROUP",
            RoleAssignmentType::User => "USER",
            RoleAssignmentType::Noop => "",
            RoleAssignmentType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RoleAssignmentType {
    fn default() -> RoleAssignmentType {
        RoleAssignmentType::Noop
    }
}
impl RoleAssignmentType {
    pub fn is_noop(&self) -> bool {
        matches!(self, RoleAssignmentType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum RoleType {
    #[serde(rename = "API_ACCESS_MANAGEMENT_ADMIN")]
    ApiAccessManagementAdmin,
    #[serde(rename = "APP_ADMIN")]
    AppAdmin,
    #[serde(rename = "HELP_DESK_ADMIN")]
    HelpDeskAdmin,
    #[serde(rename = "MOBILE_ADMIN")]
    MobileAdmin,
    #[serde(rename = "ORG_ADMIN")]
    OrgAdmin,
    #[serde(rename = "READ_ONLY_ADMIN")]
    ReadOnlyAdmin,
    #[serde(rename = "REPORT_ADMIN")]
    ReportAdmin,
    #[serde(rename = "SUPER_ADMIN")]
    SuperAdmin,
    #[serde(rename = "USER_ADMIN")]
    UserAdmin,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RoleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleType::ApiAccessManagementAdmin => "API_ACCESS_MANAGEMENT_ADMIN",
            RoleType::AppAdmin => "APP_ADMIN",
            RoleType::HelpDeskAdmin => "HELP_DESK_ADMIN",
            RoleType::MobileAdmin => "MOBILE_ADMIN",
            RoleType::OrgAdmin => "ORG_ADMIN",
            RoleType::ReadOnlyAdmin => "READ_ONLY_ADMIN",
            RoleType::ReportAdmin => "REPORT_ADMIN",
            RoleType::SuperAdmin => "SUPER_ADMIN",
            RoleType::UserAdmin => "USER_ADMIN",
            RoleType::Noop => "",
            RoleType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RoleType {
    fn default() -> RoleType {
        RoleType::Noop
    }
}
impl RoleType {
    pub fn is_noop(&self) -> bool {
        matches!(self, RoleType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SamlApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<SamlApplicationSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SamlApplicationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signOn")]
    pub sign_on: Option<SamlApplicationSettingsSignOn>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SamlApplicationSettingsSignOn {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "acsEndpoints"
    )]
    pub acs_endpoints: Vec<AcsEndpoint>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "allowMultipleAcsEndpoints"
    )]
    pub allow_multiple_acs_endpoints: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "assertionSigned"
    )]
    pub assertion_signed: bool,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "attributeStatements"
    )]
    pub attribute_statements: Vec<SamlAttributeStatement>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub audience: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "audienceOverride"
    )]
    pub audience_override: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "authnContextClassRef"
    )]
    pub authn_context_class_ref: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "defaultRelayState"
    )]
    pub default_relay_state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub destination: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "destinationOverride"
    )]
    pub destination_override: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "digestAlgorithm"
    )]
    pub digest_algorithm: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "honorForceAuthn"
    )]
    pub honor_force_authn: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "idpIssuer"
    )]
    pub idp_issuer: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "inlineHooks"
    )]
    pub inline_hooks: Vec<SignOnInlineHook>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub recipient: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "recipientOverride"
    )]
    pub recipient_override: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "requestCompressed"
    )]
    pub request_compressed: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "responseSigned"
    )]
    pub response_signed: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "signatureAlgorithm"
    )]
    pub signature_algorithm: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slo: Option<SingleLogout>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "spCertificate"
    )]
    pub sp_certificate: Option<SpCertificate>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spIssuer"
    )]
    pub sp_issuer: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ssoAcsUrl"
    )]
    pub sso_acs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ssoAcsUrlOverride"
    )]
    pub sso_acs_url_override: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "subjectNameIdFormat"
    )]
    pub subject_name_id_format: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "subjectNameIdTemplate"
    )]
    pub subject_name_id_template: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SamlAttributeStatement {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "filterType"
    )]
    pub filter_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "filterValue"
    )]
    pub filter_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub namespace: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Value {
    #[serde(rename = "ACTIVATING")]
    Activating,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "DELETING")]
    Deleting,
    #[serde(rename = "EXPIRED_PASSWORD")]
    ExpiredPassword,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Activating => "ACTIVATING",
            Value::Active => "ACTIVE",
            Value::Deleted => "DELETED",
            Value::Deleting => "DELETING",
            Value::ExpiredPassword => "EXPIRED_PASSWORD",
            Value::Inactive => "INACTIVE",
            Value::Pending => "PENDING",
            Value::Suspended => "SUSPENDED",
            Value::Noop => "",
            Value::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Noop
    }
}
impl Value {
    pub fn is_noop(&self) -> bool {
        matches!(self, Value::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ScheduledUserLifecycleAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Value>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SchemeApplicationCredentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordCredential>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "revealPassword"
    )]
    pub reveal_password: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<ApplicationCredentialsScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing: Option<ApplicationCredentialsSigning>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "userName"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ScopeData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "stringValue"
    )]
    pub string_value: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<ScopeType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ScopeType {
    #[serde(rename = "CORS")]
    Cors,
    #[serde(rename = "REDIRECT")]
    Redirect,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeType::Cors => "CORS",
            ScopeType::Redirect => "REDIRECT",
            ScopeType::Noop => "",
            ScopeType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ScopeType {
    fn default() -> ScopeType {
        ScopeType::Noop
    }
}
impl ScopeType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ScopeType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecurePasswordStoreApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SchemeApplicationCredentials>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<SecurePasswordStoreApplicationSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecurePasswordStoreApplicationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<SecurePasswordStoreApplicationSettingsData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecurePasswordStoreApplicationSettingsData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "optionalField1"
    )]
    pub optional_field_1: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "optionalField1Value"
    )]
    pub optional_field_1_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "optionalField2"
    )]
    pub optional_field_2: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "optionalField2Value"
    )]
    pub optional_field_2_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "optionalField3"
    )]
    pub optional_field_3: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "optionalField3Value"
    )]
    pub optional_field_3_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "passwordField"
    )]
    pub password_field: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "usernameField"
    )]
    pub username_field: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecurityQuestion {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub answer: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub question: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "questionText"
    )]
    pub question_text: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecurityQuestionUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<SecurityQuestion>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Session {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub amr: Vec<SessionAuthenticationMethod>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "createdAt"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "expiresAt"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idp: Option<SessionIdentityProvider>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastFactorVerification"
    )]
    pub last_factor_verification: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastPasswordVerification"
    )]
    pub last_password_verification: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SessionStatus>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "userId"
    )]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SessionAuthenticationMethod {
    #[serde(rename = "fpt")]
    Fpt,
    #[serde(rename = "geo")]
    Geo,
    #[serde(rename = "hwk")]
    Hwk,
    #[serde(rename = "kba")]
    Kba,
    #[serde(rename = "mfa")]
    Mfa,
    #[serde(rename = "otp")]
    Otp,
    #[serde(rename = "pwd")]
    Pwd,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "swk")]
    Swk,
    #[serde(rename = "tel")]
    Tel,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SessionAuthenticationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionAuthenticationMethod::Fpt => "fpt",
            SessionAuthenticationMethod::Geo => "geo",
            SessionAuthenticationMethod::Hwk => "hwk",
            SessionAuthenticationMethod::Kba => "kba",
            SessionAuthenticationMethod::Mfa => "mfa",
            SessionAuthenticationMethod::Otp => "otp",
            SessionAuthenticationMethod::Pwd => "pwd",
            SessionAuthenticationMethod::Sms => "sms",
            SessionAuthenticationMethod::Swk => "swk",
            SessionAuthenticationMethod::Tel => "tel",
            SessionAuthenticationMethod::Noop => "",
            SessionAuthenticationMethod::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SessionAuthenticationMethod {
    fn default() -> SessionAuthenticationMethod {
        SessionAuthenticationMethod::Noop
    }
}
impl SessionAuthenticationMethod {
    pub fn is_noop(&self) -> bool {
        matches!(self, SessionAuthenticationMethod::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SessionIdentityProvider {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<SessionIdentityProviderType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SessionIdentityProviderType {
    #[serde(rename = "ACTIVE_DIRECTORY")]
    ActiveDirectory,
    #[serde(rename = "FEDERATION")]
    Federation,
    #[serde(rename = "LDAP")]
    Ldap,
    #[serde(rename = "OKTA")]
    Okta,
    #[serde(rename = "SOCIAL")]
    Social,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SessionIdentityProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionIdentityProviderType::ActiveDirectory => "ACTIVE_DIRECTORY",
            SessionIdentityProviderType::Federation => "FEDERATION",
            SessionIdentityProviderType::Ldap => "LDAP",
            SessionIdentityProviderType::Okta => "OKTA",
            SessionIdentityProviderType::Social => "SOCIAL",
            SessionIdentityProviderType::Noop => "",
            SessionIdentityProviderType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SessionIdentityProviderType {
    fn default() -> SessionIdentityProviderType {
        SessionIdentityProviderType::Noop
    }
}
impl SessionIdentityProviderType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SessionIdentityProviderType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SessionStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "MFA_ENROLL")]
    MfaEnroll,
    #[serde(rename = "MFA_REQUIRED")]
    MfaRequired,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionStatus::Active => "ACTIVE",
            SessionStatus::MfaEnroll => "MFA_ENROLL",
            SessionStatus::MfaRequired => "MFA_REQUIRED",
            SessionStatus::Noop => "",
            SessionStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SessionStatus {
    fn default() -> SessionStatus {
        SessionStatus::Noop
    }
}
impl SessionStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, SessionStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SignOnInlineHook {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SingleLogout {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issuer: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "logoutUrl"
    )]
    pub logout_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SmsTemplate {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translations: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<SmsTemplateType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SmsTemplateType {
    #[serde(rename = "SMS_VERIFY_CODE")]
    SmsVerifyCode,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SmsTemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmsTemplateType::SmsVerifyCode => "SMS_VERIFY_CODE",
            SmsTemplateType::Noop => "",
            SmsTemplateType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SmsTemplateType {
    fn default() -> SmsTemplateType {
        SmsTemplateType::Noop
    }
}
impl SmsTemplateType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SmsTemplateType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SmsUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<SmsUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SmsUserFactorProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "phoneNumber"
    )]
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TokenType {
    #[serde(rename = "ACCESS")]
    Access,
    #[serde(rename = "REFRESH")]
    Refresh,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Access => "ACCESS",
            TokenType::Refresh => "REFRESH",
            TokenType::Noop => "",
            TokenType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TokenType {
    fn default() -> TokenType {
        TokenType::Noop
    }
}
impl TokenType {
    pub fn is_noop(&self) -> bool {
        matches!(self, TokenType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SocialAuthToken {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "expiresAt"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "tokenAuthScheme"
    )]
    pub token_auth_scheme: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenType")]
    pub token_type: Option<TokenType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SpCertificate {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "x5c"
    )]
    pub x_5c: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SwaApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<SwaApplicationSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SwaApplicationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<SwaApplicationSettingsData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SwaApplicationSettingsData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "buttonField"
    )]
    pub button_field: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "loginUrlRegex"
    )]
    pub login_url_regex: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "passwordField"
    )]
    pub password_field: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "usernameField"
    )]
    pub username_field: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SwaThreeFieldApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<SwaThreeFieldApplicationSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SwaThreeFieldApplicationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<SwaThreeFieldApplicationSettingsData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SwaThreeFieldApplicationSettingsData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "buttonSelector"
    )]
    pub button_selector: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "extraFieldSelector"
    )]
    pub extra_field_selector: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "extraFieldValue"
    )]
    pub extra_field_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "loginUrlRegex"
    )]
    pub login_url_regex: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "passwordSelector"
    )]
    pub password_selector: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "targetURL"
    )]
    pub target_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "userNameSelector"
    )]
    pub user_name_selector: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TempPassword {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "tempPassword"
    )]
    pub temp_password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ThreatInsightConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "excludeZones"
    )]
    pub exclude_zones: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TokenAuthorizationServerPolicyRuleAction {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "accessTokenLifetimeMinutes"
    )]
    pub access_token_lifetime_minutes: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "inlineHook"
    )]
    pub inline_hook: Option<TokenAuthorizationServerPolicyRuleActionInlineHook>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "refreshTokenLifetimeMinutes"
    )]
    pub refresh_token_lifetime_minutes: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "refreshTokenWindowMinutes"
    )]
    pub refresh_token_window_minutes: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TokenAuthorizationServerPolicyRuleActionInlineHook {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TokenUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<WebUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TotpUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<WebUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TrustedOrigin {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "createdBy"
    )]
    pub created_by: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "lastUpdatedBy"
    )]
    pub last_updated_by: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub origin: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<ScopeData>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct U2FUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<WebUserFactorProfile>,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct User {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub activated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<UserCredentials>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastLogin"
    )]
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "passwordChanged"
    )]
    pub password_changed: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<UserProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "statusChanged"
    )]
    pub status_changed: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transitioningToStatus"
    )]
    pub transitioning_to_status: Option<UserStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<UserType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserActivationToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "activationToken"
    )]
    pub activation_token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "activationUrl"
    )]
    pub activation_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserCredentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordCredential>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<AuthenticationProvider>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_question: Option<RecoveryQuestionCredential>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "factorType"
    )]
    pub factor_type: Option<FactorType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<FactorProvider>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<FactorStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify: Option<VerifyFactorRequest>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UserIdentifierConditionEvaluatorPatternMatchType {
    #[serde(rename = "CONTAINS")]
    Contains,
    #[serde(rename = "EQUALS")]
    Equals,
    #[serde(rename = "EXPRESSION")]
    Expression,
    #[serde(rename = "STARTS_WITH")]
    StartsWith,
    #[serde(rename = "SUFFIX")]
    Suffix,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UserIdentifierConditionEvaluatorPatternMatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserIdentifierConditionEvaluatorPatternMatchType::Contains => "CONTAINS",
            UserIdentifierConditionEvaluatorPatternMatchType::Equals => "EQUALS",
            UserIdentifierConditionEvaluatorPatternMatchType::Expression => "EXPRESSION",
            UserIdentifierConditionEvaluatorPatternMatchType::StartsWith => "STARTS_WITH",
            UserIdentifierConditionEvaluatorPatternMatchType::Suffix => "SUFFIX",
            UserIdentifierConditionEvaluatorPatternMatchType::Noop => "",
            UserIdentifierConditionEvaluatorPatternMatchType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UserIdentifierConditionEvaluatorPatternMatchType {
    fn default() -> UserIdentifierConditionEvaluatorPatternMatchType {
        UserIdentifierConditionEvaluatorPatternMatchType::Noop
    }
}
impl UserIdentifierConditionEvaluatorPatternMatchType {
    pub fn is_noop(&self) -> bool {
        matches!(self, UserIdentifierConditionEvaluatorPatternMatchType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserIdentifierConditionEvaluatorPattern {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchType")]
    pub match_type: Option<UserIdentifierConditionEvaluatorPatternMatchType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UserIdentifierPolicyRuleConditionType {
    #[serde(rename = "ATTRIBUTE")]
    Attribute,
    #[serde(rename = "IDENTIFIER")]
    Identifier,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UserIdentifierPolicyRuleConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserIdentifierPolicyRuleConditionType::Attribute => "ATTRIBUTE",
            UserIdentifierPolicyRuleConditionType::Identifier => "IDENTIFIER",
            UserIdentifierPolicyRuleConditionType::Noop => "",
            UserIdentifierPolicyRuleConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UserIdentifierPolicyRuleConditionType {
    fn default() -> UserIdentifierPolicyRuleConditionType {
        UserIdentifierPolicyRuleConditionType::Noop
    }
}
impl UserIdentifierPolicyRuleConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, UserIdentifierPolicyRuleConditionType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserIdentifierPolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub attribute: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub patterns: Vec<UserIdentifierConditionEvaluatorPattern>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<UserIdentifierPolicyRuleConditionType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserIdentityProviderLinkRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "externalId"
    )]
    pub external_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserLifecycleAttributePolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "attributeName"
    )]
    pub attribute_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "matchingValue"
    )]
    pub matching_value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UserNextLogin {
    #[serde(rename = "changePassword")]
    ChangePassword,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UserNextLogin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserNextLogin::ChangePassword => "changePassword",
            UserNextLogin::Noop => "",
            UserNextLogin::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UserNextLogin {
    fn default() -> UserNextLogin {
        UserNextLogin::Noop
    }
}
impl UserNextLogin {
    pub fn is_noop(&self) -> bool {
        matches!(self, UserNextLogin::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserPolicyRuleCondition {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub exclude: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactivity: Option<Duration>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub include: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lifecycleExpiration"
    )]
    pub lifecycle_expiration: Option<LifecycleExpirationPolicyRuleCondition>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "passwordExpiration"
    )]
    pub password_expiration: Option<Duration>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userLifecycleAttribute"
    )]
    pub user_lifecycle_attribute: Option<UserLifecycleAttributePolicyRuleCondition>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "awsRole"
    )]
    pub aws_role: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub birthday: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "costCenter"
    )]
    pub cost_center: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "countryCode"
    )]
    pub country_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub department: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub division: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "emailAliases"
    )]
    pub email_aliases: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "employeeNumber"
    )]
    pub employee_number: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "firstName"
    )]
    pub first_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "githubUsername"
    )]
    pub github_username: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "honorificPrefix"
    )]
    pub honorific_prefix: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "honorificSuffix"
    )]
    pub honorific_suffix: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "lastName"
    )]
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub locale: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub manager: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "managerId"
    )]
    pub manager_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "matrixUsername"
    )]
    pub matrix_username: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "middleName"
    )]
    pub middle_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "mobilePhone"
    )]
    pub mobile_phone: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nickName"
    )]
    pub nick_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "postalAddress"
    )]
    pub postal_address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "preferredLanguage"
    )]
    pub preferred_language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "primaryPhone"
    )]
    pub primary_phone: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "profileUrl"
    )]
    pub profile_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "secondEmail"
    )]
    pub second_email: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize",
        rename = "startDate"
    )]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "streetAddress"
    )]
    pub street_address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "userType"
    )]
    pub user_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "workCity"
    )]
    pub work_city: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "workCountryCode"
    )]
    pub work_country_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "workPostalAddress"
    )]
    pub work_postal_address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "workState"
    )]
    pub work_state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "workStreetAddress"
    )]
    pub work_street_address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "workZipCode"
    )]
    pub work_zip_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "zipCode"
    )]
    pub zip_code: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchema {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "$schema"
    )]
    pub schema: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definitions: Option<UserSchemaDefinitions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserSchemaProperties>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaAttribute {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "enum"
    )]
    pub enum_: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "externalName"
    )]
    pub external_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "externalNamespace"
    )]
    pub external_namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<UserSchemaAttributeItems>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master: Option<UserSchemaAttributeMaster>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxLength"
    )]
    pub max_length: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "minLength"
    )]
    pub min_length: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mutability: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "oneOf"
    )]
    pub one_of: Vec<UserSchemaAttributeEnum>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pattern: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub permissions: Vec<UserSchemaAttributePermission>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<UserSchemaAttributeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unique: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaAttributeEnum {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "const"
    )]
    pub const_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaAttributeItems {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "enum"
    )]
    pub enum_: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "oneOf"
    )]
    pub one_of: Vec<UserSchemaAttributeEnum>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaAttributeMaster {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub priority: Vec<UserSchemaAttributeMasterPriority>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<UserSchemaAttributeMasterType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaAttributeMasterPriority {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UserSchemaAttributeMasterType {
    #[serde(rename = "OKTA")]
    Okta,
    #[serde(rename = "OVERRIDE")]
    Override,
    #[serde(rename = "PROFILE_MASTER")]
    ProfileMaster,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UserSchemaAttributeMasterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSchemaAttributeMasterType::Okta => "OKTA",
            UserSchemaAttributeMasterType::Override => "OVERRIDE",
            UserSchemaAttributeMasterType::ProfileMaster => "PROFILE_MASTER",
            UserSchemaAttributeMasterType::Noop => "",
            UserSchemaAttributeMasterType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UserSchemaAttributeMasterType {
    fn default() -> UserSchemaAttributeMasterType {
        UserSchemaAttributeMasterType::Noop
    }
}
impl UserSchemaAttributeMasterType {
    pub fn is_noop(&self) -> bool {
        matches!(self, UserSchemaAttributeMasterType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaAttributePermission {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub principal: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UserSchemaAttributeType {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UserSchemaAttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSchemaAttributeType::Array => "array",
            UserSchemaAttributeType::Boolean => "boolean",
            UserSchemaAttributeType::Integer => "integer",
            UserSchemaAttributeType::Number => "number",
            UserSchemaAttributeType::String => "string",
            UserSchemaAttributeType::Noop => "",
            UserSchemaAttributeType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UserSchemaAttributeType {
    fn default() -> UserSchemaAttributeType {
        UserSchemaAttributeType::Noop
    }
}
impl UserSchemaAttributeType {
    pub fn is_noop(&self) -> bool {
        matches!(self, UserSchemaAttributeType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaBase {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserSchemaBaseProperties>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub required: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaBaseProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "costCenter"
    )]
    pub cost_center: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "countryCode"
    )]
    pub country_code: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "displayName"
    )]
    pub display_name: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub division: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "employeeNumber"
    )]
    pub employee_number: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firstName")]
    pub first_name: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "honorificPrefix"
    )]
    pub honorific_prefix: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "honorificSuffix"
    )]
    pub honorific_suffix: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastName")]
    pub last_name: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managerId")]
    pub manager_id: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "middleName"
    )]
    pub middle_name: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mobilePhone"
    )]
    pub mobile_phone: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nickName")]
    pub nick_name: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "postalAddress"
    )]
    pub postal_address: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "preferredLanguage"
    )]
    pub preferred_language: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "primaryPhone"
    )]
    pub primary_phone: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "profileUrl"
    )]
    pub profile_url: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "secondEmail"
    )]
    pub second_email: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "streetAddress"
    )]
    pub street_address: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userType")]
    pub user_type: Option<UserSchemaAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zipCode")]
    pub zip_code: Option<UserSchemaAttribute>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaDefinitions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<UserSchemaBase>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<UserSchemaPublic>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<UserSchemaPropertiesProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaPropertiesProfile {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "allOf"
    )]
    pub all_of: Vec<UserSchemaPropertiesProfileItem>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaPropertiesProfileItem {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "$ref"
    )]
    pub ref_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserSchemaPublic {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserSchemaAttribute>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub required: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UserStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DEPROVISIONED")]
    Deprovisioned,
    #[serde(rename = "LOCKED_OUT")]
    LockedOut,
    #[serde(rename = "PASSWORD_EXPIRED")]
    PasswordExpired,
    #[serde(rename = "PROVISIONED")]
    Provisioned,
    #[serde(rename = "RECOVERY")]
    Recovery,
    #[serde(rename = "STAGED")]
    Staged,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => "ACTIVE",
            UserStatus::Deprovisioned => "DEPROVISIONED",
            UserStatus::LockedOut => "LOCKED_OUT",
            UserStatus::PasswordExpired => "PASSWORD_EXPIRED",
            UserStatus::Provisioned => "PROVISIONED",
            UserStatus::Recovery => "RECOVERY",
            UserStatus::Staged => "STAGED",
            UserStatus::Suspended => "SUSPENDED",
            UserStatus::Noop => "",
            UserStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UserStatus {
    fn default() -> UserStatus {
        UserStatus::Noop
    }
}
impl UserStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, UserStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserStatusPolicyRuleCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "createdBy"
    )]
    pub created_by: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "displayName"
    )]
    pub display_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastUpdated"
    )]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "lastUpdatedBy"
    )]
    pub last_updated_by: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VerifyFactorRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "activationToken"
    )]
    pub activation_token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub answer: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub attestation: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "clientData"
    )]
    pub client_data: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nextPassCode"
    )]
    pub next_pass_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "passCode"
    )]
    pub pass_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "registrationData"
    )]
    pub registration_data: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "stateToken"
    )]
    pub state_token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FactorResult {
    #[serde(rename = "CHALLENGE")]
    Challenge,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PASSCODE_REPLAYED")]
    PasscodeReplayed,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "TIMEOUT")]
    Timeout,
    #[serde(rename = "TIME_WINDOW_EXCEEDED")]
    TimeWindowExceeded,
    #[serde(rename = "WAITING")]
    Waiting,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FactorResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorResult::Challenge => "CHALLENGE",
            FactorResult::Error => "ERROR",
            FactorResult::Expired => "EXPIRED",
            FactorResult::Failed => "FAILED",
            FactorResult::PasscodeReplayed => "PASSCODE_REPLAYED",
            FactorResult::Rejected => "REJECTED",
            FactorResult::Success => "SUCCESS",
            FactorResult::Timeout => "TIMEOUT",
            FactorResult::TimeWindowExceeded => "TIME_WINDOW_EXCEEDED",
            FactorResult::Waiting => "WAITING",
            FactorResult::Noop => "",
            FactorResult::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FactorResult {
    fn default() -> FactorResult {
        FactorResult::Noop
    }
}
impl FactorResult {
    pub fn is_noop(&self) -> bool {
        matches!(self, FactorResult::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VerifyUserFactorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_embedded")]
    pub embedded: Option<Links>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_links")]
    pub links: Option<Links>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "expiresAt"
    )]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "factorResult"
    )]
    pub factor_result: Option<FactorResult>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "factorResultMessage"
    )]
    pub factor_result_message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebAuthnUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<WebAuthnUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebAuthnUserFactorProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "authenticatorName"
    )]
    pub authenticator_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "credentialId"
    )]
    pub credential_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebUserFactor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<WebUserFactorProfile>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WsFederationApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<WsFederationApplicationSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WsFederationApplicationSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<WsFederationApplicationSettingsData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WsFederationApplicationSettingsData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "attributeStatements"
    )]
    pub attribute_statements: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "audienceRestriction"
    )]
    pub audience_restriction: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "authnContextClassRef"
    )]
    pub authn_context_class_ref: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "groupFilter"
    )]
    pub group_filter: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "groupName"
    )]
    pub group_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "groupValueFormat"
    )]
    pub group_value_format: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nameIDFormat"
    )]
    pub name_id_format: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub realm: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "siteURL"
    )]
    pub site_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "usernameAttribute"
    )]
    pub username_attribute: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "wReplyOverride"
    )]
    pub w_reply_override: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "wReplyURL"
    )]
    pub w_reply_url: String,
}
