//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PartnerSettingsNewRelic {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable_subuser_statistics: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub license_key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriptionTrackingSettings {
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
    pub html_content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub landing: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub replace: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Errors {
    /**
     * The indices of the recipient(s) sent that caused the error.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub error_indices: Vec<f64>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactDbRecipientResponse {
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub error_count: f64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub error_indices: Vec<f64>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<Errors>,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub new_count: f64,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub persisted_recipients: Vec<String>,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub updated_count: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

/// All of the following types are flattened into one object:
///
/// - `CampaignsRequest`
/// - `CampaignResponse`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignResponseAllOf {
    #[serde(flatten)]
    pub campaigns_request: CampaignsRequest,
    #[serde(flatten)]
    pub campaign_response: CampaignResponse,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Operator {
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Contains => "contains",
            Operator::Eq => "eq",
            Operator::Gt => "gt",
            Operator::Lt => "lt",
            Operator::Ne => "ne",
            Operator::Noop => "",
            Operator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Operator {
    fn default() -> Operator {
        Operator::Noop
    }
}
impl Operator {
    pub fn is_noop(&self) -> bool {
        matches!(self, Operator::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AndOr {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AndOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AndOr::And => "and",
            AndOr::Or => "or",
            AndOr::Noop => "",
            AndOr::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AndOr {
    fn default() -> AndOr {
        AndOr::Noop
    }
}
impl AndOr {
    pub fn is_noop(&self) -> bool {
        matches!(self, AndOr::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbSegmentsConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and_or: Option<AndOr>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    #[serde(default, skip_serializing_if = "Operator::is_noop")]
    pub operator: Operator,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BounceResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub created: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
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
    pub status: String,
}

/// helper text or docs for troubleshooting
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Help {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Contacts {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "address2")]
    pub address_2: Option<Help>,
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
    pub company: String,
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
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Users {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub user_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

/// The DKIM record for messages sent using this authenticated domain.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Dkim {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub data: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReverseDns {
    /**
     * The DKIM record for messages sent using this authenticated domain.
     */
    #[serde()]
    pub a_record: Dkim,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub last_validation_attempt_at: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub legacy: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rdns: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subdomain: String,
    /**
     * The users who are able to send mail from the IP address.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub users: Vec<Users>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SenderId {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created_at: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub updated_at: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified: bool,
}

/// All of the following types are flattened into one object:
///
/// - `Help`
/// - `SenderRequest`
/// - `SenderId`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SenderAllOf {
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(flatten)]
    pub help: Help,
    #[serde(flatten)]
    pub sender_request: SenderRequest,
    #[serde(flatten)]
    pub sender_id: SenderId,
}

/**
* The type of the field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Type {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Date => "date",
            Type::Number => "number",
            Type::Text => "text",
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
pub struct ContactdbCustomField {
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
pub struct Subuser {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub disabled: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailSettingsAddressWhitelabel {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list: Vec<String>,
}

/**
* The type of DNS record that was generated.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LinkBranding200ResponseDnsDomainCnameType {
    #[serde(rename = "cname")]
    Cname,
    #[serde(rename = "mx")]
    Mx,
    #[serde(rename = "txt")]
    Txt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LinkBranding200ResponseDnsDomainCnameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkBranding200ResponseDnsDomainCnameType::Cname => "cname",
            LinkBranding200ResponseDnsDomainCnameType::Mx => "mx",
            LinkBranding200ResponseDnsDomainCnameType::Txt => "txt",
            LinkBranding200ResponseDnsDomainCnameType::Noop => "",
            LinkBranding200ResponseDnsDomainCnameType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LinkBranding200ResponseDnsDomainCnameType {
    fn default() -> LinkBranding200ResponseDnsDomainCnameType {
        LinkBranding200ResponseDnsDomainCnameType::Noop
    }
}
impl LinkBranding200ResponseDnsDomainCnameType {
    pub fn is_noop(&self) -> bool {
        matches!(self, LinkBranding200ResponseDnsDomainCnameType::Noop)
    }
}

/// The DNS record generated to point to your link branding subdomain.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DomainCname {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub data: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,
    /**
     * The type of DNS record that was generated.
     */
    #[serde(
        default,
        skip_serializing_if = "LinkBranding200ResponseDnsDomainCnameType::is_noop",
        rename = "type"
    )]
    pub type_: LinkBranding200ResponseDnsDomainCnameType,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

