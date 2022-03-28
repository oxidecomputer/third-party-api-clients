//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// An individual folder listed in the File Manager.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GalleryFolder {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Information about a specific template.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TemplateInstance {
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub folder_id: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A specific note for a specific member.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MemberNotes {
    /**
    * A specific note for a specific member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
}

/// Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Interest {
    /**
    * Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub display_order: i64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
* Determines how this category’s interests appear on signup forms.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Type {
    #[serde(rename = "checkboxes")]
    Checkboxes,
    #[serde(rename = "dropdown")]
    Dropdown,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Type::Checkboxes => "checkboxes",
            Type::Dropdown => "dropdown",
            Type::Hidden => "hidden",
            Type::Radio => "radio",
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

/// Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InterestCategory {
    /**
    * Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub display_order: i64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Determines how this category’s interests appear on signup forms.
    */
    #[serde(default, skip_serializing_if = "Type::is_noop", rename = "type")]
    pub type_: Type,
}

/// The events that can trigger the webhook and whether they are enabled.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Events {
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub campaign: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub cleaned: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub profile: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub subscribe: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub unsubscribe: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub upemail: bool,
}

/// The possible sources of any events that can trigger the webhook and whether they are enabled.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Sources {
    /**
    * The possible sources of any events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub admin: bool,
    /**
    * The possible sources of any events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub api: bool,
    /**
    * The possible sources of any events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub user: bool,
}

/// Configure a webhook for the given list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddWebhook {
    /**
    * Configure a webhook for the given list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Events>,
    /**
    * Configure a webhook for the given list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Sources>,
    /**
    * Configure a webhook for the given list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Information about subscribers in an Automation email queue.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriberInAutomationQueue {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
}

/// Information about a specific product variant.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceProductVariant {
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub backorders: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub inventory_quantity: i64,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sku: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

/**
* The type of pricing plan the account is on.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PricingPlanType {
    #[serde(rename = "forever_free")]
    ForeverFree,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "pay_as_you_go")]
    PayAsYouGo,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PricingPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PricingPlanType::ForeverFree => "forever_free",
            PricingPlanType::Monthly => "monthly",
            PricingPlanType::PayAsYouGo => "pay_as_you_go",
            PricingPlanType::Noop => "",
            PricingPlanType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PricingPlanType {
    fn default() -> PricingPlanType {
        PricingPlanType::Noop
    }
}
impl PricingPlanType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PricingPlanType::Noop)
    }
}

/// Information about the account contact.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Contact {
    /**
    * Information about the account contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "addr1"
    )]
    pub addr_1: String,
    /**
    * Information about the account contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "addr2"
    )]
    pub addr_2: String,
    /**
    * Information about the account contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
    * Information about the account contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * Information about the account contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
    * Information about the account contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
    * Information about the account contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// The [average campaign statistics](https://mailchimp.com/resources/research/email-marketing-benchmarks/?utm_source=mc-api&utm_medium=docs&utm_campaign=apidocs) for all campaigns in the account's specified industry.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IndustryStats {
    /**
    * The [average campaign statistics](https://mailchimp.com/resources/research/email-marketing-benchmarks/?utm_source=mc-api&utm_medium=docs&utm_campaign=apidocs) for all campaigns in the account's specified industry.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub bounce_rate: f64,
    /**
    * The [average campaign statistics](https://mailchimp.com/resources/research/email-marketing-benchmarks/?utm_source=mc-api&utm_medium=docs&utm_campaign=apidocs) for all campaigns in the account's specified industry.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * The [average campaign statistics](https://mailchimp.com/resources/research/email-marketing-benchmarks/?utm_source=mc-api&utm_medium=docs&utm_campaign=apidocs) for all campaigns in the account's specified industry.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
}

/**
* The HTTP method that should be used when accessing the URL defined in 'href'.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Method {
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Method::Delete => "DELETE",
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
            Method::Patch => "PATCH",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Noop => "",
            Method::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Method {
    fn default() -> Method {
        Method::Noop
    }
}
impl Method {
    pub fn is_noop(&self) -> bool {
        matches!(self, Method::Noop)
    }
}

/// This object represents a link from the resource where it is found to another resource or action that may be performed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Links {
    /**
    * This object represents a link from the resource where it is found to another resource or action that may be performed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub href: String,
    /**
    * This object represents a link from the resource where it is found to another resource or action that may be performed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    /**
    * This object represents a link from the resource where it is found to another resource or action that may be performed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rel: String,
    /**
    * This object represents a link from the resource where it is found to another resource or action that may be performed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub schema: String,
    /**
    * This object represents a link from the resource where it is found to another resource or action that may be performed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "targetSchema"
    )]
    pub target_schema: String,
}

/// The API root resource links to all other resources available in the API.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApiRoot {
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub account_id: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub account_industry: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub account_name: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub account_timezone: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub first_payment: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry_stats: Option<IndustryStats>,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login_id: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub member_since: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_plan_type: Option<PricingPlanType>,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub pro_enabled: bool,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_subscribers: i64,
    /**
    * The API root resource links to all other resources available in the API.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

/**
* The type of activity
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetActivityFeedChimpChatterResponseType {
    #[serde(rename = "campaigns:facebook-likes")]
    CampaignsFacebookLikes,
    #[serde(rename = "campaigns:forward-to-friend")]
    CampaignsForwardToFriend,
    #[serde(rename = "lists:imports")]
    ListsImports,
    #[serde(rename = "lists:new-subscriber")]
    ListsNewSubscriber,
    #[serde(rename = "lists:profile-updates")]
    ListsProfileUpdates,
    #[serde(rename = "lists:unsubscribes")]
    ListsUnsubscribes,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetActivityFeedChimpChatterResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetActivityFeedChimpChatterResponseType::CampaignsFacebookLikes => {
                "campaigns:facebook-likes"
            }
            GetActivityFeedChimpChatterResponseType::CampaignsForwardToFriend => {
                "campaigns:forward-to-friend"
            }
            GetActivityFeedChimpChatterResponseType::ListsImports => "lists:imports",
            GetActivityFeedChimpChatterResponseType::ListsNewSubscriber => "lists:new-subscriber",
            GetActivityFeedChimpChatterResponseType::ListsProfileUpdates => "lists:profile-updates",
            GetActivityFeedChimpChatterResponseType::ListsUnsubscribes => "lists:unsubscribes",
            GetActivityFeedChimpChatterResponseType::Noop => "",
            GetActivityFeedChimpChatterResponseType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetActivityFeedChimpChatterResponseType {
    fn default() -> GetActivityFeedChimpChatterResponseType {
        GetActivityFeedChimpChatterResponseType::Noop
    }
}
impl GetActivityFeedChimpChatterResponseType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetActivityFeedChimpChatterResponseType::Noop)
    }
}

/// A Chimp Chatter message
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChimpChatter {
    /**
    * A Chimp Chatter message
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A Chimp Chatter message
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A Chimp Chatter message
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
    * A Chimp Chatter message
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * A Chimp Chatter message
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<GetActivityFeedChimpChatterResponseType>,
    /**
    * A Chimp Chatter message
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A Chimp Chatter message
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// An array of Chimp Chatter messages. There's a maximum of 200 messages present for an account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetActivityFeedChimpChatterResponse {
    /**
    * An array of Chimp Chatter messages. There's a maximum of 200 messages present for an account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An array of Chimp Chatter messages. There's a maximum of 200 messages present for an account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub chimp_chatter: Vec<ChimpChatter>,
    /**
    * An array of Chimp Chatter messages. There's a maximum of 200 messages present for an account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// An authorized app.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Apps {
    /**
    * An authorized app.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An authorized app.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * An authorized app.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * An authorized app.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * An authorized app.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub users: Vec<String>,
}

/// An array of objects, each representing an authorized application.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAuthorizedAppsResponse {
    /**
    * An array of objects, each representing an authorized application.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An array of objects, each representing an authorized application.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub apps: Vec<Apps>,
    /**
    * An array of objects, each representing an authorized application.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* Restrict the results to automations with the specified status.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Status {
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "save")]
    Save,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Status::Paused => "paused",
            Status::Save => "save",
            Status::Sending => "sending",
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

/**
* Segment match type.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Match {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Match::All => "all",
            Match::Any => "any",
            Match::Noop => "",
            Match::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Match {
    fn default() -> Match {
        Match::Noop
    }
}
impl Match {
    pub fn is_noop(&self) -> bool {
        matches!(self, Match::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ConditionType {
    #[serde(rename = "Aim")]
    Aim,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ConditionType::Aim => "Aim",
            ConditionType::Noop => "",
            ConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ConditionType {
    fn default() -> ConditionType {
        ConditionType::Noop
    }
}
impl ConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ConditionType::Noop)
    }
}

/**
* Segment by interaction with a specific campaign.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Field {
    #[serde(rename = "aim")]
    Aim,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Field::Aim => "aim",
            Field::Noop => "",
            Field::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Field {
    fn default() -> Field {
        Field::Noop
    }
}
impl Field {
    pub fn is_noop(&self) -> bool {
        matches!(self, Field::Noop)
    }
}

/**
* The status of the member with regard to their campaign interaction. One of the following: opened, clicked, was sent, didn't open, didn't click, or was not sent.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Op {
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "noclick")]
    Noclick,
    #[serde(rename = "noopen")]
    Noopen,
    #[serde(rename = "nosent")]
    Nosent,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Op::Click => "click",
            Op::Noclick => "noclick",
            Op::Noopen => "noopen",
            Op::Nosent => "nosent",
            Op::Open => "open",
            Op::Sent => "sent",
            Op::Noop => "",
            Op::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Op {
    fn default() -> Op {
        Op::Noop
    }
}
impl Op {
    pub fn is_noop(&self) -> bool {
        matches!(self, Op::Noop)
    }
}

/// Segment by interaction with a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Conditions {
    /**
    * Segment by interaction with a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<ConditionType>,
    /**
    * Segment by interaction with a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /**
    * Segment by interaction with a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<Op>,
    /**
    * Segment by interaction with a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AutomationSegmentConditionType {
    #[serde(rename = "Automation")]
    Automation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AutomationSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AutomationSegmentConditionType::Automation => "Automation",
            AutomationSegmentConditionType::Noop => "",
            AutomationSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AutomationSegmentConditionType {
    fn default() -> AutomationSegmentConditionType {
        AutomationSegmentConditionType::Noop
    }
}
impl AutomationSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AutomationSegmentConditionType::Noop)
    }
}

/**
* Segment by interaction with an Automation workflow.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SegmentField {
    #[serde(rename = "automation")]
    Automation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SegmentField::Automation => "automation",
            SegmentField::Noop => "",
            SegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SegmentField {
    fn default() -> SegmentField {
        SegmentField::Noop
    }
}
impl SegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, SegmentField::Noop)
    }
}

/**
* The status of the member with regard to the automation workflow. One of the following: has started the workflow, has completed the workflow, has not started the workflow, or has not completed the workflow.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SegmentOperator {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "not_completed")]
    NotCompleted,
    #[serde(rename = "not_started")]
    NotStarted,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SegmentOperator::Completed => "completed",
            SegmentOperator::NotCompleted => "not_completed",
            SegmentOperator::NotStarted => "not_started",
            SegmentOperator::Started => "started",
            SegmentOperator::Noop => "",
            SegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SegmentOperator {
    fn default() -> SegmentOperator {
        SegmentOperator::Noop
    }
}
impl SegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, SegmentOperator::Noop)
    }
}

/// Segment by interaction with an Automation workflow.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationSegment {
    /**
    * Segment by interaction with an Automation workflow.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<AutomationSegmentConditionType>,
    /**
    * Segment by interaction with an Automation workflow.
    */
    #[serde(default, skip_serializing_if = "SegmentField::is_noop")]
    pub field: SegmentField,
    /**
    * The status of the member with regard to the automation workflow. One of the following: has started the workflow, has completed the workflow, has not started the workflow, or has not completed the workflow.
    */
    #[serde(default, skip_serializing_if = "SegmentOperator::is_noop")]
    pub op: SegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PollActivitySegmentConditionType {
    #[serde(rename = "CampaignPoll")]
    CampaignPoll,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PollActivitySegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PollActivitySegmentConditionType::CampaignPoll => "CampaignPoll",
            PollActivitySegmentConditionType::Noop => "",
            PollActivitySegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PollActivitySegmentConditionType {
    fn default() -> PollActivitySegmentConditionType {
        PollActivitySegmentConditionType::Noop
    }
}
impl PollActivitySegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PollActivitySegmentConditionType::Noop)
    }
}

/**
* Segment by poll activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PollActivitySegmentField {
    #[serde(rename = "poll")]
    Poll,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PollActivitySegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PollActivitySegmentField::Poll => "poll",
            PollActivitySegmentField::Noop => "",
            PollActivitySegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PollActivitySegmentField {
    fn default() -> PollActivitySegmentField {
        PollActivitySegmentField::Noop
    }
}
impl PollActivitySegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, PollActivitySegmentField::Noop)
    }
}

/**
* Members have/have not interacted with a specific poll in a Mailchimp email.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PollActivitySegmentOperator {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "notmember")]
    Notmember,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PollActivitySegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PollActivitySegmentOperator::Member => "member",
            PollActivitySegmentOperator::Notmember => "notmember",
            PollActivitySegmentOperator::Noop => "",
            PollActivitySegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PollActivitySegmentOperator {
    fn default() -> PollActivitySegmentOperator {
        PollActivitySegmentOperator::Noop
    }
}
impl PollActivitySegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, PollActivitySegmentOperator::Noop)
    }
}

/// Segment by poll activity.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PollActivitySegment {
    /**
    * Segment by poll activity.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<PollActivitySegmentConditionType>,
    /**
    * Segment by poll activity.
    */
    #[serde(default, skip_serializing_if = "PollActivitySegmentField::is_noop")]
    pub field: PollActivitySegmentField,
    /**
    * Members have/have not interacted with a specific poll in a Mailchimp email.
    */
    #[serde(default, skip_serializing_if = "PollActivitySegmentOperator::is_noop")]
    pub op: PollActivitySegmentOperator,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub value: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ConversationSegmentConditionType {
    #[serde(rename = "Conversation")]
    Conversation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ConversationSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ConversationSegmentConditionType::Conversation => "Conversation",
            ConversationSegmentConditionType::Noop => "",
            ConversationSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ConversationSegmentConditionType {
    fn default() -> ConversationSegmentConditionType {
        ConversationSegmentConditionType::Noop
    }
}
impl ConversationSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ConversationSegmentConditionType::Noop)
    }
}

/**
* Segment by interaction with a campaign via Conversations.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ConversationSegmentField {
    #[serde(rename = "conversation")]
    Conversation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ConversationSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ConversationSegmentField::Conversation => "conversation",
            ConversationSegmentField::Noop => "",
            ConversationSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ConversationSegmentField {
    fn default() -> ConversationSegmentField {
        ConversationSegmentField::Noop
    }
}
impl ConversationSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, ConversationSegmentField::Noop)
    }
}

/// Segment by interaction with a campaign via Conversations.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConversationSegment {
    /**
    * Segment by interaction with a campaign via Conversations.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<ConversationSegmentConditionType>,
    /**
    * Segment by interaction with a campaign via Conversations.
    */
    #[serde(default, skip_serializing_if = "ConversationSegmentField::is_noop")]
    pub field: ConversationSegmentField,
    /**
    * Members have/have not interacted with a specific poll in a Mailchimp email.
    */
    #[serde(default, skip_serializing_if = "PollActivitySegmentOperator::is_noop")]
    pub op: PollActivitySegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DateSegmentConditionType {
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DateSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DateSegmentConditionType::Date => "Date",
            DateSegmentConditionType::Noop => "",
            DateSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DateSegmentConditionType {
    fn default() -> DateSegmentConditionType {
        DateSegmentConditionType::Noop
    }
}
impl DateSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DateSegmentConditionType::Noop)
    }
}

/**
* The type of date field to segment on: The opt-in time for a signup, the date the subscriber was last updated, or the date of their last ecomm purchase.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DateSegmentField {
    #[serde(rename = "ecomm_date")]
    EcommDate,
    #[serde(rename = "info_changed")]
    InfoChanged,
    #[serde(rename = "timestamp_opt")]
    TimestampOpt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DateSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DateSegmentField::EcommDate => "ecomm_date",
            DateSegmentField::InfoChanged => "info_changed",
            DateSegmentField::TimestampOpt => "timestamp_opt",
            DateSegmentField::Noop => "",
            DateSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DateSegmentField {
    fn default() -> DateSegmentField {
        DateSegmentField::Noop
    }
}
impl DateSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, DateSegmentField::Noop)
    }
}

/**
* When the event took place:  Before, after, is a specific date, is not a specific date, is blank, or is not blank.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DateSegmentOperator {
    #[serde(rename = "blank")]
    Blank,
    #[serde(rename = "blank_not")]
    BlankNot,
    #[serde(rename = "greater")]
    Greater,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "notwithin")]
    Notwithin,
    #[serde(rename = "within")]
    Within,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DateSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DateSegmentOperator::Blank => "blank",
            DateSegmentOperator::BlankNot => "blank_not",
            DateSegmentOperator::Greater => "greater",
            DateSegmentOperator::Is => "is",
            DateSegmentOperator::Less => "less",
            DateSegmentOperator::Not => "not",
            DateSegmentOperator::Notwithin => "notwithin",
            DateSegmentOperator::Within => "within",
            DateSegmentOperator::Noop => "",
            DateSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DateSegmentOperator {
    fn default() -> DateSegmentOperator {
        DateSegmentOperator::Noop
    }
}
impl DateSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, DateSegmentOperator::Noop)
    }
}

/// Segment by a specific date field.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DateSegment {
    /**
    * Segment by a specific date field.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<DateSegmentConditionType>,
    /**
    * Segment by a specific date field.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub extra: String,
    /**
    * The type of date field to segment on: The opt-in time for a signup, the date the subscriber was last updated, or the date of their last ecomm purchase.
    */
    #[serde(default, skip_serializing_if = "DateSegmentField::is_noop")]
    pub field: DateSegmentField,
    /**
    * When the event took place:  Before, after, is a specific date, is not a specific date, is blank, or is not blank.
    */
    #[serde(default, skip_serializing_if = "DateSegmentOperator::is_noop")]
    pub op: DateSegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailClientSegmentConditionType {
    #[serde(rename = "EmailClient")]
    EmailClient,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailClientSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailClientSegmentConditionType::EmailClient => "EmailClient",
            EmailClientSegmentConditionType::Noop => "",
            EmailClientSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailClientSegmentConditionType {
    fn default() -> EmailClientSegmentConditionType {
        EmailClientSegmentConditionType::Noop
    }
}
impl EmailClientSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailClientSegmentConditionType::Noop)
    }
}

/**
* Segment by use of a particular email client.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailClientSegmentField {
    #[serde(rename = "email_client")]
    EmailClient,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailClientSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailClientSegmentField::EmailClient => "email_client",
            EmailClientSegmentField::Noop => "",
            EmailClientSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailClientSegmentField {
    fn default() -> EmailClientSegmentField {
        EmailClientSegmentField::Noop
    }
}
impl EmailClientSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailClientSegmentField::Noop)
    }
}

/**
* The operation to determine whether we select clients that match the value, or clients that do not match the value.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailClientSegmentOperator {
    #[serde(rename = "client_is")]
    ClientIs,
    #[serde(rename = "client_not")]
    ClientNot,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailClientSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailClientSegmentOperator::ClientIs => "client_is",
            EmailClientSegmentOperator::ClientNot => "client_not",
            EmailClientSegmentOperator::Noop => "",
            EmailClientSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailClientSegmentOperator {
    fn default() -> EmailClientSegmentOperator {
        EmailClientSegmentOperator::Noop
    }
}
impl EmailClientSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailClientSegmentOperator::Noop)
    }
}

/// Segment by use of a particular email client.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailClientSegment {
    /**
    * Segment by use of a particular email client.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<EmailClientSegmentConditionType>,
    /**
    * Segment by use of a particular email client.
    */
    #[serde(default, skip_serializing_if = "EmailClientSegmentField::is_noop")]
    pub field: EmailClientSegmentField,
    /**
    * The operation to determine whether we select clients that match the value, or clients that do not match the value.
    */
    #[serde(default, skip_serializing_if = "EmailClientSegmentOperator::is_noop")]
    pub op: EmailClientSegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LanguageSegmentConditionType {
    #[serde(rename = "Language")]
    Language,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LanguageSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LanguageSegmentConditionType::Language => "Language",
            LanguageSegmentConditionType::Noop => "",
            LanguageSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LanguageSegmentConditionType {
    fn default() -> LanguageSegmentConditionType {
        LanguageSegmentConditionType::Noop
    }
}
impl LanguageSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, LanguageSegmentConditionType::Noop)
    }
}

/**
* Segmenting based off of a subscriber's language.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LanguageSegmentField {
    #[serde(rename = "language")]
    Language,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LanguageSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LanguageSegmentField::Language => "language",
            LanguageSegmentField::Noop => "",
            LanguageSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LanguageSegmentField {
    fn default() -> LanguageSegmentField {
        LanguageSegmentField::Noop
    }
}
impl LanguageSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, LanguageSegmentField::Noop)
    }
}

/**
* Whether the member's language is or is not set to a specific language.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LanguageSegmentOperator {
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LanguageSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LanguageSegmentOperator::Is => "is",
            LanguageSegmentOperator::Not => "not",
            LanguageSegmentOperator::Noop => "",
            LanguageSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LanguageSegmentOperator {
    fn default() -> LanguageSegmentOperator {
        LanguageSegmentOperator::Noop
    }
}
impl LanguageSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, LanguageSegmentOperator::Noop)
    }
}

/// Segment by language.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LanguageSegment {
    /**
    * Segment by language.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<LanguageSegmentConditionType>,
    /**
    * Segmenting based off of a subscriber's language.
    */
    #[serde(default, skip_serializing_if = "LanguageSegmentField::is_noop")]
    pub field: LanguageSegmentField,
    /**
    * Whether the member's language is or is not set to a specific language.
    */
    #[serde(default, skip_serializing_if = "LanguageSegmentOperator::is_noop")]
    pub op: LanguageSegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MemberRatingSegmentConditionType {
    #[serde(rename = "MemberRating")]
    MemberRating,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MemberRatingSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MemberRatingSegmentConditionType::MemberRating => "MemberRating",
            MemberRatingSegmentConditionType::Noop => "",
            MemberRatingSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MemberRatingSegmentConditionType {
    fn default() -> MemberRatingSegmentConditionType {
        MemberRatingSegmentConditionType::Noop
    }
}
impl MemberRatingSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, MemberRatingSegmentConditionType::Noop)
    }
}

/**
* Segment by member rating.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MemberRatingSegmentField {
    #[serde(rename = "rating")]
    Rating,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MemberRatingSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MemberRatingSegmentField::Rating => "rating",
            MemberRatingSegmentField::Noop => "",
            MemberRatingSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MemberRatingSegmentField {
    fn default() -> MemberRatingSegmentField {
        MemberRatingSegmentField::Noop
    }
}
impl MemberRatingSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, MemberRatingSegmentField::Noop)
    }
}

/**
* Members who have have a rating that is/not exactly a given number or members who have a rating greater/less than a given number.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MemberRatingSegmentOperator {
    #[serde(rename = "greater")]
    Greater,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MemberRatingSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MemberRatingSegmentOperator::Greater => "greater",
            MemberRatingSegmentOperator::Is => "is",
            MemberRatingSegmentOperator::Less => "less",
            MemberRatingSegmentOperator::Not => "not",
            MemberRatingSegmentOperator::Noop => "",
            MemberRatingSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MemberRatingSegmentOperator {
    fn default() -> MemberRatingSegmentOperator {
        MemberRatingSegmentOperator::Noop
    }
}
impl MemberRatingSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, MemberRatingSegmentOperator::Noop)
    }
}

/// Segment by member rating.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MemberRatingSegment {
    /**
    * Segment by member rating.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<MemberRatingSegmentConditionType>,
    /**
    * Segment by member rating.
    */
    #[serde(default, skip_serializing_if = "MemberRatingSegmentField::is_noop")]
    pub field: MemberRatingSegmentField,
    /**
    * Members who have have a rating that is/not exactly a given number or members who have a rating greater/less than a given number.
    */
    #[serde(default, skip_serializing_if = "MemberRatingSegmentOperator::is_noop")]
    pub op: MemberRatingSegmentOperator,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub value: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SignupSourceSegmentType {
    #[serde(rename = "SignupSource")]
    SignupSource,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SignupSourceSegmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SignupSourceSegmentType::SignupSource => "SignupSource",
            SignupSourceSegmentType::Noop => "",
            SignupSourceSegmentType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SignupSourceSegmentType {
    fn default() -> SignupSourceSegmentType {
        SignupSourceSegmentType::Noop
    }
}
impl SignupSourceSegmentType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SignupSourceSegmentType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SignupSourceSegmentField {
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SignupSourceSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SignupSourceSegmentField::Source => "source",
            SignupSourceSegmentField::Noop => "",
            SignupSourceSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SignupSourceSegmentField {
    fn default() -> SignupSourceSegmentField {
        SignupSourceSegmentField::Noop
    }
}
impl SignupSourceSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, SignupSourceSegmentField::Noop)
    }
}

/**
* Whether the member's signup source was/was not a particular value.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SignupSourceSegmentOperator {
    #[serde(rename = "source_is")]
    SourceIs,
    #[serde(rename = "source_not")]
    SourceNot,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SignupSourceSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SignupSourceSegmentOperator::SourceIs => "source_is",
            SignupSourceSegmentOperator::SourceNot => "source_not",
            SignupSourceSegmentOperator::Noop => "",
            SignupSourceSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SignupSourceSegmentOperator {
    fn default() -> SignupSourceSegmentOperator {
        SignupSourceSegmentOperator::Noop
    }
}
impl SignupSourceSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, SignupSourceSegmentOperator::Noop)
    }
}

/// Segment by signup source.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SignupSourceSegment {
    #[serde(default, skip_serializing_if = "SignupSourceSegmentType::is_noop")]
    pub condition_type: SignupSourceSegmentType,
    #[serde(default, skip_serializing_if = "SignupSourceSegmentField::is_noop")]
    pub field: SignupSourceSegmentField,
    /**
    * Whether the member's signup source was/was not a particular value.
    */
    #[serde(default, skip_serializing_if = "SignupSourceSegmentOperator::is_noop")]
    pub op: SignupSourceSegmentOperator,
    /**
    * Segment by signup source.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SurveyMonkeySegmentConditionType {
    #[serde(rename = "SurveyMonkey")]
    SurveyMonkey,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SurveyMonkeySegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SurveyMonkeySegmentConditionType::SurveyMonkey => "SurveyMonkey",
            SurveyMonkeySegmentConditionType::Noop => "",
            SurveyMonkeySegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SurveyMonkeySegmentConditionType {
    fn default() -> SurveyMonkeySegmentConditionType {
        SurveyMonkeySegmentConditionType::Noop
    }
}
impl SurveyMonkeySegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SurveyMonkeySegmentConditionType::Noop)
    }
}

/**
* Segment by interaction with a SurveyMonkey survey.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SurveyMonkeySegmentField {
    #[serde(rename = "survey_monkey")]
    SurveyMonkey,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SurveyMonkeySegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SurveyMonkeySegmentField::SurveyMonkey => "survey_monkey",
            SurveyMonkeySegmentField::Noop => "",
            SurveyMonkeySegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SurveyMonkeySegmentField {
    fn default() -> SurveyMonkeySegmentField {
        SurveyMonkeySegmentField::Noop
    }
}
impl SurveyMonkeySegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, SurveyMonkeySegmentField::Noop)
    }
}

/// Segment by interaction with a SurveyMonkey survey.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SurveyMonkeySegment {
    /**
    * Segment by interaction with a SurveyMonkey survey.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<SurveyMonkeySegmentConditionType>,
    /**
    * Segment by interaction with a SurveyMonkey survey.
    */
    #[serde(default, skip_serializing_if = "SurveyMonkeySegmentField::is_noop")]
    pub field: SurveyMonkeySegmentField,
    /**
    * The status of the member with regard to the automation workflow. One of the following: has started the workflow, has completed the workflow, has not started the workflow, or has not completed the workflow.
    */
    #[serde(default, skip_serializing_if = "SegmentOperator::is_noop")]
    pub op: SegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum VipSegmentConditionType {
    #[serde(rename = "VIP")]
    Vip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VipSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VipSegmentConditionType::Vip => "VIP",
            VipSegmentConditionType::Noop => "",
            VipSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for VipSegmentConditionType {
    fn default() -> VipSegmentConditionType {
        VipSegmentConditionType::Noop
    }
}
impl VipSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, VipSegmentConditionType::Noop)
    }
}

/**
* Segment by VIP status.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum VipSegmentField {
    #[serde(rename = "gmonkey")]
    Gmonkey,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VipSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            VipSegmentField::Gmonkey => "gmonkey",
            VipSegmentField::Noop => "",
            VipSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for VipSegmentField {
    fn default() -> VipSegmentField {
        VipSegmentField::Noop
    }
}
impl VipSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, VipSegmentField::Noop)
    }
}

/// Segment by VIP status.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VipSegment {
    /**
    * Segment by VIP status.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<VipSegmentConditionType>,
    /**
    * Segment by VIP status.
    */
    #[serde(default, skip_serializing_if = "VipSegmentField::is_noop")]
    pub field: VipSegmentField,
    /**
    * Members have/have not interacted with a specific poll in a Mailchimp email.
    */
    #[serde(default, skip_serializing_if = "PollActivitySegmentOperator::is_noop")]
    pub op: PollActivitySegmentOperator,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum InterestsSegmentConditionType {
    #[serde(rename = "Interests")]
    Interests,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for InterestsSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InterestsSegmentConditionType::Interests => "Interests",
            InterestsSegmentConditionType::Noop => "",
            InterestsSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for InterestsSegmentConditionType {
    fn default() -> InterestsSegmentConditionType {
        InterestsSegmentConditionType::Noop
    }
}
impl InterestsSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, InterestsSegmentConditionType::Noop)
    }
}

/**
* Whether the member is a part of one, all, or none of the groups.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum InterestsSegmentOperator {
    #[serde(rename = "interestcontains")]
    Interestcontains,
    #[serde(rename = "interestcontainsall")]
    Interestcontainsall,
    #[serde(rename = "interestnotcontains")]
    Interestnotcontains,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for InterestsSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InterestsSegmentOperator::Interestcontains => "interestcontains",
            InterestsSegmentOperator::Interestcontainsall => "interestcontainsall",
            InterestsSegmentOperator::Interestnotcontains => "interestnotcontains",
            InterestsSegmentOperator::Noop => "",
            InterestsSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for InterestsSegmentOperator {
    fn default() -> InterestsSegmentOperator {
        InterestsSegmentOperator::Noop
    }
}
impl InterestsSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, InterestsSegmentOperator::Noop)
    }
}

/// Segment by an interest group merge field.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InterestsSegment {
    /**
    * Segment by an interest group merge field.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<InterestsSegmentConditionType>,
    /**
    * Segment by an interest group merge field.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
    * Segment by an interest group merge field.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<InterestsSegmentOperator>,
    /**
    * Segment by an interest group merge field.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceCategorySegmentConditionType {
    #[serde(rename = "EcommCategory")]
    EcommCategory,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceCategorySegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceCategorySegmentConditionType::EcommCategory => "EcommCategory",
            EcommerceCategorySegmentConditionType::Noop => "",
            EcommerceCategorySegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceCategorySegmentConditionType {
    fn default() -> EcommerceCategorySegmentConditionType {
        EcommerceCategorySegmentConditionType::Noop
    }
}
impl EcommerceCategorySegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceCategorySegmentConditionType::Noop)
    }
}

/**
* Segment by purchases in specific items or categories.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceCategorySegmentField {
    #[serde(rename = "ecomm_cat")]
    EcommCat,
    #[serde(rename = "ecomm_prod")]
    EcommProd,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceCategorySegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceCategorySegmentField::EcommCat => "ecomm_cat",
            EcommerceCategorySegmentField::EcommProd => "ecomm_prod",
            EcommerceCategorySegmentField::Noop => "",
            EcommerceCategorySegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceCategorySegmentField {
    fn default() -> EcommerceCategorySegmentField {
        EcommerceCategorySegmentField::Noop
    }
}
impl EcommerceCategorySegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceCategorySegmentField::Noop)
    }
}

/**
* A member who has purchased from a category/specific item that is/is not a specific name, where the category/item name contains/doesn't contain a specific phrase or string, or a category/item name that starts/ends with a string.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceCategorySegmentOperator {
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "ends")]
    Ends,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "notcontain")]
    Notcontain,
    #[serde(rename = "starts")]
    Starts,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceCategorySegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceCategorySegmentOperator::Contains => "contains",
            EcommerceCategorySegmentOperator::Ends => "ends",
            EcommerceCategorySegmentOperator::Is => "is",
            EcommerceCategorySegmentOperator::Not => "not",
            EcommerceCategorySegmentOperator::Notcontain => "notcontain",
            EcommerceCategorySegmentOperator::Starts => "starts",
            EcommerceCategorySegmentOperator::Noop => "",
            EcommerceCategorySegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceCategorySegmentOperator {
    fn default() -> EcommerceCategorySegmentOperator {
        EcommerceCategorySegmentOperator::Noop
    }
}
impl EcommerceCategorySegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceCategorySegmentOperator::Noop)
    }
}

/// Segment by purchases in specific items or categories.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommerceCategorySegment {
    /**
    * Segment by purchases in specific items or categories.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<EcommerceCategorySegmentConditionType>,
    /**
    * Segment by purchases in specific items or categories.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<EcommerceCategorySegmentField>,
    /**
    * Segment by purchases in specific items or categories.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<EcommerceCategorySegmentOperator>,
    /**
    * Segment by purchases in specific items or categories.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceNumberSegmentConditionType {
    #[serde(rename = "EcommNumber")]
    EcommNumber,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceNumberSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceNumberSegmentConditionType::EcommNumber => "EcommNumber",
            EcommerceNumberSegmentConditionType::Noop => "",
            EcommerceNumberSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceNumberSegmentConditionType {
    fn default() -> EcommerceNumberSegmentConditionType {
        EcommerceNumberSegmentConditionType::Noop
    }
}
impl EcommerceNumberSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceNumberSegmentConditionType::Noop)
    }
}

/**
* Segment by average spent total, number of orders, total number of products purchased, or average number of products per order.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceNumberSegmentField {
    #[serde(rename = "ecomm_avg_ord")]
    EcommAvgOrd,
    #[serde(rename = "ecomm_orders")]
    EcommOrders,
    #[serde(rename = "ecomm_prod_all")]
    EcommProdAll,
    #[serde(rename = "ecomm_spent_avg")]
    EcommSpentAvg,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceNumberSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceNumberSegmentField::EcommAvgOrd => "ecomm_avg_ord",
            EcommerceNumberSegmentField::EcommOrders => "ecomm_orders",
            EcommerceNumberSegmentField::EcommProdAll => "ecomm_prod_all",
            EcommerceNumberSegmentField::EcommSpentAvg => "ecomm_spent_avg",
            EcommerceNumberSegmentField::Noop => "",
            EcommerceNumberSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceNumberSegmentField {
    fn default() -> EcommerceNumberSegmentField {
        EcommerceNumberSegmentField::Noop
    }
}
impl EcommerceNumberSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceNumberSegmentField::Noop)
    }
}

/// Segment by average spent total, number of orders, total number of products purchased, or average number of products per order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommerceNumberSegment {
    /**
    * Segment by average spent total, number of orders, total number of products purchased, or average number of products per order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<EcommerceNumberSegmentConditionType>,
    /**
    * Segment by average spent total, number of orders, total number of products purchased, or average number of products per order.
    */
    #[serde(default, skip_serializing_if = "EcommerceNumberSegmentField::is_noop")]
    pub field: EcommerceNumberSegmentField,
    /**
    * Members who have have a rating that is/not exactly a given number or members who have a rating greater/less than a given number.
    */
    #[serde(default, skip_serializing_if = "MemberRatingSegmentOperator::is_noop")]
    pub op: MemberRatingSegmentOperator,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub value: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommercePurchasedSegmentConditionType {
    #[serde(rename = "EcommPurchased")]
    EcommPurchased,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommercePurchasedSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommercePurchasedSegmentConditionType::EcommPurchased => "EcommPurchased",
            EcommercePurchasedSegmentConditionType::Noop => "",
            EcommercePurchasedSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommercePurchasedSegmentConditionType {
    fn default() -> EcommercePurchasedSegmentConditionType {
        EcommercePurchasedSegmentConditionType::Noop
    }
}
impl EcommercePurchasedSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommercePurchasedSegmentConditionType::Noop)
    }
}

