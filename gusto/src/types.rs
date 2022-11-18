//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The representation of an employee in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Employee {
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub company_id: f64,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_uuid: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub custom_fields: Vec<EmployeeCustomField>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub date_of_birth: Option<chrono::NaiveDate>,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub department: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub eligible_paid_time_off: Vec<PaidTimeOff>,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub garnishments: Vec<Garnishment>,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub has_ssn: bool,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_address: Option<Location>,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub jobs: Vec<Job>,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub manager_id: f64,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub middle_initial: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub onboarded: bool,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub preferred_first_name: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssn: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub terminated: bool,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub terminations: Vec<Termination>,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub two_percent_shareholder: bool,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
    /**
     * The representation of an employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub work_email: String,
}

/// The representation of an address in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Location {
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub company_id: i64,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub employee_id: i64,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub filing_address: bool,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub mailing_address: bool,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_number: String,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_1: String,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_2: String,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
    /**
     * The representation of an address in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// The representation of paid time off in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PaidTimeOff {
    /**
     * The representation of paid time off in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub accrual_balance: String,
    /**
     * The representation of paid time off in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub accrual_period: String,
    /**
     * The representation of paid time off in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub accrual_rate: String,
    /**
     * The representation of paid time off in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub accrual_unit: String,
    /**
     * The representation of paid time off in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub maximum_accrual_balance: String,
    /**
     * The representation of paid time off in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The representation of paid time off in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub paid_at_termination: bool,
}

/// Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Garnishment {
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub annual_maximum: f64,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub court_ordered: bool,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub deduct_as_percentage: bool,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub employee_id: i64,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub pay_period_maximum: f64,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub recurring: bool,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub times: i64,
    /**
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

/// The representation of a termination in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Termination {
    /**
     * The representation of a termination in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub effective_date: Option<chrono::NaiveDate>,
    /**
     * The representation of a termination in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub employee_id: i64,
    /**
     * The representation of a termination in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The representation of a termination in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub run_termination_payroll: bool,
    /**
     * The representation of a termination in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

/**
* The unit accompanying the compensation rate. If the employee is an owner, rate should be 'Paycheck'.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PaymentUnit {
    #[serde(rename = "Hour")]
    Hour,
    #[serde(rename = "Month")]
    Month,
    #[serde(rename = "Paycheck")]
    Paycheck,
    #[serde(rename = "Week")]
    Week,
    #[serde(rename = "Year")]
    Year,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PaymentUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentUnit::Hour => "Hour",
            PaymentUnit::Month => "Month",
            PaymentUnit::Paycheck => "Paycheck",
            PaymentUnit::Week => "Week",
            PaymentUnit::Year => "Year",
            PaymentUnit::Noop => "",
            PaymentUnit::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PaymentUnit {
    fn default() -> PaymentUnit {
        PaymentUnit::Noop
    }
}
impl PaymentUnit {
    pub fn is_noop(&self) -> bool {
        matches!(self, PaymentUnit::Noop)
    }
}

/**
* The FLSA status for this compensation. Salaried ('Exempt') employees are paid a fixed salary every pay period. Salaried with overtime ('Salaried Nonexempt') employees are paid a fixed salary every pay period, and receive overtime pay when applicable. Hourly ('Nonexempt') employees are paid for the hours they work, and receive overtime pay when applicable. Owners ('Owner') are employees that own at least twenty percent of the company.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum FlsaStatus {
    #[serde(rename = "Exempt")]
    Exempt,
    #[serde(rename = "Nonexempt")]
    Nonexempt,
    #[serde(rename = "Owner")]
    Owner,
    #[serde(rename = "Salaried Nonexempt")]
    SalariedNonexempt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FlsaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlsaStatus::Exempt => "Exempt",
            FlsaStatus::Nonexempt => "Nonexempt",
            FlsaStatus::Owner => "Owner",
            FlsaStatus::SalariedNonexempt => "Salaried Nonexempt",
            FlsaStatus::Noop => "",
            FlsaStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FlsaStatus {
    fn default() -> FlsaStatus {
        FlsaStatus::Noop
    }
}
impl FlsaStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, FlsaStatus::Noop)
    }
}

/// The representation of compensation in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Compensation {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub effective_date: Option<chrono::NaiveDate>,
    /**
     * The representation of compensation in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flsa_status: Option<FlsaStatus>,
    /**
     * The representation of compensation in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The representation of compensation in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub job_id: i64,
    /**
     * The representation of compensation in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_unit: Option<PaymentUnit>,
    /**
     * The representation of compensation in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rate: String,
    /**
     * The representation of compensation in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct JobLocation {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     * Whether the employee is terminated.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub inactive: bool,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_1: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_2: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// The representation of a job in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Job {
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub compensations: Vec<Compensation>,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub current_compensation_id: i64,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub employee_id: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub hire_date: Option<chrono::NaiveDate>,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<JobLocation>,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub location_id: i64,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payment_unit: String,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub primary: bool,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rate: String,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * The representation of a job in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

/// The representation of an admin user in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Admin {
    /**
     * The representation of an admin user in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The representation of an admin user in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The representation of an admin user in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
}

/**
* The tax payer type of the company.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EntityType {
    #[serde(rename = "Association")]
    Association,
    #[serde(rename = "C-Corporation")]
    CCorporation,
    #[serde(rename = "Co-ownership")]
    CoOwnership,
    #[serde(rename = "General partnership")]
    GeneralPartnership,
    #[serde(rename = "Joint venture")]
    JointVenture,
    #[serde(rename = "LLC")]
    Llc,
    #[serde(rename = "LLP")]
    Llp,
    #[serde(rename = "Limited partnership")]
    LimitedPartnership,
    #[serde(rename = "Non-Profit")]
    NonProfit,
    #[serde(rename = "S-Corporation")]
    SCorporation,
    #[serde(rename = "Sole proprietor")]
    SoleProprietor,
    #[serde(rename = "Trusteeship")]
    Trusteeship,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Association => "Association",
            EntityType::CCorporation => "C-Corporation",
            EntityType::CoOwnership => "Co-ownership",
            EntityType::GeneralPartnership => "General partnership",
            EntityType::JointVenture => "Joint venture",
            EntityType::Llc => "LLC",
            EntityType::Llp => "LLP",
            EntityType::LimitedPartnership => "Limited partnership",
            EntityType::NonProfit => "Non-Profit",
            EntityType::SCorporation => "S-Corporation",
            EntityType::SoleProprietor => "Sole proprietor",
            EntityType::Trusteeship => "Trusteeship",
            EntityType::Noop => "",
            EntityType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EntityType {
    fn default() -> EntityType {
        EntityType::Noop
    }
}
impl EntityType {
    pub fn is_noop(&self) -> bool {
        matches!(self, EntityType::Noop)
    }
}

/**
* The Gusto product tier of the company.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Tier {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "concierge")]
    Concierge,
    #[serde(rename = "contractor_only")]
    ContractorOnly,
    #[serde(rename = "core")]
    Core,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tier::Basic => "basic",
            Tier::Complete => "complete",
            Tier::Concierge => "concierge",
            Tier::ContractorOnly => "contractor_only",
            Tier::Core => "core",
            Tier::Noop => "",
            Tier::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Tier {
    fn default() -> Tier {
        Tier::Noop
    }
}
impl Tier {
    pub fn is_noop(&self) -> bool {
        matches!(self, Tier::Noop)
    }
}

/**
* The status of the company in Gusto. "Approved" companies may run payroll with Gusto. "Not Approved" companies may not yet run payroll with Gusto. In order to run payroll, the company may need to complete onboarding or contact support. "Suspended" companies may not run payroll with Gusto. In order to unsuspend their account, the company must contact support.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CompanyStatus {
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Not Approved")]
    NotApproved,
    #[serde(rename = "Suspended")]
    Suspended,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CompanyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompanyStatus::Approved => "Approved",
            CompanyStatus::NotApproved => "Not Approved",
            CompanyStatus::Suspended => "Suspended",
            CompanyStatus::Noop => "",
            CompanyStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CompanyStatus {
    fn default() -> CompanyStatus {
        CompanyStatus::Noop
    }
}
impl CompanyStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, CompanyStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Hourly {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub multiple: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Fixed {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// The available company-wide compensation rates for the company.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Compensations {
    /**
     * The available company-wide compensation rates for the company.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fixed: Vec<Fixed>,
    /**
     * The available company-wide compensation rates for the company.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub hourly: Vec<Hourly>,
    /**
     * The available company-wide compensation rates for the company.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub paid_time_off: Vec<Fixed>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct HomeAddress {
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
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_1: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_2: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// The primary signatory of the company.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PrimarySignatory {
    /**
     * The primary signatory of the company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The primary signatory of the company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The primary signatory of the company.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_address: Option<HomeAddress>,
    /**
     * The primary signatory of the company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The primary signatory of the company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub middle_initial: String,
    /**
     * The primary signatory of the company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
}

/// The primary payroll admin of the company.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PrimaryPayrollAdmin {
    /**
     * The primary payroll admin of the company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The primary payroll admin of the company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The primary payroll admin of the company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The primary payroll admin of the company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
}

/// The representation of a company in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Company {
    /**
     * The representation of a company in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_status: Option<CompanyStatus>,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensations: Option<Compensations>,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ein: String,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_suspended: bool,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub locations: Vec<Location>,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_payroll_admin: Option<PrimaryPayrollAdmin>,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_signatory: Option<PrimarySignatory>,
    /**
     * The Gusto product tier of the company.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trade_name: String,
    /**
     * The representation of a company in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
}

/**
* The contractor's wage type, either "Fixed" or "Hourly".
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum WageType {
    #[serde(rename = "Fixed")]
    Fixed,
    #[serde(rename = "Hourly")]
    Hourly,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for WageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WageType::Fixed => "Fixed",
            WageType::Hourly => "Hourly",
            WageType::Noop => "",
            WageType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for WageType {
    fn default() -> WageType {
        WageType::Noop
    }
}
impl WageType {
    pub fn is_noop(&self) -> bool {
        matches!(self, WageType::Noop)
    }
}

/**
* The contractor's type, either "Individual" or "Business".
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Type {
    #[serde(rename = "Business")]
    Business,
    #[serde(rename = "Individual")]
    Individual,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Business => "Business",
            Type::Individual => "Individual",
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

/// The contractor’s home address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Address {
    /**
     * The contractor’s home address.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * The contractor’s home address.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * The contractor’s home address.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * The contractor’s home address.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_1: String,
    /**
     * The contractor’s home address.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_2: String,
    /**
     * The contractor’s home address.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// The representation of a contractor (individual or business) in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Contractor {
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub business_name: String,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub company_id: f64,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ein: String,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hourly_rate: String,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_active: bool,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub middle_initial: String,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
    /**
     * The representation of a contractor (individual or business) in Gusto.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wage_type: Option<WageType>,
}

/**
* The payment method.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PaymentMethod {
    #[serde(rename = "Check")]
    Check,
    #[serde(rename = "Correction Payment")]
    CorrectionPayment,
    #[serde(rename = "Direct Deposit")]
    DirectDeposit,
    #[serde(rename = "Historical Payment")]
    HistoricalPayment,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethod::Check => "Check",
            PaymentMethod::CorrectionPayment => "Correction Payment",
            PaymentMethod::DirectDeposit => "Direct Deposit",
            PaymentMethod::HistoricalPayment => "Historical Payment",
            PaymentMethod::Noop => "",
            PaymentMethod::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PaymentMethod {
    fn default() -> PaymentMethod {
        PaymentMethod::Noop
    }
}
impl PaymentMethod {
    pub fn is_noop(&self) -> bool {
        matches!(self, PaymentMethod::Noop)
    }
}

/// The representation of a single contractor payment.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContractorPayment {
    /**
     * The representation of a single contractor payment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bonus: String,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub contractor_id: f64,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hourly_rate: String,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hours: String,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reimbursement: String,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub wage: String,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub wage_total: String,
    /**
     * The representation of a single contractor payment.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wage_type: Option<WageType>,
}

/// The wage and reimbursement totals for all contractor payments within a given time period.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Total {
    /**
     * The wage and reimbursement totals for all contractor payments within a given time period.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reimbursements: String,
    /**
     * The wage and reimbursement totals for all contractor payments within a given time period.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub wages: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContractorPayments {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub contractor_id: f64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub payments: Vec<ContractorPayment>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reimbursement_total: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub wage_total: String,
}

/// The representation of the summary of contractor payments for a given company in a given time period.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ContractorPaymentSummary {
    /**
     * The representation of the summary of contractor payments for a given company in a given time period.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub contractor_payments: Vec<ContractorPayments>,
    /**
     * The representation of the summary of contractor payments for a given company in a given time period.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
}

/**
* The status of the time off request.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Status {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Approved => "approved",
            Status::Denied => "denied",
            Status::Pending => "pending",
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
* The type of time off request.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum RequestType {
    #[serde(rename = "sick")]
    Sick,
    #[serde(rename = "vacation")]
    Vacation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestType::Sick => "sick",
            RequestType::Vacation => "vacation",
            RequestType::Noop => "",
            RequestType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RequestType {
    fn default() -> RequestType {
        RequestType::Noop
    }
}
impl RequestType {
    pub fn is_noop(&self) -> bool {
        matches!(self, RequestType::Noop)
    }
}

/// An object that represents the days in the time off request. The keys of the object are the dates, formatted as a YYYY-MM-DD string. The values of the object are the number of hours requested off for each day, formatted as a string representation of a numeric decimal to the thousands place.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Days {}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeOffRequestEmployee {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Initiator {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

/// This value will be null if the request has not been approved.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Approver {
    /**
     * This value will be null if the request has not been approved.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    /**
     * This value will be null if the request has not been approved.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

/// The representation of a time off request.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeOffRequest {
    /**
     * This value will be null if the request has not been approved.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<Approver>,
    /**
     * The representation of a time off request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<Days>,
    /**
     * The representation of a time off request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee: Option<TimeOffRequestEmployee>,
    /**
     * The representation of a time off request.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_note: String,
    /**
     * The representation of a time off request.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employer_note: String,
    /**
     * The representation of a time off request.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    /**
     * The representation of a time off request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<RequestType>,
    /**
     * The representation of a time off request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Companies {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub locations: Vec<Location>,
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
    pub trade_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PayrollAdmin {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub companies: Vec<Companies>,
}

/// An object containing each of the user's permissions.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Roles {
    /**
     * An object containing each of the user's permissions.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_admin: Option<PayrollAdmin>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CurrentUser {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Roles>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Frequency {
    #[serde(rename = "Every other week")]
    EveryOtherWeek,
    #[serde(rename = "Every week")]
    EveryWeek,
    #[serde(rename = "Monthly")]
    Monthly,
    #[serde(rename = "Twice per month")]
    TwicePerMonth,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequency::EveryOtherWeek => "Every other week",
            Frequency::EveryWeek => "Every week",
            Frequency::Monthly => "Monthly",
            Frequency::TwicePerMonth => "Twice per month",
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

/// The representation of a pay schedule.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PaySchedule {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub anchor_pay_date: Option<chrono::NaiveDate>,
    /**
     * The representation of a pay schedule.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub auto_pilot: bool,
    /**
     * The representation of a pay schedule.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub day_1: i64,
    /**
     * The representation of a pay schedule.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub day_2: i64,
    /**
     * The representation of a pay schedule.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    /**
     * The representation of a pay schedule.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * The representation of a pay schedule.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The representation of a pay schedule.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
}

/**
* Bank account type
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AccountType {
    #[serde(rename = "Checking")]
    Checking,
    #[serde(rename = "Savings")]
    Savings,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Checking => "Checking",
            AccountType::Savings => "Savings",
            AccountType::Noop => "",
            AccountType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AccountType {
    fn default() -> AccountType {
        AccountType::Noop
    }
}
impl AccountType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AccountType::Noop)
    }
}

/**
* The verification status of the bank account.
*   
*   'awaiting_deposits' means the bank account is just created and money is being transferred.
*   'ready_for_verification' means the micro-deposits are completed and the verification process can begin by using the verify endpoint.
*   'verified' means the bank account is verified.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum VerificationStatus {
    #[serde(rename = "awaiting_deposits")]
    AwaitingDeposits,
    #[serde(rename = "ready_for_verification")]
    ReadyForVerification,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationStatus::AwaitingDeposits => "awaiting_deposits",
            VerificationStatus::ReadyForVerification => "ready_for_verification",
            VerificationStatus::Verified => "verified",
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

/// The company bank account
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CompanyBankAccount {
    /**
     * The company bank account
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    /**
     * The company bank account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_uuid: String,
    /**
     * The company bank account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hidden_account_number: String,
    /**
     * The company bank account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub routing_number: String,
    /**
     * The company bank account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
    /**
     * The company bank account
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<VerificationStatus>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SupportedBenefit {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     *
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub healthcare: bool,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     *
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub imputed: bool,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     *
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub posttax: bool,
    /**
     *
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub pretax: bool,
    /**
     *
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub retirement: bool,
    /**
     *
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub yearly_limit: bool,
}

/// The representation of a company benefit.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CompanyBenefit {
    /**
     * The representation of a company benefit.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    /**
     * The representation of a company benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub benefit_id: f64,
    /**
     * The representation of a company benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub company_id: f64,
    /**
     * The representation of a company benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The representation of a company benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     * The representation of a company benefit.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "responsible_for_employee_w2"
    )]
    pub responsible_for_employee_w_2: bool,
    /**
     * The representation of a company benefit.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub responsible_for_employer_taxes: bool,
    /**
     * The representation of a company benefit.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub supports_percentage_amounts: bool,
    /**
     * The representation of a company benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EarningType {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
}

/**
* Whether the employee deduction reduces taxable income or not. Only valid for Group Term Life benefits. Note: when the value is not "unset", coverage amount and coverage salary multiplier are ignored.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DeductionReducesTaxableIncome {
    #[serde(rename = "does_not_reduce_taxable_income")]
    DoesNotReduceTaxableIncome,
    #[serde(rename = "reduces_taxable_income")]
    ReducesTaxableIncome,
    #[serde(rename = "unset")]
    Unset,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DeductionReducesTaxableIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeductionReducesTaxableIncome::DoesNotReduceTaxableIncome => {
                "does_not_reduce_taxable_income"
            }
            DeductionReducesTaxableIncome::ReducesTaxableIncome => "reduces_taxable_income",
            DeductionReducesTaxableIncome::Unset => "unset",
            DeductionReducesTaxableIncome::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DeductionReducesTaxableIncome {
    fn default() -> DeductionReducesTaxableIncome {
        DeductionReducesTaxableIncome::Unset
    }
}

/// The representation of an employee benefit.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmployeeBenefit {
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub catch_up: bool,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub company_benefit_id: f64,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_contribution: String,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_contribution_annual_maximum: String,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub contribute_as_percentage: bool,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub coverage_amount: String,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub coverage_salary_multiplier: String,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub deduct_as_percentage: bool,
    /**
     * Whether the employee deduction reduces taxable income or not. Only valid for Group Term Life benefits. Note: when the value is not "unset", coverage amount and coverage salary multiplier are ignored.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduction_reduces_taxable_income: Option<DeductionReducesTaxableIncome>,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_deduction: String,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_deduction_annual_maximum: String,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub employee_id: f64,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub limit_option: String,
    /**
     * The representation of an employee benefit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EligibleEmployees {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub id: f64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub job_ids: Vec<f64>,
}

/// Information about the payroll for the pay period.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Payroll {
    /**
     * Information about the payroll for the pay period.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payroll_deadline: String,
    /**
     * Information about the payroll for the pay period.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub processed: bool,
}

/// The representation of a pay period.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PayPeriod {
    /**
     * The representation of a pay period.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub eligible_employees: Vec<EligibleEmployees>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub end_date: Option<chrono::NaiveDate>,
    /**
     * The representation of a pay period.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub pay_schedule_id: f64,
    /**
     * The representation of a pay period.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pay_schedule_uuid: String,
    /**
     * The representation of a pay period.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll: Option<Payroll>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PayrollPayPeriod {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub pay_schedule_id: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pay_schedule_uuid: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
}

/// The subtotals for the payroll.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Totals {
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub additional_earnings: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub benefits: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub check_amount: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub child_support_debit: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_debit: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deferred_payroll_taxes: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_benefits_deductions: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_bonuses: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_cash_tips: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_commissions: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_paycheck_tips: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_taxes: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employer_taxes: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gross_pay: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub net_pay: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub net_pay_debit: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub owners_draw: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reimbursement_debit: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reimbursements: String,
    /**
     * The subtotals for the payroll.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tax_debit: String,
}

/**
* The employee's compensation payment method. This value is only available for processed payrolls.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PayrollEmployeeCompensationsPaymentMethod {
    #[serde(rename = "Check")]
    Check,
    #[serde(rename = "Direct Deposit")]
    DirectDeposit,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PayrollEmployeeCompensationsPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PayrollEmployeeCompensationsPaymentMethod::Check => "Check",
            PayrollEmployeeCompensationsPaymentMethod::DirectDeposit => "Direct Deposit",
            PayrollEmployeeCompensationsPaymentMethod::Noop => "",
            PayrollEmployeeCompensationsPaymentMethod::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PayrollEmployeeCompensationsPaymentMethod {
    fn default() -> PayrollEmployeeCompensationsPaymentMethod {
        PayrollEmployeeCompensationsPaymentMethod::Noop
    }
}
impl PayrollEmployeeCompensationsPaymentMethod {
    pub fn is_noop(&self) -> bool {
        matches!(self, PayrollEmployeeCompensationsPaymentMethod::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FixedCompensations {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub amount: String,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub job_id: f64,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct HourlyCompensations {
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub compensation_multiplier: f64,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hours: String,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub job_id: f64,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PayrollEmployeeCompensationsPaidTimeOff {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hours: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Benefits {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_contribution: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_deduction: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub imputed: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Deductions {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub amount: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Taxes {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub amount: String,
    /**
     * Whether the employee is terminated.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub employer: bool,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmployeeCompensations {
    /**
     * An array of employee benefits for the pay period. Benefits are only included for processed payroll when the include parameter is present.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub benefits: Vec<Benefits>,
    /**
     * An array of employee deductions for the pay period. Deductions are only included for processed payroll when the include parameter is present.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub deductions: Vec<Deductions>,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub employee_id: f64,
    /**
     * An array of fixed compensations for the employee. Fixed compensations include tips, bonuses, and one time reimbursements. If this payroll has been procesed, only fixed compensations with a value greater than 0.00 are returned. For an unprocess payroll, all active fixed compensations are returned.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fixed_compensations: Vec<FixedCompensations>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gross_pay: String,
    /**
     * An array of hourly compensations for the employee. Hourly compensations include regular, overtime, and double overtime hours. If this payroll has been procesed, only hourly compensations with a value greater than 0.00 are returned. For an unprocess payroll, all active hourly compensations are returned.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub hourly_compensations: Vec<HourlyCompensations>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub net_pay: String,
    /**
     * An array of all paid time off the employee is eligible for this pay period.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub paid_time_off: Vec<PayrollEmployeeCompensationsPaidTimeOff>,
    /**
     * The employee's compensation payment method. This value is only available for processed payrolls.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PayrollEmployeeCompensationsPaymentMethod>,
    /**
     * An array of employer and employee taxes for the pay period. Taxes are only included for processed payroll when the include parameter is present.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub taxes: Vec<Taxes>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PayrollData {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub calculated_at: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub check_date: Option<chrono::NaiveDate>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub company_id: f64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_uuid: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub employee_compensations: Vec<EmployeeCompensations>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period: Option<PayrollPayPeriod>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payroll_deadline: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub payroll_id: f64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payroll_uuid: String,
    /**
     *
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub processed: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub processed_date: Option<chrono::NaiveDate>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totals: Option<Totals>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CustomFieldType {
    #[serde(rename = "currency")]
    Currency,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CustomFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomFieldType::Currency => "currency",
            CustomFieldType::Date => "date",
            CustomFieldType::Number => "number",
            CustomFieldType::Radio => "radio",
            CustomFieldType::Text => "text",
            CustomFieldType::Noop => "",
            CustomFieldType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CustomFieldType {
    fn default() -> CustomFieldType {
        CustomFieldType::Noop
    }
}
impl CustomFieldType {
    pub fn is_noop(&self) -> bool {
        matches!(self, CustomFieldType::Noop)
    }
}

/// A custom field of an employee
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmployeeCustomField {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_custom_field_id: String,
    /**
     * A custom field of an employee
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A custom field of an employee
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub selection_options: Vec<String>,
    #[serde(rename = "type")]
    pub type_: CustomFieldType,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// A custom field on a company
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CompanyCustomField {
    /**
     * A custom field on a company
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A custom field on a company
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub selection_options: Vec<String>,
    #[serde(rename = "type")]
    pub type_: CustomFieldType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GustoPersonType {
    #[serde(rename = "Candidate")]
    Candidate,
    #[serde(rename = "Contractor")]
    Contractor,
    #[serde(rename = "Employee")]
    Employee,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GustoPersonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GustoPersonType::Candidate => "Candidate",
            GustoPersonType::Contractor => "Contractor",
            GustoPersonType::Employee => "Employee",
            GustoPersonType::Noop => "",
            GustoPersonType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GustoPersonType {
    fn default() -> GustoPersonType {
        GustoPersonType::Noop
    }
}
impl GustoPersonType {
    pub fn is_noop(&self) -> bool {
        matches!(self, GustoPersonType::Noop)
    }
}

/// The representation of a job applicant in Gusto.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct JobApplicant {
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub company_id: f64,
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_uuid: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub date_of_birth: Option<chrono::NaiveDate>,
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub gusto_person_id: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gusto_person_type: Option<GustoPersonType>,
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gusto_person_uuid: String,
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_title: String,
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
    /**
     * The representation of a job applicant in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uuid: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FederalTaxDetails {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ein: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub ein_verified: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filing_form: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub legal_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tax_payer_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub taxable_as_scorp: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostEmployeeYtdBenefitAmountsFromDifferentCompanyRequest {
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub benefit_id: f64,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub tax_year: f64,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ytd_company_contribution_amount: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ytd_employee_deduction_amount: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EarningTypeListResponse {
    /**
     * The default earning types for the company.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub custom: Vec<EarningType>,
    /**
     * The default earning types for the company.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub default: Vec<EarningType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Include {
    #[serde(rename = "custom_fields")]
    CustomFields,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Include {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Include::CustomFields => "custom_fields",
            Include::Noop => "",
            Include::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Include {
    fn default() -> Include {
        Include::Noop
    }
}
impl Include {
    pub fn is_noop(&self) -> bool {
        matches!(self, Include::Noop)
    }
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutEmployeesRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub date_of_birth: Option<chrono::NaiveDate>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub middle_initial: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssn: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub two_percent_shareholder: Option<bool>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostEmployeesRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub date_of_birth: Option<chrono::NaiveDate>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub middle_initial: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssn: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutJobRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub hire_date: Option<chrono::NaiveDate>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub location_id: f64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostJobRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub hire_date: Option<chrono::NaiveDate>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub location_id: f64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCompanyLocationsRequest {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filing_address: Option<bool>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<bool>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_number: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_1: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_2: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutLocationRequest {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filing_address: Option<bool>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<bool>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_number: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_1: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_2: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutComntractorRequest {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub business_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ein: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hourly_rate: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub middle_initial: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wage_type: Option<WageType>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCompanyContractorsRequest {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub business_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ein: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub middle_initial: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_onboarding: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
    /**
     * The contractor's type, either "Individual" or "Business".
     */
    #[serde(default, skip_serializing_if = "Type::is_noop", rename = "type")]
    pub type_: Type,
    /**
     * The contractor's wage type, either "Fixed" or "Hourly".
     */
    #[serde(default, skip_serializing_if = "WageType::is_noop")]
    pub wage_type: WageType,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompensationRequest {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flsa_status: Option<FlsaStatus>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_unit: Option<PaymentUnit>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rate: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostJobCompensationsRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub effective_date: Option<chrono::NaiveDate>,
    /**
     * The FLSA status for this compensation. Salaried ('Exempt') employees are paid a fixed salary every pay period. Salaried with overtime ('Salaried Nonexempt') employees are paid a fixed salary every pay period, and receive overtime pay when applicable. Hourly ('Nonexempt') employees are paid for the hours they work, and receive overtime pay when applicable. Owners ('Owner') are employees that own at least twenty percent of the company.
     */
    #[serde(default, skip_serializing_if = "FlsaStatus::is_noop")]
    pub flsa_status: FlsaStatus,
    /**
     * The unit accompanying the compensation rate. If the employee is an owner, rate should be 'Paycheck'.
     */
    #[serde(default, skip_serializing_if = "PaymentUnit::is_noop")]
    pub payment_unit: PaymentUnit,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rate: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostEmployeeGarnishmentsRequest {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub annual_maximum: f64,
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub court_ordered: Option<bool>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduct_as_percentage: Option<bool>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub pay_period_maximum: f64,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring: Option<bool>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub times: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutGarnishmentRequest {
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub annual_maximum: f64,
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub court_ordered: Option<bool>,
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduct_as_percentage: Option<bool>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub pay_period_maximum: f64,
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring: Option<bool>,
    /**
     * The unique identifier of the location in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub times: i64,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostEmployeeTerminationsRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub effective_date: Option<chrono::NaiveDate>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_termination_payroll: Option<bool>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutEmployeeHomeAddressRequest {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_1: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_2: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyPaySchedulesScheduleRequest {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_pilot: Option<bool>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCompanyBankAccountsRequest {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub account_number: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub routing_number: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyBankAccountsVerifyRequest {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub deposit_1: f64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub deposit_2: f64,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCompanyBenefitsRequest {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub benefit_id: f64,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responsible_for_employee_w2"
    )]
    pub responsible_for_employee_w_2: Option<bool>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_for_employer_taxes: Option<bool>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyBenefitRequest {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCompanyEarningTypesRequest {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyEarningTypeRequest {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostEmployeeBenefitsRequest {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catch_up: Option<bool>,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub company_benefit_id: f64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_contribution: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_contribution_annual_maximum: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contribute_as_percentage: Option<bool>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub coverage_amount: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub coverage_salary_multiplier: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduct_as_percentage: Option<bool>,
    /**
     * Whether the employee deduction reduces taxable income or not. Only valid for Group Term Life benefits. Note: when the value is not "unset", coverage amount and coverage salary multiplier are ignored.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduction_reduces_taxable_income: Option<DeductionReducesTaxableIncome>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_deduction: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_deduction_annual_maximum: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub limit_option: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutEmployeeBenefitRequest {
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catch_up: Option<bool>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_contribution: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_contribution_annual_maximum: String,
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contribute_as_percentage: Option<bool>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub coverage_amount: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub coverage_salary_multiplier: String,
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduct_as_percentage: Option<bool>,
    /**
     * Whether the employee deduction reduces taxable income or not. Only valid for Group Term Life benefits. Note: when the value is not "unset", coverage amount and coverage salary multiplier are ignored.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduction_reduces_taxable_income: Option<DeductionReducesTaxableIncome>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_deduction: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub employee_deduction_annual_maximum: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub limit_option: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum GetCompanyPayrollsInclude {
    #[serde(rename = "benefits")]
    Benefits,
    #[serde(rename = "deductions")]
    Deductions,
    #[serde(rename = "taxes")]
    Taxes,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for GetCompanyPayrollsInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetCompanyPayrollsInclude::Benefits => "benefits",
            GetCompanyPayrollsInclude::Deductions => "deductions",
            GetCompanyPayrollsInclude::Taxes => "taxes",
            GetCompanyPayrollsInclude::Noop => "",
            GetCompanyPayrollsInclude::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for GetCompanyPayrollsInclude {
    fn default() -> GetCompanyPayrollsInclude {
        GetCompanyPayrollsInclude::Noop
    }
}
impl GetCompanyPayrollsInclude {
    pub fn is_noop(&self) -> bool {
        matches!(self, GetCompanyPayrollsInclude::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OffCycleReason {
    #[serde(rename = "Bonus")]
    Bonus,
    #[serde(rename = "Correction")]
    Correction,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OffCycleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OffCycleReason::Bonus => "Bonus",
            OffCycleReason::Correction => "Correction",
            OffCycleReason::Noop => "",
            OffCycleReason::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OffCycleReason {
    fn default() -> OffCycleReason {
        OffCycleReason::Noop
    }
}
impl OffCycleReason {
    pub fn is_noop(&self) -> bool {
        matches!(self, OffCycleReason::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCompanyPayrollsRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub check_date: Option<chrono::NaiveDate>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub employee_ids: Vec<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub end_date: Option<chrono::NaiveDate>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub off_cycle: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_cycle_reason: Option<OffCycleReason>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
}

/// An array of fixed compensations for the employee. Fixed compensations include tips, bonuses, and one time reimbursements.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyPayrollsRequestEmployeeCompensationsFixed {
    /**
     * An array of fixed compensations for the employee. Fixed compensations include tips, bonuses, and one time reimbursements.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub amount: String,
    /**
     * An array of fixed compensations for the employee. Fixed compensations include tips, bonuses, and one time reimbursements.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub job_id: i64,
    /**
     * An array of fixed compensations for the employee. Fixed compensations include tips, bonuses, and one time reimbursements.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// An array of hourly compensations for the employee. Hourly compensations include regular, overtime, and double overtime hours.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyPayrollsRequestEmployeeCompensationsHourly {
    /**
     * An array of hourly compensations for the employee. Hourly compensations include regular, overtime, and double overtime hours.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hours: String,
    /**
     * An array of hourly compensations for the employee. Hourly compensations include regular, overtime, and double overtime hours.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub job_id: i64,
    /**
     * An array of hourly compensations for the employee. Hourly compensations include regular, overtime, and double overtime hours.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyPayrollsRequestEmployeeCompensations {
    /**
     * The unique identifier of the location in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub employee_id: i64,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fixed_compensations: Vec<PutCompanyPayrollsRequestEmployeeCompensationsFixed>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub hourly_compensations: Vec<PutCompanyPayrollsRequestEmployeeCompensationsHourly>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub paid_time_off: Vec<PayrollEmployeeCompensationsPaidTimeOff>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyPayrollsRequest {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub employee_compensations: Vec<PutCompanyPayrollsRequestEmployeeCompensations>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

/// Information for the user who will be the primary payroll administrator for the new company.
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct User {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * Information for the user who will be the primary payroll administrator for the new company.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostPartnerManagedCompaniesRequestCompany {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ein: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trade_name: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostPartnerManagedCompaniesRequest {
    #[serde()]
    pub company: PostPartnerManagedCompaniesRequestCompany,
    /**
     * Information for the user who will be the primary payroll administrator for the new company.
     */
    #[serde()]
    pub user: User,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostPartnerManagedCompaniesResponse {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_uuid: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Addresses {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub is_primary: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_1: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub street_2: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostProvisionRequestCompany {
    /**
     * The locations for the company. This includes mailing, work, and filing addresses.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub addresses: Vec<Addresses>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ein: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The ID of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub number_employees: f64,
    /**
     * An array of options for fields of type radio. Otherwise, null.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub states: Vec<String>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trade_name: String,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostProvisionRequest {
    #[serde()]
    pub company: PostProvisionRequestCompany,
    /**
     * Information for the user who will be the primary payroll administrator for the new company.
     */
    #[serde()]
    pub user: User,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostProvisionResponse {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub account_claim_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetEmployeeCustomFieldsResponse {
    /**
     * Custom fields are only included for the employee if the include param has the custom_fields value set
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub custom_fields: Vec<EmployeeCustomField>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCompanyCustomFieldsResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub custom_fields: Vec<CompanyCustomField>,
}

/**
* Must be "Employee" if send_offer is set to true.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OnboardingPersonType {
    #[serde(rename = "Contractor")]
    Contractor,
    #[serde(rename = "Employee")]
    Employee,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OnboardingPersonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OnboardingPersonType::Contractor => "Contractor",
            OnboardingPersonType::Employee => "Employee",
            OnboardingPersonType::Noop => "",
            OnboardingPersonType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OnboardingPersonType {
    fn default() -> OnboardingPersonType {
        OnboardingPersonType::Noop
    }
}
impl OnboardingPersonType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OnboardingPersonType::Noop)
    }
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCompanyJobApplicantsRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub date_of_birth: Option<chrono::NaiveDate>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_title: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * Must be "Employee" if send_offer is set to true.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_person_type: Option<OnboardingPersonType>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_offer: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyJobApplicantRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub date_of_birth: Option<chrono::NaiveDate>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub job_title: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /**
     * Must be "Employee" if send_offer is set to true.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_person_type: Option<OnboardingPersonType>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_offer: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_format::deserialize"
    )]
    pub start_date: Option<chrono::NaiveDate>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCompanyPayrollReversalsResponse {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub approved_at: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * The unique identifier of the location in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub reversal_payroll_id: i64,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub reversed_employee_ids: Vec<i64>,
    /**
     * The unique identifier of the location in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub reversed_payroll_id: i64,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PostCompanyAdminsRequest {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetCompanyFederalTaxDetailsResponse {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ein: String,
    /**
     * Whether the employee is terminated.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub ein_verified: bool,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filing_form: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub legal_name: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tax_payer_type: String,
    /**
     * Whether the employee is terminated.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub taxable_as_scorp: bool,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PutCompanyFederalTaxDetailsRequest {
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ein: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filing_form: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub legal_name: String,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tax_payer_type: String,
    /**
     * Whether the employee is terminated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxable_as_scorp: Option<bool>,
    /**
     * A unique identifier of the employee in Gusto.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}
