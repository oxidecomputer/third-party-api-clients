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
        match self {
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
        match self {
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
        match self {
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
        match self {
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
        match self {
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
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "airportCode"
    )]
    pub airport_code: String,
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "airlineAlliance"
    )]
    pub airline_alliance: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arrival: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure: Option<Location>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "endTimestamp"
    )]
    pub end_timestamp: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "flightNumber"
    )]
    pub flight_number: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "hotelChain"
    )]
    pub hotel_chain: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "providerCode"
    )]
    pub provider_code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "providerName"
    )]
    pub provider_name: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "startTimestamp"
    )]
    pub start_timestamp: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Person {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "billableEntity"
    )]
    pub billable_entity: String,
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "employeedId"
    )]
    pub employeed_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "managerEmail"
    )]
    pub manager_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "managerName"
    )]
    pub manager_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "managerUuid"
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "travelerType"
    )]
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
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub data: Vec<BookingReport>,
    #[serde(default)]
    pub page: Page,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Page {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "currentPage"
    )]
    pub current_page: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "pageSize"
    )]
    pub page_size: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalElements"
    )]
    pub total_elements: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalPages"
    )]
    pub total_pages: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Cnr {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "cnrCodes"
    )]
    pub cnr_codes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "publishedPrice"
    )]
    pub published_price: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BookingReport {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "airlineCreditCardSurcharge"
    )]
    pub airline_credit_card_surcharge: f64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "approvalChangedAt"
    )]
    pub approval_changed_at: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "approverEmail"
    )]
    pub approver_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "approverReason"
    )]
    pub approver_reason: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "basePrice"
    )]
    pub base_price: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "billToClient"
    )]
    pub bill_to_client: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booker: Option<Person>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "bookingDuration"
    )]
    pub booking_duration: i64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "bookingFee"
    )]
    pub booking_fee: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "bookingId"
    )]
    pub booking_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "bookingMethod"
    )]
    pub booking_method: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "bookingStatus"
    )]
    pub booking_status: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bookingType"
    )]
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "cancellationReason"
    )]
    pub cancellation_reason: String,
    /**
     * Time at which the object was created.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "cancelledAt"
    )]
    pub cancelled_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "carbonEmissions"
    )]
    pub carbon_emissions: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "carbonOffsetCost"
    )]
    pub carbon_offset_cost: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnr: Option<Cnr>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "companyOffice"
    )]
    pub company_office: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "companyPaymentMethod"
    )]
    pub company_payment_method: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "confirmationNumber"
    )]
    pub confirmation_number: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "corporateDiscountUsed"
    )]
    pub corporate_discount_used: String,
    /**
     * Time at which the object was created.
     */
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
    pub currency: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "currencyExhangeRateFromUsd"
    )]
    pub currency_exhange_rate_from_usd: f64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "customFields"
    )]
    pub custom_fields: Vec<Property>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<Location>,
    /**
     * Local date when the booking starts, e.g. checkin date for hotel, date of depart for flight
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize",
        rename = "endDate"
    )]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub etickets: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "exchangeAmount"
    )]
    pub exchange_amount: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "exchangeFee"
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
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "extrasFees"
    )]
    pub extras_fees: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "fareClass"
    )]
    pub fare_class: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub flight: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "flightMiles"
    )]
    pub flight_miles: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "grandTotal"
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
     * Time at which the object was created.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastModified"
    )]
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "leadTimeInDays"
    )]
    pub lead_time_in_days: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "nameOnCreditCard"
    )]
    pub name_on_credit_card: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "netCharge"
    )]
    pub net_charge: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "optimalPrice"
    )]
    pub optimal_price: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Location>,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "outOfPolicy"
    )]
    pub out_of_policy: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "outOfPolicyDescription"
    )]
    pub out_of_policy_description: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "outOfPolicyViolationTypes"
    )]
    pub out_of_policy_violation_types: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "outOfPolicyViolations"
    )]
    pub out_of_policy_violations: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub passengers: Vec<Passenger>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "paymentCreditCardTypeName"
    )]
    pub payment_credit_card_type_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "paymentMethod"
    )]
    pub payment_method: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "paymentMethodUsed"
    )]
    pub payment_method_used: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "paymentSchedule"
    )]
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "preferredVendor"
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
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "resortFee"
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
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "savingMissed"
    )]
    pub saving_missed: f64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub seats: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "seatsFee"
    )]
    pub seats_fee: f64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub segments: Vec<Segment>,
    /**
     * Local date when the booking starts, e.g. checkin date for hotel, date of depart for flight
     */
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
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "statementDescription"
    )]
    pub statement_description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub tax: f64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trainMiles"
    )]
    pub train_miles: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "travelSpend"
    )]
    pub travel_spend: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "tripBucksEarned"
    )]
    pub trip_bucks_earned: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "tripBucksEarnedUsd"
    )]
    pub trip_bucks_earned_usd: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "tripDescription"
    )]
    pub trip_description: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "tripFee"
    )]
    pub trip_fee: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "tripLength"
    )]
    pub trip_length: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "tripName"
    )]
    pub trip_name: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "tripUuids"
    )]
    pub trip_uuids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "unitaryPrice"
    )]
    pub unitary_price: f64,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "usdGrandTotal"
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
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "vipFee"
    )]
    pub vip_fee: f64,
}