/**
* Segment by whether someone has purchased anything.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommercePurchasedSegmentField {
    #[serde(rename = "ecomm_purchased")]
    EcommPurchased,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommercePurchasedSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommercePurchasedSegmentField::EcommPurchased => "ecomm_purchased",
            EcommercePurchasedSegmentField::Noop => "",
            EcommercePurchasedSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommercePurchasedSegmentField {
    fn default() -> EcommercePurchasedSegmentField {
        EcommercePurchasedSegmentField::Noop
    }
}
impl EcommercePurchasedSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommercePurchasedSegmentField::Noop)
    }
}

/// Segment by whether someone has purchased anything.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommercePurchasedSegment {
    /**
    * Segment by whether someone has purchased anything.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<EcommercePurchasedSegmentConditionType>,
    /**
    * Segment by whether someone has purchased anything.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<EcommercePurchasedSegmentField>,
    /**
    * Segment by whether someone has purchased anything.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<PollActivitySegmentOperator>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceSpentSegmentConditionType {
    #[serde(rename = "EcommSpent")]
    EcommSpent,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceSpentSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceSpentSegmentConditionType::EcommSpent => "EcommSpent",
            EcommerceSpentSegmentConditionType::Noop => "",
            EcommerceSpentSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceSpentSegmentConditionType {
    fn default() -> EcommerceSpentSegmentConditionType {
        EcommerceSpentSegmentConditionType::Noop
    }
}
impl EcommerceSpentSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceSpentSegmentConditionType::Noop)
    }
}

/**
* Segment by amount spent on a single order or across all orders.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceSpentSegmentField {
    #[serde(rename = "ecomm_spent_all")]
    EcommSpentAll,
    #[serde(rename = "ecomm_spent_one")]
    EcommSpentOne,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceSpentSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceSpentSegmentField::EcommSpentAll => "ecomm_spent_all",
            EcommerceSpentSegmentField::EcommSpentOne => "ecomm_spent_one",
            EcommerceSpentSegmentField::Noop => "",
            EcommerceSpentSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceSpentSegmentField {
    fn default() -> EcommerceSpentSegmentField {
        EcommerceSpentSegmentField::Noop
    }
}
impl EcommerceSpentSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceSpentSegmentField::Noop)
    }
}

/**
* Members who have spent 'more' or 'less' than then specified value.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceSpentSegmentOperator {
    #[serde(rename = "greater")]
    Greater,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceSpentSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceSpentSegmentOperator::Greater => "greater",
            EcommerceSpentSegmentOperator::Less => "less",
            EcommerceSpentSegmentOperator::Noop => "",
            EcommerceSpentSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceSpentSegmentOperator {
    fn default() -> EcommerceSpentSegmentOperator {
        EcommerceSpentSegmentOperator::Noop
    }
}
impl EcommerceSpentSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceSpentSegmentOperator::Noop)
    }
}

/// Segment by amount spent on a single order or across all orders.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommerceSpentSegment {
    /**
    * Segment by amount spent on a single order or across all orders.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<EcommerceSpentSegmentConditionType>,
    /**
    * Segment by amount spent on a single order or across all orders.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<EcommerceSpentSegmentField>,
    /**
    * Segment by amount spent on a single order or across all orders.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<EcommerceSpentSegmentOperator>,
    /**
    * Segment by amount spent on a single order or across all orders.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub value: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommercePurchasedStoreSegmentConditionType {
    #[serde(rename = "EcommStore")]
    EcommStore,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommercePurchasedStoreSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommercePurchasedStoreSegmentConditionType::EcommStore => "EcommStore",
            EcommercePurchasedStoreSegmentConditionType::Noop => "",
            EcommercePurchasedStoreSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommercePurchasedStoreSegmentConditionType {
    fn default() -> EcommercePurchasedStoreSegmentConditionType {
        EcommercePurchasedStoreSegmentConditionType::Noop
    }
}
impl EcommercePurchasedStoreSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommercePurchasedStoreSegmentConditionType::Noop)
    }
}

/**
* Segment by purchases from a specific store.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommercePurchasedStoreSegmentField {
    #[serde(rename = "ecomm_store")]
    EcommStore,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommercePurchasedStoreSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommercePurchasedStoreSegmentField::EcommStore => "ecomm_store",
            EcommercePurchasedStoreSegmentField::Noop => "",
            EcommercePurchasedStoreSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommercePurchasedStoreSegmentField {
    fn default() -> EcommercePurchasedStoreSegmentField {
        EcommercePurchasedStoreSegmentField::Noop
    }
}
impl EcommercePurchasedStoreSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommercePurchasedStoreSegmentField::Noop)
    }
}

/// Segment by purchases from a specific store.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommercePurchasedStoreSegment {
    /**
    * Segment by purchases from a specific store.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<EcommercePurchasedStoreSegmentConditionType>,
    /**
    * Segment by purchases from a specific store.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<EcommercePurchasedStoreSegmentField>,
    /**
    * Segment by purchases from a specific store.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<LanguageSegmentOperator>,
    /**
    * Segment by purchases from a specific store.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GoalActivitySegmentConditionType {
    #[serde(rename = "GoalActivity")]
    GoalActivity,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GoalActivitySegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GoalActivitySegmentConditionType::GoalActivity => "GoalActivity",
            GoalActivitySegmentConditionType::Noop => "",
            GoalActivitySegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GoalActivitySegmentConditionType {
    fn default() -> GoalActivitySegmentConditionType {
        GoalActivitySegmentConditionType::Noop
    }
}
impl GoalActivitySegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GoalActivitySegmentConditionType::Noop)
    }
}

/**
* Segment by Goal activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GoalActivitySegmentField {
    #[serde(rename = "goal")]
    Goal,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GoalActivitySegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GoalActivitySegmentField::Goal => "goal",
            GoalActivitySegmentField::Noop => "",
            GoalActivitySegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GoalActivitySegmentField {
    fn default() -> GoalActivitySegmentField {
        GoalActivitySegmentField::Noop
    }
}
impl GoalActivitySegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GoalActivitySegmentField::Noop)
    }
}

/**
* Whether the website URL is/not exactly, contains/doesn't contain, starts with/ends with a string.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GoalActivitySegmentOperator {
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "ends")]
    Ends,
    #[serde(rename = "goal_not")]
    GoalNot,
    #[serde(rename = "goal_notcontain")]
    GoalNotcontain,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "starts")]
    Starts,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GoalActivitySegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GoalActivitySegmentOperator::Contains => "contains",
            GoalActivitySegmentOperator::Ends => "ends",
            GoalActivitySegmentOperator::GoalNot => "goal_not",
            GoalActivitySegmentOperator::GoalNotcontain => "goal_notcontain",
            GoalActivitySegmentOperator::Is => "is",
            GoalActivitySegmentOperator::Starts => "starts",
            GoalActivitySegmentOperator::Noop => "",
            GoalActivitySegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GoalActivitySegmentOperator {
    fn default() -> GoalActivitySegmentOperator {
        GoalActivitySegmentOperator::Noop
    }
}
impl GoalActivitySegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, GoalActivitySegmentOperator::Noop)
    }
}

/// Segment by Goal activity.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GoalActivitySegment {
    /**
    * Segment by Goal activity.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<GoalActivitySegmentConditionType>,
    /**
    * Segment by Goal activity.
    */
    #[serde(default, skip_serializing_if = "GoalActivitySegmentField::is_noop")]
    pub field: GoalActivitySegmentField,
    /**
    * Whether the website URL is/not exactly, contains/doesn't contain, starts with/ends with a string.
    */
    #[serde(default, skip_serializing_if = "GoalActivitySegmentOperator::is_noop")]
    pub op: GoalActivitySegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GoalTimestampSegmentConditionType {
    #[serde(rename = "GoalTimestamp")]
    GoalTimestamp,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GoalTimestampSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GoalTimestampSegmentConditionType::GoalTimestamp => "GoalTimestamp",
            GoalTimestampSegmentConditionType::Noop => "",
            GoalTimestampSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GoalTimestampSegmentConditionType {
    fn default() -> GoalTimestampSegmentConditionType {
        GoalTimestampSegmentConditionType::Noop
    }
}
impl GoalTimestampSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GoalTimestampSegmentConditionType::Noop)
    }
}

/**
* Segment by most recent interaction with a website.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GoalTimestampSegmentField {
    #[serde(rename = "goal_last_visited")]
    GoalLastVisited,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GoalTimestampSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GoalTimestampSegmentField::GoalLastVisited => "goal_last_visited",
            GoalTimestampSegmentField::Noop => "",
            GoalTimestampSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GoalTimestampSegmentField {
    fn default() -> GoalTimestampSegmentField {
        GoalTimestampSegmentField::Noop
    }
}
impl GoalTimestampSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GoalTimestampSegmentField::Noop)
    }
}

/**
* Whether the website activity happened after, before, or at a given timestamp.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GoalTimestampSegmentOperator {
    #[serde(rename = "greater")]
    Greater,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GoalTimestampSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GoalTimestampSegmentOperator::Greater => "greater",
            GoalTimestampSegmentOperator::Is => "is",
            GoalTimestampSegmentOperator::Less => "less",
            GoalTimestampSegmentOperator::Noop => "",
            GoalTimestampSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GoalTimestampSegmentOperator {
    fn default() -> GoalTimestampSegmentOperator {
        GoalTimestampSegmentOperator::Noop
    }
}
impl GoalTimestampSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, GoalTimestampSegmentOperator::Noop)
    }
}

/// Segment by most recent interaction with a website.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GoalTimestampSegment {
    /**
    * Segment by most recent interaction with a website.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<GoalTimestampSegmentConditionType>,
    /**
    * Segment by most recent interaction with a website.
    */
    #[serde(default, skip_serializing_if = "GoalTimestampSegmentField::is_noop")]
    pub field: GoalTimestampSegmentField,
    /**
    * Whether the website activity happened after, before, or at a given timestamp.
    */
    #[serde(default, skip_serializing_if = "GoalTimestampSegmentOperator::is_noop")]
    pub op: GoalTimestampSegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SimilarSubscribersSegmentMemberConditionType {
    #[serde(rename = "FuzzySegment")]
    FuzzySegment,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SimilarSubscribersSegmentMemberConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SimilarSubscribersSegmentMemberConditionType::FuzzySegment => "FuzzySegment",
            SimilarSubscribersSegmentMemberConditionType::Noop => "",
            SimilarSubscribersSegmentMemberConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SimilarSubscribersSegmentMemberConditionType {
    fn default() -> SimilarSubscribersSegmentMemberConditionType {
        SimilarSubscribersSegmentMemberConditionType::Noop
    }
}
impl SimilarSubscribersSegmentMemberConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SimilarSubscribersSegmentMemberConditionType::Noop)
    }
}

/**
* Segment by similar subscribers.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SimilarSubscribersSegmentMemberField {
    #[serde(rename = "fuzzy_segment")]
    FuzzySegment,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SimilarSubscribersSegmentMemberField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SimilarSubscribersSegmentMemberField::FuzzySegment => "fuzzy_segment",
            SimilarSubscribersSegmentMemberField::Noop => "",
            SimilarSubscribersSegmentMemberField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SimilarSubscribersSegmentMemberField {
    fn default() -> SimilarSubscribersSegmentMemberField {
        SimilarSubscribersSegmentMemberField::Noop
    }
}
impl SimilarSubscribersSegmentMemberField {
    pub fn is_noop(&self) -> bool {
        matches!(self, SimilarSubscribersSegmentMemberField::Noop)
    }
}

/**
* Members who are/are not apart of a 'similar subscribers' segment.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SimilarSubscribersSegmentMemberOperator {
    #[serde(rename = "fuzzy_is")]
    FuzzyIs,
    #[serde(rename = "fuzzy_not")]
    FuzzyNot,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SimilarSubscribersSegmentMemberOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SimilarSubscribersSegmentMemberOperator::FuzzyIs => "fuzzy_is",
            SimilarSubscribersSegmentMemberOperator::FuzzyNot => "fuzzy_not",
            SimilarSubscribersSegmentMemberOperator::Noop => "",
            SimilarSubscribersSegmentMemberOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SimilarSubscribersSegmentMemberOperator {
    fn default() -> SimilarSubscribersSegmentMemberOperator {
        SimilarSubscribersSegmentMemberOperator::Noop
    }
}
impl SimilarSubscribersSegmentMemberOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, SimilarSubscribersSegmentMemberOperator::Noop)
    }
}

/// Segment by similar subscribers.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SimilarSubscribersSegmentMember {
    /**
    * Segment by similar subscribers.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<SimilarSubscribersSegmentMemberConditionType>,
    /**
    * Segment by similar subscribers.
    */
    #[serde(
        default,
        skip_serializing_if = "SimilarSubscribersSegmentMemberField::is_noop"
    )]
    pub field: SimilarSubscribersSegmentMemberField,
    /**
    * Members who are/are not apart of a 'similar subscribers' segment.
    */
    #[serde(
        default,
        skip_serializing_if = "SimilarSubscribersSegmentMemberOperator::is_noop"
    )]
    pub op: SimilarSubscribersSegmentMemberOperator,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub value: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StaticSegmentMemberConditionType {
    #[serde(rename = "StaticSegment")]
    StaticSegment,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StaticSegmentMemberConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StaticSegmentMemberConditionType::StaticSegment => "StaticSegment",
            StaticSegmentMemberConditionType::Noop => "",
            StaticSegmentMemberConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StaticSegmentMemberConditionType {
    fn default() -> StaticSegmentMemberConditionType {
        StaticSegmentMemberConditionType::Noop
    }
}
impl StaticSegmentMemberConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, StaticSegmentMemberConditionType::Noop)
    }
}

/**
* Segment by a given static segment.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StaticSegmentMemberField {
    #[serde(rename = "static_segment")]
    StaticSegment,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StaticSegmentMemberField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StaticSegmentMemberField::StaticSegment => "static_segment",
            StaticSegmentMemberField::Noop => "",
            StaticSegmentMemberField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StaticSegmentMemberField {
    fn default() -> StaticSegmentMemberField {
        StaticSegmentMemberField::Noop
    }
}
impl StaticSegmentMemberField {
    pub fn is_noop(&self) -> bool {
        matches!(self, StaticSegmentMemberField::Noop)
    }
}

/**
* Members who are/are not apart of a static segment.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StaticSegmentMemberOperator {
    #[serde(rename = "static_is")]
    StaticIs,
    #[serde(rename = "static_not")]
    StaticNot,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StaticSegmentMemberOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StaticSegmentMemberOperator::StaticIs => "static_is",
            StaticSegmentMemberOperator::StaticNot => "static_not",
            StaticSegmentMemberOperator::Noop => "",
            StaticSegmentMemberOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StaticSegmentMemberOperator {
    fn default() -> StaticSegmentMemberOperator {
        StaticSegmentMemberOperator::Noop
    }
}
impl StaticSegmentMemberOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, StaticSegmentMemberOperator::Noop)
    }
}

/// Segment by a given static segment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct StaticSegmentMember {
    /**
    * Segment by a given static segment.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<StaticSegmentMemberConditionType>,
    /**
    * Segment by a given static segment.
    */
    #[serde(default, skip_serializing_if = "StaticSegmentMemberField::is_noop")]
    pub field: StaticSegmentMemberField,
    /**
    * Members who are/are not apart of a static segment.
    */
    #[serde(default, skip_serializing_if = "StaticSegmentMemberOperator::is_noop")]
    pub op: StaticSegmentMemberOperator,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub value: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LocationBasedSegmentConditionType {
    #[serde(rename = "IPGeoCountryState")]
    IpGeoCountryState,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LocationBasedSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LocationBasedSegmentConditionType::IpGeoCountryState => "IPGeoCountryState",
            LocationBasedSegmentConditionType::Noop => "",
            LocationBasedSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LocationBasedSegmentConditionType {
    fn default() -> LocationBasedSegmentConditionType {
        LocationBasedSegmentConditionType::Noop
    }
}
impl LocationBasedSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, LocationBasedSegmentConditionType::Noop)
    }
}

/**
* Segmenting subscribers who are within a specific location.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LocationBasedSegmentField {
    #[serde(rename = "ipgeo")]
    Ipgeo,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LocationBasedSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LocationBasedSegmentField::Ipgeo => "ipgeo",
            LocationBasedSegmentField::Noop => "",
            LocationBasedSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LocationBasedSegmentField {
    fn default() -> LocationBasedSegmentField {
        LocationBasedSegmentField::Noop
    }
}
impl LocationBasedSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, LocationBasedSegmentField::Noop)
    }
}

/**
* Segment members who are within a specific country or US state.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LocationBasedSegmentOperator {
    #[serde(rename = "ipgeocountry")]
    Ipgeocountry,
    #[serde(rename = "ipgeonotcountry")]
    Ipgeonotcountry,
    #[serde(rename = "ipgeonotstate")]
    Ipgeonotstate,
    #[serde(rename = "ipgeostate")]
    Ipgeostate,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LocationBasedSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LocationBasedSegmentOperator::Ipgeocountry => "ipgeocountry",
            LocationBasedSegmentOperator::Ipgeonotcountry => "ipgeonotcountry",
            LocationBasedSegmentOperator::Ipgeonotstate => "ipgeonotstate",
            LocationBasedSegmentOperator::Ipgeostate => "ipgeostate",
            LocationBasedSegmentOperator::Noop => "",
            LocationBasedSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LocationBasedSegmentOperator {
    fn default() -> LocationBasedSegmentOperator {
        LocationBasedSegmentOperator::Noop
    }
}
impl LocationBasedSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, LocationBasedSegmentOperator::Noop)
    }
}

/// Segment by a specific country or US state.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LocationBasedSegment {
    /**
    * Segment by a specific country or US state.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<LocationBasedSegmentConditionType>,
    /**
    * Segmenting subscribers who are within a specific location.
    */
    #[serde(default, skip_serializing_if = "LocationBasedSegmentField::is_noop")]
    pub field: LocationBasedSegmentField,
    /**
    * Segment members who are within a specific country or US state.
    */
    #[serde(default, skip_serializing_if = "LocationBasedSegmentOperator::is_noop")]
    pub op: LocationBasedSegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GeolocationSegmentConditionType {
    #[serde(rename = "IPGeoIn")]
    IpGeoIn,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GeolocationSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GeolocationSegmentConditionType::IpGeoIn => "IPGeoIn",
            GeolocationSegmentConditionType::Noop => "",
            GeolocationSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GeolocationSegmentConditionType {
    fn default() -> GeolocationSegmentConditionType {
        GeolocationSegmentConditionType::Noop
    }
}
impl GeolocationSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GeolocationSegmentConditionType::Noop)
    }
}

/**
* Segment members who are within a specific geographic region.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GeolocationSegmentOperator {
    #[serde(rename = "ipgeoin")]
    Ipgeoin,
    #[serde(rename = "ipgeonotin")]
    Ipgeonotin,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GeolocationSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GeolocationSegmentOperator::Ipgeoin => "ipgeoin",
            GeolocationSegmentOperator::Ipgeonotin => "ipgeonotin",
            GeolocationSegmentOperator::Noop => "",
            GeolocationSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GeolocationSegmentOperator {
    fn default() -> GeolocationSegmentOperator {
        GeolocationSegmentOperator::Noop
    }
}
impl GeolocationSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, GeolocationSegmentOperator::Noop)
    }
}

/// Segment by a specific geographic region.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GeolocationSegment {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub addr: String,
    /**
    * Segment by a specific geographic region.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<GeolocationSegmentConditionType>,
    /**
    * Segmenting subscribers who are within a specific location.
    */
    #[serde(default, skip_serializing_if = "LocationBasedSegmentField::is_noop")]
    pub field: LocationBasedSegmentField,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lat: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lng: String,
    /**
    * Segment members who are within a specific geographic region.
    */
    #[serde(default, skip_serializing_if = "GeolocationSegmentOperator::is_noop")]
    pub op: GeolocationSegmentOperator,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub value: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UsZipCodeSegmentConditionType {
    #[serde(rename = "IPGeoInZip")]
    IpGeoInZip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UsZipCodeSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            UsZipCodeSegmentConditionType::IpGeoInZip => "IPGeoInZip",
            UsZipCodeSegmentConditionType::Noop => "",
            UsZipCodeSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UsZipCodeSegmentConditionType {
    fn default() -> UsZipCodeSegmentConditionType {
        UsZipCodeSegmentConditionType::Noop
    }
}
impl UsZipCodeSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, UsZipCodeSegmentConditionType::Noop)
    }
}

/**
* Segment members who are within a specific US zip code.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UsZipCodeSegmentOperator {
    #[serde(rename = "ipgeoinzip")]
    Ipgeoinzip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UsZipCodeSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            UsZipCodeSegmentOperator::Ipgeoinzip => "ipgeoinzip",
            UsZipCodeSegmentOperator::Noop => "",
            UsZipCodeSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UsZipCodeSegmentOperator {
    fn default() -> UsZipCodeSegmentOperator {
        UsZipCodeSegmentOperator::Noop
    }
}
impl UsZipCodeSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, UsZipCodeSegmentOperator::Noop)
    }
}

/// Segment by a specific US ZIP code.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UsZipCodeSegment {
    /**
    * Segment by a specific US ZIP code.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<UsZipCodeSegmentConditionType>,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub extra: i64,
    /**
    * Segmenting subscribers who are within a specific location.
    */
    #[serde(default, skip_serializing_if = "LocationBasedSegmentField::is_noop")]
    pub field: LocationBasedSegmentField,
    /**
    * Segment members who are within a specific US zip code.
    */
    #[serde(default, skip_serializing_if = "UsZipCodeSegmentOperator::is_noop")]
    pub op: UsZipCodeSegmentOperator,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub value: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UnknownLocationBasedSegmentConditionType {
    #[serde(rename = "IPGeoUnknown")]
    IpGeoUnknown,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UnknownLocationBasedSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            UnknownLocationBasedSegmentConditionType::IpGeoUnknown => "IPGeoUnknown",
            UnknownLocationBasedSegmentConditionType::Noop => "",
            UnknownLocationBasedSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UnknownLocationBasedSegmentConditionType {
    fn default() -> UnknownLocationBasedSegmentConditionType {
        UnknownLocationBasedSegmentConditionType::Noop
    }
}
impl UnknownLocationBasedSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, UnknownLocationBasedSegmentConditionType::Noop)
    }
}

/**
* Segment members for which location information is unknown.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum UnknownLocationBasedSegmentOperator {
    #[serde(rename = "ipgeounknown")]
    Ipgeounknown,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UnknownLocationBasedSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            UnknownLocationBasedSegmentOperator::Ipgeounknown => "ipgeounknown",
            UnknownLocationBasedSegmentOperator::Noop => "",
            UnknownLocationBasedSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UnknownLocationBasedSegmentOperator {
    fn default() -> UnknownLocationBasedSegmentOperator {
        UnknownLocationBasedSegmentOperator::Noop
    }
}
impl UnknownLocationBasedSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, UnknownLocationBasedSegmentOperator::Noop)
    }
}

/// Segment members whose location information is unknown.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UnknownLocationBasedSegment {
    /**
    * Segment members whose location information is unknown.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<UnknownLocationBasedSegmentConditionType>,
    /**
    * Segmenting subscribers who are within a specific location.
    */
    #[serde(default, skip_serializing_if = "LocationBasedSegmentField::is_noop")]
    pub field: LocationBasedSegmentField,
    /**
    * Segment members for which location information is unknown.
    */
    #[serde(
        default,
        skip_serializing_if = "UnknownLocationBasedSegmentOperator::is_noop"
    )]
    pub op: UnknownLocationBasedSegmentOperator,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ZipCodeLocationBasedSegmentConditionType {
    #[serde(rename = "IPGeoZip")]
    IpGeoZip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ZipCodeLocationBasedSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ZipCodeLocationBasedSegmentConditionType::IpGeoZip => "IPGeoZip",
            ZipCodeLocationBasedSegmentConditionType::Noop => "",
            ZipCodeLocationBasedSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ZipCodeLocationBasedSegmentConditionType {
    fn default() -> ZipCodeLocationBasedSegmentConditionType {
        ZipCodeLocationBasedSegmentConditionType::Noop
    }
}
impl ZipCodeLocationBasedSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ZipCodeLocationBasedSegmentConditionType::Noop)
    }
}

/**
* Segment members who are/are not within a specific US zip code.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ZipCodeLocationBasedSegmentOperator {
    #[serde(rename = "ipgeoiszip")]
    Ipgeoiszip,
    #[serde(rename = "ipgeonotzip")]
    Ipgeonotzip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ZipCodeLocationBasedSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ZipCodeLocationBasedSegmentOperator::Ipgeoiszip => "ipgeoiszip",
            ZipCodeLocationBasedSegmentOperator::Ipgeonotzip => "ipgeonotzip",
            ZipCodeLocationBasedSegmentOperator::Noop => "",
            ZipCodeLocationBasedSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ZipCodeLocationBasedSegmentOperator {
    fn default() -> ZipCodeLocationBasedSegmentOperator {
        ZipCodeLocationBasedSegmentOperator::Noop
    }
}
impl ZipCodeLocationBasedSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, ZipCodeLocationBasedSegmentOperator::Noop)
    }
}

/// Segment by a specific US ZIP code.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ZipCodeLocationBasedSegment {
    /**
    * Segment by a specific US ZIP code.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<ZipCodeLocationBasedSegmentConditionType>,
    /**
    * Segmenting subscribers who are within a specific location.
    */
    #[serde(default, skip_serializing_if = "LocationBasedSegmentField::is_noop")]
    pub field: LocationBasedSegmentField,
    /**
    * Segment members who are/are not within a specific US zip code.
    */
    #[serde(
        default,
        skip_serializing_if = "ZipCodeLocationBasedSegmentOperator::is_noop"
    )]
    pub op: ZipCodeLocationBasedSegmentOperator,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub value: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesAgeSegmentConditionType {
    #[serde(rename = "SocialAge")]
    SocialAge,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesAgeSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesAgeSegmentConditionType::SocialAge => "SocialAge",
            SocialProfilesAgeSegmentConditionType::Noop => "",
            SocialProfilesAgeSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesAgeSegmentConditionType {
    fn default() -> SocialProfilesAgeSegmentConditionType {
        SocialProfilesAgeSegmentConditionType::Noop
    }
}
impl SocialProfilesAgeSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesAgeSegmentConditionType::Noop)
    }
}

/**
* Segment by age ranges in Social Profiles data.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesAgeSegmentField {
    #[serde(rename = "social_age")]
    SocialAge,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesAgeSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesAgeSegmentField::SocialAge => "social_age",
            SocialProfilesAgeSegmentField::Noop => "",
            SocialProfilesAgeSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesAgeSegmentField {
    fn default() -> SocialProfilesAgeSegmentField {
        SocialProfilesAgeSegmentField::Noop
    }
}
impl SocialProfilesAgeSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesAgeSegmentField::Noop)
    }
}

/**
* The age range to segment.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Value {
    #[serde(rename = "18-24")]
    OneThousandEightHundredAndTwentyFour,
    #[serde(rename = "25-34")]
    TwoThousandFiveHundredAndThirtyFour,
    #[serde(rename = "35-54")]
    ThreeThousandFiveHundredAndFiftyFour,
    #[serde(rename = "55+")]
    FiftyFive,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Value::OneThousandEightHundredAndTwentyFour => "18-24",
            Value::TwoThousandFiveHundredAndThirtyFour => "25-34",
            Value::ThreeThousandFiveHundredAndFiftyFour => "35-54",
            Value::FiftyFive => "55+",
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

/// Segment by age ranges in Social Profiles data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SocialProfilesAgeSegment {
    /**
    * Segment by age ranges in Social Profiles data.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<SocialProfilesAgeSegmentConditionType>,
    /**
    * Segment by age ranges in Social Profiles data.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesAgeSegmentField::is_noop"
    )]
    pub field: SocialProfilesAgeSegmentField,
    /**
    * Whether the member's language is or is not set to a specific language.
    */
    #[serde(default, skip_serializing_if = "LanguageSegmentOperator::is_noop")]
    pub op: LanguageSegmentOperator,
    /**
    * The age range to segment.
    */
    #[serde(default, skip_serializing_if = "Value::is_noop")]
    pub value: Value,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesGenderSegmentConditionType {
    #[serde(rename = "SocialGender")]
    SocialGender,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesGenderSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesGenderSegmentConditionType::SocialGender => "SocialGender",
            SocialProfilesGenderSegmentConditionType::Noop => "",
            SocialProfilesGenderSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesGenderSegmentConditionType {
    fn default() -> SocialProfilesGenderSegmentConditionType {
        SocialProfilesGenderSegmentConditionType::Noop
    }
}
impl SocialProfilesGenderSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesGenderSegmentConditionType::Noop)
    }
}