/// The DNS record generated to verify who created the link branding.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OwnerCname {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub data: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,
    /**
     * The DNS record generated to verify who created the link branding.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<LinkBranding200ResponseDnsDomainCnameType>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

/// The DNS records generated for this link branding.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Dns {
    /**
     * The DNS record generated to point to your link branding subdomain.
     */
    #[serde()]
    pub domain_cname: DomainCname,
    /**
     * The DNS records generated for this link branding.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_cname: Option<OwnerCname>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LinkBranding200Response {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    /**
     * The DNS records generated for this link branding.
     */
    #[serde()]
    pub dns: Dns,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub legacy: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subdomain: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub user_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FromEmailObject {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApiKeyNameScopes {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
}

/// All of the following types are flattened into one object:
///
/// - `ApiKeyNameScopes`
/// - `ApiKeyNameId`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApiKeyNameScopesAllOf {
    #[serde(flatten)]
    pub api_key_name_scopes: ApiKeyNameScopes,
    #[serde(flatten)]
    pub api_key_name_id: ApiKeyNameId,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbSegments {
    /**
     * The conditions for a recipient to be included in this segment.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conditions: Vec<ContactdbSegmentsConditions>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub list_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub recipient_count: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApiKeyNameId {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub api_key_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// The individual events and their stats.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AdvancedStatsOpens {
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailSettingsTemplate {
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
    pub html_content: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IpWarmupResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_date: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Monitor {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub frequency: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GlobalErrorResponseSchemaErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help: Option<Help>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GlobalErrorResponseSchema {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<GlobalErrorResponseSchemaErrors>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

/// The individual events and their stats.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AdvancedStatsMailboxProvider {
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub blocks: i64,
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounces: i64,
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub deferred: i64,
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub delivered: i64,
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub drops: i64,
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub processed: i64,
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub requests: i64,
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub spam_reports: i64,
}

/// All of the following types are flattened into one object:
///
/// - `AdvancedStatsClicksOpensAllOf`
/// - `AdvancedStatsMailboxProvider`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AdvancedStatsMailboxProviderAllOf {
    #[serde(flatten)]
    pub advanced_stats_clicks_opens_all_of: AdvancedStatsClicksOpensAllOf,
    /**
     * The individual events and their stats.
     */
    #[serde(flatten)]
    pub advanced_stats_mailbox_provider: AdvancedStatsMailboxProvider,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbCustomFieldWithId {
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
}

/// All of the following types are flattened into one object:
///
/// - `ContactdbCustomField`
/// - `ContactdbCustomFieldWithId`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbCustomFieldWithAllOf {
    #[serde(flatten)]
    pub contactdb_custom_field: ContactdbCustomField,
    #[serde(flatten)]
    pub contactdb_custom_field_with_id: ContactdbCustomFieldWithId,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IpPool {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GoogleAnalyticsSettings {
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
    pub utm_campaign: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub utm_content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub utm_medium: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub utm_source: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub utm_term: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebhooksEventWebhookResponse {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub bounce: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub click: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub deferred: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub delivered: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub dropped: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub group_resubscribe: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub group_unsubscribe: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_token_url: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub open: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub processed: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub spam_report: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub unsubscribe: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserProfile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
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
    pub company: String,
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
    pub first_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub website: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailSettingsFooter {
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
    pub html_content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_content: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Metrics {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub blocks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounce_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounces: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub deferred: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub delivered: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub invalid_emails: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub processed: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub requests: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub spam_report_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub spam_reports: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribe_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribes: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Stats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CategoryStats {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stats: Vec<Stats>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ParseSetting {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hostname: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub send_raw: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub spam_check: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TransactionalTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<Warning>,
}

/// All of the following types are flattened into one object:
///
/// - `TransactionalTemplatesTemplateLean`
/// - `TransactionalTemplate`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TransactionalTemplateAllOf {
    #[serde(flatten)]
    pub transactional_templates_template_lean: TransactionalTemplatesTemplateLean,
    #[serde(flatten)]
    pub transactional_template: TransactionalTemplate,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbList {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub recipient_count: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SuppressionGroup {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_default: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_email_sent_at: Option<serde_json::Value>,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub unsubscribes: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailSettingsBouncePurge {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub hard_bounces: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub soft_bounces: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TransactionalTemplateVersionOutput {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub warnings: Vec<Warning>,
}

/// All of the following types are flattened into one object:
///
/// - `TransactionalTemplateVersionOutput`
/// - `TransactionalTemplateVersionCreate`
/// - `TransactionalTemplatesVersionOutputLean`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TransactionalTemplateVersionOutputAllOf {
    #[serde(flatten)]
    pub transactional_template_version_output: TransactionalTemplateVersionOutput,
    #[serde(flatten)]
    pub transactional_template_version_create: TransactionalTemplateVersionCreate,
    #[serde(flatten)]
    pub transactional_templates_version_output_lean: TransactionalTemplatesVersionOutputLean,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Permissions {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub api: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mail: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub web: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Credentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailSettingsForwardSpam {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
}

/**
* The editor used in the UI.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Editor {
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "design")]
    Design,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Editor::Code => "code",
            Editor::Design => "design",
            Editor::Noop => "",
            Editor::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Editor {
    fn default() -> Editor {
        Editor::Noop
    }
}
impl Editor {
    pub fn is_noop(&self) -> bool {
        matches!(self, Editor::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignsRequest {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_unsubscribe_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor: Option<Editor>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_pool: String,
    /**
     * The IDs of the lists you are sending this campaign to. You can have both segment IDs and list IDs
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_content: String,
    /**
     * The IDs of the lists you are sending this campaign to. You can have both segment IDs and list IDs
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segment_ids: Vec<i64>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub sender_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub suppression_group_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubuserStatsMetrics {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub blocks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounce_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounces: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub deferred: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub delivered: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub invalid_emails: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub processed: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub requests: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub spam_report_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub spam_reports: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribe_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribes: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubuserStats {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<SubuserStatsMetrics>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubuserStatsData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stats: Vec<SubuserStats>,
}

/**
* The status of the scheduled send.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Status {
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Cancel => "cancel",
            Status::Pause => "pause",
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

/// The status of the scheduled send.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserScheduledSendStatus {
    /**
     * The status of the scheduled send.
     */
    #[serde(default, skip_serializing_if = "Status::is_noop")]
    pub status: Status,
}

/// All of the following types are flattened into one object:
///
/// - `MailBatchId`
/// - `UserScheduledSendStatus`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UserScheduledSendStatusAllOf {
    #[serde(flatten)]
    pub mail_batch_id: MailBatchId,
    /**
     * The status of the scheduled send.
     */
    #[serde(flatten)]
    pub user_scheduled_send_status: UserScheduledSendStatus,
}

/// All of the following types are flattened into one object:
///
/// - `AdvancedStatsOpens`
/// - `AdvancedStatsClicks`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AdvancedStatsClicksOpensAllOf {
    /**
     * The individual events and their stats.
     */
    #[serde(flatten)]
    pub advanced_stats_opens: AdvancedStatsOpens,
    /**
     * The individual events and their stats.
     */
    #[serde(flatten)]
    pub advanced_stats_clicks: AdvancedStatsClicks,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbSegmentsWithId {
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
}

/// All of the following types are flattened into one object:
///
/// - `ContactdbSegments`
/// - `ContactdbSegmentsWithId`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbSegmentsWithAllOf {
    #[serde(flatten)]
    pub contactdb_segments: ContactdbSegments,
    #[serde(flatten)]
    pub contactdb_segments_with_id: ContactdbSegmentsWithId,
}

/// The individual events and their stats.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AdvancedStatsClicks {
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
     * The individual events and their stats.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Recipients {
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub created_at: f64,
    /**
     * The custom fields assigned to this recipient and their values.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub custom_fields: Vec<ContactdbCustomFieldWithValueAllOf>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub last_clicked: f64,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub last_emailed: f64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub last_opened: f64,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub updated_at: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbRecipient {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub recipients: Vec<Recipients>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailSettingsPatch {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailSettingsForwardBounce {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailBatchId {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub batch_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreditAllocation {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubuserPost {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub authorization_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_allocation: Option<CreditAllocation>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signup_session_token: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub user_id: f64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbRecipientCount {
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub recipient_count: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbCustomFieldWithValue {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// All of the following types are flattened into one object:
///
/// - `ContactdbCustomFieldWithAllOf`
/// - `ContactdbCustomFieldWithValue`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactdbCustomFieldWithValueAllOf {
    #[serde(flatten)]
    pub contactdb_custom_field_with_all_of: ContactdbCustomFieldWithAllOf,
    #[serde(flatten)]
    pub contactdb_custom_field_with_value: ContactdbCustomFieldWithValue,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TransactionalTemplateVersionCreate {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub active: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor: Option<Editor>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub generate_plain_content: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_content: String,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub plain_content: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub test_data: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TransactionalTemplatesVersionOutputLean {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub active: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor: Option<Editor>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub generate_plain_content: bool,
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub template_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub thumbnail_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
}

/**
* Defines the generation of the template.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Generation {
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Generation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Generation::Dynamic => "dynamic",
            Generation::Legacy => "legacy",
            Generation::Noop => "",
            Generation::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Generation {
    fn default() -> Generation {
        Generation::Noop
    }
}
impl Generation {
    pub fn is_noop(&self) -> bool {
        matches!(self, Generation::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TransactionalTemplatesTemplateLean {
    /**
     * Defines the generation of the template.
     */
    #[serde(default, skip_serializing_if = "Generation::is_noop")]
    pub generation: Generation,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "updated_at "
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub versions: Vec<TransactionalTemplatesVersionOutputLean>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactDetails {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<SelfMetadata>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_1: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_2: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub alternate_emails: Vec<String>,
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
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Help>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub last_name: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state_province_region: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
}

/// Result map of the import job.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Results {
    /**
     * Result map of the import job.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub created_count: f64,
    /**
     * Result map of the import job.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub deleted_count: f64,
    /**
     * Result map of the import job.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub errored_count: f64,
    /**
     * Result map of the import job.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub errors_url: String,
    /**
     * Result map of the import job.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub requested_count: f64,
    /**
     * Result map of the import job.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub updated_count: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactImport {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub finished_at: String,
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
    pub job_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Results>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub started_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CustomFields {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "custom_field_name1"
    )]
    pub custom_field_name_1: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "custom_field_name2"
    )]
    pub custom_field_name_2: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Contact {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_1: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_2: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub alternate_emails: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub primary_email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state_province_region: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SingleContactRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
}

/**
* The export job's status. Allowed values: `pending`, `ready`, or `failure`.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ContactExportStatus {
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ContactExportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContactExportStatus::Failure => "failure",
            ContactExportStatus::Pending => "pending",
            ContactExportStatus::Ready => "ready",
            ContactExportStatus::Noop => "",
            ContactExportStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ContactExportStatus {
    fn default() -> ContactExportStatus {
        ContactExportStatus::Noop
    }
}
impl ContactExportStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ContactExportStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactExport {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<Metadata>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub completed_at: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub contact_count: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expires_at: String,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub message: String,
    /**
     * The export job's status. Allowed values: `pending`, `ready`, or `failure`.
     */
    #[serde(default, skip_serializing_if = "ContactExportStatus::is_noop")]
    pub status: ContactExportStatus,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub urls: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactSummary {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<SelfMetadata>,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub created_at: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub last_name: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub updated_at: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_1: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_2: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub alternate_emails: Vec<String>,
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
    pub custom_fields: Option<Help>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state_province_region: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactDetails2 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<SelfMetadata>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_1: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_2: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub alternate_emails: Vec<String>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Help>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub facebook: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub line: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_number: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segment_ids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state_province_region: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unique_name: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub whatsapp: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SelfMetadata {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Error {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parameter: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Link {
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
    pub rel: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Metadata {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub count: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub prev: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Webhook {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nonce: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct List {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<SelfMetadata>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub contact_count: i64,
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
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FieldType {
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "Number")]
    Number,
    #[serde(rename = "Text")]
    Text,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Date => "Date",
            FieldType::Number => "Number",
            FieldType::Text => "Text",
            FieldType::Noop => "",
            FieldType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FieldType {
    fn default() -> FieldType {
        FieldType::Noop
    }
}
impl FieldType {
    pub fn is_noop(&self) -> bool {
        matches!(self, FieldType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReservedFieldDefinitionsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<FieldType>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub read_only: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CustomFieldDefinitionsResponse {
    #[serde(default, skip_serializing_if = "FieldType::is_noop")]
    pub field_type: FieldType,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentWrite {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub query_dsl: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentSummary {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub contacts_count: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The license key provided with your New Relic account.
     */
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sample_update: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parent_list_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub sample_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct L {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub t: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub v: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentQueryJsonContactsL {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l: Option<L>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub op: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<L>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentQueryJsonContactsLR {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub args: Vec<L>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub t: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub v: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct R {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l: Option<SegmentQueryJsonContactsLR>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub op: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<L>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentQueryJsonContactsLData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l: Option<SegmentQueryJsonContactsL>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub op: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<R>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentQueryJsonContactsR {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub t: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub v: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentQueryJsonContactsRData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l: Option<L>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub op: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<SegmentQueryJsonContactsR>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentQueryJsonContacts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l: Option<SegmentQueryJsonContactsLData>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub op: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<SegmentQueryJsonContactsRData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentQueryJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<SegmentQueryJsonContacts>,
}

/// The user may choose to create up to 120 custom fields or none at all. This is not a reserved field.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactResponseCustomFields {
    /**
     * The user may choose to create up to 120 custom fields or none at all. This is not a reserved field.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "custom_field_name1"
    )]
    pub custom_field_name_1: String,
    /**
     * The user may choose to create up to 120 custom fields or none at all. This is not a reserved field.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "custom_field_name2"
    )]
    pub custom_field_name_2: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_1: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_2: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub alternate_emails: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * The user may choose to create up to 120 custom fields or none at all. This is not a reserved field.
     */
    #[serde()]
    pub custom_fields: ContactResponseCustomFields,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub postal_code: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segment_ids: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state_province_region: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TneSenderId {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
}

/// Only verified sender identities can be used to send email.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Verified {
    /**
     * Only verified sender identities can be used to send email.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * Only verified sender identities can be used to send email.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub status: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TneSenderData {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created_at: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub updated_at: i64,
    /**
     * Only verified sender identities can be used to send email.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<Verified>,
}

/// All of the following types are flattened into one object:
///
/// - `TneSenderId`
/// - `SendersRequestBody`
/// - `TneSenderData`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TneSenderAllOf {
    #[serde(flatten)]
    pub tne_sender_id: TneSenderId,
    #[serde(flatten)]
    pub senders_request_body: SendersRequestBody,
    #[serde(flatten)]
    pub tne_sender_data: TneSenderData,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApiError {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApiErrors {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<ApiError>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MetadataType {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub prev: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DesignInput {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_content: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_content: String,
}

/// All of the following types are flattened into one object:
///
/// - `DesignDuplicateInput`
/// - `DesignCommonFieldsAllOf`
/// - `DesignInput`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DesignInputAllOf {
    #[serde(flatten)]
    pub design_duplicate_input: DesignDuplicateInput,
    #[serde(flatten)]
    pub design_common_fields_all_of: DesignCommonFieldsAllOf,
    #[serde(flatten)]
    pub design_input: DesignInput,
}

/// All of the following types are flattened into one object:
///
/// - `DesignInputAllOf`
/// - `DesignOutputSummaryAllOf`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DesignOutputAllOf {
    #[serde(flatten)]
    pub design_input_all_of: DesignInputAllOf,
    #[serde(flatten)]
    pub design_output_summary_all_of: DesignOutputSummaryAllOf,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DesignOutputSummary {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub thumbnail_url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// All of the following types are flattened into one object:
///
/// - `DesignDuplicateInput`
/// - `DesignCommonFieldsAllOf`
/// - `DesignOutputSummary`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DesignOutputSummaryAllOf {
    #[serde(flatten)]
    pub design_duplicate_input: DesignDuplicateInput,
    #[serde(flatten)]
    pub design_common_fields_all_of: DesignCommonFieldsAllOf,
    #[serde(flatten)]
    pub design_output_summary: DesignOutputSummary,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DesignDuplicateInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor: Option<Editor>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactDetails3 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<SelfMetadata>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_1: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_line_2: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub alternate_emails: Vec<String>,
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
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Help>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub facebook: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub line: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_number: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segment_ids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state_province_region: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unique_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub whatsapp: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Warning {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ErrorsData {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parameter: String,
}

/// If the request is incorrect, an array of errors will be returned.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ErrorsDataType {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<ErrorsData>,
}

/**
* This is the A/B phase of the Single Send stat returned. If the `group_by` parameter doesn't include `ab_phase` in the request, then the value is "all".
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AbPhase {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "test")]
    Test,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AbPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbPhase::All => "all",
            AbPhase::Send => "send",
            AbPhase::Test => "test",
            AbPhase::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AbPhase {
    fn default() -> AbPhase {
        AbPhase::All
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendsResponseResults {
    /**
     * This is the A/B phase of the Single Send stat returned. If the `group_by` parameter doesn't include `ab_phase` in the request, then the value is "all".
     */
    #[serde(default)]
    pub ab_phase: AbPhase,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ab_variation: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub aggregation: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<MetricsData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendsResponse {
    #[serde(rename = "_metadata")]
    pub metadata: Metadata,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<SinglesendsResponseResults>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationsResponseResults {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub aggregation: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<MetricsData>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub step_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<Metadata>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<AutomationsResponseResults>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MetricsData {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounce_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounces: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub delivered: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub invalid_emails: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub requests: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub spam_report_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub spam_reports: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribes: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SinglesendSearchStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "triggered")]
    Triggered,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SinglesendSearchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SinglesendSearchStatus::Draft => "draft",
            SinglesendSearchStatus::Scheduled => "scheduled",
            SinglesendSearchStatus::Triggered => "triggered",
            SinglesendSearchStatus::Noop => "",
            SinglesendSearchStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SinglesendSearchStatus {
    fn default() -> SinglesendSearchStatus {
        SinglesendSearchStatus::Noop
    }
}
impl SinglesendSearchStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, SinglesendSearchStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendSearch {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
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
    pub status: Vec<SinglesendSearchStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SendTo {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub all: bool,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segment_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailConfig {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_unsubscribe_url: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub design_id: String,
    /**
     * The editor used in the UI.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor: Option<Editor>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub generate_plain_content: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_content: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_pool: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_content: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub sender_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub suppression_group_id: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendRequest {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_config: Option<EmailConfig>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_to: Option<SendTo>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendSchedule {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SinglesendSearchStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Warnings {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub warning_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendWarning {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub warnings: Vec<Warnings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReplyTo {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebhooksEventWebhookUpdateWithOAuthRequest {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounce: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferred: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivered: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dropped: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_resubscribe: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_unsubscribe: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_token_url: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processed: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spam_report: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe: Option<bool>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebhooksEventWebhookRequest {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounce: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferred: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivered: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dropped: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_resubscribe: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_unsubscribe: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_token_url: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processed: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spam_report: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe: Option<bool>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReplyEmailObject {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationsLinkStatsResponseResults {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub step_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub url_location: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationsLinkStatsResponse {
    #[serde(rename = "_metadata")]
    pub metadata: LinkTrackingMetadata,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<AutomationsLinkStatsResponseResults>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_clicks: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LinkTrackingMetadata {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub count: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub prev: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendsLinkStatsResponseResults {
    /**
     * This is the A/B phase of the Single Send stat returned. If the `group_by` parameter doesn't include `ab_phase` in the request, then the value is "all".
     */
    #[serde(default)]
    pub ab_phase: AbPhase,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ab_variation: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub url_location: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendsLinkStatsResponse {
    #[serde(rename = "_metadata")]
    pub metadata: LinkTrackingMetadata,
    /**
     * This is the index of the link's location in the email contents.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<SinglesendsLinkStatsResponseResults>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_clicks: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Subusers {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub user_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DomainAuthentication200Response {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub last_validation_attempt_at: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub subusers: Vec<Subusers>,
}

/// All of the following types are flattened into one object:
///
/// - `AuthenticationDomain`
/// - `DomainAuthentication200Response`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DomainAuthentication200ResponseAllOf {
    #[serde(flatten)]
    pub authentication_domain: AuthenticationDomain,
    #[serde(flatten)]
    pub domain_authentication_200_response: DomainAuthentication200Response,
}

/**
* What differs between the A/B tests
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AbTestSummaryType {
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "subject")]
    Subject,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AbTestSummaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbTestSummaryType::Content => "content",
            AbTestSummaryType::Subject => "subject",
            AbTestSummaryType::Noop => "",
            AbTestSummaryType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AbTestSummaryType {
    fn default() -> AbTestSummaryType {
        AbTestSummaryType::Noop
    }
}
impl AbTestSummaryType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AbTestSummaryType::Noop)
    }
}

/**
* How the winner will be decided
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum WinnerCriteria {
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for WinnerCriteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WinnerCriteria::Click => "click",
            WinnerCriteria::Manual => "manual",
            WinnerCriteria::Open => "open",
            WinnerCriteria::Noop => "",
            WinnerCriteria::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for WinnerCriteria {
    fn default() -> WinnerCriteria {
        WinnerCriteria::Noop
    }
}
impl WinnerCriteria {
    pub fn is_noop(&self) -> bool {
        matches!(self, WinnerCriteria::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AbTestSummary {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub duration: String,
    /**
     * Last day to select an A/B Test Winner
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub expiration_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub test_percentage: i64,
    /**
     * What differs between the A/B tests
     */
    #[serde(
        default,
        skip_serializing_if = "AbTestSummaryType::is_noop",
        rename = "type"
    )]
    pub type_: AbTestSummaryType,
    /**
     * How the winner will be decided
     */
    #[serde(default, skip_serializing_if = "WinnerCriteria::is_noop")]
    pub winner_criteria: WinnerCriteria,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub winner_selected_at: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub winning_template_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendResponseShort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abtest: Option<AbTestSummary>,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_abtest: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "SinglesendSearchStatus::is_noop")]
    pub status: SinglesendSearchStatus,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CcBccEmailObject {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VerifiedSenderRequestSchema {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
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
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nickname: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VerifiedSenderResponseSchema {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
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
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nickname: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Result {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created_at: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IpAccessResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<Result>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct StatsAdvancedGlobal {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub blocks: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounce_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounces: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub deferred: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub delivered: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub invalid_emails: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub processed: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub requests: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub spam_report_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub spam_reports: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribe_drops: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribes: i64,
}

/// All of the following types are flattened into one object:
///
/// - `AdvancedStatsClicksOpensAllOf`
/// - `StatsAdvancedGlobal`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct StatsAdvancedGlobalAllOf {
    #[serde(flatten)]
    pub advanced_stats_clicks_opens_all_of: AdvancedStatsClicksOpensAllOf,
    #[serde(flatten)]
    pub stats_advanced_global: StatsAdvancedGlobal,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct StatsAdvancedBaseSchema {
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Help>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct StatsAdvancedBaseSchemaData {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The individual email activity stats.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stats: Vec<StatsAdvancedBaseSchema>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FullSegment {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub contacts_sample: Vec<ContactResponse>,
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_json: Option<Help>,
}

/// All of the following types are flattened into one object:
///
/// - `SegmentSummary`
/// - `FullSegment`
/// - `SegmentWriteV2`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FullSegmentAllOf {
    #[serde(flatten)]
    pub segment_summary: SegmentSummary,
    #[serde(flatten)]
    pub full_segment: FullSegment,
    #[serde(flatten)]
    pub segment_write_v2: SegmentWriteV2,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct From {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SendersRequestBody {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_2: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    #[serde()]
    pub from: From,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nickname: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<ReplyTo>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EnforcedTlsRequestResponse {
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub require_tls: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub require_valid_cert: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendResponse {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "SinglesendSearchStatus::is_noop")]
    pub status: SinglesendSearchStatus,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub warnings: Vec<Warnings>,
}

/// All of the following types are flattened into one object:
///
/// - `SinglesendRequest`
/// - `SinglesendResponse`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SinglesendResponseAllOf {
    #[serde(flatten)]
    pub singlesend_request: SinglesendRequest,
    #[serde(flatten)]
    pub singlesend_response: SinglesendResponse,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DesignCommonFields {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub generate_plain_content: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
}

/// All of the following types are flattened into one object:
///
/// - `DesignDuplicateInput`
/// - `DesignCommonFields`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DesignCommonFieldsAllOf {
    #[serde(flatten)]
    pub design_duplicate_input: DesignDuplicateInput,
    #[serde(flatten)]
    pub design_common_fields: DesignCommonFields,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InvalidEmail {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
}

/**
* The message's status.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailActivityResponseCommonFieldsStatus {
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "not delivered")]
    NotDelivered,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailActivityResponseCommonFieldsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailActivityResponseCommonFieldsStatus::Delivered => "delivered",
            EmailActivityResponseCommonFieldsStatus::NotDelivered => "not delivered",
            EmailActivityResponseCommonFieldsStatus::Processed => "processed",
            EmailActivityResponseCommonFieldsStatus::Noop => "",
            EmailActivityResponseCommonFieldsStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailActivityResponseCommonFieldsStatus {
    fn default() -> EmailActivityResponseCommonFieldsStatus {
        EmailActivityResponseCommonFieldsStatus::Noop
    }
}
impl EmailActivityResponseCommonFieldsStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailActivityResponseCommonFieldsStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailActivityResponseCommonFields {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub msg_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EmailActivityResponseCommonFieldsStatus>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to_email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SuppressionsRequestBody {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub recipient_emails: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SuppressionGroupRequestBase {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_default: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoCertificateBody {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub intergration_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub not_after: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub not_before: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_certificate: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoIntegration {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub audience_url: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub last_updated: f64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_signon_url: String,
}

/// All of the following types are flattened into one object:
///
/// - `CreateIntegrationRequest`
/// - `SsoIntegration`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoIntegrationAllOf {
    #[serde(flatten)]
    pub create_integration_request: CreateIntegrationRequest,
    #[serde(flatten)]
    pub sso_integration: SsoIntegration,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreateIntegrationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_integration: Option<bool>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub entity_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signin_url: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signout_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoTeammateResponse {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_sso: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

/// All of the following types are flattened into one object:
///
/// - `SsoTeammateCommonFields`
/// - `SsoTeammateResponse`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoTeammateResponseAllOf {
    #[serde(flatten)]
    pub sso_teammate_common_fields: SsoTeammateCommonFields,
    #[serde(flatten)]
    pub sso_teammate_response: SsoTeammateResponse,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetScopesResponse {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
}

/// All of the following types are flattened into one object:
///
/// - `SsoTeammateCommonFields`
/// - `GetScopesResponse`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoTeammateRequestAllOf {
    #[serde(flatten)]
    pub sso_teammate_common_fields: SsoTeammateCommonFields,
    #[serde(flatten)]
    pub get_scopes_response: GetScopesResponse,
}

/**
* A Teammate can be an “admin,” “owner,” or “teammate.” Each role is associated with the scope of the Teammate’s permissions.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UserType {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "teammate")]
    Teammate,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::Admin => "admin",
            UserType::Owner => "owner",
            UserType::Teammate => "teammate",
            UserType::Noop => "",
            UserType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UserType {
    fn default() -> UserType {
        UserType::Noop
    }
}
impl UserType {
    pub fn is_noop(&self) -> bool {
        matches!(self, UserType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoTeammatesPatchResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * A Teammate can be an “admin,” “owner,” or “teammate.” Each role is associated with the scope of the Teammate’s permissions.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub website: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// All of the following types are flattened into one object:
///
/// - `SsoTeammateResponseAllOf`
/// - `SsoTeammatesPatchResponse`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoTeammatesPatchResponseAllOf {
    #[serde(flatten)]
    pub sso_teammate_response_all_of: SsoTeammateResponseAllOf,
    #[serde(flatten)]
    pub sso_teammates_patch_response: SsoTeammatesPatchResponse,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoErrorResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ClickTracking {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable_text: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SsoTeammateCommonFields {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_admin: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_read_only: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SpamReportsResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BlocksResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IpPoolsPoolResp {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SenderRequestFrom {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SenderRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_2: String,
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
    pub from: Option<SenderRequestFrom>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nickname: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<SenderRequestFrom>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// Segment status indicates whether the segment's contacts will be updated periodically
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentStatusResponse {
    /**
     * Segment status indicates whether the segment's contacts will be updated periodically
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_message: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub query_validation: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AllSegmentsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<MetadataType>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub contacts_count: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sample_update: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub parent_list_ids: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub query_version: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sample_updated_at: String,
    /**
     * Segment status indicates whether the segment's contacts will be updated periodically
     */
    #[serde()]
    pub status: SegmentStatusResponse,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentSummaryV2 {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<SegmentSummary>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub contacts_count: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub contacts_sample: Vec<ContactResponse>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sample_update: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub parent_list_ids: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub query_dsl: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub query_version: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sample_updated_at: String,
    /**
     * Segment status indicates whether the segment's contacts will be updated periodically
     */
    #[serde()]
    pub status: SegmentStatusResponse,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ErrorsSeg {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// If the request is incorrect, an array of errors will be returned.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ErrorsSegData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<ErrorsSeg>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentWriteV2 {
    /**
     * The license key provided with your New Relic account.
     */
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
    pub parent_list_ids: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub query_dsl: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentUpdate {
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
    pub query_dsl: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AbbvMessageStatus {
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "not_delivered")]
    NotDelivered,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AbbvMessageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbbvMessageStatus::Delivered => "delivered",
            AbbvMessageStatus::NotDelivered => "not_delivered",
            AbbvMessageStatus::Processed => "processed",
            AbbvMessageStatus::Noop => "",
            AbbvMessageStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AbbvMessageStatus {
    fn default() -> AbbvMessageStatus {
        AbbvMessageStatus::Noop
    }
}
impl AbbvMessageStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, AbbvMessageStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Messages {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks_count: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_event_time: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub msg_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens_count: i64,
    #[serde(default, skip_serializing_if = "AbbvMessageStatus::is_noop")]
    pub status: AbbvMessageStatus,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to_email: String,
}

/**
* Name of event
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EventName {
    #[serde(rename = "bounced")]
    Bounced,
    #[serde(rename = "clicked")]
    Clicked,
    #[serde(rename = "deferred")]
    Deferred,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "dropped")]
    Dropped,
    #[serde(rename = "group_resubscribe")]
    GroupResubscribe,
    #[serde(rename = "group_unsubscribe")]
    GroupUnsubscribe,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "spam_report")]
    SpamReport,
    #[serde(rename = "unsubscribe")]
    Unsubscribe,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EventName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventName::Bounced => "bounced",
            EventName::Clicked => "clicked",
            EventName::Deferred => "deferred",
            EventName::Delivered => "delivered",
            EventName::Dropped => "dropped",
            EventName::GroupResubscribe => "group_resubscribe",
            EventName::GroupUnsubscribe => "group_unsubscribe",
            EventName::Opened => "opened",
            EventName::Processed => "processed",
            EventName::SpamReport => "spam_report",
            EventName::Unsubscribe => "unsubscribe",
            EventName::Noop => "",
            EventName::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EventName {
    fn default() -> EventName {
        EventName::Noop
    }
}
impl EventName {
    pub fn is_noop(&self) -> bool {
        matches!(self, EventName::Noop)
    }
}

/**
* Use to distinguish between types of bounces
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum BounceType {
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "bounced")]
    Bounced,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for BounceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BounceType::Blocked => "blocked",
            BounceType::Bounced => "bounced",
            BounceType::Expired => "expired",
            BounceType::Noop => "",
            BounceType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for BounceType {
    fn default() -> BounceType {
        BounceType::Noop
    }
}
impl BounceType {
    pub fn is_noop(&self) -> bool {
        matches!(self, BounceType::Noop)
    }
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Event {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub attempt_num: i64,
    /**
     * Use to distinguish between types of bounces
     */
    #[serde(default, skip_serializing_if = "BounceType::is_noop")]
    pub bounce_type: BounceType,
    /**
     * Name of event
     */
    #[serde(default, skip_serializing_if = "EventName::is_noop")]
    pub event_name: EventName,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub http_user_agent: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mx_server: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub processed: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
* Whether or not the outbound IP is dedicated vs shared
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OutboundIpType {
    #[serde(rename = "dedicated")]
    Dedicated,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OutboundIpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutboundIpType::Dedicated => "dedicated",
            OutboundIpType::Shared => "shared",
            OutboundIpType::Noop => "",
            OutboundIpType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OutboundIpType {
    fn default() -> OutboundIpType {
        OutboundIpType::Noop
    }
}
impl OutboundIpType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OutboundIpType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Message {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub api_key_id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub asm_group_id: i64,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
    /**
     * List of events related to email message
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub events: Vec<Event>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub msg_id: String,
    /**
     * This is the IP of the user who sent the message.
     */
    #[serde()]
    pub originating_ip: std::net::Ipv4Addr,
    /**
     * This is the IP of the user who sent the message.
     */
    #[serde()]
    pub outbound_ip: std::net::Ipv4Addr,
    /**
     * Whether or not the outbound IP is dedicated vs shared
     */
    #[serde(default, skip_serializing_if = "OutboundIpType::is_noop")]
    pub outbound_ip_type: OutboundIpType,
    /**
     * The message's status.
     */
    #[serde(
        default,
        skip_serializing_if = "EmailActivityResponseCommonFieldsStatus::is_noop"
    )]
    pub status: EmailActivityResponseCommonFieldsStatus,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teammate: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub template_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to_email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unique_args: String,
}

/// The DNS records for this authenticated domain.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DomainAuthenticationDns {
    /**
     * The DKIM record for messages sent using this authenticated domain.
     */
    #[serde()]
    pub dkim: Dkim,
    /**
     * The DKIM record for messages sent using this authenticated domain.
     */
    #[serde()]
    pub domain_spf: Dkim,
    /**
     * The DKIM record for messages sent using this authenticated domain.
     */
    #[serde()]
    pub mail_server: Dkim,
    /**
     * The DKIM record for messages sent using this authenticated domain.
     */
    #[serde()]
    pub subdomain_spf: Dkim,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DomainAuthentication {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub automatic_security: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub custom_spf: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    /**
     * The DNS records for this authenticated domain.
     */
    #[serde()]
    pub dns: DomainAuthenticationDns,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ips: Vec<String>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub legacy: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subdomain: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub user_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

/// The DNS records used to authenticate the sending domain.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthenticationDomainDns {
    /**
     * The DKIM record for messages sent using this authenticated domain.
     */
    #[serde(rename = "dkim1")]
    pub dkim_1: Dkim,
    /**
     * The DKIM record for messages sent using this authenticated domain.
     */
    #[serde(rename = "dkim2")]
    pub dkim_2: Dkim,
    /**
     * The DKIM record for messages sent using this authenticated domain.
     */
    #[serde()]
    pub mail_cname: Dkim,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AuthenticationDomain {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub automatic_security: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub custom_spf: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    /**
     * The DNS records used to authenticate the sending domain.
     */
    #[serde()]
    pub dns: AuthenticationDomainDns,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ips: Vec<String>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub legacy: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subdomain: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub user_id: f64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

/**
* Dictates how the stats are time-sliced. Currently, `"total"` and `"day"` are supported.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AggregatedBy {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "total")]
    Total,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AggregatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregatedBy::Day => "day",
            AggregatedBy::Total => "total",
            AggregatedBy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AggregatedBy {
    fn default() -> AggregatedBy {
        AggregatedBy::Total
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GroupBy {
    #[serde(rename = "ab_phase")]
    AbPhase,
    #[serde(rename = "ab_variation")]
    AbVariation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GroupBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupBy::AbPhase => "ab_phase",
            GroupBy::AbVariation => "ab_variation",
            GroupBy::Noop => "",
            GroupBy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GroupBy {
    fn default() -> GroupBy {
        GroupBy::Noop
    }
}
impl GroupBy {
    pub fn is_noop(&self) -> bool {
        matches!(self, GroupBy::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TraitAutomationQueryParamsGroupBy {
    #[serde(rename = "step_id")]
    StepId,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TraitAutomationQueryParamsGroupBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraitAutomationQueryParamsGroupBy::StepId => "step_id",
            TraitAutomationQueryParamsGroupBy::Noop => "",
            TraitAutomationQueryParamsGroupBy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TraitAutomationQueryParamsGroupBy {
    fn default() -> TraitAutomationQueryParamsGroupBy {
        TraitAutomationQueryParamsGroupBy::Noop
    }
}
impl TraitAutomationQueryParamsGroupBy {
    pub fn is_noop(&self) -> bool {
        matches!(self, TraitAutomationQueryParamsGroupBy::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AbPhaseId {
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AbPhaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbPhaseId::Send => "send",
            AbPhaseId::Test => "test",
            AbPhaseId::Noop => "",
            AbPhaseId::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AbPhaseId {
    fn default() -> AbPhaseId {
        AbPhaseId::Noop
    }
}
impl AbPhaseId {
    pub fn is_noop(&self) -> bool {
        matches!(self, AbPhaseId::Noop)
    }
}

/**
* How to group the statistics. Must be either "day", "week", or "month".
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TraitStatsAdvancedBaseQueryStringsAggregatedBy {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TraitStatsAdvancedBaseQueryStringsAggregatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraitStatsAdvancedBaseQueryStringsAggregatedBy::Day => "day",
            TraitStatsAdvancedBaseQueryStringsAggregatedBy::Month => "month",
            TraitStatsAdvancedBaseQueryStringsAggregatedBy::Week => "week",
            TraitStatsAdvancedBaseQueryStringsAggregatedBy::Noop => "",
            TraitStatsAdvancedBaseQueryStringsAggregatedBy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TraitStatsAdvancedBaseQueryStringsAggregatedBy {
    fn default() -> TraitStatsAdvancedBaseQueryStringsAggregatedBy {
        TraitStatsAdvancedBaseQueryStringsAggregatedBy::Noop
    }
}
impl TraitStatsAdvancedBaseQueryStringsAggregatedBy {
    pub fn is_noop(&self) -> bool {
        matches!(self, TraitStatsAdvancedBaseQueryStringsAggregatedBy::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TraitPagination200Response {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<Metadata>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TraitGlobalErrors500Response {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMessagesMsgResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<TraitGlobalErrors500Response>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TraitCancelScheduledSendsErrors400Response {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help: Option<Help>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TraitCancelScheduledSendsErrors400ResponseData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<TraitCancelScheduledSendsErrors400Response>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TraitMakoErrorResponse400Errors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parameter: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TraitMakoErrorResponse400 {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<TraitMakoErrorResponse400Errors>,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Personalizations {
    /**
     * An array of recipients who will receive a copy of your email. Each object in this array must contain the recipient's email address. Each object in the array may optionally contain the recipient's name.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub bcc: Vec<CcBccEmailObject>,
    /**
     * An array of recipients who will receive a copy of your email. Each object in this array must contain the recipient's email address. Each object in the array may optionally contain the recipient's name.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub cc: Vec<CcBccEmailObject>,
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_args: Option<Help>,
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_template_data: Option<Help>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FromEmailObject>,
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Help>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub send_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<Help>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub to: Vec<ReplyTo>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Content {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/**
* The attachment's content-disposition, specifying how you would like the attachment to be displayed. For example, `“inline”` results in the attached file are displayed automatically within the message while `“attachment”` results in the attached file require some action to be taken before it is displayed, such as opening or downloading the file.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Disposition {
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "inline")]
    Inline,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Disposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Disposition::Attachment => "attachment",
            Disposition::Inline => "inline",
            Disposition::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Disposition {
    fn default() -> Disposition {
        Disposition::Attachment
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Attachments {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_id: String,
    /**
     * The attachment's content-disposition, specifying how you would like the attachment to be displayed. For example, `“inline”` results in the attached file are displayed automatically within the message while `“attachment”` results in the attached file require some action to be taken before it is displayed, such as opening or downloading the file.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<Disposition>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filename: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// An object allowing you to specify how to handle unsubscribes.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Asm {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub group_id: i64,
    /**
     * The IDs of the lists you are sending this campaign to. You can have both segment IDs and list IDs
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub groups_to_display: Vec<i64>,
}

/// Allows you to bypass all unsubscribe groups and suppressions to ensure that the email is delivered to every single recipient. This should only be used in emergencies when it is absolutely necessary that every recipient receives your email. This filter cannot be combined with any other bypass filters. See our [documentation](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) for more about bypass filters.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BypassListManagement {
    /**
     * Allows you to bypass all unsubscribe groups and suppressions to ensure that the email is delivered to every single recipient. This should only be used in emergencies when it is absolutely necessary that every recipient receives your email. This filter cannot be combined with any other bypass filters. See our [documentation](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) for more about bypass filters.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
}

/// Allows you to bypass the spam report list to ensure that the email is delivered to recipients. Bounce and unsubscribe lists will still be checked; addresses on these other lists will not receive the message. This filter cannot be combined with the `bypass_list_management` filter. See our [documentation](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) for more about bypass filters.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BypassSpamManagement {
    /**
     * Allows you to bypass the spam report list to ensure that the email is delivered to recipients. Bounce and unsubscribe lists will still be checked; addresses on these other lists will not receive the message. This filter cannot be combined with the `bypass_list_management` filter. See our [documentation](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) for more about bypass filters.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
}

/// Allows you to bypass the bounce list to ensure that the email is delivered to recipients. Spam report and unsubscribe lists will still be checked; addresses on these other lists will not receive the message. This filter cannot be combined with the `bypass_list_management` filter. See our [documentation](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) for more about bypass filters.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BypassBounceManagement {
    /**
     * Allows you to bypass the bounce list to ensure that the email is delivered to recipients. Spam report and unsubscribe lists will still be checked; addresses on these other lists will not receive the message. This filter cannot be combined with the `bypass_list_management` filter. See our [documentation](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) for more about bypass filters.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
}

/// Allows you to bypass the global unsubscribe list to ensure that the email is delivered to recipients. Bounce and spam report lists will still be checked; addresses on these other lists will not receive the message. This filter applies only to global unsubscribes and will not bypass group unsubscribes. This filter cannot be combined with the `bypass_list_management` filter. See our [documentation](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) for more about bypass filters.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BypassUnsubscribeManagement {
    /**
     * Allows you to bypass the global unsubscribe list to ensure that the email is delivered to recipients. Bounce and spam report lists will still be checked; addresses on these other lists will not receive the message. This filter applies only to global unsubscribes and will not bypass group unsubscribes. This filter cannot be combined with the `bypass_list_management` filter. See our [documentation](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) for more about bypass filters.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
}

/// The default footer that you would like included on every email.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Footer {
    /**
     * The default footer that you would like included on every email.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
    /**
     * The default footer that you would like included on every email.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    /**
     * The default footer that you would like included on every email.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

/// Sandbox Mode allows you to send a test email to ensure that your request body is valid and formatted correctly.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SandboxMode {
    /**
     * Sandbox Mode allows you to send a test email to ensure that your request body is valid and formatted correctly.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
}

/// A collection of different mail settings that you can use to specify how you would like this email to be handled.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailSettings {
    /**
     * A collection of different mail settings that you can use to specify how you would like this email to be handled.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass_bounce_management: Option<BypassBounceManagement>,
    /**
     * A collection of different mail settings that you can use to specify how you would like this email to be handled.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass_list_management: Option<BypassListManagement>,
    /**
     * A collection of different mail settings that you can use to specify how you would like this email to be handled.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass_spam_management: Option<BypassSpamManagement>,
    /**
     * A collection of different mail settings that you can use to specify how you would like this email to be handled.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass_unsubscribe_management: Option<BypassUnsubscribeManagement>,
    /**
     * A collection of different mail settings that you can use to specify how you would like this email to be handled.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<Footer>,
    /**
     * A collection of different mail settings that you can use to specify how you would like this email to be handled.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sandbox_mode: Option<SandboxMode>,
}

/// Allows you to track if a recipient clicked a link in your email.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMailSendRequestTrackingSettingsClick {
    /**
     * Allows you to track if a recipient clicked a link in your email.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
    /**
     * Allows you to track if a recipient clicked a link in your email.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable_text: bool,
}

/// Allows you to track if the email was opened by including a single pixel image in the body of the content. When the pixel is loaded, Twilio SendGrid can log that the email was opened.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenTracking {
    /**
     * Allows you to track if the email was opened by including a single pixel image in the body of the content. When the pixel is loaded, Twilio SendGrid can log that the email was opened.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
    /**
     * Allows you to track if the email was opened by including a single pixel image in the body of the content. When the pixel is loaded, Twilio SendGrid can log that the email was opened.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub substitution_tag: String,
}

/// Allows you to insert a subscription management link at the bottom of the text and HTML bodies of your email. If you would like to specify the location of the link within your email, you may use the `substitution_tag`.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriptionTracking {
    /**
     * Allows you to insert a subscription management link at the bottom of the text and HTML bodies of your email. If you would like to specify the location of the link within your email, you may use the `substitution_tag`.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
    /**
     * Allows you to insert a subscription management link at the bottom of the text and HTML bodies of your email. If you would like to specify the location of the link within your email, you may use the `substitution_tag`.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    /**
     * Allows you to insert a subscription management link at the bottom of the text and HTML bodies of your email. If you would like to specify the location of the link within your email, you may use the `substitution_tag`.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub substitution_tag: String,
    /**
     * Allows you to insert a subscription management link at the bottom of the text and HTML bodies of your email. If you would like to specify the location of the link within your email, you may use the `substitution_tag`.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

/// Allows you to enable tracking provided by Google Analytics.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Ganalytics {
    /**
     * Allows you to enable tracking provided by Google Analytics.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable: bool,
    /**
     * Allows you to enable tracking provided by Google Analytics.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub utm_campaign: String,
    /**
     * Allows you to enable tracking provided by Google Analytics.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub utm_content: String,
    /**
     * Allows you to enable tracking provided by Google Analytics.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub utm_medium: String,
    /**
     * Allows you to enable tracking provided by Google Analytics.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub utm_source: String,
    /**
     * Allows you to enable tracking provided by Google Analytics.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub utm_term: String,
}

/// Settings to determine how you would like to track the metrics of how your recipients interact with your email.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TrackingSettings {
    /**
     * Settings to determine how you would like to track the metrics of how your recipients interact with your email.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click_tracking: Option<PostMailSendRequestTrackingSettingsClick>,
    /**
     * Settings to determine how you would like to track the metrics of how your recipients interact with your email.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ganalytics: Option<Ganalytics>,
    /**
     * Settings to determine how you would like to track the metrics of how your recipients interact with your email.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_tracking: Option<OpenTracking>,
    /**
     * Settings to determine how you would like to track the metrics of how your recipients interact with your email.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_tracking: Option<SubscriptionTracking>,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMailSendRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asm: Option<Asm>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub attachments: Vec<Attachments>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub batch_id: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
    /**
     * An array where you can specify the content of your email. You can include multiple [MIME types](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types) of content, but you must specify at least one MIME type. To include more than one MIME type, add another object to the array containing the `type` and `value` parameters.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub content: Vec<Content>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_args: String,
    #[serde()]
    pub from: FromEmailObject,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Help>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_pool_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail_settings: Option<MailSettings>,
    /**
     * An array of messages and their metadata. Each object within personalizations can be thought of as an envelope - it defines who should receive an individual message and how that message should be handled. See our [Personalizations documentation](https://sendgrid.com/docs/for-developers/sending-email/personalizations/) for examples.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub personalizations: Vec<Personalizations>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<ReplyEmailObject>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub send_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub template_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_settings: Option<TrackingSettings>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CancelPauseAScheduledSendRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub batch_id: String,
    /**
     * The status of the scheduled send.
     */
    #[serde(default, skip_serializing_if = "Status::is_noop")]
    pub status: Status,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetApiKeysResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<ApiKeyNameId>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreateApiKeysRequest {
    /**
     * The license key provided with your New Relic account.
     */
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
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreateApiKeysResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub api_key: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub api_key_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetApiKeysKeyResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<ApiKeyNameScopesAllOf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutApiKeysKeyRequest {
    /**
     * The license key provided with your New Relic account.
     */
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
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetScopesResponseErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<serde_json::Value>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetScopesResponseData {
    /**
     * This 401 response indicates that the user making the call doesn't have the authorization to view the list of scopes.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<GetScopesResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Ips {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostAccessSettingsWhitelistRequest {
    /**
     * An array containing the IP(s) you want to allow.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ips: Vec<Ips>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteAccessSettingsWhitelistRequest {
    /**
     * The IDs of the lists you are sending this campaign to. You can have both segment IDs and list IDs
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAccessSettingsActivityResponseResult {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub allowed: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub auth_method: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub first_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub last_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAccessSettingsActivityResponse {
    /**
     * An array containing the IPs that recently attempted to access your account.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<GetAccessSettingsActivityResponseResult>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostSsoCertificatesRequest {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub integration_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_certificate: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchSsoCertificatesCertRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub integration_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_certificate: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchSsoTeammatesUsernameRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMailSettingsResponseResult {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMailSettingsResponse {
    /**
     * The list of all mail settings.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<GetMailSettingsResponseResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchMailSettingsAddressWhitelistRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchMailSettingsTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_content: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchMailSettingsTemplateResponse {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_content: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchPartnerSettingsNewRelicRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_subuser_statistics: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub license_key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetPartnerSettingsResponse {
    /**
     * The list of all mail settings.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<GetMailSettingsResponseResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTeammatesResponseResult {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_admin: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * A Teammate can be an “admin,” “owner,” or “teammate.” Each role is associated with the scope of the Teammate’s permissions.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub website: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTeammatesResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<GetTeammatesResponseResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostTeammatesRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostTeammatesResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_admin: bool,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostTeammatesResponseErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostSendersResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<PostTeammatesResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetScopesRequestsResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scope_group_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTeammatesPendingResponseResult {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub expiration_date: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_admin: bool,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTeammatesPendingResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<GetTeammatesPendingResponseResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTeammatesUsernameResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_admin: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * A Teammate can be an “admin,” “owner,” or “teammate.” Each role is associated with the scope of the Teammate’s permissions.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub website: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchTeammatesUsernameRequest {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchScopesRequestsApproveResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scope_group_name: String,
}

/**
* The type of alert.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetAlertsResponseType {
    #[serde(rename = "stats_notification")]
    StatsNotification,
    #[serde(rename = "usage_limit")]
    UsageLimit,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetAlertsResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetAlertsResponseType::StatsNotification => "stats_notification",
            GetAlertsResponseType::UsageLimit => "usage_limit",
            GetAlertsResponseType::Noop => "",
            GetAlertsResponseType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetAlertsResponseType {
    fn default() -> GetAlertsResponseType {
        GetAlertsResponseType::Noop
    }
}
impl GetAlertsResponseType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetAlertsResponseType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAlertsResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_to: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub frequency: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub percentage: i64,
    /**
     * The type of alert.
     */
    #[serde(
        default,
        skip_serializing_if = "GetAlertsResponseType::is_noop",
        rename = "type"
    )]
    pub type_: GetAlertsResponseType,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostAlertsRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_to: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub frequency: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub percentage: i64,
    /**
     * The type of alert.
     */
    #[serde(
        default,
        skip_serializing_if = "GetAlertsResponseType::is_noop",
        rename = "type"
    )]
    pub type_: GetAlertsResponseType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostAlertsResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_to: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub frequency: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub percentage: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub updated_at: i64,
}

/**
* The type of alert.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetAlertsAlertResponseType {
    #[serde(rename = "stats_notification")]
    StatsNotification,
    #[serde(rename = "usage_alert")]
    UsageAlert,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetAlertsAlertResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetAlertsAlertResponseType::StatsNotification => "stats_notification",
            GetAlertsAlertResponseType::UsageAlert => "usage_alert",
            GetAlertsAlertResponseType::Noop => "",
            GetAlertsAlertResponseType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetAlertsAlertResponseType {
    fn default() -> GetAlertsAlertResponseType {
        GetAlertsAlertResponseType::Noop
    }
}
impl GetAlertsAlertResponseType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetAlertsAlertResponseType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAlertsAlertResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_to: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub frequency: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub percentage: i64,
    /**
     * The type of alert.
     */
    #[serde(
        default,
        skip_serializing_if = "GetAlertsAlertResponseType::is_noop",
        rename = "type"
    )]
    pub type_: GetAlertsAlertResponseType,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchAlertsAlertRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_to: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub frequency: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub percentage: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetUserProfileResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub website: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/**
* The type of account for this user.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetUserAccountResponseType {
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetUserAccountResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetUserAccountResponseType::Free => "free",
            GetUserAccountResponseType::Paid => "paid",
            GetUserAccountResponseType::Noop => "",
            GetUserAccountResponseType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetUserAccountResponseType {
    fn default() -> GetUserAccountResponseType {
        GetUserAccountResponseType::Noop
    }
}
impl GetUserAccountResponseType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetUserAccountResponseType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetUserAccountResponse {
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub reputation: f64,
    /**
     * The type of account for this user.
     */
    #[serde(
        default,
        skip_serializing_if = "GetUserAccountResponseType::is_noop",
        rename = "type"
    )]
    pub type_: GetUserAccountResponseType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetUserEmailResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutUserEmailRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutUserUsernameRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutUserUsernameResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetUserCreditsResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_reset: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_reset: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub overage: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub remain: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reset_frequency: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub used: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutUserPasswordRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub new_password: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub old_password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostSubusersRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The IP addresses that should be assigned to this subuser.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ips: Vec<std::net::Ipv4Addr>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub password: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchSubusersSubuserNameRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetSubusersReputationsResponse {
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub reputation: f64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutSubusersSubuserNameIpsResponse {
    /**
     * The IP addresses that should be assigned to this subuser.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ips: Vec<std::net::Ipv4Addr>,
}

/**
* The direction you want to sort.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SortByDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SortByDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortByDirection::Asc => "asc",
            SortByDirection::Desc => "desc",
            SortByDirection::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SortByDirection {
    fn default() -> SortByDirection {
        SortByDirection::Desc
    }
}

/**
* The metric that you want to sort by. Metrics that you can sort by are: `blocks`, `bounces`, `clicks`, `delivered`, `opens`, `requests`, `unique_clicks`, `unique_opens`, and `unsubscribes`.'
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SortByMetric {
    #[serde(rename = "blocks")]
    Blocks,
    #[serde(rename = "bounces")]
    Bounces,
    #[serde(rename = "clicks")]
    Clicks,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "opens")]
    Opens,
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "unique_clicks")]
    UniqueClicks,
    #[serde(rename = "unique_opens")]
    UniqueOpens,
    #[serde(rename = "unsubscribes")]
    Unsubscribes,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SortByMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortByMetric::Blocks => "blocks",
            SortByMetric::Bounces => "bounces",
            SortByMetric::Clicks => "clicks",
            SortByMetric::Delivered => "delivered",
            SortByMetric::Opens => "opens",
            SortByMetric::Requests => "requests",
            SortByMetric::UniqueClicks => "unique_clicks",
            SortByMetric::UniqueOpens => "unique_opens",
            SortByMetric::Unsubscribes => "unsubscribes",
            SortByMetric::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SortByMetric {
    fn default() -> SortByMetric {
        SortByMetric::Delivered
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelLinksRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub subdomain: String,
}

/// The DNS record generated for the sending domain used for this branded link.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelLinksValidateResponseValidationResultsDomainCname {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

/// The individual validation results for each of the DNS records associated with this branded link.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ValidationResults {
    /**
     * The DNS record generated for the sending domain used for this branded link.
     */
    #[serde()]
    pub domain_cname: PostWhitelabelLinksValidateResponseValidationResultsDomainCname,
    /**
     * The individual validation results for each of the DNS records associated with this branded link.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_cname: Option<PostWhitelabelLinksValidateResponseValidationResultsDomainCname>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelLinksValidateResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
    /**
     * The individual validation results for each of the DNS records associated with this branded link.
     */
    #[serde()]
    pub validation_results: ValidationResults,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelLinksValidateResponseErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingSegmentsResponse {
    /**
     * The reasons why the validation failed.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<PostWhitelabelLinksValidateResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelLinksLinkSubuserRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchWhitelabelLinksRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostIpsWarmupRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostIpsWarmupResponseErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<serde_json::Value>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostIpsWarmupResponse {
    /**
     * The errors that were encountered.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<PostIpsWarmupResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelIpsRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub subdomain: String,
}

/// The specific results of the validation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelIpsValidateResponseValidationResults {
    /**
     * The specific results of the validation.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a_record: Option<PostWhitelabelLinksValidateResponseValidationResultsDomainCname>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelIpsValidateResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
    /**
     * The specific results of the validation.
     */
    #[serde()]
    pub validation_results: PostWhitelabelIpsValidateResponseValidationResults,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostValidationsEmailRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub source: String,
}

/**
* A generic classification of whether or not the email address is valid.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Verdict {
    #[serde(rename = "Invalid")]
    Invalid,
    #[serde(rename = "Risky")]
    Risky,
    #[serde(rename = "Valid")]
    Valid,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verdict::Invalid => "Invalid",
            Verdict::Risky => "Risky",
            Verdict::Valid => "Valid",
            Verdict::Noop => "",
            Verdict::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Verdict {
    fn default() -> Verdict {
        Verdict::Noop
    }
}
impl Verdict {
    pub fn is_noop(&self) -> bool {
        matches!(self, Verdict::Noop)
    }
}

/// Checks on the domain portion of the email address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Domain {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_mx_or_a_record: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_valid_address_syntax: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_suspected_disposable_address: bool,
}

/// Checks on the local part of the email address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LocalPart {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_suspected_role_address: bool,
}

/// Additional checks on the email address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Additional {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_known_bounces: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_suspected_bounces: bool,
}

/// Granular checks for email address validity.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Checks {
    /**
     * Additional checks on the email address.
     */
    #[serde()]
    pub additional: Additional,
    /**
     * Checks on the domain portion of the email address.
     */
    #[serde()]
    pub domain: Domain,
    /**
     * Checks on the local part of the email address.
     */
    #[serde()]
    pub local_part: LocalPart,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostValidationsEmailResponseResult {
    /**
     * Granular checks for email address validity.
     */
    #[serde()]
    pub checks: Checks,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_address: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub local: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub score: f64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub source: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub suggestion: String,
    /**
     * A generic classification of whether or not the email address is valid.
     */
    #[serde(default, skip_serializing_if = "Verdict::is_noop")]
    pub verdict: Verdict,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostValidationsEmailResponse {
    #[serde()]
    pub result: PostValidationsEmailResponseResult,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelDnsEmailRequest {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub domain_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub link_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelDnsEmailResponseErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelDnsEmailResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<PostWhitelabelDnsEmailResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetIpsAssignedResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub pools: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_date: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub warmup: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetIpsPoolsPoolNameResponse {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ips: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pool_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutIpsPoolsPoolNameRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteIpsPoolsPoolNameResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetIpsResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub assigned_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub pools: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rdns: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub start_date: f64,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub subusers: Vec<String>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub warmup: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub whitelabeled: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostIpsRequest {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub subusers: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmup: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostIpsResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub subusers: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostIpsResponseData {
    /**
     * List of IP objects.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ips: Vec<PostIpsResponse>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub remaining_ips: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub warmup: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetIpsRemainingResponseResults {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub period: String,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price_per_ip: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub remaining: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetIpsRemainingResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<GetIpsRemainingResponseResults>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetIpsIpAddressResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub pools: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rdns: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub start_date: i64,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub subusers: Vec<String>,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub warmup: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub whitelabeled: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelDomainsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic_security: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_dkim_selector: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_spf: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ips: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subdomain: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchWhitelabelDomainsDomainRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_spf: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

/// The CNAME record for the authenticated domain.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MailCname {
    /**
     * The CNAME record for the authenticated domain.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * The CNAME record for the authenticated domain.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

/// A DNS record for this authenticated domain.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Dkim1 {
    /**
     * A DNS record for this authenticated domain.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * A DNS record for this authenticated domain.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

/// The SPF record for the authenticated domain.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Spf {
    /**
     * The SPF record for the authenticated domain.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * The SPF record for the authenticated domain.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
}

/// The individual DNS records that are checked when validating, including the reason for any invalid DNS records.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelDomainsValidateResponseValidationResults {
    /**
     * The individual DNS records that are checked when validating, including the reason for any invalid DNS records.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dkim1")]
    pub dkim_1: Option<Dkim1>,
    /**
     * The individual DNS records that are checked when validating, including the reason for any invalid DNS records.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dkim2")]
    pub dkim_2: Option<Dkim1>,
    /**
     * The individual DNS records that are checked when validating, including the reason for any invalid DNS records.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail_cname: Option<MailCname>,
    /**
     * The individual DNS records that are checked when validating, including the reason for any invalid DNS records.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spf: Option<Spf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelDomainsValidateResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub valid: bool,
    /**
     * The individual DNS records that are checked when validating, including the reason for any invalid DNS records.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_results: Option<PostWhitelabelDomainsValidateResponseValidationResults>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostWhitelabelDomainsValidateResponseData {
    /**
     * The reasons why the validation failed.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<PostWhitelabelLinksValidateResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetVerifiedSendersDomainsResponseResults {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub hard_failures: Vec<String>,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub soft_failures: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetVerifiedSendersDomainsResponse {
    #[serde()]
    pub results: GetVerifiedSendersDomainsResponseResults,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetVerifiedSendersStepsCompletedResponseResults {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub domain_verified: bool,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub sender_verified: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetVerifiedSendersStepsCompletedResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<GetVerifiedSendersStepsCompletedResponseResults>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetVerifiedSendersResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<VerifiedSenderResponseSchema>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostVerifiedSendersResponseErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostVerifiedSendersResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<PostVerifiedSendersResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetVerifiedSendersVerifyTokenResponseErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchVerifiedSendersResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<GetVerifiedSendersVerifyTokenResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteVerifiedSendersResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<GetVerifiedSendersVerifyTokenResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutDesignRequest {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_plain_content: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_content: String,
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
    pub plain_content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListDesignsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<MetadataType>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<DesignOutputSummaryAllOf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMcContatsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<SelfMetadata>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub contact_count: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<ContactDetails3>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMcListsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<Error>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutMcContactsRequest {
    /**
     * One or more contacts objects that you intend to upsert. The available fields for a contact, including the required `email` field are described below.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub contacts: Vec<ContactRequest>,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutMcContactsResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_id: String,
}

/// The deletion job has been accepted and is being processed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteMcContactsResponse {
    /**
     * helper text or docs for troubleshooting
     */
    #[serde()]
    pub job_id: Help,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteMcContactsResponseData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<Help>,
}

/// `billable_breakdown` will only appear to the parent user in an account with subusers.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BillableBreakdown {
    /**
     * `billable_breakdown` will only appear to the parent user in an account with subusers.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Help>,
    /**
     * `billable_breakdown` will only appear to the parent user in an account with subusers.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMcContactsCountResponse {
    /**
     * `billable_breakdown` will only appear to the parent user in an account with subusers.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billable_breakdown: Option<BillableBreakdown>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub billable_count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub contact_count: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Lists {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ID"
    )]
    pub id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "Name"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMarketingContactsExportsResponseResultMetadata {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub prev: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMarketingContactsExportsResponseResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<GetMarketingContactsExportsResponseResultMetadata>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub completed_at: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expires_at: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub export_type: String,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub lists: Vec<Lists>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segments: Vec<Lists>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub urls: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMarketingContactsExportsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<GetMarketingContactsExportsResponseResultMetadata>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<GetMarketingContactsExportsResponseResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMarketingContactsExportsResponseErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMarketingContactsExportsResponseData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<GetMarketingContactsExportsResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Notifications {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub email: bool,
}

/**
* File type for export file. Choose from `json` or `csv`.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FileType {
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "json")]
    Json,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Csv => "csv",
            FileType::Json => "json",
            FileType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FileType {
    fn default() -> FileType {
        FileType::Csv
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMcContactsExportsRequest {
    /**
     * File type for export file. Choose from `json` or `csv`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<FileType>,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub max_file_size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Notifications>,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segment_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMcContactsExportsResponse {
    #[serde(rename = "_metadata")]
    pub metadata: Metadata,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMcContactsSearchRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub query: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMcContactsSearchResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<SelfMetadata>,
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub contact_count: f64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<ContactDetails3>,
}

/**
* Upload file type.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PutMcContactsImportsRequestFileType {
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PutMcContactsImportsRequestFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PutMcContactsImportsRequestFileType::Csv => "csv",
            PutMcContactsImportsRequestFileType::Noop => "",
            PutMcContactsImportsRequestFileType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PutMcContactsImportsRequestFileType {
    fn default() -> PutMcContactsImportsRequestFileType {
        PutMcContactsImportsRequestFileType::Noop
    }
}
impl PutMcContactsImportsRequestFileType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PutMcContactsImportsRequestFileType::Noop)
    }
}

/// All of the following types are flattened into one object:
///
/// - `String`
/// - `serde_json::Value`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FieldMappingsAnyOf {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(flatten)]
    pub string: String,
    #[serde(flatten)]
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutMcContactsImportsRequest {
    /**
     * Import file header to reserved/custom field mapping.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub field_mappings: Vec<FieldMappingsAnyOf>,
    /**
     * Upload file type.
     */
    #[serde(
        default,
        skip_serializing_if = "PutMcContactsImportsRequestFileType::is_noop"
    )]
    pub file_type: PutMcContactsImportsRequestFileType,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UploadHeaders {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub header: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutMcContactsImportsResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_id: String,
    /**
     * A list of headers that must be included in PUT request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub upload_headers: Vec<UploadHeaders>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub upload_uri: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutMcContactsImportsResponseData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<Error>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMarketingContactsImportsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Error>,
}

/// Array of IDs
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingContactsBatchRequest {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingContactsBatchResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<ContactDetails3>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingContactsSearchEmailsRequest {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub emails: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingContactsSearchEmailsResponse {
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Help>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingSendersRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_2: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    #[serde()]
    pub from: From,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nickname: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<ReplyTo>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMcListsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<Metadata>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<List>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMcListsContactsCountResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub billable_count: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub contact_count: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMcListsResponseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_sample: Option<ContactDetails2>,
}

/// All of the following types are flattened into one object:
///
/// - `List`
/// - `GetMcListsResponseData`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMcListsResponseAllOf {
    #[serde(flatten)]
    pub list: List,
    #[serde(flatten)]
    pub get_mc_lists_response_data: GetMcListsResponseData,
}

/// The delete has been accepted and is processing.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteListsResponse {
    /**
     * The delete has been accepted and is processing.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchMcListsRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// The removal is accepted and processing.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteMcListsContactsResponse {
    /**
     * The removal is accepted and processing.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMcFieldDefinitionsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<MetadataType>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub custom_fields: Vec<CustomFieldDefinitionsResponse>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub reserved_fields: Vec<ReservedFieldDefinitionsResponse>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMcFieldDefinitionsRequest {
    #[serde(default, skip_serializing_if = "FieldType::is_noop")]
    pub field_type: FieldType,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMcFieldDefinitionsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<MetadataType>,
}

/// All of the following types are flattened into one object:
///
/// - `CustomFieldDefinitionsResponse`
/// - `PostMcFieldDefinitionsResponse`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMcFieldDefinitionsResponseAllOf {
    #[serde(flatten)]
    pub custom_field_definitions_response: CustomFieldDefinitionsResponse,
    #[serde(flatten)]
    pub post_mc_field_definitions_response: PostMcFieldDefinitionsResponse,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMarketingSegmentsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<SegmentSummary>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingSegmentsRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parent_list_id: String,
}

/// All of the following types are flattened into one object:
///
/// - `SegmentWriteV2`
/// - `PostMarketingSegmentsRequest`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingSegmentsRequestAllOf {
    #[serde(flatten)]
    pub segment_write_v2: SegmentWriteV2,
    #[serde(flatten)]
    pub post_marketing_segments_request: PostMarketingSegmentsRequest,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingSegmentsDeleteRequest {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ids: Vec<String>,
}

/// resources in which segment is being used
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Resources {
    /**
     * resources in which segment is being used
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ids: Vec<String>,
    /**
     * resources in which segment is being used
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingSegmentsDeleteResponseErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * resources in which segment is being used
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingSegmentsDeleteResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<PostMarketingSegmentsDeleteResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMarketingSinglesendsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<MetadataType>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<SinglesendResponseShort>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingSinglesendsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<SsoErrorResponse>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingSinglesendsRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
* This is the ISO 8601 time at which to send the Single Send; must be in future, or the string "now"
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SendAt {
    #[serde(rename = "now")]
    Now,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SendAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SendAt::Now => "now",
            SendAt::Noop => "",
            SendAt::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SendAt {
    fn default() -> SendAt {
        SendAt::Noop
    }
}
impl SendAt {
    pub fn is_noop(&self) -> bool {
        matches!(self, SendAt::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutMarketingSinglesendsScheduleRequest {
    /**
     * This is the ISO 8601 time at which to send the Single Send; must be in future, or the string "now"
     */
    #[serde(default, skip_serializing_if = "SendAt::is_noop")]
    pub send_at: SendAt,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PutMarketingSinglesendsScheduleResponseStatus {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PutMarketingSinglesendsScheduleResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PutMarketingSinglesendsScheduleResponseStatus::Scheduled => "scheduled",
            PutMarketingSinglesendsScheduleResponseStatus::Noop => "",
            PutMarketingSinglesendsScheduleResponseStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PutMarketingSinglesendsScheduleResponseStatus {
    fn default() -> PutMarketingSinglesendsScheduleResponseStatus {
        PutMarketingSinglesendsScheduleResponseStatus::Noop
    }
}
impl PutMarketingSinglesendsScheduleResponseStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, PutMarketingSinglesendsScheduleResponseStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutMarketingSinglesendsScheduleResponse {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PutMarketingSinglesendsScheduleResponseStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMarketingSinglesendsCategoriesResponse {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMarketingTestSendEmailRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_unsubscribe_url: String,
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub emails: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_address: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub sender_id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub suppression_group_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub template_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version_id_override: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetSendersResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<SenderAllOf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostSendersRequest {
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Help>,
    /**
     * helper text or docs for troubleshooting
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Help>,
}

/// All of the following types are flattened into one object:
///
/// - `SenderRequest`
/// - `PostSendersRequest`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostSendersRequestAllOf {
    #[serde(flatten)]
    pub sender_request: SenderRequest,
    #[serde(flatten)]
    pub post_senders_request: PostSendersRequest,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListAllListsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lists: Vec<ContactdbList>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchContactdbListsListResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
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
    pub recipient_count: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetContactdbRecipientsSearchResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub recipients: Vec<ContactdbRecipient>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListRecipientsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub recipients: Vec<Help>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostContactdbRecipientsRequest {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub age: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchContactdbRecipientsRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetContactdbStatusResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetContactdbStatusResponseData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub status: Vec<GetContactdbStatusResponse>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetContactdbRecipientsRecipientListsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lists: Vec<ContactdbList>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostContactdbRecipientsSearchRequest {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conditions: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub list_id: i64,
}

/// All of the following types are flattened into one object:
///
/// - `String`
/// - `i64`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ValueAnyOf {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(flatten)]
    pub string: String,
    #[serde(flatten)]
    pub i64: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostContactdbRecipientsSearchResponseCustomFields {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueAnyOf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostContactdbRecipientsSearchResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created_at: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub custom_fields: Vec<PostContactdbRecipientsSearchResponseCustomFields>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub last_clicked: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub last_emailed: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub last_opened: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostContactdbRecipientsSearchResponseData {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipient_count: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub recipients: Vec<PostContactdbRecipientsSearchResponse>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListAllCustomFieldsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub custom_fields: Vec<ContactdbCustomFieldWithAllOf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostContactdbCustomFieldsRequest {
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
pub struct ReservedFields {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetContactdbReservedFieldsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub reserved_fields: Vec<ReservedFields>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListAllSegmentsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segments: Vec<ContactdbSegments>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchContactdbSegmentsSegmentRequest {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conditions: Vec<ContactdbSegmentsConditions>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub list_id: f64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListRecipientsOnASegmentResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub recipients: Vec<ContactdbRecipient>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCategoriesResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCategoriesResponseData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<ErrorsSeg>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCampaignsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<CampaignResponseAllOf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCampaignsCampaignResponse {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_unsubscribe_url: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_content: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_pool: String,
    /**
     * The IDs of the lists you are sending this campaign to. You can have both segment IDs and list IDs
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub list_ids: Vec<i64>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_content: String,
    /**
     * The IDs of the lists you are sending this campaign to. You can have both segment IDs and list IDs
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segment_ids: Vec<i64>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub sender_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub suppression_group_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateACampaignRequest {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<String>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_content: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_content: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SendACampaignResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ScheduleACampaignRequest {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub send_at: i64,
}

/**
* The status of your campaign.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ScheduleACampaignResponseStatus {
    #[serde(rename = "Scheduled")]
    Scheduled,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ScheduleACampaignResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScheduleACampaignResponseStatus::Scheduled => "Scheduled",
            ScheduleACampaignResponseStatus::Noop => "",
            ScheduleACampaignResponseStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ScheduleACampaignResponseStatus {
    fn default() -> ScheduleACampaignResponseStatus {
        ScheduleACampaignResponseStatus::Noop
    }
}
impl ScheduleACampaignResponseStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ScheduleACampaignResponseStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ScheduleACampaignResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub send_at: i64,
    /**
     * The status of your campaign.
     */
    #[serde(
        default,
        skip_serializing_if = "ScheduleACampaignResponseStatus::is_noop"
    )]
    pub status: ScheduleACampaignResponseStatus,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateAScheduledCampaignResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub send_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SendATestCampaignRequest {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to: String,
}

/**
* Comma-delimited list specifying which generations of templates to return. Options are `legacy`, `dynamic` or `legacy,dynamic`.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Generations {
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "legacy,dynamic")]
    LegacyDynamic,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Generations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Generations::Dynamic => "dynamic",
            Generations::Legacy => "legacy",
            Generations::LegacyDynamic => "legacy,dynamic",
            Generations::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Generations {
    fn default() -> Generations {
        Generations::Legacy
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTemplatesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "_metadata")]
    pub metadata: Option<MetadataType>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<TransactionalTemplatesTemplateLean>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTemplatesResponseErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_id: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTemplatesResponseData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<GetTemplatesResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostTemplatesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<Generation>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostTemplatesTemplateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchTemplatesTemplateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetUserWebhooksParseSettingsResponse {
    /**
     * The list of your current inbound parse settings.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<ParseSetting>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetUserWebhooksParseStatsResponseMetrics {
    /**
     * The number of errors found while adding recipients.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub received: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetUserWebhooksParseStatsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<GetUserWebhooksParseStatsResponseMetrics>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetUserWebhooksParseStatsResponseData {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The Parse Webhook usage statistics.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stats: Vec<GetUserWebhooksParseStatsResponse>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetUserWebhooksEventSettingsSignedResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTrackingSettingsOpenResponse {
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchUserWebhooksEventSettingsSignedResponseErrors {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchUserWebhooksEventSettingsSignedResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<PatchUserWebhooksEventSettingsSignedResponseErrors>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostUserWebhooksEventTestRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oauth_token_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMessagesResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub messages: Vec<Messages>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PostMessagesDownloadResponseStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PostMessagesDownloadResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostMessagesDownloadResponseStatus::Pending => "pending",
            PostMessagesDownloadResponseStatus::Noop => "",
            PostMessagesDownloadResponseStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PostMessagesDownloadResponseStatus {
    fn default() -> PostMessagesDownloadResponseStatus {
        PostMessagesDownloadResponseStatus::Noop
    }
}
impl PostMessagesDownloadResponseStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, PostMessagesDownloadResponseStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostMessagesDownloadResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PostMessagesDownloadResponseStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMessagesDownloadResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub csv: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub presigned_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTrackingSettingsResponseResult {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetTrackingSettingsResponse {
    /**
     * The list of all tracking settings.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub result: Vec<GetTrackingSettingsResponseResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PatchTrackingSettingsOpenRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetStatsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<StatsAdvancedGlobalAllOf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetStatsResponseData {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The individual email activity stats.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stats: Vec<GetStatsResponse>,
}

/**
* The country you would like to see statistics for. Currently only supported for US and CA.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Country {
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Country::Ca => "CA",
            Country::Us => "US",
            Country::Noop => "",
            Country::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Country {
    fn default() -> Country {
        Country::Noop
    }
}
impl Country {
    pub fn is_noop(&self) -> bool {
        matches!(self, Country::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetGeoStatsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<AdvancedStatsClicksOpensAllOf>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetGeoStatsResponseData {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The list of statistics.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stats: Vec<GetGeoStatsResponse>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetDevicesStatsResponse {
    /**
     * The individual events and their stats.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<AdvancedStatsOpens>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetClientsStatsResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The list of statistics.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stats: Vec<GetDevicesStatsResponse>,
}

/**
* Specifies the type of client to retrieve stats for. Must be either "phone", "tablet", "webmail", or "desktop".
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ClientType {
    #[serde(rename = "desktop")]
    Desktop,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "tablet")]
    Tablet,
    #[serde(rename = "webmail")]
    Webmail,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ClientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientType::Desktop => "desktop",
            ClientType::Phone => "phone",
            ClientType::Tablet => "tablet",
            ClientType::Webmail => "webmail",
            ClientType::Noop => "",
            ClientType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ClientType {
    fn default() -> ClientType {
        ClientType::Noop
    }
}
impl ClientType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ClientType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMailboxProvidersStatsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<AdvancedStatsMailboxProviderAllOf>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetMailboxProvidersStatsResponseData {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The list of statistics.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stats: Vec<GetMailboxProvidersStatsResponse>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetBrowsersStatsResponse {
    /**
     * The individual events and their stats.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<AdvancedStatsClicks>,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetBrowsersStatsResponseData {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The list of statistics.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stats: Vec<GetBrowsersStatsResponse>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteSuppressionBouncesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_all: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub emails: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteSuppressionBlocksRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_all: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub emails: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetSuppressionUnsubscribesResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RetrieveAGlobalSuppressionResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub recipient_email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostAsmGroupsResponse {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_default: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAsmGroupsGroupResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_email_sent_at: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribes: i64,
}

/// All of the following types are flattened into one object:
///
/// - `SuppressionGroupRequestBase`
/// - `GetAsmGroupsGroupResponse`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAsmGroupsGroupResponseAllOf {
    #[serde(flatten)]
    pub suppression_group_request_base: SuppressionGroupRequestBase,
    #[serde(flatten)]
    pub get_asm_groups_group_response: GetAsmGroupsGroupResponse,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostAsmGroupsGroupSuppressionsResponse {
    /**
     * The recipient IDs of the recipients that already existed from this request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub recipient_emails: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAsmSuppressionsResponse {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub created_at: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub group_id: i64,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Suppressions {
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_default: bool,
    /**
     * The license key provided with your New Relic account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub suppressed: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAsmSuppressionsEmailResponse {
    /**
     * The array of suppression groups.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub suppressions: Vec<Suppressions>,
}
