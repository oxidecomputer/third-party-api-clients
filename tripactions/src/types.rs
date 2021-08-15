//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TravelerType {
    #[serde(rename = "GUEST")]
    Guest,
    #[serde(rename = "PASSENGER")]
    Passenger,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TravelerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TravelerType::Guest => "GUEST",
            TravelerType::Passenger => "PASSENGER",
            TravelerType::Noop => "",
            TravelerType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TravelerType {
    fn default() -> TravelerType {
        TravelerType::Noop
    }
}
impl TravelerType {
    pub fn is_noop(&self) -> bool {
        matches!(self, TravelerType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum BookingStatus {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "TICKETED")]
    Ticketed,
    #[serde(rename = "VOIDED")]
    Voided,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            BookingStatus::Accepted => "ACCEPTED",
            BookingStatus::Canceled => "CANCELED",
            BookingStatus::Confirmed => "CONFIRMED",
            BookingStatus::Ticketed => "TICKETED",
            BookingStatus::Voided => "VOIDED",
            BookingStatus::Noop => "",
            BookingStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for BookingStatus {
    fn default() -> BookingStatus {
        BookingStatus::Noop
    }
}
impl BookingStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, BookingStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum BookingType {
    #[serde(rename = "BLACK_CAR")]
    BlackCar,
    #[serde(rename = "CAR")]
    Car,
    #[serde(rename = "FLIGHT")]
    Flight,
    #[serde(rename = "HOTEL")]
    Hotel,
    #[serde(rename = "RAIL")]
    Rail,
    #[serde(rename = "TRANSPORTATION")]
    Transportation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for BookingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            BookingType::BlackCar => "BLACK_CAR",
            BookingType::Car => "CAR",
            BookingType::Flight => "FLIGHT",
            BookingType::Hotel => "HOTEL",
            BookingType::Rail => "RAIL",
            BookingType::Transportation => "TRANSPORTATION",
            BookingType::Noop => "",
            BookingType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for BookingType {
    fn default() -> BookingType {
        BookingType::Noop
    }
}
impl BookingType {
    pub fn is_noop(&self) -> bool {
        matches!(self, BookingType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PaymentSchedule {
    #[serde(rename = "LATER")]
    Later,
    #[serde(rename = "NOW")]
    Now,
    #[serde(rename = "PARTIAL")]
    Partial,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PaymentSchedule::Later => "LATER",
            PaymentSchedule::Now => "NOW",
            PaymentSchedule::Partial => "PARTIAL",
            PaymentSchedule::Unknown => "UNKNOWN",
            PaymentSchedule::Noop => "",
            PaymentSchedule::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PaymentSchedule {
    fn default() -> PaymentSchedule {
        PaymentSchedule::Noop
    }
}
impl PaymentSchedule {
    pub fn is_noop(&self) -> bool {
        matches!(self, PaymentSchedule::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PassengerStatus {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PassengerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PassengerStatus::Disabled => "DISABLED",
            PassengerStatus::Enabled => "ENABLED",
            PassengerStatus::Noop => "",
            PassengerStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PassengerStatus {
    fn default() -> PassengerStatus {
        PassengerStatus::Noop
    }
}
impl PassengerStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, PassengerStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Location {
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub airport_code: Option<serde_json::Value>,
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
    pub state: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Segment {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub airline_alliance: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arrival: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure: Option<Location>,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub flight_number: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hotel_chain: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub provider_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub provider_name: String,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Person {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub billable_entity: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cost_center: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub department: String,
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
    pub employeed_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub manager_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub manager_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub manager_uuid: String,
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
    pub region: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subsidiary: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Passenger {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PassengerStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traveler_type: Option<TravelerType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Property {
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
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BookingReportResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<BookingReport>,
    #[serde(default)]
    pub page: Page,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Page {
    /**
     * Nearest airport code to this location
     */
    #[serde()]
    pub current_page: serde_json::Value,
    /**
     * Nearest airport code to this location
     */
    #[serde()]
    pub page_size: serde_json::Value,
    /**
     * Nearest airport code to this location
     */
    #[serde()]
    pub total_elements: serde_json::Value,
    /**
     * Nearest airport code to this location
     */
    #[serde()]
    pub total_pages: serde_json::Value,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Cnr {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cnr_codes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub published_price: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BookingReport {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub airline_credit_card_surcharge: f64,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_changed_at: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub approver_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub approver_reason: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub base_price: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bill_to_client: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booker: Option<Person>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub booking_duration: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub booking_fee: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub booking_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub booking_method: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub booking_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booking_type: Option<BookingType>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cabin: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cancellation_reason: String,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<serde_json::Value>,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carbon_emissions: Option<serde_json::Value>,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carbon_offset_cost: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnr: Option<Cnr>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_office: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_payment_method: String,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub corporate_discount_used: String,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub currency: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub currency_exhange_rate_from_usd: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_fields: Vec<Property>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<Location>,
    /**
     * Local date when the booking starts, e.g. checkin date for hotel, date of depart for flight
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub etickets: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub exchange_amount: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub exchange_fee: f64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub expensed: bool,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub extras_fees: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fare_class: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub flight: String,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flight_miles: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub grand_total: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub gst: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub hst: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub inventory: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub invoice: String,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub lead_time_in_days: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name_on_credit_card: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub net_charge: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub optimal_price: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Location>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub out_of_policy: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub out_of_policy_description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub out_of_policy_violation_types: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub out_of_policy_violations: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passengers: Vec<Passenger>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payment_credit_card_type_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payment_method: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payment_method_used: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<PaymentSchedule>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pcc: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pdf: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub preferred_vendor: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub projects: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub purpose: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub qst: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub resort_fee: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub saving: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub saving_missed: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub seats: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub seats_fee: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<Segment>,
    /**
     * Local date when the booking starts, e.g. checkin date for hotel, date of depart for flight
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statement_description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub tax: f64,
    /**
     * Nearest airport code to this location
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub train_miles: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub travel_spend: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub trip_bucks_earned: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub trip_bucks_earned_usd: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trip_description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub trip_fee: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trip_length: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trip_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trip_uuids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unitary_price: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub usd_grand_total: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub vat: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vendor: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub vip_fee: f64,
}