/**
* Segment by listed gender in Social Profiles data.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesGenderSegmentField {
    #[serde(rename = "social_gender")]
    SocialGender,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesGenderSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesGenderSegmentField::SocialGender => "social_gender",
            SocialProfilesGenderSegmentField::Noop => "",
            SocialProfilesGenderSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesGenderSegmentField {
    fn default() -> SocialProfilesGenderSegmentField {
        SocialProfilesGenderSegmentField::Noop
    }
}
impl SocialProfilesGenderSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesGenderSegmentField::Noop)
    }
}

/**
* The Social Profiles gender to segment.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesGenderSegmentOperator {
    #[serde(rename = "female")]
    Female,
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesGenderSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesGenderSegmentOperator::Female => "female",
            SocialProfilesGenderSegmentOperator::Male => "male",
            SocialProfilesGenderSegmentOperator::Noop => "",
            SocialProfilesGenderSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesGenderSegmentOperator {
    fn default() -> SocialProfilesGenderSegmentOperator {
        SocialProfilesGenderSegmentOperator::Noop
    }
}
impl SocialProfilesGenderSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesGenderSegmentOperator::Noop)
    }
}

/// Segment by listed gender in Social Profiles data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SocialProfilesGenderSegment {
    /**
    * Segment by listed gender in Social Profiles data.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<SocialProfilesGenderSegmentConditionType>,
    /**
    * Segment by listed gender in Social Profiles data.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesGenderSegmentField::is_noop"
    )]
    pub field: SocialProfilesGenderSegmentField,
    /**
    * Whether the member's language is or is not set to a specific language.
    */
    #[serde(default, skip_serializing_if = "LanguageSegmentOperator::is_noop")]
    pub op: LanguageSegmentOperator,
    /**
    * The Social Profiles gender to segment.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesGenderSegmentOperator::is_noop"
    )]
    pub value: SocialProfilesGenderSegmentOperator,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesInfluenceSegmentConditionType {
    #[serde(rename = "SocialInfluence")]
    SocialInfluence,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesInfluenceSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesInfluenceSegmentConditionType::SocialInfluence => "SocialInfluence",
            SocialProfilesInfluenceSegmentConditionType::Noop => "",
            SocialProfilesInfluenceSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesInfluenceSegmentConditionType {
    fn default() -> SocialProfilesInfluenceSegmentConditionType {
        SocialProfilesInfluenceSegmentConditionType::Noop
    }
}
impl SocialProfilesInfluenceSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesInfluenceSegmentConditionType::Noop)
    }
}

/**
* Segment by influence rating in Social Profiles data.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesInfluenceSegmentField {
    #[serde(rename = "social_influence")]
    SocialInfluence,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesInfluenceSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesInfluenceSegmentField::SocialInfluence => "social_influence",
            SocialProfilesInfluenceSegmentField::Noop => "",
            SocialProfilesInfluenceSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesInfluenceSegmentField {
    fn default() -> SocialProfilesInfluenceSegmentField {
        SocialProfilesInfluenceSegmentField::Noop
    }
}
impl SocialProfilesInfluenceSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesInfluenceSegmentField::Noop)
    }
}

/// Segment by influence rating in Social Profiles data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SocialProfilesInfluenceSegment {
    /**
    * Segment by influence rating in Social Profiles data.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<SocialProfilesInfluenceSegmentConditionType>,
    /**
    * Segment by influence rating in Social Profiles data.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesInfluenceSegmentField::is_noop"
    )]
    pub field: SocialProfilesInfluenceSegmentField,
    /**
    * Members who have have a rating that is/not exactly a given number or members who have a rating greater/less than a given number.
    */
    #[serde(default, skip_serializing_if = "MemberRatingSegmentOperator::is_noop")]
    pub op: MemberRatingSegmentOperator,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub value: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesNetworkSegmentConditionType {
    #[serde(rename = "SocialNetworkMember")]
    SocialNetworkMember,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesNetworkSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesNetworkSegmentConditionType::SocialNetworkMember => "SocialNetworkMember",
            SocialProfilesNetworkSegmentConditionType::Noop => "",
            SocialProfilesNetworkSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesNetworkSegmentConditionType {
    fn default() -> SocialProfilesNetworkSegmentConditionType {
        SocialProfilesNetworkSegmentConditionType::Noop
    }
}
impl SocialProfilesNetworkSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesNetworkSegmentConditionType::Noop)
    }
}

/**
* Segment by social network in Social Profiles data.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesNetworkSegmentField {
    #[serde(rename = "social_network")]
    SocialNetwork,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesNetworkSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesNetworkSegmentField::SocialNetwork => "social_network",
            SocialProfilesNetworkSegmentField::Noop => "",
            SocialProfilesNetworkSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesNetworkSegmentField {
    fn default() -> SocialProfilesNetworkSegmentField {
        SocialProfilesNetworkSegmentField::Noop
    }
}
impl SocialProfilesNetworkSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesNetworkSegmentField::Noop)
    }
}

/**
* The social network to segment against.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesNetworkSegmentOperator {
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "flickr")]
    Flickr,
    #[serde(rename = "foursquare")]
    Foursquare,
    #[serde(rename = "lastfm")]
    Lastfm,
    #[serde(rename = "linkedin")]
    Linkedin,
    #[serde(rename = "myspace")]
    Myspace,
    #[serde(rename = "quora")]
    Quora,
    #[serde(rename = "twitter")]
    Twitter,
    #[serde(rename = "vimeo")]
    Vimeo,
    #[serde(rename = "yelp")]
    Yelp,
    #[serde(rename = "youtube")]
    Youtube,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesNetworkSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesNetworkSegmentOperator::Facebook => "facebook",
            SocialProfilesNetworkSegmentOperator::Flickr => "flickr",
            SocialProfilesNetworkSegmentOperator::Foursquare => "foursquare",
            SocialProfilesNetworkSegmentOperator::Lastfm => "lastfm",
            SocialProfilesNetworkSegmentOperator::Linkedin => "linkedin",
            SocialProfilesNetworkSegmentOperator::Myspace => "myspace",
            SocialProfilesNetworkSegmentOperator::Quora => "quora",
            SocialProfilesNetworkSegmentOperator::Twitter => "twitter",
            SocialProfilesNetworkSegmentOperator::Vimeo => "vimeo",
            SocialProfilesNetworkSegmentOperator::Yelp => "yelp",
            SocialProfilesNetworkSegmentOperator::Youtube => "youtube",
            SocialProfilesNetworkSegmentOperator::Noop => "",
            SocialProfilesNetworkSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesNetworkSegmentOperator {
    fn default() -> SocialProfilesNetworkSegmentOperator {
        SocialProfilesNetworkSegmentOperator::Noop
    }
}
impl SocialProfilesNetworkSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesNetworkSegmentOperator::Noop)
    }
}

/// Segment by social network in Social Profiles data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SocialProfilesNetworkSegment {
    /**
    * Segment by social network in Social Profiles data.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<SocialProfilesNetworkSegmentConditionType>,
    /**
    * Segment by social network in Social Profiles data.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesNetworkSegmentField::is_noop"
    )]
    pub field: SocialProfilesNetworkSegmentField,
    /**
    * Members have/have not interacted with a specific poll in a Mailchimp email.
    */
    #[serde(default, skip_serializing_if = "PollActivitySegmentOperator::is_noop")]
    pub op: PollActivitySegmentOperator,
    /**
    * The social network to segment against.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesNetworkSegmentOperator::is_noop"
    )]
    pub value: SocialProfilesNetworkSegmentOperator,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesNetworkFollowSegmentConditionType {
    #[serde(rename = "SocialNetworkFollow")]
    SocialNetworkFollow,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesNetworkFollowSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesNetworkFollowSegmentConditionType::SocialNetworkFollow => {
                "SocialNetworkFollow"
            }
            SocialProfilesNetworkFollowSegmentConditionType::Noop => "",
            SocialProfilesNetworkFollowSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesNetworkFollowSegmentConditionType {
    fn default() -> SocialProfilesNetworkFollowSegmentConditionType {
        SocialProfilesNetworkFollowSegmentConditionType::Noop
    }
}
impl SocialProfilesNetworkFollowSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesNetworkFollowSegmentConditionType::Noop)
    }
}

/**
* Members who are/not following a linked account on a given social network.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesNetworkFollowSegmentOperator {
    #[serde(rename = "follow")]
    Follow,
    #[serde(rename = "notfollow")]
    Notfollow,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesNetworkFollowSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesNetworkFollowSegmentOperator::Follow => "follow",
            SocialProfilesNetworkFollowSegmentOperator::Notfollow => "notfollow",
            SocialProfilesNetworkFollowSegmentOperator::Noop => "",
            SocialProfilesNetworkFollowSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesNetworkFollowSegmentOperator {
    fn default() -> SocialProfilesNetworkFollowSegmentOperator {
        SocialProfilesNetworkFollowSegmentOperator::Noop
    }
}
impl SocialProfilesNetworkFollowSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesNetworkFollowSegmentOperator::Noop)
    }
}

/**
* The social network to segment against.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SocialProfilesNetworkFollowSegmentOperatorData {
    #[serde(rename = "twitter_follow")]
    TwitterFollow,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SocialProfilesNetworkFollowSegmentOperatorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SocialProfilesNetworkFollowSegmentOperatorData::TwitterFollow => "twitter_follow",
            SocialProfilesNetworkFollowSegmentOperatorData::Noop => "",
            SocialProfilesNetworkFollowSegmentOperatorData::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SocialProfilesNetworkFollowSegmentOperatorData {
    fn default() -> SocialProfilesNetworkFollowSegmentOperatorData {
        SocialProfilesNetworkFollowSegmentOperatorData::Noop
    }
}
impl SocialProfilesNetworkFollowSegmentOperatorData {
    pub fn is_noop(&self) -> bool {
        matches!(self, SocialProfilesNetworkFollowSegmentOperatorData::Noop)
    }
}

/// Segment by social network in Social Profiles data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SocialProfilesNetworkFollowSegment {
    /**
    * Segment by social network in Social Profiles data.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<SocialProfilesNetworkFollowSegmentConditionType>,
    /**
    * Segment by social network in Social Profiles data.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesNetworkSegmentField::is_noop"
    )]
    pub field: SocialProfilesNetworkSegmentField,
    /**
    * Members who are/not following a linked account on a given social network.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesNetworkFollowSegmentOperator::is_noop"
    )]
    pub op: SocialProfilesNetworkFollowSegmentOperator,
    /**
    * The social network to segment against.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesNetworkFollowSegmentOperatorData::is_noop"
    )]
    pub value: SocialProfilesNetworkFollowSegmentOperatorData,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AddressMergeFieldSegmentConditionType {
    #[serde(rename = "AddressMerge")]
    AddressMerge,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AddressMergeFieldSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AddressMergeFieldSegmentConditionType::AddressMerge => "AddressMerge",
            AddressMergeFieldSegmentConditionType::Noop => "",
            AddressMergeFieldSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AddressMergeFieldSegmentConditionType {
    fn default() -> AddressMergeFieldSegmentConditionType {
        AddressMergeFieldSegmentConditionType::Noop
    }
}
impl AddressMergeFieldSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AddressMergeFieldSegmentConditionType::Noop)
    }
}

/**
* Whether the member's address merge field contains/does not contain a value or is/is not blank.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AddressMergeFieldSegmentOperator {
    #[serde(rename = "blank")]
    Blank,
    #[serde(rename = "blank_not")]
    BlankNot,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "notcontain")]
    Notcontain,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AddressMergeFieldSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AddressMergeFieldSegmentOperator::Blank => "blank",
            AddressMergeFieldSegmentOperator::BlankNot => "blank_not",
            AddressMergeFieldSegmentOperator::Contains => "contains",
            AddressMergeFieldSegmentOperator::Notcontain => "notcontain",
            AddressMergeFieldSegmentOperator::Noop => "",
            AddressMergeFieldSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AddressMergeFieldSegmentOperator {
    fn default() -> AddressMergeFieldSegmentOperator {
        AddressMergeFieldSegmentOperator::Noop
    }
}
impl AddressMergeFieldSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, AddressMergeFieldSegmentOperator::Noop)
    }
}

/// Segment by an address-type merge field.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddressMergeFieldSegment {
    /**
    * Segment by an address-type merge field.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<AddressMergeFieldSegmentConditionType>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
    * Whether the member's address merge field contains/does not contain a value or is/is not blank.
    */
    #[serde(
        default,
        skip_serializing_if = "AddressMergeFieldSegmentOperator::is_noop"
    )]
    pub op: AddressMergeFieldSegmentOperator,
    /**
    * Segment by an address-type merge field.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AddressZipMergeFieldSegmentConditionType {
    #[serde(rename = "ZipMerge")]
    ZipMerge,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AddressZipMergeFieldSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AddressZipMergeFieldSegmentConditionType::ZipMerge => "ZipMerge",
            AddressZipMergeFieldSegmentConditionType::Noop => "",
            AddressZipMergeFieldSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AddressZipMergeFieldSegmentConditionType {
    fn default() -> AddressZipMergeFieldSegmentConditionType {
        AddressZipMergeFieldSegmentConditionType::Noop
    }
}
impl AddressZipMergeFieldSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AddressZipMergeFieldSegmentConditionType::Noop)
    }
}

/**
* Whether the member's address merge field is within a given distance from a city or zip.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AddressZipMergeFieldSegmentOperator {
    #[serde(rename = "geoin")]
    Geoin,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AddressZipMergeFieldSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AddressZipMergeFieldSegmentOperator::Geoin => "geoin",
            AddressZipMergeFieldSegmentOperator::Noop => "",
            AddressZipMergeFieldSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AddressZipMergeFieldSegmentOperator {
    fn default() -> AddressZipMergeFieldSegmentOperator {
        AddressZipMergeFieldSegmentOperator::Noop
    }
}
impl AddressZipMergeFieldSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, AddressZipMergeFieldSegmentOperator::Noop)
    }
}

/// Segment by an address-type merge field within a given distance.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddressZipMergeFieldSegment {
    /**
    * Segment by an address-type merge field within a given distance.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<AddressZipMergeFieldSegmentConditionType>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub extra: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
    * Whether the member's address merge field is within a given distance from a city or zip.
    */
    #[serde(
        default,
        skip_serializing_if = "AddressZipMergeFieldSegmentOperator::is_noop"
    )]
    pub op: AddressZipMergeFieldSegmentOperator,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum BirthdayMergeFieldSegmentConditionType {
    #[serde(rename = "BirthdayMerge")]
    BirthdayMerge,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for BirthdayMergeFieldSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            BirthdayMergeFieldSegmentConditionType::BirthdayMerge => "BirthdayMerge",
            BirthdayMergeFieldSegmentConditionType::Noop => "",
            BirthdayMergeFieldSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for BirthdayMergeFieldSegmentConditionType {
    fn default() -> BirthdayMergeFieldSegmentConditionType {
        BirthdayMergeFieldSegmentConditionType::Noop
    }
}
impl BirthdayMergeFieldSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, BirthdayMergeFieldSegmentConditionType::Noop)
    }
}

/**
* Whether the member's birthday merge information is/is not a certain date or is/is not blank.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum BirthdayMergeFieldSegmentOperator {
    #[serde(rename = "blank")]
    Blank,
    #[serde(rename = "blank_not")]
    BlankNot,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for BirthdayMergeFieldSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            BirthdayMergeFieldSegmentOperator::Blank => "blank",
            BirthdayMergeFieldSegmentOperator::BlankNot => "blank_not",
            BirthdayMergeFieldSegmentOperator::Is => "is",
            BirthdayMergeFieldSegmentOperator::Not => "not",
            BirthdayMergeFieldSegmentOperator::Noop => "",
            BirthdayMergeFieldSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for BirthdayMergeFieldSegmentOperator {
    fn default() -> BirthdayMergeFieldSegmentOperator {
        BirthdayMergeFieldSegmentOperator::Noop
    }
}
impl BirthdayMergeFieldSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, BirthdayMergeFieldSegmentOperator::Noop)
    }
}

/// Segment by a contact's birthday.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BirthdayMergeFieldSegment {
    /**
    * Segment by a contact's birthday.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<BirthdayMergeFieldSegmentConditionType>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
    * Whether the member's birthday merge information is/is not a certain date or is/is not blank.
    */
    #[serde(
        default,
        skip_serializing_if = "BirthdayMergeFieldSegmentOperator::is_noop"
    )]
    pub op: BirthdayMergeFieldSegmentOperator,
    /**
    * Segment by a contact's birthday.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DateMergeFieldSegmentConditionType {
    #[serde(rename = "DateMerge")]
    DateMerge,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DateMergeFieldSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DateMergeFieldSegmentConditionType::DateMerge => "DateMerge",
            DateMergeFieldSegmentConditionType::Noop => "",
            DateMergeFieldSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DateMergeFieldSegmentConditionType {
    fn default() -> DateMergeFieldSegmentConditionType {
        DateMergeFieldSegmentConditionType::Noop
    }
}
impl DateMergeFieldSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DateMergeFieldSegmentConditionType::Noop)
    }
}

/**
* Whether the member's merge information is/is not, is greater/less than a value or is/is not blank.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DateMergeFieldSegmentOperator {
    #[serde(rename = "blank")]
    Blank,
    #[serde(rename = "blank_not")]
    BlankNot,
    #[serde(rename = "greater")]
    Greater,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DateMergeFieldSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DateMergeFieldSegmentOperator::Blank => "blank",
            DateMergeFieldSegmentOperator::BlankNot => "blank_not",
            DateMergeFieldSegmentOperator::Greater => "greater",
            DateMergeFieldSegmentOperator::Is => "is",
            DateMergeFieldSegmentOperator::Less => "less",
            DateMergeFieldSegmentOperator::Not => "not",
            DateMergeFieldSegmentOperator::Noop => "",
            DateMergeFieldSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DateMergeFieldSegmentOperator {
    fn default() -> DateMergeFieldSegmentOperator {
        DateMergeFieldSegmentOperator::Noop
    }
}
impl DateMergeFieldSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, DateMergeFieldSegmentOperator::Noop)
    }
}

/// Segment by a given date merge field.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DateMergeFieldSegment {
    /**
    * Segment by a given date merge field.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<DateMergeFieldSegmentConditionType>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
    * Whether the member's merge information is/is not, is greater/less than a value or is/is not blank.
    */
    #[serde(
        default,
        skip_serializing_if = "DateMergeFieldSegmentOperator::is_noop"
    )]
    pub op: DateMergeFieldSegmentOperator,
    /**
    * Segment by a given date merge field.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DropdownRadioMergeFieldSegmentConditionType {
    #[serde(rename = "SelectMerge")]
    SelectMerge,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DropdownRadioMergeFieldSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DropdownRadioMergeFieldSegmentConditionType::SelectMerge => "SelectMerge",
            DropdownRadioMergeFieldSegmentConditionType::Noop => "",
            DropdownRadioMergeFieldSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DropdownRadioMergeFieldSegmentConditionType {
    fn default() -> DropdownRadioMergeFieldSegmentConditionType {
        DropdownRadioMergeFieldSegmentConditionType::Noop
    }
}
impl DropdownRadioMergeFieldSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DropdownRadioMergeFieldSegmentConditionType::Noop)
    }
}

/**
* Whether the member's merge information is/is not a value or is/is not blank.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DropdownRadioMergeFieldSegmentOperator {
    #[serde(rename = "blank")]
    Blank,
    #[serde(rename = "blank_not")]
    BlankNot,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "notcontain")]
    Notcontain,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DropdownRadioMergeFieldSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DropdownRadioMergeFieldSegmentOperator::Blank => "blank",
            DropdownRadioMergeFieldSegmentOperator::BlankNot => "blank_not",
            DropdownRadioMergeFieldSegmentOperator::Contains => "contains",
            DropdownRadioMergeFieldSegmentOperator::Is => "is",
            DropdownRadioMergeFieldSegmentOperator::Not => "not",
            DropdownRadioMergeFieldSegmentOperator::Notcontain => "notcontain",
            DropdownRadioMergeFieldSegmentOperator::Noop => "",
            DropdownRadioMergeFieldSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DropdownRadioMergeFieldSegmentOperator {
    fn default() -> DropdownRadioMergeFieldSegmentOperator {
        DropdownRadioMergeFieldSegmentOperator::Noop
    }
}
impl DropdownRadioMergeFieldSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, DropdownRadioMergeFieldSegmentOperator::Noop)
    }
}

/// An individual segment condition
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DropdownRadioMergeFieldSegment {
    /**
    * An individual segment condition
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<DropdownRadioMergeFieldSegmentConditionType>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
    * Whether the member's merge information is/is not a value or is/is not blank.
    */
    #[serde(
        default,
        skip_serializing_if = "DropdownRadioMergeFieldSegmentOperator::is_noop"
    )]
    pub op: DropdownRadioMergeFieldSegmentOperator,
    /**
    * An individual segment condition
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TextNumberMergeFieldSegmentConditionType {
    #[serde(rename = "TextMerge")]
    TextMerge,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TextNumberMergeFieldSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TextNumberMergeFieldSegmentConditionType::TextMerge => "TextMerge",
            TextNumberMergeFieldSegmentConditionType::Noop => "",
            TextNumberMergeFieldSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TextNumberMergeFieldSegmentConditionType {
    fn default() -> TextNumberMergeFieldSegmentConditionType {
        TextNumberMergeFieldSegmentConditionType::Noop
    }
}
impl TextNumberMergeFieldSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, TextNumberMergeFieldSegmentConditionType::Noop)
    }
}

/**
* Whether the member's merge information is/is not, contains/does not contain, starts/ends with, or is greater/less than a value
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TextNumberMergeFieldSegmentOperator {
    #[serde(rename = "blank")]
    Blank,
    #[serde(rename = "blank_not")]
    BlankNot,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "ends")]
    Ends,
    #[serde(rename = "greater")]
    Greater,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "notcontain")]
    Notcontain,
    #[serde(rename = "starts")]
    Starts,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TextNumberMergeFieldSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TextNumberMergeFieldSegmentOperator::Blank => "blank",
            TextNumberMergeFieldSegmentOperator::BlankNot => "blank_not",
            TextNumberMergeFieldSegmentOperator::Contains => "contains",
            TextNumberMergeFieldSegmentOperator::Ends => "ends",
            TextNumberMergeFieldSegmentOperator::Greater => "greater",
            TextNumberMergeFieldSegmentOperator::Is => "is",
            TextNumberMergeFieldSegmentOperator::Less => "less",
            TextNumberMergeFieldSegmentOperator::Not => "not",
            TextNumberMergeFieldSegmentOperator::Notcontain => "notcontain",
            TextNumberMergeFieldSegmentOperator::Starts => "starts",
            TextNumberMergeFieldSegmentOperator::Noop => "",
            TextNumberMergeFieldSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TextNumberMergeFieldSegmentOperator {
    fn default() -> TextNumberMergeFieldSegmentOperator {
        TextNumberMergeFieldSegmentOperator::Noop
    }
}
impl TextNumberMergeFieldSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, TextNumberMergeFieldSegmentOperator::Noop)
    }
}

/// Segment by a given text or number merge field.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TextNumberMergeFieldSegment {
    /**
    * Segment by a given text or number merge field.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<TextNumberMergeFieldSegmentConditionType>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    /**
    * Whether the member's merge information is/is not, contains/does not contain, starts/ends with, or is greater/less than a value
    */
    #[serde(
        default,
        skip_serializing_if = "TextNumberMergeFieldSegmentOperator::is_noop"
    )]
    pub op: TextNumberMergeFieldSegmentOperator,
    /**
    * Segment by a given text or number merge field.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailSegmentConditionType {
    #[serde(rename = "EmailAddress")]
    EmailAddress,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailSegmentConditionType::EmailAddress => "EmailAddress",
            EmailSegmentConditionType::Noop => "",
            EmailSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailSegmentConditionType {
    fn default() -> EmailSegmentConditionType {
        EmailSegmentConditionType::Noop
    }
}
impl EmailSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailSegmentConditionType::Noop)
    }
}

/**
* Segmenting based off of a subscriber's email address.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailSegmentField {
    #[serde(rename = "EMAIL")]
    Email,
    #[serde(rename = "merge0")]
    Merge0,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailSegmentField::Email => "EMAIL",
            EmailSegmentField::Merge0 => "merge0",
            EmailSegmentField::Noop => "",
            EmailSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailSegmentField {
    fn default() -> EmailSegmentField {
        EmailSegmentField::Noop
    }
}
impl EmailSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailSegmentField::Noop)
    }
}

/**
* Whether the email address is/not exactly, contains/doesn't contain, starts/ends with a string.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailSegmentOperator {
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "ends")]
    Ends,
    #[serde(rename = "greater")]
    Greater,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "notcontain")]
    Notcontain,
    #[serde(rename = "starts")]
    Starts,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailSegmentOperator::Contains => "contains",
            EmailSegmentOperator::Ends => "ends",
            EmailSegmentOperator::Greater => "greater",
            EmailSegmentOperator::Is => "is",
            EmailSegmentOperator::Less => "less",
            EmailSegmentOperator::Not => "not",
            EmailSegmentOperator::Notcontain => "notcontain",
            EmailSegmentOperator::Starts => "starts",
            EmailSegmentOperator::Noop => "",
            EmailSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailSegmentOperator {
    fn default() -> EmailSegmentOperator {
        EmailSegmentOperator::Noop
    }
}
impl EmailSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailSegmentOperator::Noop)
    }
}

/// Segment by email address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailSegment {
    /**
    * Segment by email address.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<EmailSegmentConditionType>,
    /**
    * Segmenting based off of a subscriber's email address.
    */
    #[serde(default, skip_serializing_if = "EmailSegmentField::is_noop")]
    pub field: EmailSegmentField,
    /**
    * Whether the email address is/not exactly, contains/doesn't contain, starts/ends with a string.
    */
    #[serde(default, skip_serializing_if = "EmailSegmentOperator::is_noop")]
    pub op: EmailSegmentOperator,
    /**
    * Segment by email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PredictedGenderSegmentConditionType {
    #[serde(rename = "PredictedGender")]
    PredictedGender,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PredictedGenderSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PredictedGenderSegmentConditionType::PredictedGender => "PredictedGender",
            PredictedGenderSegmentConditionType::Noop => "",
            PredictedGenderSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PredictedGenderSegmentConditionType {
    fn default() -> PredictedGenderSegmentConditionType {
        PredictedGenderSegmentConditionType::Noop
    }
}
impl PredictedGenderSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PredictedGenderSegmentConditionType::Noop)
    }
}

/**
* Segment by predicted gender.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PredictedGenderSegmentField {
    #[serde(rename = "predicted_gender")]
    PredictedGender,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PredictedGenderSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PredictedGenderSegmentField::PredictedGender => "predicted_gender",
            PredictedGenderSegmentField::Noop => "",
            PredictedGenderSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PredictedGenderSegmentField {
    fn default() -> PredictedGenderSegmentField {
        PredictedGenderSegmentField::Noop
    }
}
impl PredictedGenderSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, PredictedGenderSegmentField::Noop)
    }
}

/// Segment by predicted gender.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PredictedGenderSegment {
    /**
    * Segment by predicted gender.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<PredictedGenderSegmentConditionType>,
    /**
    * Segment by predicted gender.
    */
    #[serde(default, skip_serializing_if = "PredictedGenderSegmentField::is_noop")]
    pub field: PredictedGenderSegmentField,
    /**
    * Whether the member's language is or is not set to a specific language.
    */
    #[serde(default, skip_serializing_if = "LanguageSegmentOperator::is_noop")]
    pub op: LanguageSegmentOperator,
    /**
    * The Social Profiles gender to segment.
    */
    #[serde(
        default,
        skip_serializing_if = "SocialProfilesGenderSegmentOperator::is_noop"
    )]
    pub value: SocialProfilesGenderSegmentOperator,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PredictedAgeSegmentConditionType {
    #[serde(rename = "PredictedAge")]
    PredictedAge,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PredictedAgeSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PredictedAgeSegmentConditionType::PredictedAge => "PredictedAge",
            PredictedAgeSegmentConditionType::Noop => "",
            PredictedAgeSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PredictedAgeSegmentConditionType {
    fn default() -> PredictedAgeSegmentConditionType {
        PredictedAgeSegmentConditionType::Noop
    }
}
impl PredictedAgeSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PredictedAgeSegmentConditionType::Noop)
    }
}

/**
* Segment by predicted age.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PredictedAgeSegmentField {
    #[serde(rename = "predicted_age_range")]
    PredictedAgeRange,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PredictedAgeSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PredictedAgeSegmentField::PredictedAgeRange => "predicted_age_range",
            PredictedAgeSegmentField::Noop => "",
            PredictedAgeSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PredictedAgeSegmentField {
    fn default() -> PredictedAgeSegmentField {
        PredictedAgeSegmentField::Noop
    }
}
impl PredictedAgeSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, PredictedAgeSegmentField::Noop)
    }
}

/**
* Members who are/not the exact criteria listed.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PredictedAgeSegmentOperator {
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PredictedAgeSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PredictedAgeSegmentOperator::Is => "is",
            PredictedAgeSegmentOperator::Noop => "",
            PredictedAgeSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PredictedAgeSegmentOperator {
    fn default() -> PredictedAgeSegmentOperator {
        PredictedAgeSegmentOperator::Noop
    }
}
impl PredictedAgeSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, PredictedAgeSegmentOperator::Noop)
    }
}

/**
* The predicted age to segment.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PredictedAgeSegmentOperatorData {
    #[serde(rename = "18-24")]
    OneThousandEightHundredAndTwentyFour,
    #[serde(rename = "25-34")]
    TwoThousandFiveHundredAndThirtyFour,
    #[serde(rename = "35-44")]
    ThreeThousandFiveHundredAndFortyFour,
    #[serde(rename = "45-54")]
    FourThousandFiveHundredAndFiftyFour,
    #[serde(rename = "55-64")]
    FiveThousandFiveHundredAndSixtyFour,
    #[serde(rename = "65+")]
    SixtyFive,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PredictedAgeSegmentOperatorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PredictedAgeSegmentOperatorData::OneThousandEightHundredAndTwentyFour => "18-24",
            PredictedAgeSegmentOperatorData::TwoThousandFiveHundredAndThirtyFour => "25-34",
            PredictedAgeSegmentOperatorData::ThreeThousandFiveHundredAndFortyFour => "35-44",
            PredictedAgeSegmentOperatorData::FourThousandFiveHundredAndFiftyFour => "45-54",
            PredictedAgeSegmentOperatorData::FiveThousandFiveHundredAndSixtyFour => "55-64",
            PredictedAgeSegmentOperatorData::SixtyFive => "65+",
            PredictedAgeSegmentOperatorData::Noop => "",
            PredictedAgeSegmentOperatorData::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PredictedAgeSegmentOperatorData {
    fn default() -> PredictedAgeSegmentOperatorData {
        PredictedAgeSegmentOperatorData::Noop
    }
}
impl PredictedAgeSegmentOperatorData {
    pub fn is_noop(&self) -> bool {
        matches!(self, PredictedAgeSegmentOperatorData::Noop)
    }
}

/// Segment by predicted age.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PredictedAgeSegment {
    /**
    * Segment by predicted age.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<PredictedAgeSegmentConditionType>,
    /**
    * Segment by predicted age.
    */
    #[serde(default, skip_serializing_if = "PredictedAgeSegmentField::is_noop")]
    pub field: PredictedAgeSegmentField,
    /**
    * Members who are/not the exact criteria listed.
    */
    #[serde(default, skip_serializing_if = "PredictedAgeSegmentOperator::is_noop")]
    pub op: PredictedAgeSegmentOperator,
    /**
    * The predicted age to segment.
    */
    #[serde(
        default,
        skip_serializing_if = "PredictedAgeSegmentOperatorData::is_noop"
    )]
    pub value: PredictedAgeSegmentOperatorData,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NewSubscribersPrebuiltSegmentConditionType {
    #[serde(rename = "NewSubscribers")]
    NewSubscribers,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NewSubscribersPrebuiltSegmentConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NewSubscribersPrebuiltSegmentConditionType::NewSubscribers => "NewSubscribers",
            NewSubscribersPrebuiltSegmentConditionType::Noop => "",
            NewSubscribersPrebuiltSegmentConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NewSubscribersPrebuiltSegmentConditionType {
    fn default() -> NewSubscribersPrebuiltSegmentConditionType {
        NewSubscribersPrebuiltSegmentConditionType::Noop
    }
}
impl NewSubscribersPrebuiltSegmentConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, NewSubscribersPrebuiltSegmentConditionType::Noop)
    }
}

/**
* Segment by when people subscribed.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NewSubscribersPrebuiltSegmentField {
    #[serde(rename = "timestamp_opt")]
    TimestampOpt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NewSubscribersPrebuiltSegmentField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NewSubscribersPrebuiltSegmentField::TimestampOpt => "timestamp_opt",
            NewSubscribersPrebuiltSegmentField::Noop => "",
            NewSubscribersPrebuiltSegmentField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NewSubscribersPrebuiltSegmentField {
    fn default() -> NewSubscribersPrebuiltSegmentField {
        NewSubscribersPrebuiltSegmentField::Noop
    }
}
impl NewSubscribersPrebuiltSegmentField {
    pub fn is_noop(&self) -> bool {
        matches!(self, NewSubscribersPrebuiltSegmentField::Noop)
    }
}

/**
* Whe the event took place, namely within a time frame.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NewSubscribersPrebuiltSegmentOperator {
    #[serde(rename = "date_within")]
    DateWithin,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NewSubscribersPrebuiltSegmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NewSubscribersPrebuiltSegmentOperator::DateWithin => "date_within",
            NewSubscribersPrebuiltSegmentOperator::Noop => "",
            NewSubscribersPrebuiltSegmentOperator::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NewSubscribersPrebuiltSegmentOperator {
    fn default() -> NewSubscribersPrebuiltSegmentOperator {
        NewSubscribersPrebuiltSegmentOperator::Noop
    }
}
impl NewSubscribersPrebuiltSegmentOperator {
    pub fn is_noop(&self) -> bool {
        matches!(self, NewSubscribersPrebuiltSegmentOperator::Noop)
    }
}

/// Segment by when people subscribed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NewSubscribersPrebuiltSegment {
    /**
    * Segment by when people subscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<NewSubscribersPrebuiltSegmentConditionType>,
    /**
    * Segment by when people subscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<NewSubscribersPrebuiltSegmentField>,
    /**
    * Segment by when people subscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<NewSubscribersPrebuiltSegmentOperator>,
    /**
    * Segment by when people subscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// All of the following types:
///
/// - `Conditions`
/// - `AutomationSegment`
/// - `PollActivitySegment`
/// - `ConversationSegment`
/// - `DateSegment`
/// - `EmailClientSegment`
/// - `LanguageSegment`
/// - `MemberRatingSegment`
/// - `SignupSourceSegment`
/// - `SurveyMonkeySegment`
/// - `VipSegment`
/// - `InterestsSegment`
/// - `EcommerceCategorySegment`
/// - `EcommerceNumberSegment`
/// - `EcommercePurchasedSegment`
/// - `EcommerceSpentSegment`
/// - `EcommercePurchasedStoreSegment`
/// - `GoalActivitySegment`
/// - `GoalTimestampSegment`
/// - `SimilarSubscribersSegmentMember`
/// - `StaticSegmentMember`
/// - `LocationBasedSegment`
/// - `GeolocationSegment`
/// - `UsZipCodeSegment`
/// - `UnknownLocationBasedSegment`
/// - `ZipCodeLocationBasedSegment`
/// - `SocialProfilesAgeSegment`
/// - `SocialProfilesGenderSegment`
/// - `SocialProfilesInfluenceSegment`
/// - `SocialProfilesNetworkSegment`
/// - `SocialProfilesNetworkFollowSegment`
/// - `AddressMergeFieldSegment`
/// - `AddressZipMergeFieldSegment`
/// - `BirthdayMergeFieldSegment`
/// - `DateMergeFieldSegment`
/// - `DropdownRadioMergeFieldSegment`
/// - `TextNumberMergeFieldSegment`
/// - `EmailSegment`
/// - `PredictedGenderSegment`
/// - `PredictedAgeSegment`
/// - `NewSubscribersPrebuiltSegment`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ConditionsOneOf {
    /**
    * Segment by interaction with a specific campaign.
    */
    Conditions(Conditions),
    /**
    * Segment by interaction with an Automation workflow.
    */
    AutomationSegment(AutomationSegment),
    /**
    * Segment by poll activity.
    */
    PollActivitySegment(PollActivitySegment),
    /**
    * Segment by interaction with a campaign via Conversations.
    */
    ConversationSegment(ConversationSegment),
    /**
    * Segment by a specific date field.
    */
    DateSegment(DateSegment),
    /**
    * Segment by use of a particular email client.
    */
    EmailClientSegment(EmailClientSegment),
    /**
    * Segment by language.
    */
    LanguageSegment(LanguageSegment),
    /**
    * Segment by member rating.
    */
    MemberRatingSegment(MemberRatingSegment),
    /**
    * Segment by signup source.
    */
    SignupSourceSegment(SignupSourceSegment),
    /**
    * Segment by interaction with a SurveyMonkey survey.
    */
    SurveyMonkeySegment(SurveyMonkeySegment),
    /**
    * Segment by VIP status.
    */
    VipSegment(VipSegment),
    /**
    * Segment by an interest group merge field.
    */
    InterestsSegment(InterestsSegment),
    /**
    * Segment by purchases in specific items or categories.
    */
    EcommerceCategorySegment(EcommerceCategorySegment),
    /**
    * Segment by average spent total, number of orders, total number of products purchased, or average number of products per order.
    */
    EcommerceNumberSegment(EcommerceNumberSegment),
    /**
    * Segment by whether someone has purchased anything.
    */
    EcommercePurchasedSegment(EcommercePurchasedSegment),
    /**
    * Segment by amount spent on a single order or across all orders.
    */
    EcommerceSpentSegment(EcommerceSpentSegment),
    /**
    * Segment by purchases from a specific store.
    */
    EcommercePurchasedStoreSegment(EcommercePurchasedStoreSegment),
    /**
    * Segment by Goal activity.
    */
    GoalActivitySegment(GoalActivitySegment),
    /**
    * Segment by most recent interaction with a website.
    */
    GoalTimestampSegment(GoalTimestampSegment),
    /**
    * Segment by similar subscribers.
    */
    SimilarSubscribersSegmentMember(SimilarSubscribersSegmentMember),
    /**
    * Segment by a given static segment.
    */
    StaticSegmentMember(StaticSegmentMember),
    /**
    * Segment by a specific country or US state.
    */
    LocationBasedSegment(LocationBasedSegment),
    /**
    * Segment by a specific geographic region.
    */
    GeolocationSegment(GeolocationSegment),
    /**
    * Segment by a specific US ZIP code.
    */
    UsZipCodeSegment(UsZipCodeSegment),
    /**
    * Segment members whose location information is unknown.
    */
    UnknownLocationBasedSegment(UnknownLocationBasedSegment),
    /**
    * Segment by a specific US ZIP code.
    */
    ZipCodeLocationBasedSegment(ZipCodeLocationBasedSegment),
    /**
    * Segment by age ranges in Social Profiles data.
    */
    SocialProfilesAgeSegment(SocialProfilesAgeSegment),
    /**
    * Segment by listed gender in Social Profiles data.
    */
    SocialProfilesGenderSegment(SocialProfilesGenderSegment),
    /**
    * Segment by influence rating in Social Profiles data.
    */
    SocialProfilesInfluenceSegment(SocialProfilesInfluenceSegment),
    /**
    * Segment by social network in Social Profiles data.
    */
    SocialProfilesNetworkSegment(SocialProfilesNetworkSegment),
    /**
    * Segment by social network in Social Profiles data.
    */
    SocialProfilesNetworkFollowSegment(SocialProfilesNetworkFollowSegment),
    /**
    * Segment by an address-type merge field.
    */
    AddressMergeFieldSegment(AddressMergeFieldSegment),
    /**
    * Segment by an address-type merge field within a given distance.
    */
    AddressZipMergeFieldSegment(AddressZipMergeFieldSegment),
    /**
    * Segment by a contact's birthday.
    */
    BirthdayMergeFieldSegment(BirthdayMergeFieldSegment),
    /**
    * Segment by a given date merge field.
    */
    DateMergeFieldSegment(DateMergeFieldSegment),
    /**
    * An individual segment condition
    */
    DropdownRadioMergeFieldSegment(DropdownRadioMergeFieldSegment),
    /**
    * Segment by a given text or number merge field.
    */
    TextNumberMergeFieldSegment(TextNumberMergeFieldSegment),
    /**
    * Segment by email address.
    */
    EmailSegment(EmailSegment),
    /**
    * Segment by predicted gender.
    */
    PredictedGenderSegment(PredictedGenderSegment),
    /**
    * Segment by predicted age.
    */
    PredictedAgeSegment(PredictedAgeSegment),
    /**
    * Segment by when people subscribed.
    */
    NewSubscribersPrebuiltSegment(NewSubscribersPrebuiltSegment),
}

impl ConditionsOneOf {
    pub fn address_merge_field_segment(&self) -> Option<&AddressMergeFieldSegment> {
        if let ConditionsOneOf::AddressMergeFieldSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn address_zip_merge_field_segment(&self) -> Option<&AddressZipMergeFieldSegment> {
        if let ConditionsOneOf::AddressZipMergeFieldSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn automation_segment(&self) -> Option<&AutomationSegment> {
        if let ConditionsOneOf::AutomationSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn birthday_merge_field_segment(&self) -> Option<&BirthdayMergeFieldSegment> {
        if let ConditionsOneOf::BirthdayMergeFieldSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn conditions(&self) -> Option<&Conditions> {
        if let ConditionsOneOf::Conditions(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn conversation_segment(&self) -> Option<&ConversationSegment> {
        if let ConditionsOneOf::ConversationSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn date_merge_field_segment(&self) -> Option<&DateMergeFieldSegment> {
        if let ConditionsOneOf::DateMergeFieldSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn date_segment(&self) -> Option<&DateSegment> {
        if let ConditionsOneOf::DateSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn dropdown_radio_merge_field_segment(&self) -> Option<&DropdownRadioMergeFieldSegment> {
        if let ConditionsOneOf::DropdownRadioMergeFieldSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn ecommerce_category_segment(&self) -> Option<&EcommerceCategorySegment> {
        if let ConditionsOneOf::EcommerceCategorySegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn ecommerce_number_segment(&self) -> Option<&EcommerceNumberSegment> {
        if let ConditionsOneOf::EcommerceNumberSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn ecommerce_purchased_segment(&self) -> Option<&EcommercePurchasedSegment> {
        if let ConditionsOneOf::EcommercePurchasedSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn ecommerce_purchased_store_segment(&self) -> Option<&EcommercePurchasedStoreSegment> {
        if let ConditionsOneOf::EcommercePurchasedStoreSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn ecommerce_spent_segment(&self) -> Option<&EcommerceSpentSegment> {
        if let ConditionsOneOf::EcommerceSpentSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn email_client_segment(&self) -> Option<&EmailClientSegment> {
        if let ConditionsOneOf::EmailClientSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn email_segment(&self) -> Option<&EmailSegment> {
        if let ConditionsOneOf::EmailSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn geolocation_segment(&self) -> Option<&GeolocationSegment> {
        if let ConditionsOneOf::GeolocationSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn goal_activity_segment(&self) -> Option<&GoalActivitySegment> {
        if let ConditionsOneOf::GoalActivitySegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn goal_timestamp_segment(&self) -> Option<&GoalTimestampSegment> {
        if let ConditionsOneOf::GoalTimestampSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn interests_segment(&self) -> Option<&InterestsSegment> {
        if let ConditionsOneOf::InterestsSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn language_segment(&self) -> Option<&LanguageSegment> {
        if let ConditionsOneOf::LanguageSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn location_based_segment(&self) -> Option<&LocationBasedSegment> {
        if let ConditionsOneOf::LocationBasedSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn member_rating_segment(&self) -> Option<&MemberRatingSegment> {
        if let ConditionsOneOf::MemberRatingSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn new_subscribers_prebuilt_segment(&self) -> Option<&NewSubscribersPrebuiltSegment> {
        if let ConditionsOneOf::NewSubscribersPrebuiltSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn poll_activity_segment(&self) -> Option<&PollActivitySegment> {
        if let ConditionsOneOf::PollActivitySegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn predicted_age_segment(&self) -> Option<&PredictedAgeSegment> {
        if let ConditionsOneOf::PredictedAgeSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn predicted_gender_segment(&self) -> Option<&PredictedGenderSegment> {
        if let ConditionsOneOf::PredictedGenderSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn signup_source_segment(&self) -> Option<&SignupSourceSegment> {
        if let ConditionsOneOf::SignupSourceSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn similar_subscribers_segment_member(&self) -> Option<&SimilarSubscribersSegmentMember> {
        if let ConditionsOneOf::SimilarSubscribersSegmentMember(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn social_profiles_age_segment(&self) -> Option<&SocialProfilesAgeSegment> {
        if let ConditionsOneOf::SocialProfilesAgeSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn social_profiles_gender_segment(&self) -> Option<&SocialProfilesGenderSegment> {
        if let ConditionsOneOf::SocialProfilesGenderSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn social_profiles_influence_segment(&self) -> Option<&SocialProfilesInfluenceSegment> {
        if let ConditionsOneOf::SocialProfilesInfluenceSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn social_profiles_network_follow_segment(
        &self,
    ) -> Option<&SocialProfilesNetworkFollowSegment> {
        if let ConditionsOneOf::SocialProfilesNetworkFollowSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn social_profiles_network_segment(&self) -> Option<&SocialProfilesNetworkSegment> {
        if let ConditionsOneOf::SocialProfilesNetworkSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn static_segment_member(&self) -> Option<&StaticSegmentMember> {
        if let ConditionsOneOf::StaticSegmentMember(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn survey_monkey_segment(&self) -> Option<&SurveyMonkeySegment> {
        if let ConditionsOneOf::SurveyMonkeySegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn text_number_merge_field_segment(&self) -> Option<&TextNumberMergeFieldSegment> {
        if let ConditionsOneOf::TextNumberMergeFieldSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn unknown_location_based_segment(&self) -> Option<&UnknownLocationBasedSegment> {
        if let ConditionsOneOf::UnknownLocationBasedSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn us_zip_code_segment(&self) -> Option<&UsZipCodeSegment> {
        if let ConditionsOneOf::UsZipCodeSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn vip_segment(&self) -> Option<&VipSegment> {
        if let ConditionsOneOf::VipSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn zip_code_location_based_segment(&self) -> Option<&ZipCodeLocationBasedSegment> {
        if let ConditionsOneOf::ZipCodeLocationBasedSegment(ref_) = self {
            return Some(ref_);
        }
        None
    }
}

/// An object representing all segmentation options.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentOpts {
    /**
    * An object representing all segmentation options.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conditions: Vec<ConditionsOneOf>,
    /**
    * An object representing all segmentation options.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub match_: Option<Match>,
    /**
    * An object representing all segmentation options.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub saved_segment_id: i64,
}

/// List settings for the Automation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct List {
    /**
    * List settings for the Automation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * List settings for the Automation.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * List settings for the Automation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_name: String,
    /**
    * List settings for the Automation.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<SegmentOpts>,
    /**
    * List settings for the Automation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
}

/// The settings for the Automation workflow.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Settings {
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub authenticate: bool,
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_footer: bool,
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub inline_css: bool,
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to: String,
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to_name: String,
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_conversation: bool,
}

/// Deprecated
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Salesforce {
    /**
    * Deprecated
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub campaign: bool,
    /**
    * Deprecated
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub notes: bool,
}

/// Deprecated
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Capsule {
    /**
    * Deprecated
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub notes: bool,
}

/// The tracking options for the Automation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Tracking {
    /**
    * The tracking options for the Automation.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capsule: Option<Capsule>,
    /**
    * The tracking options for the Automation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clicktale: String,
    /**
    * The tracking options for the Automation.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "ecomm360"
    )]
    pub ecomm_360: bool,
    /**
    * The tracking options for the Automation.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub goal_tracking: bool,
    /**
    * The tracking options for the Automation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub google_analytics: String,
    /**
    * The tracking options for the Automation.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub html_clicks: bool,
    /**
    * The tracking options for the Automation.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub opens: bool,
    /**
    * The tracking options for the Automation.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<Salesforce>,
    /**
    * The tracking options for the Automation.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub text_clicks: bool,
}

/**
* The type of Automation workflow.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum WorkflowType {
    #[serde(rename = "abandonedBrowse")]
    AbandonedBrowse,
    #[serde(rename = "abandonedCart")]
    AbandonedCart,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "bestCustomers")]
    BestCustomers,
    #[serde(rename = "categoryFollowup")]
    CategoryFollowup,
    #[serde(rename = "dateAdded")]
    DateAdded,
    #[serde(rename = "emailFollowup")]
    EmailFollowup,
    #[serde(rename = "emailSeries")]
    EmailSeries,
    #[serde(rename = "groupAdd")]
    GroupAdd,
    #[serde(rename = "groupRemove")]
    GroupRemove,
    #[serde(rename = "mandrill")]
    Mandrill,
    #[serde(rename = "productFollowup")]
    ProductFollowup,
    #[serde(rename = "purchaseFollowup")]
    PurchaseFollowup,
    #[serde(rename = "recurringEvent")]
    RecurringEvent,
    #[serde(rename = "specialEvent")]
    SpecialEvent,
    #[serde(rename = "visitUrl")]
    VisitUrl,
    #[serde(rename = "welcomeSeries")]
    WelcomeSeries,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            WorkflowType::AbandonedBrowse => "abandonedBrowse",
            WorkflowType::AbandonedCart => "abandonedCart",
            WorkflowType::Api => "api",
            WorkflowType::BestCustomers => "bestCustomers",
            WorkflowType::CategoryFollowup => "categoryFollowup",
            WorkflowType::DateAdded => "dateAdded",
            WorkflowType::EmailFollowup => "emailFollowup",
            WorkflowType::EmailSeries => "emailSeries",
            WorkflowType::GroupAdd => "groupAdd",
            WorkflowType::GroupRemove => "groupRemove",
            WorkflowType::Mandrill => "mandrill",
            WorkflowType::ProductFollowup => "productFollowup",
            WorkflowType::PurchaseFollowup => "purchaseFollowup",
            WorkflowType::RecurringEvent => "recurringEvent",
            WorkflowType::SpecialEvent => "specialEvent",
            WorkflowType::VisitUrl => "visitUrl",
            WorkflowType::WelcomeSeries => "welcomeSeries",
            WorkflowType::Noop => "",
            WorkflowType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for WorkflowType {
    fn default() -> WorkflowType {
        WorkflowType::Noop
    }
}
impl WorkflowType {
    pub fn is_noop(&self) -> bool {
        matches!(self, WorkflowType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Days {
    #[serde(rename = "friday")]
    Friday,
    #[serde(rename = "monday")]
    Monday,
    #[serde(rename = "saturday")]
    Saturday,
    #[serde(rename = "sunday")]
    Sunday,
    #[serde(rename = "thursday")]
    Thursday,
    #[serde(rename = "tuesday")]
    Tuesday,
    #[serde(rename = "wednesday")]
    Wednesday,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Days {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Days::Friday => "friday",
            Days::Monday => "monday",
            Days::Saturday => "saturday",
            Days::Sunday => "sunday",
            Days::Thursday => "thursday",
            Days::Tuesday => "tuesday",
            Days::Wednesday => "wednesday",
            Days::Noop => "",
            Days::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Days {
    fn default() -> Days {
        Days::Noop
    }
}
impl Days {
    pub fn is_noop(&self) -> bool {
        matches!(self, Days::Noop)
    }
}

/**
* When to send the Automation email.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailSendTimeSettings {
    #[serde(rename = "send_asap")]
    SendAsap,
    #[serde(rename = "send_at")]
    SendAt,
    #[serde(rename = "send_between")]
    SendBetween,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailSendTimeSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailSendTimeSettings::SendAsap => "send_asap",
            EmailSendTimeSettings::SendAt => "send_at",
            EmailSendTimeSettings::SendBetween => "send_between",
            EmailSendTimeSettings::Noop => "",
            EmailSendTimeSettings::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailSendTimeSettings {
    fn default() -> EmailSendTimeSettings {
        EmailSendTimeSettings::Noop
    }
}
impl EmailSendTimeSettings {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailSendTimeSettings::Noop)
    }
}

/// The hours an Automation workflow can send.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Hours {
    /**
    * When to send the Automation email.
    */
    #[serde(
        default,
        skip_serializing_if = "EmailSendTimeSettings::is_noop",
        rename = "type"
    )]
    pub type_: EmailSendTimeSettings,
}

/// A workflow's runtime settings for an Automation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Runtime {
    /**
    * A workflow's runtime settings for an Automation.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub days: Vec<Days>,
    /**
    * A workflow's runtime settings for an Automation.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<Hours>,
}

/// Available triggers for Automation workflows.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TriggerSettings {
    /**
    * Available triggers for Automation workflows.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime: Option<Runtime>,
    /**
    * Available triggers for Automation workflows.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub workflow_emails_count: i64,
    /**
    * Available triggers for Automation workflows.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_title: String,
    /**
    * The type of Automation workflow.
    */
    #[serde(default, skip_serializing_if = "WorkflowType::is_noop")]
    pub workflow_type: WorkflowType,
}

/// A summary of opens and clicks for sent campaigns.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReportSummary {
    /**
    * A summary of opens and clicks for sent campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * A summary of opens and clicks for sent campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * A summary of opens and clicks for sent campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
    /**
    * A summary of opens and clicks for sent campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * A summary of opens and clicks for sent campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscriber_clicks: i64,
    /**
    * A summary of opens and clicks for sent campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
}

/// A summary of an individual Automation workflow's settings and content.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Automations {
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_sent: i64,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<List>,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<ReportSummary>,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<Tracking>,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_settings: Option<TriggerSettings>,
}

/// An array of objects, each representing an Automation workflow.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAutomationsResponse {
    /**
    * An array of objects, each representing an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An array of objects, each representing an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub automations: Vec<Automations>,
    /**
    * An array of objects, each representing an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// List settings for the Automation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Recipients {
    /**
    * List settings for the Automation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * List settings for the Automation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
}

/// The settings for the Automation workflow.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationCampaignSettings {
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
    * The settings for the Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to: String,
}

/// Trigger settings for the Automation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationTrigger {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_type: String,
}

/// A summary of an individual Automation workflow's settings and content.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationWorkflow {
    /**
    * List settings for the Automation.
    */
    #[serde()]
    pub recipients: Recipients,
    /**
    * A summary of an individual Automation workflow's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AutomationCampaignSettings>,
    /**
    * Trigger settings for the Automation.
    */
    #[serde()]
    pub trigger_settings: AutomationTrigger,
}

/**
* The type of delay for an Automation email.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DelayType {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "now")]
    Now,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DelayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DelayType::Day => "day",
            DelayType::Hour => "hour",
            DelayType::Now => "now",
            DelayType::Week => "week",
            DelayType::Noop => "",
            DelayType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DelayType {
    fn default() -> DelayType {
        DelayType::Noop
    }
}
impl DelayType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DelayType::Noop)
    }
}

/**
* Whether the delay settings describe before or after the delay action of an Automation email.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Direction {
    #[serde(rename = "after")]
    After,
    #[serde(rename = "before")]
    Before,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Direction::After => "after",
            Direction::Before => "before",
            Direction::Noop => "",
            Direction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Noop
    }
}
impl Direction {
    pub fn is_noop(&self) -> bool {
        matches!(self, Direction::Noop)
    }
}

/**
* The action that triggers the delay of an Automation email.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Action {
    #[serde(rename = "annual")]
    Annual,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "birthday")]
    Birthday,
    #[serde(rename = "campaign_sent")]
    CampaignSent,
    #[serde(rename = "campaign_specific_clicked")]
    CampaignSpecificClicked,
    #[serde(rename = "clicked_email")]
    ClickedEmail,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date_added")]
    DateAdded,
    #[serde(rename = "ecomm_abandoned_cart")]
    EcommAbandonedCart,
    #[serde(rename = "ecomm_bought_any")]
    EcommBoughtAny,
    #[serde(rename = "ecomm_bought_category")]
    EcommBoughtCategory,
    #[serde(rename = "ecomm_bought_product")]
    EcommBoughtProduct,
    #[serde(rename = "ecomm_not_bought_any")]
    EcommNotBoughtAny,
    #[serde(rename = "goal")]
    Goal,
    #[serde(rename = "group_add")]
    GroupAdd,
    #[serde(rename = "group_remove")]
    GroupRemove,
    #[serde(rename = "mandrill_any")]
    MandrillAny,
    #[serde(rename = "mandrill_clicked")]
    MandrillClicked,
    #[serde(rename = "mandrill_opened")]
    MandrillOpened,
    #[serde(rename = "mandrill_sent")]
    MandrillSent,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "merge_changed")]
    MergeChanged,
    #[serde(rename = "not_clicked_email")]
    NotClickedEmail,
    #[serde(rename = "not_opened_email")]
    NotOpenedEmail,
    #[serde(rename = "opened_email")]
    OpenedEmail,
    #[serde(rename = "previous_campaign_clicked_any")]
    PreviousCampaignClickedAny,
    #[serde(rename = "previous_campaign_not_clicked_any")]
    PreviousCampaignNotClickedAny,
    #[serde(rename = "previous_campaign_not_opened")]
    PreviousCampaignNotOpened,
    #[serde(rename = "previous_campaign_opened")]
    PreviousCampaignOpened,
    #[serde(rename = "previous_campaign_sent")]
    PreviousCampaignSent,
    #[serde(rename = "previous_campaign_specific_clicked")]
    PreviousCampaignSpecificClicked,
    #[serde(rename = "signup")]
    Signup,
    #[serde(rename = "tag_add")]
    TagAdd,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Action::Annual => "annual",
            Action::Api => "api",
            Action::Birthday => "birthday",
            Action::CampaignSent => "campaign_sent",
            Action::CampaignSpecificClicked => "campaign_specific_clicked",
            Action::ClickedEmail => "clicked_email",
            Action::Date => "date",
            Action::DateAdded => "date_added",
            Action::EcommAbandonedCart => "ecomm_abandoned_cart",
            Action::EcommBoughtAny => "ecomm_bought_any",
            Action::EcommBoughtCategory => "ecomm_bought_category",
            Action::EcommBoughtProduct => "ecomm_bought_product",
            Action::EcommNotBoughtAny => "ecomm_not_bought_any",
            Action::Goal => "goal",
            Action::GroupAdd => "group_add",
            Action::GroupRemove => "group_remove",
            Action::MandrillAny => "mandrill_any",
            Action::MandrillClicked => "mandrill_clicked",
            Action::MandrillOpened => "mandrill_opened",
            Action::MandrillSent => "mandrill_sent",
            Action::Manual => "manual",
            Action::MergeChanged => "merge_changed",
            Action::NotClickedEmail => "not_clicked_email",
            Action::NotOpenedEmail => "not_opened_email",
            Action::OpenedEmail => "opened_email",
            Action::PreviousCampaignClickedAny => "previous_campaign_clicked_any",
            Action::PreviousCampaignNotClickedAny => "previous_campaign_not_clicked_any",
            Action::PreviousCampaignNotOpened => "previous_campaign_not_opened",
            Action::PreviousCampaignOpened => "previous_campaign_opened",
            Action::PreviousCampaignSent => "previous_campaign_sent",
            Action::PreviousCampaignSpecificClicked => "previous_campaign_specific_clicked",
            Action::Signup => "signup",
            Action::TagAdd => "tag_add",
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

/// The delay settings for an Automation email.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Delay {
    /**
    * The delay settings for an Automation email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /**
    * The delay settings for an Automation email.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action_description: String,
    /**
    * The delay settings for an Automation email.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub amount: i64,
    /**
    * The delay settings for an Automation email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /**
    * The delay settings for an Automation email.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_description: String,
    /**
    * The delay settings for an Automation email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<DelayType>,
}

/// An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentOptions {
    /**
    * An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conditions: Vec<ConditionsOneOf>,
    /**
    * An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub match_: Option<Match>,
    /**
    * An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub prebuilt_segment_id: String,
    /**
    * An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub saved_segment_id: i64,
}

/// List settings for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationEmailsList {
    /**
    * List settings for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * List settings for the campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * List settings for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_name: String,
    /**
    * List settings for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipient_count: i64,
    /**
    * List settings for the campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<SegmentOptions>,
    /**
    * List settings for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub segment_text: String,
}

/// Settings for the campaign including the email subject, from name, and from email address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignSettings {
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub authenticate: bool,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub auto_fb_post: Vec<String>,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_footer: bool,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_tweet: bool,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub drag_and_drop: bool,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fb_comments: bool,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub inline_css: bool,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub preview_text: String,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to: String,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject_line: String,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub template_id: i64,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// The tracking options for a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignTrackingOptions {
    /**
    * The tracking options for a campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capsule: Option<Capsule>,
    /**
    * The tracking options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clicktale: String,
    /**
    * The tracking options for a campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "ecomm360"
    )]
    pub ecomm_360: bool,
    /**
    * The tracking options for a campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub goal_tracking: bool,
    /**
    * The tracking options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub google_analytics: String,
    /**
    * The tracking options for a campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub html_clicks: bool,
    /**
    * The tracking options for a campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub opens: bool,
    /**
    * The tracking options for a campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<Salesforce>,
    /**
    * The tracking options for a campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub text_clicks: bool,
}

/// The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SocialCard {
    /**
    * The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// For sent campaigns, a summary of opens and clicks.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignReportSummary {
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscriber_clicks: i64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
}

/// A summary of an individual Automation workflow email.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Emails {
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<Delay>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_sent: i64,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_logo_merge_tag: bool,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub needs_block_refresh: bool,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub position: i64,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<AutomationEmailsList>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<CampaignReportSummary>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<CampaignSettings>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_card: Option<SocialCard>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CampaignTrackingOptions>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_settings: Option<TriggerSettings>,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub web_id: i64,
    /**
    * A summary of an individual Automation workflow email.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_id: String,
}

/// A summary of the emails in an Automation workflow.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationEmails {
    /**
    * A summary of the emails in an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of the emails in an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub emails: Vec<Emails>,
    /**
    * A summary of the emails in an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Settings for the campaign including the email subject, from name, and from email address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateInformationAboutASpecificWorkflowEmailCampaignSettings {
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub preview_text: String,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to: String,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject_line: String,
    /**
    * Settings for the campaign including the email subject, from name, and from email address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/**
* Whether the delay settings describe before or after the delay action of an automation email.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DelayDirection {
    #[serde(rename = "after")]
    After,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DelayDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DelayDirection::After => "after",
            DelayDirection::Noop => "",
            DelayDirection::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DelayDirection {
    fn default() -> DelayDirection {
        DelayDirection::Noop
    }
}
impl DelayDirection {
    pub fn is_noop(&self) -> bool {
        matches!(self, DelayDirection::Noop)
    }
}

/**
* The action that triggers the delay of an automation emails.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DelayAction {
    #[serde(rename = "ecomm_abandoned_browse")]
    EcommAbandonedBrowse,
    #[serde(rename = "ecomm_abandoned_cart")]
    EcommAbandonedCart,
    #[serde(rename = "signup")]
    Signup,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DelayAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            DelayAction::EcommAbandonedBrowse => "ecomm_abandoned_browse",
            DelayAction::EcommAbandonedCart => "ecomm_abandoned_cart",
            DelayAction::Signup => "signup",
            DelayAction::Noop => "",
            DelayAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DelayAction {
    fn default() -> DelayAction {
        DelayAction::Noop
    }
}
impl DelayAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, DelayAction::Noop)
    }
}

/// The delay settings for an automation email.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutomationDelay {
    /**
    * The action that triggers the delay of an automation emails.
    */
    #[serde(default, skip_serializing_if = "DelayAction::is_noop")]
    pub action: DelayAction,
    /**
    * The delay settings for an automation email.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub amount: i64,
    /**
    * The delay settings for an automation email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<DelayDirection>,
    /**
    * The delay settings for an automation email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<DelayType>,
}

/// Update information about an individual Automation workflow email.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateInformationAboutASpecificWorkflowEmail {
    /**
    * Update information about an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<AutomationDelay>,
    /**
    * Update information about an individual Automation workflow email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateInformationAboutASpecificWorkflowEmailCampaignSettings>,
}

/// Information about subscribers in an Automation email queue.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Queue {
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub next_send: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_id: String,
}

/// An automation workflow
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAutomationsEmailsQueueResponse {
    /**
    * An automation workflow
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An automation workflow
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * An automation workflow
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub queue: Vec<Queue>,
    /**
    * An automation workflow
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * An automation workflow
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_id: String,
}

/// Information about subscribers in an Automation email queue.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriberInAutomationQueueData {
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub next_send: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about subscribers in an Automation email queue.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_id: String,
}

/// A summary of a subscriber removed from an Automation workflow.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Subscribers {
    /**
    * A summary of a subscriber removed from an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of a subscriber removed from an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * A summary of a subscriber removed from an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A summary of a subscriber removed from an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A summary of a subscriber removed from an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_id: String,
}

/// A summary of the subscribers who were removed from an Automation workflow.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RemovedSubscribers {
    /**
    * A summary of the subscribers who were removed from an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of the subscribers who were removed from an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub subscribers: Vec<Subscribers>,
    /**
    * A summary of the subscribers who were removed from an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * A summary of the subscribers who were removed from an Automation workflow.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_id: String,
}

/**
* The status of the batch call. [Learn more](https://mailchimp.com/developer/marketing/guides/run-async-requests-batch-endpoint/#check-the-status-of-a-batch-operation) about the batch operation status.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum BatchOperationsStatus {
    #[serde(rename = "finalizing")]
    Finalizing,
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "preprocessing")]
    Preprocessing,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for BatchOperationsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            BatchOperationsStatus::Finalizing => "finalizing",
            BatchOperationsStatus::Finished => "finished",
            BatchOperationsStatus::Pending => "pending",
            BatchOperationsStatus::Preprocessing => "preprocessing",
            BatchOperationsStatus::Started => "started",
            BatchOperationsStatus::Noop => "",
            BatchOperationsStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for BatchOperationsStatus {
    fn default() -> BatchOperationsStatus {
        BatchOperationsStatus::Noop
    }
}
impl BatchOperationsStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, BatchOperationsStatus::Noop)
    }
}

/// The status of a batch request
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Batch {
    /**
    * The status of a batch request
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The status of a batch request
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The status of a batch request
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub errored_operations: i64,
    /**
    * The status of a batch request
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub finished_operations: i64,
    /**
    * The status of a batch request
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * The status of a batch request
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub response_body_url: String,
    /**
    * The status of a batch request
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BatchOperationsStatus>,
    /**
    * The status of a batch request
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The status of a batch request
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_operations: i64,
}

/// A summary of batch requests that have been made.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchOperations {
    /**
    * A summary of batch requests that have been made.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of batch requests that have been made.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub batches: Vec<Batch>,
    /**
    * A summary of batch requests that have been made.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* The HTTP method to use for the operation.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum HttpMethod {
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            HttpMethod::Delete => "DELETE",
            HttpMethod::Get => "GET",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Noop => "",
            HttpMethod::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for HttpMethod {
    fn default() -> HttpMethod {
        HttpMethod::Noop
    }
}
impl HttpMethod {
    pub fn is_noop(&self) -> bool {
        matches!(self, HttpMethod::Noop)
    }
}

/// Any request query parameters. Example parameters: {"count":10, "offset":0}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Params {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Operations {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
    * The HTTP method to use for the operation.
    */
    #[serde(default, skip_serializing_if = "HttpMethod::is_noop")]
    pub method: HttpMethod,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub operation_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostBatchesRequest {
    /**
    * An array of objects that describes operations to perform.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub operations: Vec<Operations>,
}

/// A webhook configured for batch status updates.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Webhooks {
    /**
    * A webhook configured for batch status updates.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A webhook configured for batch status updates.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A webhook configured for batch status updates.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Manage webhooks for batch requests.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchWebhooks {
    /**
    * Manage webhooks for batch requests.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Manage webhooks for batch requests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * Manage webhooks for batch requests.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub webhooks: Vec<Webhooks>,
}

/// Add a new Batch Webook.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchWebhook {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A folder used to organize templates.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Folders {
    /**
    * A folder used to organize templates.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A folder used to organize templates.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    /**
    * A folder used to organize templates.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A folder used to organize templates.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A list of template folders
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TemplateFolders {
    /**
    * A list of template folders
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of template folders
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub folders: Vec<Folders>,
    /**
    * A list of template folders
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A folder used to organize campaigns.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignFolder {
    /**
    * A folder used to organize campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A folder used to organize campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    /**
    * A folder used to organize campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A folder used to organize campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A list of campaign folders
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignFolders {
    /**
    * A list of campaign folders
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of campaign folders
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub folders: Vec<CampaignFolder>,
    /**
    * A list of campaign folders
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* There are four types of [campaigns](https://mailchimp.com/help/getting-started-with-campaigns/) you can create in Mailchimp. A/B Split campaigns have been deprecated and variate campaigns should be used instead.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CampaignType {
    #[serde(rename = "absplit")]
    Absplit,
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "rss")]
    Rss,
    #[serde(rename = "variate")]
    Variate,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CampaignType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CampaignType::Absplit => "absplit",
            CampaignType::Plaintext => "plaintext",
            CampaignType::Regular => "regular",
            CampaignType::Rss => "rss",
            CampaignType::Variate => "variate",
            CampaignType::Noop => "",
            CampaignType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CampaignType {
    fn default() -> CampaignType {
        CampaignType::Noop
    }
}
impl CampaignType {
    pub fn is_noop(&self) -> bool {
        matches!(self, CampaignType::Noop)
    }
}

/**
* The status of the campaign.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetCampaignsStatus {
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "save")]
    Save,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetCampaignsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetCampaignsStatus::Paused => "paused",
            GetCampaignsStatus::Save => "save",
            GetCampaignsStatus::Schedule => "schedule",
            GetCampaignsStatus::Sending => "sending",
            GetCampaignsStatus::Sent => "sent",
            GetCampaignsStatus::Noop => "",
            GetCampaignsStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetCampaignsStatus {
    fn default() -> GetCampaignsStatus {
        GetCampaignsStatus::Noop
    }
}
impl GetCampaignsStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetCampaignsStatus::Noop)
    }
}

/**
* Returns files sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SortField {
    #[serde(rename = "create_time")]
    CreateTime,
    #[serde(rename = "send_time")]
    SendTime,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SortField::CreateTime => "create_time",
            SortField::SendTime => "send_time",
            SortField::Noop => "",
            SortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SortField {
    fn default() -> SortField {
        SortField::Noop
    }
}
impl SortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, SortField::Noop)
    }
}

/**
* Determines the order direction for sorted results.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SortDir {
    #[serde(rename = "ASC")]
    Asc,
    #[serde(rename = "DESC")]
    Desc,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SortDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SortDir::Asc => "ASC",
            SortDir::Desc => "DESC",
            SortDir::Noop => "",
            SortDir::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SortDir {
    fn default() -> SortDir {
        SortDir::Noop
    }
}
impl SortDir {
    pub fn is_noop(&self) -> bool {
        matches!(self, SortDir::Noop)
    }
}

/**
* The current status of the campaign.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CampaignStatus {
    #[serde(rename = "archived")]
    Archived,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "canceling")]
    Canceling,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "save")]
    Save,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CampaignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CampaignStatus::Archived => "archived",
            CampaignStatus::Canceled => "canceled",
            CampaignStatus::Canceling => "canceling",
            CampaignStatus::Paused => "paused",
            CampaignStatus::Save => "save",
            CampaignStatus::Schedule => "schedule",
            CampaignStatus::Sending => "sending",
            CampaignStatus::Sent => "sent",
            CampaignStatus::Noop => "",
            CampaignStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CampaignStatus {
    fn default() -> CampaignStatus {
        CampaignStatus::Noop
    }
}
impl CampaignStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, CampaignStatus::Noop)
    }
}

/**
* How the campaign's content is put together.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ContentType {
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "multichannel")]
    Multichannel,
    #[serde(rename = "template")]
    Template,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ContentType::Html => "html",
            ContentType::Multichannel => "multichannel",
            ContentType::Template => "template",
            ContentType::Url => "url",
            ContentType::Noop => "",
            ContentType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ContentType {
    fn default() -> ContentType {
        ContentType::Noop
    }
}
impl ContentType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ContentType::Noop)
    }
}

/// The settings for your campaign, including subject, from name, reply-to address, and more.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCampaignsResponseCampaignSettings {
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub authenticate: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub auto_fb_post: Vec<String>,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_footer: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_tweet: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub drag_and_drop: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fb_comments: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub folder_id: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub inline_css: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub preview_text: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject_line: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub template_id: i64,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub timewarp: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to_name: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_conversation: bool,
}

/**
* The combination that performs the best. This may be determined automatically by click rate, open rate, or total revenue -- or you may choose manually based on the reporting data you find the most valuable. For Multivariate Campaigns testing send_time, winner_criteria is ignored. For Multivariate Campaigns with 'manual' as the winner_criteria, the winner must be chosen in the Mailchimp web application.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum WinnerCriteria {
    #[serde(rename = "clicks")]
    Clicks,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "opens")]
    Opens,
    #[serde(rename = "total_revenue")]
    TotalRevenue,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for WinnerCriteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            WinnerCriteria::Clicks => "clicks",
            WinnerCriteria::Manual => "manual",
            WinnerCriteria::Opens => "opens",
            WinnerCriteria::TotalRevenue => "total_revenue",
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
pub struct Combinations {
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub content_description: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub from_name: i64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipients: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub reply_to: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub send_time: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subject_line: i64,
}

/// The settings specific to A/B test campaigns.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ABTestOptions {
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub combinations: Vec<Combinations>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub contents: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub from_names: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub reply_to_addresses: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub send_times: Vec<Option<chrono::DateTime<chrono::Utc>>>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub subject_lines: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub test_size: i64,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub wait_time: i64,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winner_criteria: Option<WinnerCriteria>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub winning_campaign_id: String,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub winning_combination_id: String,
}

/**
* The frequency of the RSS Campaign.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Frequency {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Frequency::Daily => "daily",
            Frequency::Monthly => "monthly",
            Frequency::Weekly => "weekly",
            Frequency::Noop => "",
            Frequency::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Frequency {
    fn default() -> Frequency {
        Frequency::Noop
    }
}
impl Frequency {
    pub fn is_noop(&self) -> bool {
        matches!(self, Frequency::Noop)
    }
}

/// The days of the week to send a daily RSS Campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DailySend {
    /**
    * The days of the week to send a daily RSS Campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub friday: bool,
    /**
    * The days of the week to send a daily RSS Campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub monday: bool,
    /**
    * The days of the week to send a daily RSS Campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub saturday: bool,
    /**
    * The days of the week to send a daily RSS Campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub sunday: bool,
    /**
    * The days of the week to send a daily RSS Campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub thursday: bool,
    /**
    * The days of the week to send a daily RSS Campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub tuesday: bool,
    /**
    * The days of the week to send a daily RSS Campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub wednesday: bool,
}

/// The schedule for sending the RSS Campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Schedule {
    /**
    * The schedule for sending the RSS Campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_send: Option<DailySend>,
    /**
    * The schedule for sending the RSS Campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub hour: i64,
    /**
    * The schedule for sending the RSS Campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub monthly_send_date: f64,
    /**
    * The schedule for sending the RSS Campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_send_day: Option<Days>,
}

/// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RssOpts {
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub constrain_rss_img: bool,
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub feed_url: String,
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_sent: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}

/**
* The type of AB split to run.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SplitTest {
    #[serde(rename = "from_name")]
    FromName,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "subject")]
    Subject,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SplitTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SplitTest::FromName => "from_name",
            SplitTest::Schedule => "schedule",
            SplitTest::Subject => "subject",
            SplitTest::Noop => "",
            SplitTest::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SplitTest {
    fn default() -> SplitTest {
        SplitTest::Noop
    }
}
impl SplitTest {
    pub fn is_noop(&self) -> bool {
        matches!(self, SplitTest::Noop)
    }
}

/**
* How we should evaluate a winner. Based on 'opens', 'clicks', or 'manual'.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PickWinner {
    #[serde(rename = "clicks")]
    Clicks,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "opens")]
    Opens,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PickWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PickWinner::Clicks => "clicks",
            PickWinner::Manual => "manual",
            PickWinner::Opens => "opens",
            PickWinner::Noop => "",
            PickWinner::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PickWinner {
    fn default() -> PickWinner {
        PickWinner::Noop
    }
}
impl PickWinner {
    pub fn is_noop(&self) -> bool {
        matches!(self, PickWinner::Noop)
    }
}

/**
* How unit of time for measuring the winner ('hours' or 'days'). This cannot be changed after a campaign is sent.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum WaitTime {
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for WaitTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            WaitTime::Days => "days",
            WaitTime::Hours => "hours",
            WaitTime::Noop => "",
            WaitTime::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for WaitTime {
    fn default() -> WaitTime {
        WaitTime::Noop
    }
}
impl WaitTime {
    pub fn is_noop(&self) -> bool {
        matches!(self, WaitTime::Noop)
    }
}

/// [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AbSplitOpts {
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name_a: String,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name_b: String,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pick_winner: Option<PickWinner>,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_email_a: String,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_email_b: String,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_time_a: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_time_b: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub send_time_winner: String,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub split_size: i64,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_test: Option<SplitTest>,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject_a: String,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject_b: String,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub wait_time: i64,
    /**
    * [A/B Testing](https://mailchimp.com/help/about-ab-testing-campaigns/) options for a campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_units: Option<WaitTime>,
}

/// E-Commerce stats for a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Ecommerce {
    /**
    * E-Commerce stats for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_orders: i64,
    /**
    * E-Commerce stats for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_revenue: f64,
    /**
    * E-Commerce stats for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_spent: f64,
}

/// For sent campaigns, a summary of opens, clicks, and e-commerce data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCampaignsResponseCampaignReportSummary {
    /**
    * For sent campaigns, a summary of opens, clicks, and e-commerce data.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * For sent campaigns, a summary of opens, clicks, and e-commerce data.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * For sent campaigns, a summary of opens, clicks, and e-commerce data.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<Ecommerce>,
    /**
    * For sent campaigns, a summary of opens, clicks, and e-commerce data.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
    /**
    * For sent campaigns, a summary of opens, clicks, and e-commerce data.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * For sent campaigns, a summary of opens, clicks, and e-commerce data.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscriber_clicks: i64,
    /**
    * For sent campaigns, a summary of opens, clicks, and e-commerce data.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
}

/**
* The current state of a campaign delivery.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CampaignDeliveryStatus {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "canceling")]
    Canceling,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "delivering")]
    Delivering,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CampaignDeliveryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CampaignDeliveryStatus::Canceled => "canceled",
            CampaignDeliveryStatus::Canceling => "canceling",
            CampaignDeliveryStatus::Delivered => "delivered",
            CampaignDeliveryStatus::Delivering => "delivering",
            CampaignDeliveryStatus::Noop => "",
            CampaignDeliveryStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CampaignDeliveryStatus {
    fn default() -> CampaignDeliveryStatus {
        CampaignDeliveryStatus::Noop
    }
}
impl CampaignDeliveryStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, CampaignDeliveryStatus::Noop)
    }
}

/// Updates on campaigns in the process of sending.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeliveryStatus {
    /**
    * Updates on campaigns in the process of sending.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub can_cancel: bool,
    /**
    * Updates on campaigns in the process of sending.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_canceled: i64,
    /**
    * Updates on campaigns in the process of sending.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_sent: i64,
    /**
    * Updates on campaigns in the process of sending.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
    * Updates on campaigns in the process of sending.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CampaignDeliveryStatus>,
}

/// A summary of an individual campaign's settings and content.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Campaign {
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ab_split_opts: Option<AbSplitOpts>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<DeliveryStatus>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_sent: i64,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub long_archive_url: String,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub needs_block_refresh: bool,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parent_campaign_id: String,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<AutomationEmailsList>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<GetCampaignsResponseCampaignReportSummary>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub resendable: bool,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rss_opts: Option<RssOpts>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<GetCampaignsResponseCampaignSettings>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_card: Option<SocialCard>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CampaignStatus>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CampaignTrackingOptions>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<CampaignType>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variate_settings: Option<ABTestOptions>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub web_id: i64,
}

/// An array of campaigns.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCampaignsResponse {
    /**
    * An array of campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An array of campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub campaigns: Vec<Campaign>,
    /**
    * An array of campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreatedCampaignListSegmentOptions {
    /**
    * An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conditions: Vec<ConditionsOneOf>,
    /**
    * An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub match_: Option<Match>,
    /**
    * An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub saved_segment_id: i64,
}

/// List settings for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreatedCampaignList {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * List settings for the campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<CreatedCampaignListSegmentOptions>,
}

/// The settings for your campaign, including subject, from name, reply-to address, and more.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreatedCampaignSettings {
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub authenticate: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub auto_fb_post: Vec<String>,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_footer: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_tweet: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fb_comments: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub folder_id: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub inline_css: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub preview_text: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject_line: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub template_id: i64,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to_name: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_conversation: bool,
}

/// The settings specific to A/B test campaigns.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VariateSettings {
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub from_names: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub reply_to_addresses: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub send_times: Vec<Option<chrono::DateTime<chrono::Utc>>>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub subject_lines: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub test_size: i64,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub wait_time: i64,
    /**
    * The combination that performs the best. This may be determined automatically by click rate, open rate, or total revenue -- or you may choose manually based on the reporting data you find the most valuable. For Multivariate Campaigns testing send_time, winner_criteria is ignored. For Multivariate Campaigns with 'manual' as the winner_criteria, the winner must be chosen in the Mailchimp web application.
    */
    #[serde(default, skip_serializing_if = "WinnerCriteria::is_noop")]
    pub winner_criteria: WinnerCriteria,
}

/// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options, specific to an RSS campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RssOptions {
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options, specific to an RSS campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub constrain_rss_img: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub feed_url: String,
    /**
    * The frequency of the RSS Campaign.
    */
    #[serde(default, skip_serializing_if = "Frequency::is_noop")]
    pub frequency: Frequency,
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options, specific to an RSS campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}

/**
* How the campaign's content is put together. The old drag and drop editor uses 'template' while the new editor uses 'multichannel'. Defaults to template.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CreatedCampaignContentType {
    #[serde(rename = "multichannel")]
    Multichannel,
    #[serde(rename = "template")]
    Template,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CreatedCampaignContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CreatedCampaignContentType::Multichannel => "multichannel",
            CreatedCampaignContentType::Template => "template",
            CreatedCampaignContentType::Noop => "",
            CreatedCampaignContentType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CreatedCampaignContentType {
    fn default() -> CreatedCampaignContentType {
        CreatedCampaignContentType::Noop
    }
}
impl CreatedCampaignContentType {
    pub fn is_noop(&self) -> bool {
        matches!(self, CreatedCampaignContentType::Noop)
    }
}

/// A summary of an individual campaign's settings and content.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreatedCampaign {
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<CreatedCampaignContentType>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<CreatedCampaignList>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rss_opts: Option<RssOptions>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreatedCampaignSettings>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_card: Option<SocialCard>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CampaignTrackingOptions>,
    /**
    * There are four types of [campaigns](https://mailchimp.com/help/getting-started-with-campaigns/) you can create in Mailchimp. A/B Split campaigns have been deprecated and variate campaigns should be used instead.
    */
    #[serde(
        default,
        skip_serializing_if = "CampaignType::is_noop",
        rename = "type"
    )]
    pub type_: CampaignType,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variate_settings: Option<VariateSettings>,
}

/// List settings for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignList {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * List settings for the campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<SegmentOptions>,
}

/// The settings for your campaign, including subject, from name, reply-to address, and more.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignSettingsData {
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub authenticate: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub auto_fb_post: Vec<String>,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_footer: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_tweet: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fb_comments: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub folder_id: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub inline_css: bool,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub preview_text: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reply_to: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject_line: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub template_id: i64,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to_name: String,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_conversation: bool,
}

/// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignRssOptions {
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub constrain_rss_img: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub feed_url: String,
    /**
    * The frequency of the RSS Campaign.
    */
    #[serde(default, skip_serializing_if = "Frequency::is_noop")]
    pub frequency: Frequency,
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}

/// A summary of an individual campaign's settings and content.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignData {
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<CampaignList>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rss_opts: Option<CampaignRssOptions>,
    /**
    * The settings for your campaign, including subject, from name, reply-to address, and more.
    */
    #[serde()]
    pub settings: CampaignSettingsData,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_card: Option<SocialCard>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CampaignTrackingOptions>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variate_settings: Option<VariateSettings>,
}

/// List settings for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignListData {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * List settings for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_name: String,
    /**
    * List settings for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipient_count: i64,
    /**
    * List settings for the campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<SegmentOptions>,
    /**
    * List settings for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub segment_text: String,
}

/// The settings specific to A/B test campaigns.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignABTestOptions {
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub combinations: Vec<Combinations>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub contents: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub from_names: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub reply_to_addresses: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub send_times: Vec<Option<chrono::DateTime<chrono::Utc>>>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub subject_lines: Vec<String>,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub test_size: i64,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub wait_time: i64,
    /**
    * The combination that performs the best. This may be determined automatically by click rate, open rate, or total revenue -- or you may choose manually based on the reporting data you find the most valuable. For Multivariate Campaigns testing send_time, winner_criteria is ignored. For Multivariate Campaigns with 'manual' as the winner_criteria, the winner must be chosen in the Mailchimp web application.
    */
    #[serde(default, skip_serializing_if = "WinnerCriteria::is_noop")]
    pub winner_criteria: WinnerCriteria,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub winning_campaign_id: String,
    /**
    * The settings specific to A/B test campaigns.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub winning_combination_id: String,
}

/// [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignRssOptionsData {
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub constrain_rss_img: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub feed_url: String,
    /**
    * The frequency of the RSS Campaign.
    */
    #[serde(default, skip_serializing_if = "Frequency::is_noop")]
    pub frequency: Frequency,
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_sent: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}

/// For sent campaigns, a summary of opens and clicks.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignReportSummaryData {
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<Ecommerce>,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscriber_clicks: i64,
    /**
    * For sent campaigns, a summary of opens and clicks.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
}

/// A summary of an individual campaign's settings and content.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignDataType {
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ab_split_opts: Option<AbSplitOpts>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<DeliveryStatus>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_sent: i64,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub long_archive_url: String,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub needs_block_refresh: bool,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parent_campaign_id: String,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<CampaignListData>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<CampaignReportSummaryData>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub resendable: bool,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rss_opts: Option<CampaignRssOptionsData>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<GetCampaignsResponseCampaignSettings>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_card: Option<SocialCard>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CampaignStatus>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CampaignTrackingOptions>,
    /**
    * There are four types of [campaigns](https://mailchimp.com/help/getting-started-with-campaigns/) you can create in Mailchimp. A/B Split campaigns have been deprecated and variate campaigns should be used instead.
    */
    #[serde(
        default,
        skip_serializing_if = "CampaignType::is_noop",
        rename = "type"
    )]
    pub type_: CampaignType,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variate_settings: Option<CampaignABTestOptions>,
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub web_id: i64,
}

/// Choose whether the campaign should use [Batch Delivery](https://mailchimp.com/help/schedule-batch-delivery/). Cannot be set to `true` for campaigns using [Timewarp](https://mailchimp.com/help/use-timewarp/).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchDelivery {
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub batch_count: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub batch_delay: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCampaignsActionsScheduleRequest {
    /**
    * Choose whether the campaign should use [Batch Delivery](https://mailchimp.com/help/schedule-batch-delivery/). Cannot be set to `true` for campaigns using [Timewarp](https://mailchimp.com/help/use-timewarp/).
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_delivery: Option<BatchDelivery>,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub schedule_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timewarp: Option<bool>,
}

/**
* Choose the type of test email to send.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SendType {
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SendType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SendType::Html => "html",
            SendType::Plaintext => "plaintext",
            SendType::Noop => "",
            SendType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SendType {
    fn default() -> SendType {
        SendType::Noop
    }
}
impl SendType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SendType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCampaignsActionsTestRequest {
    /**
    * Choose the type of test email to send.
    */
    #[serde(default, skip_serializing_if = "SendType::is_noop")]
    pub send_type: SendType,
    /**
    * A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub test_emails: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VariateContents {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_label: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_text: String,
}

/// The HTML and plain-text content for a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignContent {
    /**
    * The HTML and plain-text content for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The HTML and plain-text content for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_html: String,
    /**
    * The HTML and plain-text content for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    /**
    * The HTML and plain-text content for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_text: String,
    /**
    * The HTML and plain-text content for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub variate_contents: Vec<VariateContents>,
}

/// Use this template to generate the HTML content of the campaign
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Template {
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * Use this template to generate the HTML content of the campaign
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sections: Option<Params>,
}

/**
* The type of encoded file. Defaults to zip.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ArchiveType {
    #[serde(rename = "tar")]
    Tar,
    #[serde(rename = "tar.bz2")]
    TarBz2,
    #[serde(rename = "tar.gz")]
    TarGz,
    #[serde(rename = "tbz")]
    Tbz,
    #[serde(rename = "tgz")]
    Tgz,
    #[serde(rename = "zip")]
    Zip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ArchiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ArchiveType::Tar => "tar",
            ArchiveType::TarBz2 => "tar.bz2",
            ArchiveType::TarGz => "tar.gz",
            ArchiveType::Tbz => "tbz",
            ArchiveType::Tgz => "tgz",
            ArchiveType::Zip => "zip",
            ArchiveType::Noop => "",
            ArchiveType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ArchiveType {
    fn default() -> ArchiveType {
        ArchiveType::Noop
    }
}
impl ArchiveType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ArchiveType::Noop)
    }
}

/// Available when uploading an archive to create campaign content. The archive should include all campaign content and images. [Learn more](https://mailchimp.com/help/import-a-custom-html-template/).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Archive {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_content: String,
    /**
    * Available when uploading an archive to create campaign content. The archive should include all campaign content and images. [Learn more](https://mailchimp.com/help/import-a-custom-html-template/).
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_type: Option<ArchiveType>,
}

/// Use this template to generate the HTML content for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TemplateContent {
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * Use this template to generate the HTML content for the campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sections: Option<Params>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignContentVariateContents {
    /**
    * Available when uploading an archive to create campaign content. The archive should include all campaign content and images. [Learn more](https://mailchimp.com/help/import-a-custom-html-template/).
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<Archive>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_label: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_text: String,
    /**
    * Use this template to generate the HTML content for the campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplateContent>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The HTML and plain-text content for a campaign
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignContentData {
    /**
    * The HTML and plain-text content for a campaign
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<Archive>,
    /**
    * The HTML and plain-text content for a campaign
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    /**
    * The HTML and plain-text content for a campaign
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub plain_text: String,
    /**
    * The HTML and plain-text content for a campaign
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
    /**
    * The HTML and plain-text content for a campaign
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * The HTML and plain-text content for a campaign
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub variate_contents: Vec<CampaignContentVariateContents>,
}

/**
* The source of the feedback.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Source {
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Source::Android => "android",
            Source::Api => "api",
            Source::Email => "email",
            Source::Ios => "ios",
            Source::Sms => "sms",
            Source::Web => "web",
            Source::Noop => "",
            Source::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Source {
    fn default() -> Source {
        Source::Noop
    }
}
impl Source {
    pub fn is_noop(&self) -> bool {
        matches!(self, Source::Noop)
    }
}

/// A specific feedback message from a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Feedback {
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub block_id: i64,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub feedback_id: i64,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_complete: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub parent_id: i64,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A summary of the comment feedback for a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignReports {
    /**
    * A summary of the comment feedback for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of the comment feedback for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A summary of the comment feedback for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub feedback: Vec<Feedback>,
    /**
    * A summary of the comment feedback for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A specific feedback message from a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignFeedback {
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub block_id: i64,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_complete: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// A specific feedback message from a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignFeedbackData {
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub block_id: i64,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub feedback_id: i64,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_complete: bool,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub parent_id: i64,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A specific feedback message from a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignFeedbackDataType {
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub block_id: i64,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_complete: bool,
    /**
    * A specific feedback message from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/**
* The item type.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SendChecklistItemsType {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SendChecklistItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SendChecklistItemsType::Error => "error",
            SendChecklistItemsType::Success => "success",
            SendChecklistItemsType::Warning => "warning",
            SendChecklistItemsType::Noop => "",
            SendChecklistItemsType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SendChecklistItemsType {
    fn default() -> SendChecklistItemsType {
        SendChecklistItemsType::Noop
    }
}
impl SendChecklistItemsType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SendChecklistItemsType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Items {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub details: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub heading: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * The item type.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<SendChecklistItemsType>,
}

/// The send checklist for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SendChecklist {
    /**
    * The send checklist for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The send checklist for the campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_ready: bool,
    /**
    * The send checklist for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<Items>,
}

/// The script used to connect your site with Mailchimp.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Script {
    /**
    * The script used to connect your site with Mailchimp.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fragment: String,
    /**
    * The script used to connect your site with Mailchimp.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Information about a specific connected site.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Sites {
    /**
    * Information about a specific connected site.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific connected site.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific connected site.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
    * Information about a specific connected site.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub foreign_id: String,
    /**
    * Information about a specific connected site.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub platform: String,
    /**
    * Information about a specific connected site.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_script: Option<Script>,
    /**
    * Information about a specific connected site.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * Information about a specific connected site.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A collection of connected sites in the account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConnectedSites {
    /**
    * A collection of connected sites in the account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of connected sites in the account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub sites: Vec<Sites>,
    /**
    * A collection of connected sites in the account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Information about a specific connected site.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConnectedSite {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub foreign_id: String,
}

/**
* Whether a conversation message has been marked as read.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum IsRead {
    #[serde(rename = "false")]
    False,
    #[serde(rename = "true")]
    True,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IsRead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IsRead::False => "false",
            IsRead::True => "true",
            IsRead::Noop => "",
            IsRead::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IsRead {
    fn default() -> IsRead {
        IsRead::Noop
    }
}
impl IsRead {
    pub fn is_noop(&self) -> bool {
        matches!(self, IsRead::Noop)
    }
}

/// The most recent message in the conversation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LastMessage {
    /**
    * The most recent message in the conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    /**
    * The most recent message in the conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_label: String,
    /**
    * The most recent message in the conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
    * The most recent message in the conversation.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub read: bool,
    /**
    * The most recent message in the conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    /**
    * The most recent message in the conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/// Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Conversation {
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_label: String,
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message: Option<LastMessage>,
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub message_count: i64,
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    /**
    * Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unread_messages: i64,
}

/// A collection of this account's tracked conversations.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TrackedConversations {
    /**
    * A collection of this account's tracked conversations.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of this account's tracked conversations.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conversations: Vec<Conversation>,
    /**
    * A collection of this account's tracked conversations.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConversationMessage {
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conversation_id: String,
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_label: String,
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub list_id: i64,
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub read: bool,
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
    /**
    * An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/// Messages from a specific conversation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CollectionOfConversationMessages {
    /**
    * Messages from a specific conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Messages from a specific conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conversation_id: String,
    /**
    * Messages from a specific conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conversation_messages: Vec<ConversationMessage>,
    /**
    * Messages from a specific conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* Returns files sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetFileManagerFilesSortField {
    #[serde(rename = "added_date")]
    AddedDate,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetFileManagerFilesSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetFileManagerFilesSortField::AddedDate => "added_date",
            GetFileManagerFilesSortField::Noop => "",
            GetFileManagerFilesSortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetFileManagerFilesSortField {
    fn default() -> GetFileManagerFilesSortField {
        GetFileManagerFilesSortField::Noop
    }
}
impl GetFileManagerFilesSortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetFileManagerFilesSortField::Noop)
    }
}

/**
* The type of file in the File Manager.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FileType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FileType::File => "file",
            FileType::Image => "image",
            FileType::Noop => "",
            FileType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FileType {
    fn default() -> FileType {
        FileType::Noop
    }
}
impl FileType {
    pub fn is_noop(&self) -> bool {
        matches!(self, FileType::Noop)
    }
}

/// An individual file listed in the File Manager.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Files {
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub folder_id: i64,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_size_url: String,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub height: i64,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub thumbnail_url: String,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<FileType>,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub width: i64,
}

/// A list of available images and files stored in the File Manager for the account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FileManager {
    /**
    * A list of available images and files stored in the File Manager for the account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of available images and files stored in the File Manager for the account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub files: Vec<Files>,
    /**
    * A list of available images and files stored in the File Manager for the account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_file_size: f64,
    /**
    * A list of available images and files stored in the File Manager for the account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// An individual file listed in the File Manager.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GalleryFile {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub file_data: String,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub folder_id: i64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// An individual file listed in the File Manager.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GalleryFileData {
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub folder_id: i64,
    /**
    * An individual file listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// An individual folder listed in the File Manager.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FileManagerFoldersGalleryFolder {
    /**
    * An individual folder listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An individual folder listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * An individual folder listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    /**
    * An individual folder listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub file_count: i64,
    /**
    * An individual folder listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * An individual folder listed in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A list of all folders in the File Manager.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FileManagerFolders {
    /**
    * A list of all folders in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of all folders in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub folders: Vec<FileManagerFoldersGalleryFolder>,
    /**
    * A list of all folders in the File Manager.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* Returns files sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetListsSortField {
    #[serde(rename = "date_created")]
    DateCreated,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetListsSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetListsSortField::DateCreated => "date_created",
            GetListsSortField::Noop => "",
            GetListsSortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetListsSortField {
    fn default() -> GetListsSortField {
        GetListsSortField::Noop
    }
}
impl GetListsSortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetListsSortField::Noop)
    }
}

/// [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListContact {
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignDefaults {
    /**
    * [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    /**
    * [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
    * [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
    * [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
}

/**
* Legacy - visibility settings are no longer used
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Visibility {
    #[serde(rename = "prv")]
    Prv,
    #[serde(rename = "pub")]
    Pub,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Visibility::Prv => "prv",
            Visibility::Pub => "pub",
            Visibility::Noop => "",
            Visibility::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Visibility {
    fn default() -> Visibility {
        Visibility::Noop
    }
}
impl Visibility {
    pub fn is_noop(&self) -> bool {
        matches!(self, Visibility::Noop)
    }
}

/// Stats for the list. Many of these are cached for at least five minutes.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Stats {
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub avg_sub_rate: f64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub avg_unsub_rate: f64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub campaign_count: i64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub campaign_last_sent: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub cleaned_count: i64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub cleaned_count_since_send: i64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_sub_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_unsub_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub member_count: i64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub member_count_since_send: i64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub merge_field_count: i64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub target_sub_rate: f64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_contacts: i64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribe_count: i64,
    /**
    * Stats for the list. Many of these are cached for at least five minutes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribe_count_since_send: i64,
}

/// Information about a specific list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Lists {
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub beamer_address: String,
    /**
    * Information about a specific list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub campaign_defaults: Option<CampaignDefaults>,
    /**
    * Information about a specific list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<ListContact>,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub double_optin: bool,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub email_type_option: bool,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_welcome: bool,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub list_rating: i64,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub marketing_permissions: bool,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub modules: Vec<String>,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notify_on_subscribe: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notify_on_unsubscribe: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission_reminder: String,
    /**
    * Information about a specific list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribe_url_long: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribe_url_short: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_archive_bar: bool,
    /**
    * Information about a specific list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub web_id: i64,
}

/// Do particular authorization constraints around this collection limit creation of new instances?
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Constraints {
    /**
    * Do particular authorization constraints around this collection limit creation of new instances?
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub current_total_instances: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub max_instances: i64,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub may_create: bool,
}

/// A collection of subscriber lists for this account. Lists contain subscribers who have opted-in to receive correspondence from you or your organization.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriberLists {
    /**
    * A collection of subscriber lists for this account. Lists contain subscribers who have opted-in to receive correspondence from you or your organization.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of subscriber lists for this account. Lists contain subscribers who have opted-in to receive correspondence from you or your organization.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    /**
    * An array of objects, each representing a list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lists: Vec<Lists>,
    /**
    * A collection of subscriber lists for this account. Lists contain subscribers who have opted-in to receive correspondence from you or your organization.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriberListContact {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriberListCampaignDefaults {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_name: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject: String,
}

/// Information about a specific list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriberList {
    /**
    * [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    */
    #[serde()]
    pub campaign_defaults: SubscriberListCampaignDefaults,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde()]
    pub contact: SubscriberListContact,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub double_optin: bool,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub email_type_option: bool,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub marketing_permissions: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notify_on_subscribe: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notify_on_unsubscribe: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission_reminder: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_archive_bar: bool,
}

/**
* Subscriber's current status.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MembersSubscribeUnsubscribeFromAListInBatchStatus {
    #[serde(rename = "cleaned")]
    Cleaned,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MembersSubscribeUnsubscribeFromAListInBatchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MembersSubscribeUnsubscribeFromAListInBatchStatus::Cleaned => "cleaned",
            MembersSubscribeUnsubscribeFromAListInBatchStatus::Pending => "pending",
            MembersSubscribeUnsubscribeFromAListInBatchStatus::Subscribed => "subscribed",
            MembersSubscribeUnsubscribeFromAListInBatchStatus::Unsubscribed => "unsubscribed",
            MembersSubscribeUnsubscribeFromAListInBatchStatus::Noop => "",
            MembersSubscribeUnsubscribeFromAListInBatchStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MembersSubscribeUnsubscribeFromAListInBatchStatus {
    fn default() -> MembersSubscribeUnsubscribeFromAListInBatchStatus {
        MembersSubscribeUnsubscribeFromAListInBatchStatus::Noop
    }
}
impl MembersSubscribeUnsubscribeFromAListInBatchStatus {
    pub fn is_noop(&self) -> bool {
        matches!(
            self,
            MembersSubscribeUnsubscribeFromAListInBatchStatus::Noop
        )
    }
}

/// Subscriber location information.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Location {
    /**
    * Subscriber location information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub latitude: f64,
    /**
    * Subscriber location information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub longitude: f64,
}

/// Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Members {
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_type: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub interests: bool,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_opt: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_signup: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MembersSubscribeUnsubscribeFromAListInBatchStatus>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_opt: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_signup: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// Members to subscribe to or unsubscribe from a list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MembersSubscribeUnsubscribeFromAListInBatch {
    /**
    * An array of objects, each representing an email address and the subscription status for a specific list. Up to 500 members may be added or updated with each API call.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members: Vec<Members>,
    /**
    * Members to subscribe to or unsubscribe from a list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub update_existing: bool,
}

/**
* Subscriber's status. This value is required only if the email address is not already present on the list.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StatusIfNew {
    #[serde(rename = "cleaned")]
    Cleaned,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StatusIfNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            StatusIfNew::Cleaned => "cleaned",
            StatusIfNew::Pending => "pending",
            StatusIfNew::Subscribed => "subscribed",
            StatusIfNew::Transactional => "transactional",
            StatusIfNew::Unsubscribed => "unsubscribed",
            StatusIfNew::Noop => "",
            StatusIfNew::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StatusIfNew {
    fn default() -> StatusIfNew {
        StatusIfNew::Noop
    }
}
impl StatusIfNew {
    pub fn is_noop(&self) -> bool {
        matches!(self, StatusIfNew::Noop)
    }
}

/// Open and click rates for this subscriber.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriberStats {
    /**
    * Open and click rates for this subscriber.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub avg_click_rate: f64,
    /**
    * Open and click rates for this subscriber.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub avg_open_rate: f64,
}

/// Subscriber location information.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchUpdateListMembersNewLocation {
    /**
    * Subscriber location information.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country_code: String,
    /**
    * Subscriber location information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub dstoff: i64,
    /**
    * Subscriber location information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub gmtoff: i64,
    /**
    * Subscriber location information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub latitude: f64,
    /**
    * Subscriber location information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub longitude: f64,
    /**
    * Subscriber location information.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
}

/// The most recent Note added about this member.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Notes {
    /**
    * The most recent Note added about this member.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The most recent Note added about this member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    /**
    * The most recent Note added about this member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    /**
    * The most recent Note added about this member.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub note_id: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Tags {
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NewMembers {
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_client: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_type: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub interests: bool,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_opt: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_signup: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_changed: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_note: Option<Notes>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<BatchUpdateListMembersNewLocation>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub member_rating: i64,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<SubscriberStats>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusIfNew>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<Tags>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub tags_count: i64,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_opt: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_signup: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unique_email_id: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/**
* A unique code that identifies this specifc error.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ErrorCode {
    #[serde(rename = "ERROR_CONTACT_EXISTS")]
    ErrorContactExists,
    #[serde(rename = "ERROR_GENERIC")]
    ErrorGeneric,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ErrorCode::ErrorContactExists => "ERROR_CONTACT_EXISTS",
            ErrorCode::ErrorGeneric => "ERROR_GENERIC",
            ErrorCode::Noop => "",
            ErrorCode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ErrorCode {
    fn default() -> ErrorCode {
        ErrorCode::Noop
    }
}
impl ErrorCode {
    pub fn is_noop(&self) -> bool {
        matches!(self, ErrorCode::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Errors {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,
    /**
    * A unique code that identifies this specifc error.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<ErrorCode>,
}

/// Batch update list members.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchUpdateListMembers {
    /**
    * Batch update list members.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Batch update list members.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub error_count: i64,
    /**
    * Batch update list members.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<Errors>,
    /**
    * Batch update list members.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub new_members: Vec<NewMembers>,
    /**
    * Batch update list members.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_created: i64,
    /**
    * Batch update list members.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_updated: i64,
    /**
    * Batch update list members.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub updated_members: Vec<NewMembers>,
}

/// [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriberListContactData {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// Information about a specific list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubscriberListData {
    /**
    * [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    */
    #[serde()]
    pub campaign_defaults: SubscriberListCampaignDefaults,
    /**
    * [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    */
    #[serde()]
    pub contact: SubscriberListContactData,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub double_optin: bool,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub email_type_option: bool,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub marketing_permissions: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notify_on_subscribe: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notify_on_unsubscribe: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission_reminder: String,
    /**
    * Information about a specific list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_archive_bar: bool,
}

/// Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AbuseReports {
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// A collection of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AbuseComplaints {
    /**
    * A collection of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub abuse_reports: Vec<AbuseReports>,
    /**
    * A collection of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A collection of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// One day's worth of list activity. Doesn't include Automation activity.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Activity {
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub day: String,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_sent: i64,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub hard_bounce: i64,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub other_adds: i64,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub other_removes: i64,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipient_clicks: i64,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub soft_bounce: i64,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subs: i64,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
    /**
    * One day's worth of list activity. Doesn't include Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubs: i64,
}

/// Up to the previous 180 days of daily detailed aggregated activity stats for a specific list. Does not include AutoResponder or Automation activity.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListActivity {
    /**
    * Up to the previous 180 days of daily detailed aggregated activity stats for a specific list. Does not include AutoResponder or Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Up to the previous 180 days of daily detailed aggregated activity stats for a specific list. Does not include AutoResponder or Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub activity: Vec<Activity>,
    /**
    * Up to the previous 180 days of daily detailed aggregated activity stats for a specific list. Does not include AutoResponder or Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Up to the previous 180 days of daily detailed aggregated activity stats for a specific list. Does not include AutoResponder or Automation activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// The email client.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Clients {
    /**
    * The email client.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client: String,
    /**
    * The email client.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub members: i64,
}

/// The top email clients based on user-agent strings.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailClients {
    /**
    * The top email clients based on user-agent strings.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The top email clients based on user-agent strings.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub clients: Vec<Clients>,
    /**
    * The top email clients based on user-agent strings.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * The top email clients based on user-agent strings.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* Returns files sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetListsGrowthHistorySortField {
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetListsGrowthHistorySortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetListsGrowthHistorySortField::Month => "month",
            GetListsGrowthHistorySortField::Noop => "",
            GetListsGrowthHistorySortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetListsGrowthHistorySortField {
    fn default() -> GetListsGrowthHistorySortField {
        GetListsGrowthHistorySortField::Noop
    }
}
impl GetListsGrowthHistorySortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetListsGrowthHistorySortField::Noop)
    }
}

/// A summary of a specific list's growth activity for a specific month and year.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct History {
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub cleaned: i64,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub deleted: i64,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub existing: i64,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub imports: i64,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub month: String,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub optins: i64,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub pending: i64,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub reconfirm: i64,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscribed: i64,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub transactional: i64,
    /**
    * A summary of a specific list's growth activity for a specific month and year.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribed: i64,
}

/// A month-by-month summary of a specific list's growth activity.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GrowthHistory {
    /**
    * A month-by-month summary of a specific list's growth activity.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A month-by-month summary of a specific list's growth activity.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub history: Vec<History>,
    /**
    * A month-by-month summary of a specific list's growth activity.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A month-by-month summary of a specific list's growth activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Categories {
    /**
    * Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub display_order: i64,
    /**
    * Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
}

/// Information about this list's interest categories.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InterestGroupings {
    /**
    * Information about this list's interest categories.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about this list's interest categories.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub categories: Vec<Categories>,
    /**
    * Information about this list's interest categories.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Information about this list's interest categories.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InterestsInterest {
    /**
    * Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category_id: String,
    /**
    * Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub display_order: i64,
    /**
    * Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriber_count: String,
}

/// A list of this category's interests
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InterestsData {
    /**
    * A list of this category's interests
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of this category's interests
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category_id: String,
    /**
    * A list of this category's interests
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub interests: Vec<InterestsInterest>,
    /**
    * A list of this category's interests
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A list of this category's interests
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* The type of segment. Static segments are now known as tags. Learn more about [tags](https://mailchimp.com/help/getting-started-tags?utm_source=mc-api&utm_medium=docs&utm_campaign=apidocs).
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CollectionOfSegmentsType {
    #[serde(rename = "fuzzy")]
    Fuzzy,
    #[serde(rename = "saved")]
    Saved,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CollectionOfSegmentsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CollectionOfSegmentsType::Fuzzy => "fuzzy",
            CollectionOfSegmentsType::Saved => "saved",
            CollectionOfSegmentsType::Static => "static",
            CollectionOfSegmentsType::Noop => "",
            CollectionOfSegmentsType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CollectionOfSegmentsType {
    fn default() -> CollectionOfSegmentsType {
        CollectionOfSegmentsType::Noop
    }
}
impl CollectionOfSegmentsType {
    pub fn is_noop(&self) -> bool {
        matches!(self, CollectionOfSegmentsType::Noop)
    }
}

/// The conditions of the segment. Static segments (tags) and fuzzy segments don't have conditions.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Options {
    /**
    * The conditions of the segment. Static segments (tags) and fuzzy segments don't have conditions.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conditions: Vec<ConditionsOneOf>,
    /**
    * The conditions of the segment. Static segments (tags) and fuzzy segments don't have conditions.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub match_: Option<Match>,
}

/// Information about a specific segment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Segments {
    /**
    * Information about a specific segment.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific segment.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific segment.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * Information about a specific segment.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Information about a specific segment.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub member_count: i64,
    /**
    * Information about a specific segment.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * Information about a specific segment.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    /**
    * Information about a specific segment.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<CollectionOfSegmentsType>,
    /**
    * Information about a specific segment.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A list of available segments.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CollectionOfSegments {
    /**
    * A list of available segments.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of available segments.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A list of available segments.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segments: Vec<Segments>,
    /**
    * A list of available segments.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListConditions {
    /**
    * The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conditions: Vec<ConditionsOneOf>,
    /**
    * The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub match_: Option<Match>,
}

/// Information about a specific list segment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListData {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * Information about a specific list segment.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<ListConditions>,
    /**
    * Information about a specific list segment.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub static_segment: Vec<String>,
}

/// Members to add/remove to/from a static segment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MembersAddRemoveFromAStaticSegment {
    /**
    * Members to add/remove to/from a static segment
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members_to_add: Vec<String>,
    /**
    * Members to add/remove to/from a static segment
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members_to_remove: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchAddRemoveListMembersFromStaticSegmentErrors {
    /**
    * A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub email_addresses: Vec<String>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,
}

/// Batch add/remove List members to/from static segment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchAddRemoveListMembersFromStaticSegment {
    /**
    * Batch add/remove List members to/from static segment
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Batch add/remove List members to/from static segment
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub error_count: i64,
    /**
    * Batch add/remove List members to/from static segment
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<BatchAddRemoveListMembersFromStaticSegmentErrors>,
    /**
    * Batch add/remove List members to/from static segment
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members_added: Vec<NewMembers>,
    /**
    * Batch add/remove List members to/from static segment
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members_removed: Vec<NewMembers>,
    /**
    * Batch add/remove List members to/from static segment
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_added: i64,
    /**
    * Batch add/remove List members to/from static segment
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_removed: i64,
}

/// The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListConditionsData {
    /**
    * The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub conditions: Vec<Vec<ConditionsOneOf>>,
    /**
    * The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub match_: Option<Match>,
}

/// Information about a specific list segment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListDataType {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * Information about a specific list segment.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<ListConditionsData>,
    /**
    * Information about a specific list segment.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub static_segment: Vec<String>,
}

/// Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListMembers {
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_client: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_type: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub interests: bool,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_opt: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_signup: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_changed: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_note: Option<Notes>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<BatchUpdateListMembersNewLocation>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub member_rating: i64,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<SubscriberStats>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusIfNew>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_opt: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_signup: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unique_email_id: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// View members in a specific list segment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SegmentMembers {
    /**
    * View members in a specific list segment.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * View members in a specific list segment.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members: Vec<ListMembers>,
    /**
    * View members in a specific list segment.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A list of tags matching the input query.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TagSearchResults {
    /**
    * A list of tags matching the input query.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<String>,
    /**
    * A list of tags matching the input query.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* The subscriber's status.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetListsMembersStatus {
    #[serde(rename = "archived")]
    Archived,
    #[serde(rename = "cleaned")]
    Cleaned,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetListsMembersStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetListsMembersStatus::Archived => "archived",
            GetListsMembersStatus::Cleaned => "cleaned",
            GetListsMembersStatus::Pending => "pending",
            GetListsMembersStatus::Subscribed => "subscribed",
            GetListsMembersStatus::Transactional => "transactional",
            GetListsMembersStatus::Unsubscribed => "unsubscribed",
            GetListsMembersStatus::Noop => "",
            GetListsMembersStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetListsMembersStatus {
    fn default() -> GetListsMembersStatus {
        GetListsMembersStatus::Noop
    }
}
impl GetListsMembersStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetListsMembersStatus::Noop)
    }
}

/**
* Used to filter list members by interests. Must be accompanied by interest_category_id and interest_ids. "any" will match a member with any of the interest supplied, "all" will only match members with every interest supplied, and "none" will match members without any of the interest supplied.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum InterestMatch {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for InterestMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InterestMatch::All => "all",
            InterestMatch::Any => "any",
            InterestMatch::None => "none",
            InterestMatch::Noop => "",
            InterestMatch::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for InterestMatch {
    fn default() -> InterestMatch {
        InterestMatch::Noop
    }
}
impl InterestMatch {
    pub fn is_noop(&self) -> bool {
        matches!(self, InterestMatch::Noop)
    }
}

/**
* Returns files sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetListsMembersSortField {
    #[serde(rename = "last_changed")]
    LastChanged,
    #[serde(rename = "timestamp_opt")]
    TimestampOpt,
    #[serde(rename = "timestamp_signup")]
    TimestampSignup,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetListsMembersSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetListsMembersSortField::LastChanged => "last_changed",
            GetListsMembersSortField::TimestampOpt => "timestamp_opt",
            GetListsMembersSortField::TimestampSignup => "timestamp_signup",
            GetListsMembersSortField::Noop => "",
            GetListsMembersSortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetListsMembersSortField {
    fn default() -> GetListsMembersSortField {
        GetListsMembersSortField::Noop
    }
}
impl GetListsMembersSortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetListsMembersSortField::Noop)
    }
}

/// Ecommerce stats for the list member if the list is attached to a store.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommerceData {
    /**
    * Ecommerce stats for the list member if the list is attached to a store.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * Ecommerce stats for the list member if the list is attached to a store.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub number_of_orders: f64,
    /**
    * Ecommerce stats for the list member if the list is attached to a store.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_revenue: f64,
}

/// Open and click rates for this subscriber.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListMembersSubscriberStats {
    /**
    * Open and click rates for this subscriber.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub avg_click_rate: f64,
    /**
    * Open and click rates for this subscriber.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub avg_open_rate: f64,
    /**
    * Open and click rates for this subscriber.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecommerce_data: Option<EcommerceData>,
}

/// A single marketing permission a subscriber has either opted-in to or opted-out of.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MarketingPermission {
    /**
    * A single marketing permission a subscriber has either opted-in to or opted-out of.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
    * A single marketing permission a subscriber has either opted-in to or opted-out of.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub marketing_permission_id: String,
    /**
    * A single marketing permission a subscriber has either opted-in to or opted-out of.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

/// Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListMembersData {
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_client: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_type: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub interests: bool,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_opt: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_signup: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_changed: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_note: Option<Notes>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<BatchUpdateListMembersNewLocation>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub marketing_permissions: Vec<MarketingPermission>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub member_rating: i64,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub source: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<ListMembersSubscriberStats>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GetListsMembersStatus>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<Tags>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub tags_count: i64,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_opt: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_signup: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unique_email_id: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unsubscribe_reason: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub web_id: i64,
}

/// Manage members of a specific Mailchimp list, including currently subscribed, unsubscribed, and bounced members.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListMembersDataType {
    /**
    * Manage members of a specific Mailchimp list, including currently subscribed, unsubscribed, and bounced members.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Manage members of a specific Mailchimp list, including currently subscribed, unsubscribed, and bounced members.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Manage members of a specific Mailchimp list, including currently subscribed, unsubscribed, and bounced members.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members: Vec<ListMembersData>,
    /**
    * Manage members of a specific Mailchimp list, including currently subscribed, unsubscribed, and bounced members.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A single marketing permission a subscriber has either opted-in to or opted-out of.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MarketingPermissions {
    /**
    * A single marketing permission a subscriber has either opted-in to or opted-out of.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
    * A single marketing permission a subscriber has either opted-in to or opted-out of.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub marketing_permission_id: String,
}

/// Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddListMembers {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_type: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub interests: bool,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_opt: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_signup: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub marketing_permissions: Vec<MarketingPermissions>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * Subscriber's status. This value is required only if the email address is not already present on the list.
    */
    #[serde(default, skip_serializing_if = "StatusIfNew::is_noop")]
    pub status: StatusIfNew,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<String>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_opt: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_signup: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddListMembersData {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_type: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub interests: bool,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_opt: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_signup: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub marketing_permissions: Vec<MarketingPermissions>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusIfNew>,
    /**
    * Subscriber's status. This value is required only if the email address is not already present on the list.
    */
    #[serde(default, skip_serializing_if = "StatusIfNew::is_noop")]
    pub status_if_new: StatusIfNew,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_opt: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_signup: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddListMembersDataType {
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_type: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub interests: bool,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_opt: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_signup: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub marketing_permissions: Vec<MarketingPermissions>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MembersSubscribeUnsubscribeFromAListInBatchStatus>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_opt: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp_signup: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// Member activity events.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MemberActivity {
    /**
    * Member activity events.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    /**
    * Member activity events.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Member activity events.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parent_campaign: String,
    /**
    * Member activity events.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Member activity events.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Member activity events.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
    * Member activity events.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The last 50 member events for a list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MemberActivityEvents {
    /**
    * The last 50 member events for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The last 50 member events for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub activity: Vec<MemberActivity>,
    /**
    * The last 50 member events for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * The last 50 member events for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * The last 50 member events for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ActivityType {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ActivityType::Open => "open",
            ActivityType::Noop => "",
            ActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ActivityType {
    fn default() -> ActivityType {
        ActivityType::Noop
    }
}
impl ActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ActivityType::Noop)
    }
}

/// Activity feed item representing opening an email.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailOpens {
    /**
    * Activity feed item representing opening an email.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ActivityType>,
    /**
    * Activity feed item representing opening an email.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Activity feed item representing opening an email.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_title: String,
    /**
    * Activity feed item representing opening an email.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailClicksActivityType {
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailClicksActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailClicksActivityType::Click => "click",
            EmailClicksActivityType::Noop => "",
            EmailClicksActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailClicksActivityType {
    fn default() -> EmailClicksActivityType {
        EmailClicksActivityType::Noop
    }
}
impl EmailClicksActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailClicksActivityType::Noop)
    }
}

/// Activity feed item representing having a link clicked by a contact.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailClicks {
    /**
    * Activity feed item representing having a link clicked by a contact.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<EmailClicksActivityType>,
    /**
    * Activity feed item representing having a link clicked by a contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Activity feed item representing having a link clicked by a contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_title: String,
    /**
    * Activity feed item representing having a link clicked by a contact.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item representing having a link clicked by a contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub link_clicked: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailBouncedActivityType {
    #[serde(rename = "bounce")]
    Bounce,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailBouncedActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailBouncedActivityType::Bounce => "bounce",
            EmailBouncedActivityType::Noop => "",
            EmailBouncedActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailBouncedActivityType {
    fn default() -> EmailBouncedActivityType {
        EmailBouncedActivityType::Noop
    }
}
impl EmailBouncedActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailBouncedActivityType::Noop)
    }
}

/**
* The type of bounce.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum BounceType {
    #[serde(rename = "hard")]
    Hard,
    #[serde(rename = "soft")]
    Soft,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for BounceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            BounceType::Hard => "hard",
            BounceType::Soft => "soft",
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

/// Activity feed item representing an email to this contact bouncing.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailBounced {
    /**
    * Activity feed item representing an email to this contact bouncing.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<EmailBouncedActivityType>,
    /**
    * Activity feed item representing an email to this contact bouncing.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub bounce_has_open_activity: bool,
    /**
    * Activity feed item representing an email to this contact bouncing.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounce_type: Option<BounceType>,
    /**
    * Activity feed item representing an email to this contact bouncing.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Activity feed item representing an email to this contact bouncing.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_title: String,
    /**
    * Activity feed item representing an email to this contact bouncing.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ListUnsubscribedActivityType {
    #[serde(rename = "unsub")]
    Unsub,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ListUnsubscribedActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ListUnsubscribedActivityType::Unsub => "unsub",
            ListUnsubscribedActivityType::Noop => "",
            ListUnsubscribedActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ListUnsubscribedActivityType {
    fn default() -> ListUnsubscribedActivityType {
        ListUnsubscribedActivityType::Noop
    }
}
impl ListUnsubscribedActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ListUnsubscribedActivityType::Noop)
    }
}

/// Activity feed item representing this contact unsubscribing from a list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListUnsubscribed {
    /**
    * Activity feed item representing this contact unsubscribing from a list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ListUnsubscribedActivityType>,
    /**
    * Activity feed item representing this contact unsubscribing from a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Activity feed item representing this contact unsubscribing from a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_title: String,
    /**
    * Activity feed item representing this contact unsubscribing from a list.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item representing this contact unsubscribing from a list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_admin_unsubscribed: bool,
    /**
    * Activity feed item representing this contact unsubscribing from a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unsubscribe_reason: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EmailSentActivityType {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EmailSentActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EmailSentActivityType::Sent => "sent",
            EmailSentActivityType::Noop => "",
            EmailSentActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EmailSentActivityType {
    fn default() -> EmailSentActivityType {
        EmailSentActivityType::Noop
    }
}
impl EmailSentActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EmailSentActivityType::Noop)
    }
}

/// Activity feed item representing having an email sent to the contact.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailSent {
    /**
    * Activity feed item representing having an email sent to the contact.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<EmailSentActivityType>,
    /**
    * Activity feed item representing having an email sent to the contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Activity feed item representing having an email sent to the contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_title: String,
    /**
    * Activity feed item representing having an email sent to the contact.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/// Activity feed item representing an individual reply in a conversation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailConversation {
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ConversationSegmentField>,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_title: String,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from_email: String,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_read: bool,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_user: bool,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message_text: String,
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub thread_id: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NoteActivityType {
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NoteActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            NoteActivityType::Note => "note",
            NoteActivityType::Noop => "",
            NoteActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NoteActivityType {
    fn default() -> NoteActivityType {
        NoteActivityType::Noop
    }
}
impl NoteActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, NoteActivityType::Noop)
    }
}

/// Activity feed item representing a note on the contact record.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Note {
    /**
    * Activity feed item representing a note on the contact record.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<NoteActivityType>,
    /**
    * Activity feed item representing a note on the contact record.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    /**
    * Activity feed item representing a note on the contact record.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item representing a note on the contact record.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    /**
    * Activity feed item representing a note on the contact record.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_id: String,
    /**
    * Activity feed item representing a note on the contact record.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_text: String,
    /**
    * Activity feed item representing a note on the contact record.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MarketingPermissionActivityType {
    #[serde(rename = "marketing_permission")]
    MarketingPermission,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MarketingPermissionActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MarketingPermissionActivityType::MarketingPermission => "marketing_permission",
            MarketingPermissionActivityType::Noop => "",
            MarketingPermissionActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MarketingPermissionActivityType {
    fn default() -> MarketingPermissionActivityType {
        MarketingPermissionActivityType::Noop
    }
}
impl MarketingPermissionActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, MarketingPermissionActivityType::Noop)
    }
}

/// Activity feed item indicating if a marketing permission was added or updated.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MarketingPermissionData {
    /**
    * Activity feed item indicating if a marketing permission was added or updated.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<MarketingPermissionActivityType>,
    /**
    * Activity feed item indicating if a marketing permission was added or updated.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item indicating if a marketing permission was added or updated.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub marketing_permission_opted_in: bool,
    /**
    * Activity feed item indicating if a marketing permission was added or updated.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub marketing_permisson_text: String,
    /**
    * Activity feed item indicating if a marketing permission was added or updated.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_by: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PostcardSentActivityType {
    #[serde(rename = "postcard_sent")]
    PostcardSent,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PostcardSentActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PostcardSentActivityType::PostcardSent => "postcard_sent",
            PostcardSentActivityType::Noop => "",
            PostcardSentActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PostcardSentActivityType {
    fn default() -> PostcardSentActivityType {
        PostcardSentActivityType::Noop
    }
}
impl PostcardSentActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PostcardSentActivityType::Noop)
    }
}

/// Activity feed item representing a time when a contact was sent a particular postcard.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostcardSent {
    /**
    * Activity feed item representing a time when a contact was sent a particular postcard.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<PostcardSentActivityType>,
    /**
    * Activity feed item representing a time when a contact was sent a particular postcard.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item representing a time when a contact was sent a particular postcard.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_id: String,
    /**
    * Activity feed item representing a time when a contact was sent a particular postcard.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_title: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SquatterSignupActivityType {
    #[serde(rename = "squatter_signup")]
    SquatterSignup,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SquatterSignupActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SquatterSignupActivityType::SquatterSignup => "squatter_signup",
            SquatterSignupActivityType::Noop => "",
            SquatterSignupActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SquatterSignupActivityType {
    fn default() -> SquatterSignupActivityType {
        SquatterSignupActivityType::Noop
    }
}
impl SquatterSignupActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SquatterSignupActivityType::Noop)
    }
}

/// Activity feed item to representing a contact signing up for the audience from a squatter page.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SquatterSignup {
    /**
    * Activity feed item to representing a contact signing up for the audience from a squatter page.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<SquatterSignupActivityType>,
    /**
    * Activity feed item to representing a contact signing up for the audience from a squatter page.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item to representing a contact signing up for the audience from a squatter page.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_id: String,
    /**
    * Activity feed item to representing a contact signing up for the audience from a squatter page.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_title: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum WebsiteSignupActivityType {
    #[serde(rename = "website_signup")]
    WebsiteSignup,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for WebsiteSignupActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            WebsiteSignupActivityType::WebsiteSignup => "website_signup",
            WebsiteSignupActivityType::Noop => "",
            WebsiteSignupActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for WebsiteSignupActivityType {
    fn default() -> WebsiteSignupActivityType {
        WebsiteSignupActivityType::Noop
    }
}
impl WebsiteSignupActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, WebsiteSignupActivityType::Noop)
    }
}

/// Activity feed item to representing a contact signing up for the contact through a website page.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebsiteSignup {
    /**
    * Activity feed item to representing a contact signing up for the contact through a website page.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<WebsiteSignupActivityType>,
    /**
    * Activity feed item to representing a contact signing up for the contact through a website page.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item to representing a contact signing up for the contact through a website page.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_id: String,
    /**
    * Activity feed item to representing a contact signing up for the contact through a website page.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_title: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LandingPageSignupActivityType {
    #[serde(rename = "landing_page_signup")]
    LandingPageSignup,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LandingPageSignupActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LandingPageSignupActivityType::LandingPageSignup => "landing_page_signup",
            LandingPageSignupActivityType::Noop => "",
            LandingPageSignupActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LandingPageSignupActivityType {
    fn default() -> LandingPageSignupActivityType {
        LandingPageSignupActivityType::Noop
    }
}
impl LandingPageSignupActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, LandingPageSignupActivityType::Noop)
    }
}

/// Activity feed item to representing a contact signing up for the list via a landing page.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LandingPageSignup {
    /**
    * Activity feed item to representing a contact signing up for the list via a landing page.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<LandingPageSignupActivityType>,
    /**
    * Activity feed item to representing a contact signing up for the list via a landing page.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item to representing a contact signing up for the list via a landing page.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_id: String,
    /**
    * Activity feed item to representing a contact signing up for the list via a landing page.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_title: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceSignupActivityType {
    #[serde(rename = "ecommerce_signup")]
    EcommerceSignup,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceSignupActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceSignupActivityType::EcommerceSignup => "ecommerce_signup",
            EcommerceSignupActivityType::Noop => "",
            EcommerceSignupActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceSignupActivityType {
    fn default() -> EcommerceSignupActivityType {
        EcommerceSignupActivityType::Noop
    }
}
impl EcommerceSignupActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceSignupActivityType::Noop)
    }
}

/// Activity feed item to representing a contact signing up for the list via a ecommerce store.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommerceSignup {
    /**
    * Activity feed item to representing a contact signing up for the list via a ecommerce store.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<EcommerceSignupActivityType>,
    /**
    * Activity feed item to representing a contact signing up for the list via a ecommerce store.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item to representing a contact signing up for the list via a ecommerce store.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_name: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GenericSignupActivityType {
    #[serde(rename = "generic_signup")]
    GenericSignup,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GenericSignupActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GenericSignupActivityType::GenericSignup => "generic_signup",
            GenericSignupActivityType::Noop => "",
            GenericSignupActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GenericSignupActivityType {
    fn default() -> GenericSignupActivityType {
        GenericSignupActivityType::Noop
    }
}
impl GenericSignupActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GenericSignupActivityType::Noop)
    }
}

/// Activity feed item that represents a contact signing up for the audience via a generic some generic method (specifically, one we can't link to).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GenericSignup {
    /**
    * Activity feed item that represents a contact signing up for the audience via a generic some generic method (specifically, one we can't link to).
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<GenericSignupActivityType>,
    /**
    * Activity feed item that represents a contact signing up for the audience via a generic some generic method (specifically, one we can't link to).
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item that represents a contact signing up for the audience via a generic some generic method (specifically, one we can't link to).
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signup_category: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EcommerceOrderActivityType {
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EcommerceOrderActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EcommerceOrderActivityType::Order => "order",
            EcommerceOrderActivityType::Noop => "",
            EcommerceOrderActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EcommerceOrderActivityType {
    fn default() -> EcommerceOrderActivityType {
        EcommerceOrderActivityType::Noop
    }
}
impl EcommerceOrderActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EcommerceOrderActivityType::Noop)
    }
}

/// Information about a specific order line.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Lines {
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub discount: f64,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_id: String,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_title: String,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_variant_id: String,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_variant_title: String,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

/// Activity feed item that represents an order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommerceOrder {
    /**
    * Activity feed item that represents an order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<EcommerceOrderActivityType>,
    /**
    * Activity feed item that represents an order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item that represents an order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_id: String,
    /**
    * Activity feed item that represents an order.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub order_items: Vec<Lines>,
    /**
    * Activity feed item that represents an order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_total: String,
    /**
    * Activity feed item that represents an order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_url: String,
    /**
    * Activity feed item that represents an order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_id: String,
    /**
    * Activity feed item that represents an order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_title: String,
    /**
    * Activity feed item that represents an order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub outreach_type: String,
    /**
    * Activity feed item that represents an order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_name: String,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ContactActivityEventType {
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ContactActivityEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ContactActivityEventType::Event => "event",
            ContactActivityEventType::Noop => "",
            ContactActivityEventType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ContactActivityEventType {
    fn default() -> ContactActivityEventType {
        ContactActivityEventType::Noop
    }
}
impl ContactActivityEventType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ContactActivityEventType::Noop)
    }
}

/// Activity feed item that represents a generic event.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContactActivityEvent {
    /**
    * Activity feed item that represents a generic event.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ContactActivityEventType>,
    /**
    * Activity feed item that represents a generic event.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Activity feed item that represents a generic event.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event_name: String,
    /**
    * Activity feed item that represents a generic event.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub event_properties: Vec<String>,
}

/**
* The type of event activity.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SurveyResponseActivityType {
    #[serde(rename = "survey_response")]
    SurveyResponse,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SurveyResponseActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SurveyResponseActivityType::SurveyResponse => "survey_response",
            SurveyResponseActivityType::Noop => "",
            SurveyResponseActivityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SurveyResponseActivityType {
    fn default() -> SurveyResponseActivityType {
        SurveyResponseActivityType::Noop
    }
}
impl SurveyResponseActivityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SurveyResponseActivityType::Noop)
    }
}

/// Represents when a contact completes and submits a survey
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SurveyResponse {
    /**
    * Represents when a contact completes and submits a survey
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<SurveyResponseActivityType>,
    /**
    * Represents when a contact completes and submits a survey
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Represents when a contact completes and submits a survey
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub survey_id: String,
    /**
    * Represents when a contact completes and submits a survey
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub survey_title: String,
}

/// All of the following types:
///
/// - `EmailOpens`
/// - `EmailClicks`
/// - `EmailBounced`
/// - `ListUnsubscribed`
/// - `EmailSent`
/// - `EmailConversation`
/// - `Note`
/// - `MarketingPermissionData`
/// - `PostcardSent`
/// - `SquatterSignup`
/// - `WebsiteSignup`
/// - `LandingPageSignup`
/// - `EcommerceSignup`
/// - `GenericSignup`
/// - `EcommerceOrder`
/// - `ContactActivityEvent`
/// - `SurveyResponse`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ActivityOneOf {
    /**
    * Activity feed item representing opening an email.
    */
    EmailOpens(EmailOpens),
    /**
    * Activity feed item representing having a link clicked by a contact.
    */
    EmailClicks(EmailClicks),
    /**
    * Activity feed item representing an email to this contact bouncing.
    */
    EmailBounced(EmailBounced),
    /**
    * Activity feed item representing this contact unsubscribing from a list.
    */
    ListUnsubscribed(ListUnsubscribed),
    /**
    * Activity feed item representing having an email sent to the contact.
    */
    EmailSent(EmailSent),
    /**
    * Activity feed item representing an individual reply in a conversation.
    */
    EmailConversation(EmailConversation),
    /**
    * Activity feed item representing a note on the contact record.
    */
    Note(Note),
    /**
    * Activity feed item indicating if a marketing permission was added or updated.
    */
    MarketingPermissionData(MarketingPermissionData),
    /**
    * Activity feed item representing a time when a contact was sent a particular postcard.
    */
    PostcardSent(PostcardSent),
    /**
    * Activity feed item to representing a contact signing up for the audience from a squatter page.
    */
    SquatterSignup(SquatterSignup),
    /**
    * Activity feed item to representing a contact signing up for the contact through a website page.
    */
    WebsiteSignup(WebsiteSignup),
    /**
    * Activity feed item to representing a contact signing up for the list via a landing page.
    */
    LandingPageSignup(LandingPageSignup),
    /**
    * Activity feed item to representing a contact signing up for the list via a ecommerce store.
    */
    EcommerceSignup(EcommerceSignup),
    /**
    * Activity feed item that represents a contact signing up for the audience via a generic some generic method (specifically, one we can't link to).
    */
    GenericSignup(GenericSignup),
    /**
    * Activity feed item that represents an order.
    */
    EcommerceOrder(EcommerceOrder),
    /**
    * Activity feed item that represents a generic event.
    */
    ContactActivityEvent(ContactActivityEvent),
    /**
    * Represents when a contact completes and submits a survey
    */
    SurveyResponse(SurveyResponse),
}

impl ActivityOneOf {
    pub fn contact_activity_event(&self) -> Option<&ContactActivityEvent> {
        if let ActivityOneOf::ContactActivityEvent(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn ecommerce_order(&self) -> Option<&EcommerceOrder> {
        if let ActivityOneOf::EcommerceOrder(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn ecommerce_signup(&self) -> Option<&EcommerceSignup> {
        if let ActivityOneOf::EcommerceSignup(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn email_bounced(&self) -> Option<&EmailBounced> {
        if let ActivityOneOf::EmailBounced(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn email_clicks(&self) -> Option<&EmailClicks> {
        if let ActivityOneOf::EmailClicks(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn email_conversation(&self) -> Option<&EmailConversation> {
        if let ActivityOneOf::EmailConversation(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn email_opens(&self) -> Option<&EmailOpens> {
        if let ActivityOneOf::EmailOpens(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn email_sent(&self) -> Option<&EmailSent> {
        if let ActivityOneOf::EmailSent(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn generic_signup(&self) -> Option<&GenericSignup> {
        if let ActivityOneOf::GenericSignup(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn landing_page_signup(&self) -> Option<&LandingPageSignup> {
        if let ActivityOneOf::LandingPageSignup(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn list_unsubscribed(&self) -> Option<&ListUnsubscribed> {
        if let ActivityOneOf::ListUnsubscribed(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn marketing_permission_data(&self) -> Option<&MarketingPermissionData> {
        if let ActivityOneOf::MarketingPermissionData(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn note(&self) -> Option<&Note> {
        if let ActivityOneOf::Note(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn postcard_sent(&self) -> Option<&PostcardSent> {
        if let ActivityOneOf::PostcardSent(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn squatter_signup(&self) -> Option<&SquatterSignup> {
        if let ActivityOneOf::SquatterSignup(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn survey_response(&self) -> Option<&SurveyResponse> {
        if let ActivityOneOf::SurveyResponse(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn website_signup(&self) -> Option<&WebsiteSignup> {
        if let ActivityOneOf::WebsiteSignup(ref_) = self {
            return Some(ref_);
        }
        None
    }
}

/// The member activity events for a given member.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MemberActivityEventsData {
    /**
    * The member activity events for a given member.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The member activity events for a given member.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub activity: Vec<ActivityOneOf>,
    /**
    * The member activity events for a given member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * The member activity events for a given member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
}

/// A list of tags assigned to a list member.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CollectionOfTags {
    /**
    * A list of tags assigned to a list member.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of tags assigned to a list member.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<String>,
    /**
    * A list of tags assigned to a list member.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* The status for the tag on the member, pass in active to add a tag or inactive to remove it.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MemberTagsTagStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MemberTagsTagStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MemberTagsTagStatus::Active => "active",
            MemberTagsTagStatus::Inactive => "inactive",
            MemberTagsTagStatus::Noop => "",
            MemberTagsTagStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MemberTagsTagStatus {
    fn default() -> MemberTagsTagStatus {
        MemberTagsTagStatus::Noop
    }
}
impl MemberTagsTagStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, MemberTagsTagStatus::Noop)
    }
}

/// Add or remove tags on a member by declaring a tag either active or inactive on a member.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MemberTag {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * The status for the tag on the member, pass in active to add a tag or inactive to remove it.
    */
    #[serde(default, skip_serializing_if = "MemberTagsTagStatus::is_noop")]
    pub status: MemberTagsTagStatus,
}

/// A list of tags assigned to a list member.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MemberTags {
    /**
    * A list of tags assigned to a list member.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_syncing: bool,
    /**
    * A list of tags assigned to the list member.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<MemberTag>,
}

/// A specific event for a contact.
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Event {
    /**
    * A specific event for a contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A specific event for a contact.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub occurred_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A specific event for a contact.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub properties: String,
}

/// A collection of events for a given contact
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CollectionOfEvents {
    /**
    * A collection of events for a given contact
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of events for a given contact
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub events: Vec<Event>,
    /**
    * A collection of events for a given contact
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A new event for a specific list member
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EventsData {
    /**
    * A new event for a specific list member
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_syncing: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A new event for a specific list member
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub occurred_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A new event for a specific list member
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub properties: String,
}

/// A single instance of a goal activity.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Goal {
    /**
    * A single instance of a goal activity.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub data: String,
    /**
    * A single instance of a goal activity.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    /**
    * A single instance of a goal activity.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub goal_id: i64,
    /**
    * A single instance of a goal activity.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_visited_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// The last 50 Goal events for a member on a specific list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CollectionOfMemberActivityEvents {
    /**
    * The last 50 Goal events for a member on a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The last 50 Goal events for a member on a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * The last 50 Goal events for a member on a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub goals: Vec<Goal>,
    /**
    * The last 50 Goal events for a member on a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * The last 50 Goal events for a member on a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* Returns notes sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetListsMembersNotesSortField {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "note_id")]
    NoteId,
    #[serde(rename = "updated_at")]
    UpdatedAt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetListsMembersNotesSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetListsMembersNotesSortField::CreatedAt => "created_at",
            GetListsMembersNotesSortField::NoteId => "note_id",
            GetListsMembersNotesSortField::UpdatedAt => "updated_at",
            GetListsMembersNotesSortField::Noop => "",
            GetListsMembersNotesSortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetListsMembersNotesSortField {
    fn default() -> GetListsMembersNotesSortField {
        GetListsMembersNotesSortField::Noop
    }
}
impl GetListsMembersNotesSortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetListsMembersNotesSortField::Noop)
    }
}

/// A specific note for a specific member.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CollectionOfNotesMember {
    /**
    * A specific note for a specific member.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A specific note for a specific member.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A specific note for a specific member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    /**
    * A specific note for a specific member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * A specific note for a specific member.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * A specific note for a specific member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A specific note for a specific member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    /**
    * A specific note for a specific member.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// The last 10 notes for a specific list member, based on date created.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CollectionOfNotes {
    /**
    * The last 10 notes for a specific list member, based on date created.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The last 10 notes for a specific list member, based on date created.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * The last 10 notes for a specific list member, based on date created.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * The last 10 notes for a specific list member, based on date created.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub notes: Vec<CollectionOfNotesMember>,
    /**
    * The last 10 notes for a specific list member, based on date created.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* The [type](https://mailchimp.com/help/manage-audience-signup-form-fields/#Audience_field_types) for the merge field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MergeFieldType {
    #[serde(rename = "address")]
    Address,
    #[serde(rename = "birthday")]
    Birthday,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "dropdown")]
    Dropdown,
    #[serde(rename = "imageurl")]
    Imageurl,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "zip")]
    Zip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MergeFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MergeFieldType::Address => "address",
            MergeFieldType::Birthday => "birthday",
            MergeFieldType::Date => "date",
            MergeFieldType::Dropdown => "dropdown",
            MergeFieldType::Imageurl => "imageurl",
            MergeFieldType::Number => "number",
            MergeFieldType::Phone => "phone",
            MergeFieldType::Radio => "radio",
            MergeFieldType::Text => "text",
            MergeFieldType::Url => "url",
            MergeFieldType::Zip => "zip",
            MergeFieldType::Noop => "",
            MergeFieldType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MergeFieldType {
    fn default() -> MergeFieldType {
        MergeFieldType::Noop
    }
}
impl MergeFieldType {
    pub fn is_noop(&self) -> bool {
        matches!(self, MergeFieldType::Noop)
    }
}

/// Extra options for some merge field types.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MergeFieldOptions {
    /**
    * Extra options for some merge field types.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub choices: Vec<String>,
    /**
    * Extra options for some merge field types.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub date_format: Option<chrono::NaiveDate>,
    /**
    * Extra options for some merge field types.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub default_country: i64,
    /**
    * Extra options for some merge field types.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_format: String,
    /**
    * Extra options for some merge field types.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub size: i64,
}

/// A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MergeField {
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_value: String,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub display_order: i64,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub help_text: String,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub merge_id: i64,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<MergeFieldOptions>,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub public: bool,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub required: bool,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag: String,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<MergeFieldType>,
}

/// The merge fields ([audience fields](https://mailchimp.com/help/getting-started-with-merge-tags/)) for an audience.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CollectionOfMergeFields {
    /**
    * The merge fields ([audience fields](https://mailchimp.com/help/getting-started-with-merge-tags/)) for an audience.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The merge fields ([audience fields](https://mailchimp.com/help/getting-started-with-merge-tags/)) for an audience.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * The merge fields ([audience fields](https://mailchimp.com/help/getting-started-with-merge-tags/)) for an audience.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub merge_fields: Vec<MergeField>,
    /**
    * The merge fields ([audience fields](https://mailchimp.com/help/getting-started-with-merge-tags/)) for an audience.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MergeFieldData {
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_value: String,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub display_order: i64,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub help_text: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<MergeFieldOptions>,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub public: bool,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub required: bool,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag: String,
    /**
    * The [type](https://mailchimp.com/help/manage-audience-signup-form-fields/#Audience_field_types) for the merge field.
    */
    #[serde(
        default,
        skip_serializing_if = "MergeFieldType::is_noop",
        rename = "type"
    )]
    pub type_: MergeFieldType,
}

/// Extra options for some merge field types.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MergeFieldOptionsData {
    /**
    * Extra options for some merge field types.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub choices: Vec<String>,
    /**
    * Extra options for some merge field types.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub date_format: Option<chrono::NaiveDate>,
    /**
    * Extra options for some merge field types.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub default_country: i64,
    /**
    * Extra options for some merge field types.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_format: String,
}

/// A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MergeFieldDataType {
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_value: String,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub display_order: i64,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub help_text: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<MergeFieldOptionsData>,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub public: bool,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub required: bool,
    /**
    * A merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag: String,
}

/// The events that can trigger the webhook and whether they are enabled.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListWebhooksEvents {
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub campaign: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub cleaned: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub profile: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub subscribe: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub unsubscribe: bool,
    /**
    * The events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub upemail: bool,
}

/// The possible sources of any events that can trigger the webhook and whether they are enabled.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListWebhooksSources {
    /**
    * The possible sources of any events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub admin: bool,
    /**
    * The possible sources of any events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub api: bool,
    /**
    * The possible sources of any events that can trigger the webhook and whether they are enabled.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub user: bool,
}

/// Webhook configured for the given list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListWebhooks {
    /**
    * Webhook configured for the given list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Webhook configured for the given list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<ListWebhooksEvents>,
    /**
    * Webhook configured for the given list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Webhook configured for the given list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Webhook configured for the given list.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<ListWebhooksSources>,
    /**
    * Webhook configured for the given list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Manage webhooks for a specific list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListWebhooksData {
    /**
    * Manage webhooks for a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Manage webhooks for a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Manage webhooks for a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * Manage webhooks for a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub webhooks: Vec<ListWebhooks>,
}

/**
* Image alignment.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ImageAlign {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ImageAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ImageAlign::Center => "center",
            ImageAlign::Left => "left",
            ImageAlign::None => "none",
            ImageAlign::Right => "right",
            ImageAlign::Noop => "",
            ImageAlign::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ImageAlign {
    fn default() -> ImageAlign {
        ImageAlign::Noop
    }
}
impl ImageAlign {
    pub fn is_noop(&self) -> bool {
        matches!(self, ImageAlign::Noop)
    }
}

/**
* Image border style.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ImageBorderStyle {
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "dotted")]
    Dotted,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "groove")]
    Groove,
    #[serde(rename = "inset")]
    Inset,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "outset")]
    Outset,
    #[serde(rename = "ridge")]
    Ridge,
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ImageBorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ImageBorderStyle::Dashed => "dashed",
            ImageBorderStyle::Dotted => "dotted",
            ImageBorderStyle::Double => "double",
            ImageBorderStyle::Groove => "groove",
            ImageBorderStyle::Inset => "inset",
            ImageBorderStyle::None => "none",
            ImageBorderStyle::Outset => "outset",
            ImageBorderStyle::Ridge => "ridge",
            ImageBorderStyle::Solid => "solid",
            ImageBorderStyle::Noop => "",
            ImageBorderStyle::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ImageBorderStyle {
    fn default() -> ImageBorderStyle {
        ImageBorderStyle::Noop
    }
}
impl ImageBorderStyle {
    pub fn is_noop(&self) -> bool {
        matches!(self, ImageBorderStyle::Noop)
    }
}

/**
* Image link target.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ImageTarget {
    #[serde(rename = "_blank")]
    Blank,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ImageTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ImageTarget::Blank => "_blank",
            ImageTarget::Null => "null",
            ImageTarget::Noop => "",
            ImageTarget::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ImageTarget {
    fn default() -> ImageTarget {
        ImageTarget::Noop
    }
}
impl ImageTarget {
    pub fn is_noop(&self) -> bool {
        matches!(self, ImageTarget::Noop)
    }
}

/// Options for customizing your signup form header.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Header {
    /**
    * Options for customizing your signup form header.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_align: Option<ImageAlign>,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_alt: String,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_border_color: String,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_border_style: Option<ImageBorderStyle>,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_border_width: String,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_height: String,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_link: String,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_target: Option<ImageTarget>,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_width: String,
    /**
    * Options for customizing your signup form header.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

/**
* The content section name.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Section {
    #[serde(rename = "signup_message")]
    SignupMessage,
    #[serde(rename = "signup_thank_you_title")]
    SignupThankYouTitle,
    #[serde(rename = "unsub_message")]
    UnsubMessage,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Section::SignupMessage => "signup_message",
            Section::SignupThankYouTitle => "signup_thank_you_title",
            Section::UnsubMessage => "unsub_message",
            Section::Noop => "",
            Section::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Section {
    fn default() -> Section {
        Section::Noop
    }
}
impl Section {
    pub fn is_noop(&self) -> bool {
        matches!(self, Section::Noop)
    }
}

/// Collection of Content for List Signup Forms.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Contents {
    /**
    * Collection of Content for List Signup Forms.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<Section>,
    /**
    * Collection of Content for List Signup Forms.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/**
* A string that identifies the element selector.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Selector {
    #[serde(rename = "body_background")]
    BodyBackground,
    #[serde(rename = "body_link_style")]
    BodyLinkStyle,
    #[serde(rename = "forms_buttons")]
    FormsButtons,
    #[serde(rename = "forms_buttons_hovered")]
    FormsButtonsHovered,
    #[serde(rename = "forms_errors")]
    FormsErrors,
    #[serde(rename = "forms_field_label")]
    FormsFieldLabel,
    #[serde(rename = "forms_field_text")]
    FormsFieldText,
    #[serde(rename = "forms_help_text")]
    FormsHelpText,
    #[serde(rename = "forms_required")]
    FormsRequired,
    #[serde(rename = "forms_required_legend")]
    FormsRequiredLegend,
    #[serde(rename = "monkey_rewards_badge")]
    MonkeyRewardsBadge,
    #[serde(rename = "page_background")]
    PageBackground,
    #[serde(rename = "page_header")]
    PageHeader,
    #[serde(rename = "page_outer_wrapper")]
    PageOuterWrapper,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Selector::BodyBackground => "body_background",
            Selector::BodyLinkStyle => "body_link_style",
            Selector::FormsButtons => "forms_buttons",
            Selector::FormsButtonsHovered => "forms_buttons_hovered",
            Selector::FormsErrors => "forms_errors",
            Selector::FormsFieldLabel => "forms_field_label",
            Selector::FormsFieldText => "forms_field_text",
            Selector::FormsHelpText => "forms_help_text",
            Selector::FormsRequired => "forms_required",
            Selector::FormsRequiredLegend => "forms_required_legend",
            Selector::MonkeyRewardsBadge => "monkey_rewards_badge",
            Selector::PageBackground => "page_background",
            Selector::PageHeader => "page_header",
            Selector::PageOuterWrapper => "page_outer_wrapper",
            Selector::Noop => "",
            Selector::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Selector {
    fn default() -> Selector {
        Selector::Noop
    }
}
impl Selector {
    pub fn is_noop(&self) -> bool {
        matches!(self, Selector::Noop)
    }
}

/// An option for Signup Form Styles.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AnOptionSignupFormStyles {
    /**
    * An option for Signup Form Styles.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub property: String,
    /**
    * An option for Signup Form Styles.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// Collection of Element style for List Signup Forms.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Styles {
    /**
    * Collection of Element style for List Signup Forms.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub options: Vec<AnOptionSignupFormStyles>,
    /**
    * Collection of Element style for List Signup Forms.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<Selector>,
}

/// List signup form.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SignupForm {
    /**
    * List signup form.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * List signup form.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub contents: Vec<Contents>,
    /**
    * List signup form.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Header>,
    /**
    * List signup form.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * List signup form.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signup_form_url: String,
    /**
    * List signup form.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub styles: Vec<Styles>,
}

/// List Signup Forms.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListSignupForms {
    /**
    * List Signup Forms.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * List Signup Forms.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * List Signup Forms.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub signup_forms: Vec<SignupForm>,
    /**
    * List Signup Forms.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// List signup form.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SignupFormData {
    /**
    * List signup form.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub contents: Vec<Contents>,
    /**
    * List signup form.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Header>,
    /**
    * List signup form.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub styles: Vec<Styles>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Locations {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cc: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub percent: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total: i64,
}

/// A summary of List's locations.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListLocations {
    /**
    * A summary of List's locations.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of List's locations.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A summary of List's locations.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub locations: Vec<Locations>,
    /**
    * A summary of List's locations.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* Returns files sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetAllLandingPagesSortField {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetAllLandingPagesSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetAllLandingPagesSortField::CreatedAt => "created_at",
            GetAllLandingPagesSortField::UpdatedAt => "updated_at",
            GetAllLandingPagesSortField::Noop => "",
            GetAllLandingPagesSortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetAllLandingPagesSortField {
    fn default() -> GetAllLandingPagesSortField {
        GetAllLandingPagesSortField::Noop
    }
}
impl GetAllLandingPagesSortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetAllLandingPagesSortField::Noop)
    }
}

/**
* The status of this landing page.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LandingPageStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "unpublished")]
    Unpublished,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LandingPageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LandingPageStatus::Draft => "draft",
            LandingPageStatus::Published => "published",
            LandingPageStatus::Unpublished => "unpublished",
            LandingPageStatus::Noop => "",
            LandingPageStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LandingPageStatus {
    fn default() -> LandingPageStatus {
        LandingPageStatus::Noop
    }
}
impl LandingPageStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, LandingPageStatus::Noop)
    }
}

/// The tracking settings applied to this landing page.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TrackingSettings {
    /**
    * The tracking settings applied to this landing page.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enable_restricted_data_processing: bool,
    /**
    * The tracking settings applied to this landing page.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub track_with_mailchimp: bool,
}

/// A summary of an individual landing page's settings and content.
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LandingPage {
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by_source: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<LandingPageStatus>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub template_id: i64,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<TrackingSettings>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub unpublished_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub web_id: i64,
}

/// A collection of landing pages.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAllLandingPagesResponse {
    /**
    * A collection of landing pages.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of landing pages.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub landing_pages: Vec<LandingPage>,
    /**
    * A collection of landing pages.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* The type of template the landing page has.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LandingPageTemplateType {
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "signup")]
    Signup,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LandingPageTemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            LandingPageTemplateType::Product => "product",
            LandingPageTemplateType::Signup => "signup",
            LandingPageTemplateType::Noop => "",
            LandingPageTemplateType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LandingPageTemplateType {
    fn default() -> LandingPageTemplateType {
        LandingPageTemplateType::Noop
    }
}
impl LandingPageTemplateType {
    pub fn is_noop(&self) -> bool {
        matches!(self, LandingPageTemplateType::Noop)
    }
}

/// A summary of an individual page's properties.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LandingPageData {
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub template_id: i64,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<TrackingSettings>,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<LandingPageTemplateType>,
}

/// A summary of an individual page's properties.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LandingPageDataType {
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * A summary of an individual page's properties.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<TrackingSettings>,
}

/// The HTML content for a landing page.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LandingPageContent {
    /**
    * The HTML content for a landing page.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * The HTML content for a landing page.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    /**
    * The HTML content for a landing page.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub json: String,
}

/// An object describing the bounce summary for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Bounces {
    /**
    * An object describing the bounce summary for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub hard_bounces: i64,
    /**
    * An object describing the bounce summary for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub soft_bounces: i64,
    /**
    * An object describing the bounce summary for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub syntax_errors: i64,
}

/// An object describing the forwards and forward activity for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Forwards {
    /**
    * An object describing the forwards and forward activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forwards_count: i64,
    /**
    * An object describing the forwards and forward activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forwards_opens: i64,
}

/// An object describing the open activity for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Opens {
    /**
    * An object describing the open activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_open: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * An object describing the open activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
    /**
    * An object describing the open activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens_total: i64,
    /**
    * An object describing the open activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
}

/// An object describing the click activity for the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Clicks {
    /**
    * An object describing the click activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * An object describing the click activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks_total: i64,
    /**
    * An object describing the click activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_click: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * An object describing the click activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks: i64,
    /**
    * An object describing the click activity for the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_subscriber_clicks: i64,
}

/// An object describing campaign engagement on Facebook.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookLikes {
    /**
    * An object describing campaign engagement on Facebook.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub facebook_likes: i64,
    /**
    * An object describing campaign engagement on Facebook.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipient_likes: i64,
    /**
    * An object describing campaign engagement on Facebook.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_likes: i64,
}

/// The average campaign statistics for your industry.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignReportsIndustryStats {
    /**
    * The average campaign statistics for your industry.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub abuse_rate: f64,
    /**
    * The average campaign statistics for your industry.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub bounce_rate: f64,
    /**
    * The average campaign statistics for your industry.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * The average campaign statistics for your industry.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
    /**
    * The average campaign statistics for your industry.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
    * The average campaign statistics for your industry.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unopen_rate: f64,
    /**
    * The average campaign statistics for your industry.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unsub_rate: f64,
}

/// The average campaign statistics for your list. This won't be present if we haven't calculated it yet for this list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ListStats {
    /**
    * The average campaign statistics for your list. This won't be present if we haven't calculated it yet for this list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * The average campaign statistics for your list. This won't be present if we haven't calculated it yet for this list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
    /**
    * The average campaign statistics for your list. This won't be present if we haven't calculated it yet for this list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub sub_rate: f64,
    /**
    * The average campaign statistics for your list. This won't be present if we haven't calculated it yet for this list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unsub_rate: f64,
}

/// Stats for Campaign A.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct A {
    /**
    * Stats for Campaign A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub abuse_reports: i64,
    /**
    * Stats for Campaign A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounces: i64,
    /**
    * Stats for Campaign A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forwards: i64,
    /**
    * Stats for Campaign A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forwards_opens: i64,
    /**
    * Stats for Campaign A.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_open: String,
    /**
    * Stats for Campaign A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * Stats for Campaign A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipient_clicks: i64,
    /**
    * Stats for Campaign A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
    /**
    * Stats for Campaign A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubs: i64,
}

/// Stats for Campaign B.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct B {
    /**
    * Stats for Campaign B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub abuse_reports: i64,
    /**
    * Stats for Campaign B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounces: i64,
    /**
    * Stats for Campaign B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forwards: i64,
    /**
    * Stats for Campaign B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub forwards_opens: i64,
    /**
    * Stats for Campaign B.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_open: String,
    /**
    * Stats for Campaign B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * Stats for Campaign B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipient_clicks: i64,
    /**
    * Stats for Campaign B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
    /**
    * Stats for Campaign B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubs: i64,
}

/// General stats about different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AbSplit {
    /**
    * General stats about different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a: Option<A>,
    /**
    * General stats about different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b: Option<B>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Timewarp {
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounces: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub gmt_offset: i64,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_click: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_open: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Timeseries {
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_sent: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipients_clicks: i64,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
}

/// The url and password for the [VIP report](https://mailchimp.com/help/share-a-campaign-report/).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShareReport {
    /**
    * The url and password for the [VIP report](https://mailchimp.com/help/share-a-campaign-report/).
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub share_password: String,
    /**
    * The url and password for the [VIP report](https://mailchimp.com/help/share-a-campaign-report/).
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub share_url: String,
}

/// E-Commerce stats for a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceReport {
    /**
    * E-Commerce stats for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * E-Commerce stats for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_orders: i64,
    /**
    * E-Commerce stats for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_revenue: f64,
    /**
    * E-Commerce stats for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_spent: f64,
}

/// Report details about a sent campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Reports {
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ab_split: Option<AbSplit>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub abuse_reports: i64,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounces: Option<Bounces>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_title: String,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clicks: Option<Clicks>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<DeliveryStatus>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<ECommerceReport>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_sent: i64,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facebook_likes: Option<FacebookLikes>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forwards: Option<Forwards>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry_stats: Option<CampaignReportsIndustryStats>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_name: String,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_stats: Option<ListStats>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opens: Option<Opens>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub preview_text: String,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub rss_last_send: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub send_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_report: Option<ShareReport>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subject_line: String,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub timeseries: Vec<Timeseries>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub timewarp: Vec<Timewarp>,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
    * Report details about a sent campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubscribed: i64,
}

/// A list of reports containing campaigns marked as Sent.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignReportsData {
    /**
    * A list of reports containing campaigns marked as Sent.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of reports containing campaigns marked as Sent.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub reports: Vec<Reports>,
    /**
    * A list of reports containing campaigns marked as Sent.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AbuseComplaint {
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * Details of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// A list of abuse complaints for a specific list.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AbuseComplaintsData {
    /**
    * A list of abuse complaints for a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of abuse complaints for a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub abuse_reports: Vec<AbuseComplaint>,
    /**
    * A list of abuse complaints for a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A list of abuse complaints for a specific list.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* The sentiment type for a feedback message.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AdviceType {
    #[serde(rename = "negative")]
    Negative,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "positive")]
    Positive,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AdviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AdviceType::Negative => "negative",
            AdviceType::Neutral => "neutral",
            AdviceType::Positive => "positive",
            AdviceType::Noop => "",
            AdviceType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AdviceType {
    fn default() -> AdviceType {
        AdviceType::Noop
    }
}
impl AdviceType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AdviceType::Noop)
    }
}

/// Campaign feedback details.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Advice {
    /**
    * Campaign feedback details.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Campaign feedback details.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
    * Campaign feedback details.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<AdviceType>,
}

/// A list of feedback based on a campaign's statistics.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignAdviceReport {
    /**
    * A list of feedback based on a campaign's statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of feedback based on a campaign's statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub advice: Vec<Advice>,
    /**
    * A list of feedback based on a campaign's statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A list of feedback based on a campaign's statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Stats for Group A.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupA {
    /**
    * Stats for Group A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_percentage_a: f64,
    /**
    * Stats for Group A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_clicks_a: i64,
    /**
    * Stats for Group A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unique_click_percentage_a: f64,
    /**
    * Stats for Group A.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks_a: i64,
}

/// Stats for Group B.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GroupB {
    /**
    * Stats for Group B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_percentage_b: f64,
    /**
    * Stats for Group B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_clicks_b: i64,
    /**
    * Stats for Group B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unique_click_percentage_b: f64,
    /**
    * Stats for Group B.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks_b: i64,
}

/// A breakdown of clicks by different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ABSplit {
    /**
    * A breakdown of clicks by different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a: Option<GroupA>,
    /**
    * A breakdown of clicks by different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b: Option<GroupB>,
}

/// A report of links clicked in a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UrlsClicked {
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ab_split: Option<ABSplit>,
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_percentage: f64,
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_click: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_clicks: i64,
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unique_click_percentage: f64,
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks: i64,
    /**
    * A report of links clicked in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A list of URLs and unique IDs included in HTML and plain-text versions of a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ClickDetailReport {
    /**
    * A list of URLs and unique IDs included in HTML and plain-text versions of a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of URLs and unique IDs included in HTML and plain-text versions of a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A list of URLs and unique IDs included in HTML and plain-text versions of a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * A list of URLs and unique IDs included in HTML and plain-text versions of a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub urls_clicked: Vec<UrlsClicked>,
}

/// A subscriber who clicked a specific URL in a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ClickDetailMember {
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contact_status: String,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url_id: String,
    /**
    * A subscriber who clicked a specific URL in a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// A collection of members who clicked on a specific link within a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ClickDetailMembers {
    /**
    * A collection of members who clicked on a specific link within a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of members who clicked on a specific link within a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A collection of members who clicked on a specific link within a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members: Vec<ClickDetailMember>,
    /**
    * A collection of members who clicked on a specific link within a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A summary of the interaction with the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenDetailReportActivityMember {
    /**
    * A summary of the interaction with the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/// A list of a member's opens activity in a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenActivity {
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contact_status: String,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub opens: Vec<OpenDetailReportActivityMember>,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens_count: i64,
    /**
    * A list of a member's opens activity in a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// A detailed report of any campaign emails that were opened by a list member.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenDetailReport {
    /**
    * A detailed report of any campaign emails that were opened by a list member.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A detailed report of any campaign emails that were opened by a list member.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A detailed report of any campaign emails that were opened by a list member.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members: Vec<OpenActivity>,
    /**
    * A detailed report of any campaign emails that were opened by a list member.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * A detailed report of any campaign emails that were opened by a list member.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_opens: i64,
}

/// A single email domain's performance
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Domains {
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bounces: i64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub bounces_pct: f64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub clicks_pct: f64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub delivered: i64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub emails_pct: f64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub emails_sent: i64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub opens_pct: f64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unsubs: i64,
    /**
    * A single email domain's performance
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unsubs_pct: f64,
}

/// Statistics for the top-performing email domains in a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DomainPerformance {
    /**
    * Statistics for the top-performing email domains in a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Statistics for the top-performing email domains in a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Statistics for the top-performing email domains in a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub domains: Vec<Domains>,
    /**
    * Statistics for the top-performing email domains in a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * Statistics for the top-performing email domains in a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_sent: i64,
}

/// An individual tweet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Statuses {
    /**
    * An individual tweet.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub datetime: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * An individual tweet.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_retweet: bool,
    /**
    * An individual tweet.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub screen_name: String,
    /**
    * An individual tweet.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    /**
    * An individual tweet.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status_id: String,
}

/// A summary of Twitter activity for a campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Twitter {
    /**
    * A summary of Twitter activity for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_tweet: String,
    /**
    * A summary of Twitter activity for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_tweet: String,
    /**
    * A summary of Twitter activity for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub retweets: i64,
    /**
    * A summary of Twitter activity for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub statuses: Vec<Statuses>,
    /**
    * A summary of Twitter activity for a campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub tweets: i64,
}

/// An individual click location.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EepurlActivityClickSummaryLocation {
    /**
    * An individual click location.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
    * An individual click location.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub region: String,
}

/// A summary of the click-throughs on the campaign's URL.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ClickSummary {
    /**
    * A summary of the click-throughs on the campaign's URL.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * A summary of the click-throughs on the campaign's URL.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub first_click: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of the click-throughs on the campaign's URL.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_click: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of the click-throughs on the campaign's URL.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub locations: Vec<EepurlActivityClickSummaryLocation>,
}

/// A single instance of a campaign referral.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Referrer {
    /**
    * A single instance of a campaign referral.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * A single instance of a campaign referral.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub first_click: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A single instance of a campaign referral.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_click: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A single instance of a campaign referral.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub referrer: String,
}

/// A summary of social activity for the campaign, tracked by EepURL.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EepurlActivity {
    /**
    * A summary of social activity for the campaign, tracked by EepURL.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of social activity for the campaign, tracked by EepURL.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A summary of social activity for the campaign, tracked by EepURL.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clicks: Option<ClickSummary>,
    /**
    * A summary of social activity for the campaign, tracked by EepURL.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub eepurl: String,
    /**
    * A summary of social activity for the campaign, tracked by EepURL.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub referrers: Vec<Referrer>,
    /**
    * A summary of social activity for the campaign, tracked by EepURL.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * A summary of social activity for the campaign, tracked by EepURL.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter: Option<Twitter>,
}

/// A summary of the interaction with the campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailActivityMember {
    /**
    * A summary of the interaction with the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    /**
    * A summary of the interaction with the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,
    /**
    * A summary of the interaction with the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of the interaction with the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
    * A summary of the interaction with the campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A list of a member's subscriber activity in a specific campaign, including opens, clicks, and bounces.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailActivity {
    /**
    * A list of a member's subscriber activity in a specific campaign, including opens, clicks, and bounces.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of a member's subscriber activity in a specific campaign, including opens, clicks, and bounces.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub activity: Vec<EmailActivityMember>,
    /**
    * A list of a member's subscriber activity in a specific campaign, including opens, clicks, and bounces.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A list of a member's subscriber activity in a specific campaign, including opens, clicks, and bounces.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * A list of a member's subscriber activity in a specific campaign, including opens, clicks, and bounces.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * A list of a member's subscriber activity in a specific campaign, including opens, clicks, and bounces.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A list of a member's subscriber activity in a specific campaign, including opens, clicks, and bounces.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
}

/// A list of member's subscriber activity in a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailActivityData {
    /**
    * A list of member's subscriber activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of member's subscriber activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A list of member's subscriber activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub emails: Vec<EmailActivity>,
    /**
    * A list of member's subscriber activity in a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenLocations {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country_code: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub region: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub region_name: String,
}

/// Top open locations for a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OpenLocationsData {
    /**
    * Top open locations for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Top open locations for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Top open locations for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub locations: Vec<OpenLocations>,
    /**
    * Top open locations for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A subscriber's status for a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SentTo {
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub absplit_group: String,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub gmt_offset: i64,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_open: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub open_count: i64,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    /**
    * A subscriber's status for a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// A list of subscribers who were sent a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SentData {
    /**
    * A list of subscribers who were sent a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of subscribers who were sent a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A list of subscribers who were sent a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub sent_to: Vec<SentTo>,
    /**
    * A list of subscribers who were sent a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A list of reports containing child campaigns for a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CampaignSubReports {
    /**
    * A list of reports containing child campaigns for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of reports containing child campaigns for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A list of reports containing child campaigns for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub reports: Vec<Reports>,
    /**
    * A list of reports containing child campaigns for a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A member who unsubscribed from a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Unsubscribes {
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_id: String,
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<serde_json::Value>,
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A member who unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vip: bool,
}

/// A list of members who have unsubscribed from a specific campaign.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UnsubscribesData {
    /**
    * A list of members who have unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list of members who have unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * A list of members who have unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * A list of members who have unsubscribed from a specific campaign.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub unsubscribes: Vec<Unsubscribes>,
}

/**
* Returns files sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetReportsEcommerceProductActivitySortField {
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "total_purchased")]
    TotalPurchased,
    #[serde(rename = "total_revenue")]
    TotalRevenue,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetReportsEcommerceProductActivitySortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetReportsEcommerceProductActivitySortField::Title => "title",
            GetReportsEcommerceProductActivitySortField::TotalPurchased => "total_purchased",
            GetReportsEcommerceProductActivitySortField::TotalRevenue => "total_revenue",
            GetReportsEcommerceProductActivitySortField::Noop => "",
            GetReportsEcommerceProductActivitySortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetReportsEcommerceProductActivitySortField {
    fn default() -> GetReportsEcommerceProductActivitySortField {
        GetReportsEcommerceProductActivitySortField::Noop
    }
}
impl GetReportsEcommerceProductActivitySortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetReportsEcommerceProductActivitySortField::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Products {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recommendation_purchased: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recommendation_total: i64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sku: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_purchased: f64,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_revenue: f64,
}

/// A collection of ecommerce products.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetReportsEcommerceProductActivityResponse {
    /**
    * A collection of ecommerce products.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of ecommerce products.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub products: Vec<Products>,
    /**
    * A collection of ecommerce products.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/**
* Returns user templates sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetTemplatesSortField {
    #[serde(rename = "date_created")]
    DateCreated,
    #[serde(rename = "date_edited")]
    DateEdited,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetTemplatesSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetTemplatesSortField::DateCreated => "date_created",
            GetTemplatesSortField::DateEdited => "date_edited",
            GetTemplatesSortField::Name => "name",
            GetTemplatesSortField::Noop => "",
            GetTemplatesSortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetTemplatesSortField {
    fn default() -> GetTemplatesSortField {
        GetTemplatesSortField::Noop
    }
}
impl GetTemplatesSortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetTemplatesSortField::Noop)
    }
}

/// Information about a specific template.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Templates {
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category: String,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date_edited: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub drag_and_drop: bool,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub edited_by: String,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub folder_id: String,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub responsive: bool,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub share_url: String,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub thumbnail: String,
    /**
    * Information about a specific template.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// A list an account's available templates.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TemplatesData {
    /**
    * A list an account's available templates.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A list an account's available templates.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub templates: Vec<Templates>,
    /**
    * A list an account's available templates.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Default content for a template.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TemplateDefaultContent {
    /**
    * Default content for a template.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Default content for a template.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sections: Option<serde_json::Value>,
}

/// The customer's address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Address {
    /**
    * The customer's address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
    * The customer's address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
    * The customer's address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
    * The customer's address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
    * The customer's address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country_code: String,
    /**
    * The customer's address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    /**
    * The customer's address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub province: String,
    /**
    * The customer's address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub province_code: String,
}

/// Information about a specific customer.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Customer {
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific customer.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub opt_in_status: bool,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub orders_count: i64,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_spent: f64,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/**
* The Mailchimp tracking code for the order. Uses the 'mc_tc' parameter in E-Commerce tracking URLs.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TrackingCode {
    #[serde(rename = "prec")]
    Prec,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TrackingCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TrackingCode::Prec => "prec",
            TrackingCode::Noop => "",
            TrackingCode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TrackingCode {
    fn default() -> TrackingCode {
        TrackingCode::Noop
    }
}
impl TrackingCode {
    pub fn is_noop(&self) -> bool {
        matches!(self, TrackingCode::Noop)
    }
}

/// The shipping address for the order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShippingAddress {
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country_code: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub latitude: f64,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub longitude: f64,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub province: String,
    /**
    * The shipping address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub province_code: String,
}

/// The billing address for the order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BillingAddress {
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country_code: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub latitude: f64,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub longitude: f64,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub province: String,
    /**
    * The billing address for the order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub province_code: String,
}

/**
* Type of discount. For free shipping set type to fixed
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OrdersPromosType {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OrdersPromosType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrdersPromosType::Fixed => "fixed",
            OrdersPromosType::Percentage => "percentage",
            OrdersPromosType::Noop => "",
            OrdersPromosType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrdersPromosType {
    fn default() -> OrdersPromosType {
        OrdersPromosType::Noop
    }
}
impl OrdersPromosType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrdersPromosType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Promos {
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount_discounted: f64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    /**
    * Type of discount. For free shipping set type to fixed
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<OrdersPromosType>,
}

/// The outreach associated with this order. For example, an email campaign or Facebook ad.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Outreach {
    /**
    * The outreach associated with this order. For example, an email campaign or Facebook ad.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * The outreach associated with this order. For example, an email campaign or Facebook ad.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * The outreach associated with this order. For example, an email campaign or Facebook ad.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub published_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The outreach associated with this order. For example, an email campaign or Facebook ad.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// Information about a specific order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Orders {
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub cancelled_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub discount_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub financial_status: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fulfillment_status: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub landing_site: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lines: Vec<Lines>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub order_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_url: String,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outreach: Option<Outreach>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub processed_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub promos: Vec<Promos>,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub shipping_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub tax_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_code: Option<TrackingCode>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
}

/// A collection of orders in an account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersData {
    /**
    * A collection of orders in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of orders in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub orders: Vec<Orders>,
    /**
    * A collection of orders in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// The store address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceStoresAddress {
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country_code: String,
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub latitude: f64,
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub longitude: f64,
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub province: String,
    /**
    * The store address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub province_code: String,
}

/// The Connected Site associated with the store.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceStoresConnectedSite {
    /**
    * The Connected Site associated with the store.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub site_foreign_id: String,
    /**
    * The Connected Site associated with the store.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_script: Option<Script>,
}

/// abandonedCart automation details.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AbandonedCart {
    /**
    * abandonedCart automation details.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * abandonedCart automation details.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_supported: bool,
    /**
    * abandonedCart automation details.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

/// abandonedBrowse automation details. abandonedBrowse is also known as Product Retargeting Email or Retarget Site Visitors on the web.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AbandonedBrowse {
    /**
    * abandonedBrowse automation details. abandonedBrowse is also known as Product Retargeting Email or Retarget Site Visitors on the web.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * abandonedBrowse automation details. abandonedBrowse is also known as Product Retargeting Email or Retarget Site Visitors on the web.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_supported: bool,
    /**
    * abandonedBrowse automation details. abandonedBrowse is also known as Product Retargeting Email or Retarget Site Visitors on the web.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

/// Details for the automations attached to this store.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceStoresAutomations {
    /**
    * Details for the automations attached to this store.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abandoned_browse: Option<AbandonedBrowse>,
    /**
    * Details for the automations attached to this store.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abandoned_cart: Option<AbandonedCart>,
}

/// An individual store in an account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Stores {
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * An individual store in an account.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ECommerceStoresAddress>,
    /**
    * An individual store in an account.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automations: Option<ECommerceStoresAutomations>,
    /**
    * An individual store in an account.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected_site: Option<ECommerceStoresConnectedSite>,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_syncing: bool,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub money_format: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub platform: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub primary_locale: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A collection of stores in the account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceStores {
    /**
    * A collection of stores in the account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of stores in the account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub stores: Vec<Stores>,
    /**
    * A collection of stores in the account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// An individual store in an account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceStore {
    /**
    * An individual store in an account.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ECommerceStoresAddress>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_syncing: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub money_format: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub platform: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub primary_locale: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
}

/// An individual store in an account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceStoreData {
    /**
    * An individual store in an account.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ECommerceStoresAddress>,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_syncing: bool,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub money_format: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub platform: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub primary_locale: String,
    /**
    * An individual store in an account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
}

/// Information about a specific cart line item.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceCartLineItem {
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_id: String,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_title: String,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_variant_id: String,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_variant_title: String,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

/// Information about a specific cart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Carts {
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub checkout_url: String,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * Information about a specific cart.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lines: Vec<ECommerceCartLineItem>,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub order_total: f64,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub tax_total: f64,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A collection of a store's carts.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CartsData {
    /**
    * A collection of a store's carts.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of a store's carts.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub carts: Vec<Carts>,
    /**
    * A collection of a store's carts.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of a store's carts.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceCustomer {
    /**
    * Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /**
    * Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
    * Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub opt_in_status: bool,
}

/// Information about a specific cart line item.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceCartLineItemData {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_id: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_variant_id: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

/// Information about a specific cart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceCart {
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub checkout_url: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
    */
    #[serde()]
    pub customer: ECommerceCustomer,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * An array of the cart's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lines: Vec<ECommerceCartLineItemData>,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub order_total: f64,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub tax_total: f64,
}

/// Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceCartCustomer {
    /**
    * Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /**
    * Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
    * Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
    * Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub opt_in_status: bool,
}

/// Information about a specific cart line item.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceCartLineItemDataType {
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_id: String,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_variant_id: String,
    /**
    * Information about a specific cart line item.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

/// Information about a specific cart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceCartData {
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub checkout_url: String,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * Information about a specific cart.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<ECommerceCartCustomer>,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lines: Vec<ECommerceCartLineItemDataType>,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub order_total: f64,
    /**
    * Information about a specific cart.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub tax_total: f64,
}

/// A collection of a cart's line items.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CartLines {
    /**
    * A collection of a cart's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of a cart's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cart_id: String,
    /**
    * A collection of a cart's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lines: Vec<ECommerceCartLineItem>,
    /**
    * A collection of a cart's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of a cart's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// A collection of the store's customers.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Customers {
    /**
    * A collection of the store's customers.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of the store's customers.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub customers: Vec<Customer>,
    /**
    * A collection of the store's customers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of the store's customers.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Information about a specific customer.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceCustomerData {
    /**
    * Information about a specific customer.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific customer.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub opt_in_status: bool,
}

/// Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceCustomerDataType {
    /**
    * Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /**
    * Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_address: String,
    /**
    * Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific customer. Orders for existing customers should include only the `id` parameter in the `customer` object body.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub opt_in_status: bool,
}

/**
* The target that the discount applies to.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Target {
    #[serde(rename = "per_item")]
    PerItem,
    #[serde(rename = "shipping")]
    Shipping,
    #[serde(rename = "total")]
    Total,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Target::PerItem => "per_item",
            Target::Shipping => "shipping",
            Target::Total => "total",
            Target::Noop => "",
            Target::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Target {
    fn default() -> Target {
        Target::Noop
    }
}
impl Target {
    pub fn is_noop(&self) -> bool {
        matches!(self, Target::Noop)
    }
}

/// Information about an Ecommerce Store's specific Promo Rule
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PromoRules {
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub ends_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub starts_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<OrdersPromosType>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
}

/// A collection of the store's promo rules.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PromoRulesData {
    /**
    * A collection of the store's promo rules.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of the store's promo rules.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub promo_rules: Vec<PromoRules>,
    /**
    * A collection of the store's promo rules.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of the store's promo rules.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Information about an Ecommerce Store's specific Promo Rule.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommercePromoRule {
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub ends_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub starts_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The target that the discount applies to.
    */
    #[serde(default, skip_serializing_if = "Target::is_noop")]
    pub target: Target,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Type of discount. For free shipping set type to fixed
    */
    #[serde(
        default,
        skip_serializing_if = "OrdersPromosType::is_noop",
        rename = "type"
    )]
    pub type_: OrdersPromosType,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
}

/// Information about an Ecommerce Store's specific Promo Rule.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommercePromoRuleData {
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub ends_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub starts_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<OrdersPromosType>,
    /**
    * Information about an Ecommerce Store's specific Promo Rule.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
}

/// Information about an Ecommerce Store's specific Promo Code
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PromoCodes {
    /**
    * Information about an Ecommerce Store's specific Promo Code
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about an Ecommerce Store's specific Promo Code
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    /**
    * Information about an Ecommerce Store's specific Promo Code
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Code
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
    * Information about an Ecommerce Store's specific Promo Code
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about an Ecommerce Store's specific Promo Code
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub redemption_url: String,
    /**
    * Information about an Ecommerce Store's specific Promo Code
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Code
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub usage_count: i64,
}

/// A collection of the store's promo codes.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PromoCodesData {
    /**
    * A collection of the store's promo codes.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of the store's promo codes.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub promo_codes: Vec<PromoCodes>,
    /**
    * A collection of the store's promo codes.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of the store's promo codes.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Information about an Ecommerce Store's specific Promo Code.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommercePromoCode {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub redemption_url: String,
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub usage_count: i64,
}

/// Information about an Ecommerce Store's specific Promo Code.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommercePromoCodeData {
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub redemption_url: String,
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about an Ecommerce Store's specific Promo Code.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub usage_count: i64,
}

/// A collection of orders in a store.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersDataType {
    /**
    * A collection of orders in a store.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of orders in a store.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub orders: Vec<Orders>,
    /**
    * A collection of orders in a store.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of orders in a store.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceOrderPromos {
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount_discounted: f64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    /**
    * Type of discount. For free shipping set type to fixed
    */
    #[serde(
        default,
        skip_serializing_if = "OrdersPromosType::is_noop",
        rename = "type"
    )]
    pub type_: OrdersPromosType,
}

/// Information about a specific order line.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceOrderLineItem {
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub discount: f64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_id: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_variant_id: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

/// The outreach associated with this order. For example, an email campaign or Facebook ad.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceOrderOutreach {
    /**
    * The outreach associated with this order. For example, an email campaign or Facebook ad.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

/// Information about a specific order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceOrder {
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub cancelled_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * Information about a specific customer. For existing customers include only the `id` parameter in the `customer` object body.
    */
    #[serde()]
    pub customer: ECommerceCustomer,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub discount_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub financial_status: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fulfillment_status: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub landing_site: String,
    /**
    * An array of the order's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lines: Vec<ECommerceOrderLineItem>,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub order_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_url: String,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outreach: Option<ECommerceOrderOutreach>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub processed_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub promos: Vec<ECommerceOrderPromos>,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub shipping_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub tax_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_code: Option<TrackingCode>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
}

/// Information about a specific order line.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceOrderLineItemData {
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub discount: f64,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_id: String,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_variant_id: String,
    /**
    * Information about a specific order line.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

/// Information about a specific order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceOrderData {
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub campaign_id: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub cancelled_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<ECommerceCartCustomer>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub discount_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub financial_status: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fulfillment_status: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub landing_site: String,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lines: Vec<ECommerceOrderLineItemData>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub order_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_url: String,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outreach: Option<ECommerceOrderOutreach>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub processed_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub promos: Vec<ECommerceOrderPromos>,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub shipping_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub tax_total: f64,
    /**
    * Information about a specific order.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_code: Option<TrackingCode>,
    /**
    * Information about a specific order.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
}

/// A collection of an order's line items.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrderLines {
    /**
    * A collection of an order's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of an order's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lines: Vec<Lines>,
    /**
    * A collection of an order's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_id: String,
    /**
    * A collection of an order's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of an order's line items.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Information about a specific product variant.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Variants {
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub backorders: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub inventory_quantity: i64,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sku: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

/// Information about a specific product image.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Images {
    /**
    * Information about a specific product image.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific product image.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific product image.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * Information about a specific product image.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub variant_ids: Vec<String>,
}

/// Information about a specific product.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceProduct {
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub handle: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub images: Vec<Images>,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub published_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub variants: Vec<Variants>,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vendor: String,
}

/// A collection of a store's products.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProductsData {
    /**
    * A collection of a store's products.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of a store's products.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub products: Vec<ECommerceProduct>,
    /**
    * A collection of a store's products.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of a store's products.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Information about a specific product image.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceProductImage {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * Information about a specific product image.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub variant_ids: Vec<String>,
}

/// Information about a specific product.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceProductData {
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub handle: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub images: Vec<ECommerceProductImage>,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub published_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * An array of the product's variants. At least one variant is required for each product. A variant can use the same `id` and `title` as the parent product.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub variants: Vec<ECommerceProductVariant>,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vendor: String,
}

/// Information about a specific product variant.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceProductVariantData {
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub backorders: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub inventory_quantity: i64,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sku: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * Information about a specific product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

/// Information about a specific product image.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceProductImageData {
    /**
    * Information about a specific product image.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * Information about a specific product image.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * Information about a specific product image.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub variant_ids: Vec<String>,
}

/// Information about a specific product.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ECommerceProductDataType {
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub handle: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub images: Vec<ECommerceProductImageData>,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub published_at_foreign: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub variants: Vec<ECommerceProductVariantData>,
    /**
    * Information about a specific product.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vendor: String,
}

/// A collection of a product's variants.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommerceProductVariants {
    /**
    * A collection of a product's variants.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of a product's variants.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_id: String,
    /**
    * A collection of a product's variants.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of a product's variants.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
    /**
    * A collection of a product's variants.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub variants: Vec<Variants>,
}

/// A collection of a product's images.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EcommerceProductImages {
    /**
    * A collection of a product's images.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of a product's images.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub images: Vec<Images>,
    /**
    * A collection of a product's images.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub product_id: String,
    /**
    * A collection of a product's images.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_id: String,
    /**
    * A collection of a product's images.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Results {
    /**
    * A summary of an individual campaign's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaign>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub snippet: String,
}

/// Campaigns and Snippets found for given search term.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Campaigns {
    /**
    * Campaigns and Snippets found for given search term.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Campaigns and Snippets found for given search term.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<Results>,
    /**
    * Campaigns and Snippets found for given search term.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Exact matches of the provided search query.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ExactMatches {
    /**
    * Exact matches of the provided search query.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members: Vec<ListMembersData>,
    /**
    * Exact matches of the provided search query.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Partial matches of the provided search query.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FullSearch {
    /**
    * Partial matches of the provided search query.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub members: Vec<ListMembersData>,
    /**
    * Partial matches of the provided search query.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// Members found for given search term
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MembersData {
    /**
    * Members found for given search term
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Members found for given search term
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_matches: Option<ExactMatches>,
    /**
    * Members found for given search term
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_search: Option<FullSearch>,
}

/// API health status.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ApiHealthStatus {
    /**
    * API health status.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub health_status: String,
}

/**
* Returns files sorted by the specified field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetAllFacebookAdsSortField {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "end_time")]
    EndTime,
    #[serde(rename = "updated_at")]
    UpdatedAt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetAllFacebookAdsSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            GetAllFacebookAdsSortField::CreatedAt => "created_at",
            GetAllFacebookAdsSortField::EndTime => "end_time",
            GetAllFacebookAdsSortField::UpdatedAt => "updated_at",
            GetAllFacebookAdsSortField::Noop => "",
            GetAllFacebookAdsSortField::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetAllFacebookAdsSortField {
    fn default() -> GetAllFacebookAdsSortField {
        GetAllFacebookAdsSortField::Noop
    }
}
impl GetAllFacebookAdsSortField {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetAllFacebookAdsSortField::Noop)
    }
}

/**
* Supported Campaign, Ad, Page type
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OutreachType {
    #[serde(rename = "absplit")]
    Absplit,
    #[serde(rename = "automation")]
    Automation,
    #[serde(rename = "autoresponder")]
    Autoresponder,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "reconfirm")]
    Reconfirm,
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "rss")]
    Rss,
    #[serde(rename = "survey")]
    Survey,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "variate")]
    Variate,
    #[serde(rename = "website")]
    Website,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OutreachType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OutreachType::Absplit => "absplit",
            OutreachType::Automation => "automation",
            OutreachType::Autoresponder => "autoresponder",
            OutreachType::Facebook => "facebook",
            OutreachType::Google => "google",
            OutreachType::Page => "page",
            OutreachType::Plaintext => "plaintext",
            OutreachType::Reconfirm => "reconfirm",
            OutreachType::Regular => "regular",
            OutreachType::Rss => "rss",
            OutreachType::Survey => "survey",
            OutreachType::Transactional => "transactional",
            OutreachType::Variate => "variate",
            OutreachType::Website => "website",
            OutreachType::Noop => "",
            OutreachType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OutreachType {
    fn default() -> OutreachType {
        OutreachType::Noop
    }
}
impl OutreachType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OutreachType::Noop)
    }
}

/**
* Campaign, Ad, or Page status
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OutreachStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "canceling")]
    Canceling,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "partialRejected")]
    PartialRejected,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "save")]
    Save,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "somepaused")]
    Somepaused,
    #[serde(rename = "unpublished")]
    Unpublished,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OutreachStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OutreachStatus::Active => "active",
            OutreachStatus::Canceled => "canceled",
            OutreachStatus::Canceling => "canceling",
            OutreachStatus::Completed => "completed",
            OutreachStatus::Disconnected => "disconnected",
            OutreachStatus::Draft => "draft",
            OutreachStatus::PartialRejected => "partialRejected",
            OutreachStatus::Paused => "paused",
            OutreachStatus::Pending => "pending",
            OutreachStatus::Published => "published",
            OutreachStatus::Rejected => "rejected",
            OutreachStatus::Save => "save",
            OutreachStatus::Schedule => "schedule",
            OutreachStatus::Sending => "sending",
            OutreachStatus::Sent => "sent",
            OutreachStatus::Somepaused => "somepaused",
            OutreachStatus::Unpublished => "unpublished",
            OutreachStatus::Noop => "",
            OutreachStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OutreachStatus {
    fn default() -> OutreachStatus {
        OutreachStatus::Noop
    }
}
impl OutreachStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, OutreachStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsReportSummaryEcommerce {
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub average_order_revenue: f64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_revenue: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsReportSummary {
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub conversion_rate: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<FacebookAdsReportSummaryEcommerce>,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub engagements: i64,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub impressions: f64,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub open_rate: f64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub opens: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub reach: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscriber_clicks: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscribes: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_sent: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_opens: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_visits: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub visits: i64,
}

/// List settings for the outreach
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsList {
    /**
    * List settings for the outreach
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * List settings for the outreach
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub list_is_active: bool,
    /**
    * List settings for the outreach
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_name: String,
    /**
    * List settings for the outreach
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub recipient_count: i64,
    /**
    * List settings for the outreach
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<SegmentOptions>,
    /**
    * List settings for the outreach
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub segment_text: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAds {
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub canceled_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_segment: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub published_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * List settings for the outreach
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<FacebookAdsList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<FacebookAdsReportSummary>,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub show_report: bool,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Campaign, Ad, or Page status
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OutreachStatus>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub thumbnail: String,
    /**
    * Supported Campaign, Ad, Page type
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<OutreachType>,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub web_id: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsData {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email_source_name: String,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub needs_attention: bool,
    /**
    * The date and time that the account was created in ISO 8601 format.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub paused_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub was_canceled_by_facebook: bool,
}

/// Channel settings
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Channel {
    /**
    * Channel settings
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fb_placement_audience: bool,
    /**
    * Channel settings
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub fb_placement_feed: bool,
    /**
    * Channel settings
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub ig_placement_feed: bool,
}

/// Check if this ad is connected to a facebook page
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsFeedback {
    /**
    * Check if this ad is connected to a facebook page
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub audience: String,
    /**
    * Check if this ad is connected to a facebook page
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub budget: String,
    /**
    * Check if this ad is connected to a facebook page
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compliance: String,
    /**
    * Check if this ad is connected to a facebook page
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
}

/// Connected Site
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Site {
    /**
    * Connected Site
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
    * Connected Site
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * Connected Site
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
* Type of the audience
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FacebookAdsAudienceType {
    #[serde(rename = "Custom Audience")]
    CustomAudience,
    #[serde(rename = "Interest-based Audience")]
    InterestBasedAudience,
    #[serde(rename = "Lookalike Audience")]
    LookalikeAudience,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FacebookAdsAudienceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FacebookAdsAudienceType::CustomAudience => "Custom Audience",
            FacebookAdsAudienceType::InterestBasedAudience => "Interest-based Audience",
            FacebookAdsAudienceType::LookalikeAudience => "Lookalike Audience",
            FacebookAdsAudienceType::Noop => "",
            FacebookAdsAudienceType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FacebookAdsAudienceType {
    fn default() -> FacebookAdsAudienceType {
        FacebookAdsAudienceType::Noop
    }
}
impl FacebookAdsAudienceType {
    pub fn is_noop(&self) -> bool {
        matches!(self, FacebookAdsAudienceType::Noop)
    }
}

/**
* List or Facebook based audience
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SourceType {
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SourceType::Facebook => "facebook",
            SourceType::List => "list",
            SourceType::Noop => "",
            SourceType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SourceType {
    fn default() -> SourceType {
        SourceType::Noop
    }
}
impl SourceType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SourceType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmailSource {
    /**
    * Whether the webhook is triggered when a list subscriber is added.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_segment: bool,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_name: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub segment_type: String,
    /**
    * The name of the folder.
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
pub struct FacebookAdsAudienceTargetingSpecsLocations {
    /**
    * A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub cities: Vec<String>,
    /**
    * A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub countries: Vec<String>,
    /**
    * A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub regions: Vec<String>,
    /**
    * A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub zips: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsAudienceTargetingSpecsInterests {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TargetingSpecs {
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub gender: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub interests: Vec<FacebookAdsAudienceTargetingSpecsInterests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<FacebookAdsAudienceTargetingSpecsLocations>,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub max_age: i64,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub min_age: i64,
}

/// Audience settings
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Audience {
    /**
    * Audience settings
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_source: Option<EmailSource>,
    /**
    * Audience settings
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub include_source_in_target: bool,
    /**
    * Audience settings
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lookalike_country_code: String,
    /**
    * Audience settings
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceType>,
    /**
    * Audience settings
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targeting_specs: Option<TargetingSpecs>,
    /**
    * Audience settings
    */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<FacebookAdsAudienceType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Budget {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub duration: i64,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_amount: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Attachments {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub call_to_action: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub link_url: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Content {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub attachments: Vec<Attachments>,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub call_to_action: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub link_url: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// A facebook ad.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsDataType {
    /**
    * A facebook ad.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<Audience>,
    /**
    * A facebook ad.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<Budget>,
    /**
    * A facebook ad.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    /**
    * A facebook ad.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    /**
    * A facebook ad.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback: Option<FacebookAdsFeedback>,
    /**
    * A facebook ad.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_audience: bool,
    /**
    * A facebook ad.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_content: bool,
    /**
    * A facebook ad.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_connected: bool,
    /**
    * A facebook ad.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetFacebookAdsResponse {
    /**
    * A list of link types and descriptions for the API schema documents.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
}

/// All of the following types are flattened into one object:
///
/// - `FacebookAds`
/// - `FacebookAdsData`
/// - `FacebookAdsDataType`
/// - `GetFacebookAdsResponse`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsAllOf {
    #[serde(flatten)]
    pub facebook_ads: FacebookAds,
    #[serde(flatten)]
    pub facebook_ads_data: FacebookAdsData,
    /**
    * A facebook ad.
    */
    #[serde(flatten)]
    pub facebook_ads_data_type: FacebookAdsDataType,
    #[serde(flatten)]
    pub get_facebook_ads_response: GetFacebookAdsResponse,
}

/// Contains an array of facebook ads.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetAllFacebookAdsResponse {
    /**
    * Contains an array of facebook ads.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * Contains an array of facebook ads.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub facebook_ads: Vec<FacebookAdsAllOf>,
    /**
    * Contains an array of facebook ads.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsReportSummaryEcommerceData {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_revenue: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CostPerClick {
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ExtendedAt {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub datetime: String,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
}

/// Report summary of facebook ad
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsReportSummaryData {
    /**
    * Report summary of facebook ad
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_daily_budget: Option<CostPerClick>,
    /**
    * Report summary of facebook ad
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_order_amount: Option<CostPerClick>,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub click_rate: f64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub comments: i64,
    /**
    * Report summary of facebook ad
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_per_click: Option<CostPerClick>,
    /**
    * Report summary of facebook ad
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<FacebookAdsReportSummaryEcommerceData>,
    /**
    * Report summary of facebook ad
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_at: Option<ExtendedAt>,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub first_time_buyers: i64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_extended_ad_duration: bool,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub impressions: i64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub likes: i64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub reach: i64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub return_on_investment: f64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub shares: i64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_orders: i64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_products_sold: i64,
    /**
    * Report summary of facebook ad
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_clicks: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsAudienceActivityClicks {
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Impressions {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub impressions: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Revenue {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub revenue: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AudienceActivity {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub clicks: Vec<FacebookAdsAudienceActivityClicks>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub impressions: Vec<Impressions>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub revenue: Vec<Revenue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FacebookAdsDataTypeLinksObject {
    /**
    * Audience settings
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<Audience>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience_activity: Option<AudienceActivity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<Budget>,
    /**
    * Channel settings
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    /**
    * Report summary of facebook ad
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<FacebookAdsReportSummaryData>,
}

/// All of the following types are flattened into one object:
///
/// - `FacebookAds`
/// - `FacebookAdsData`
/// - `GetFacebookAdsResponse`
/// - `FacebookAdsDataTypeLinksObject`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetReportingFacebookAdsResponseAllOf {
    #[serde(flatten)]
    pub facebook_ads: FacebookAds,
    #[serde(flatten)]
    pub facebook_ads_data: FacebookAdsData,
    #[serde(flatten)]
    pub get_facebook_ads_response: GetFacebookAdsResponse,
    #[serde(flatten)]
    pub facebook_ads_data_type_links_object: FacebookAdsDataTypeLinksObject,
}

/// A collection of Facebook ads.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetReportingFacebookAdsResponse {
    /**
    * A collection of Facebook ads.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of Facebook ads.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub facebook_ads: Vec<GetReportingFacebookAdsResponseAllOf>,
    /**
    * A collection of Facebook ads.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Visits {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub val: i64,
}

/// The clicks and visits data from the last seven days.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DailyStats {
    /**
    * The clicks and visits data from the last seven days.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub clicks: Vec<Visits>,
    /**
    * The clicks and visits data from the last seven days.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub unique_visits: Vec<Visits>,
    /**
    * The clicks and visits data from the last seven days.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub visits: Vec<Visits>,
}

/// The clicks and visits data from the last five weeks.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WeeklyStats {
    /**
    * The clicks and visits data from the last five weeks.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub clicks: Vec<Visits>,
    /**
    * The clicks and visits data from the last five weeks.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub unique_visits: Vec<Visits>,
    /**
    * The clicks and visits data from the last five weeks.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub visits: Vec<Visits>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LandingPageReportTimeseries {
    /**
    * The clicks and visits data from the last seven days.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_stats: Option<DailyStats>,
    /**
    * The clicks and visits data from the last five weeks.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_stats: Option<WeeklyStats>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LandingPageReportEcommerce {
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub average_order_revenue: f64,
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency_code: String,
    /**
    * The display order for interests.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_orders: i64,
    /**
    * The price of a product variant.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub total_revenue: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Tag {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub tag_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag_name: String,
}

/// A summary of an individual landing page's settings and content.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LandingPages {
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub clicks: i64,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub conversion_rate: f64,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<LandingPageReportEcommerce>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_id: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub list_name: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub signup_tags: Vec<Tag>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscribes: i64,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeseries: Option<LandingPageReportTimeseries>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub unique_visits: i64,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub unpublished_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub visits: i64,
    /**
    * A summary of an individual landing page's settings and content.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub web_id: i64,
}

/// A collection of landing pages.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetReportingLandingPagesResponse {
    /**
    * A collection of landing pages.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "_links"
    )]
    pub links: Vec<Links>,
    /**
    * A collection of landing pages.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub landing_pages: Vec<LandingPages>,
    /**
    * A collection of landing pages.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// The verified domains currently on the account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VerifiedDomains {
    /**
    * The verified domains currently on the account.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub authenticated: bool,
    /**
    * The verified domains currently on the account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
    * The verified domains currently on the account.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub verification_email: String,
    /**
    * The verified domains currently on the account.
    */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub verification_sent: Option<chrono::DateTime<chrono::Utc>>,
    /**
    * The verified domains currently on the account.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub verified: bool,
}

/// Submit a response to the verification challenge and verify a domain for sending.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VerifyADomainSending {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
}

/// The verified domains currently on the account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VerifiedDomainsData {
    /**
    * The verified domains currently on the account.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub domains: Vec<VerifiedDomains>,
    /**
    * The verified domains currently on the account.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_items: i64,
}

/// The verified domains currently on the account.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct VerifiedDomainsDataType {
    /**
    * The name of the folder.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub verification_email: String,
}
