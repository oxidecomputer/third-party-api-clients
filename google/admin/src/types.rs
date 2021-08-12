//! The data types sent to and returned from the API client.
    use schemars::JsonSchema;
    use serde::{Serialize, Deserialize};

/// JSON template for Alias object in Directory API.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Alias {
/**
* JSON template for Alias object in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub alias: String,
/**
* JSON template for Alias object in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* JSON template for Alias object in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub id: String,
/**
* JSON template for Alias object in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* JSON template for Alias object in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub primary_email: String,
}

/// JSON response template to list aliases in Directory API.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Aliases {
/**
* JSON response template to list aliases in Directory API.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub aliases: Vec<String>,
/**
* JSON response template to list aliases in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* JSON response template to list aliases in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
}

/// An application-specific password (ASP) is used with applications that do not accept a verification code when logging into the application on certain devices. The ASP access code is used instead of the login and password you commonly use when accessing an application through a browser. For more information about ASPs and how to create one, see the [help center](https://support.google.com/a/answer/2537800#asp).
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Asp {
/**
* An application-specific password (ASP) is used with applications that do not accept a verification code when logging into the application on certain devices. The ASP access code is used instead of the login and password you commonly use when accessing an application through a browser. For more information about ASPs and how to create one, see the [help center](https://support.google.com/a/answer/2537800#asp).
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub code_id: i64,
/**
* An application-specific password (ASP) is used with applications that do not accept a verification code when logging into the application on certain devices. The ASP access code is used instead of the login and password you commonly use when accessing an application through a browser. For more information about ASPs and how to create one, see the [help center](https://support.google.com/a/answer/2537800#asp).
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub creation_time: i64,
/**
* An application-specific password (ASP) is used with applications that do not accept a verification code when logging into the application on certain devices. The ASP access code is used instead of the login and password you commonly use when accessing an application through a browser. For more information about ASPs and how to create one, see the [help center](https://support.google.com/a/answer/2537800#asp).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* An application-specific password (ASP) is used with applications that do not accept a verification code when logging into the application on certain devices. The ASP access code is used instead of the login and password you commonly use when accessing an application through a browser. For more information about ASPs and how to create one, see the [help center](https://support.google.com/a/answer/2537800#asp).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* An application-specific password (ASP) is used with applications that do not accept a verification code when logging into the application on certain devices. The ASP access code is used instead of the login and password you commonly use when accessing an application through a browser. For more information about ASPs and how to create one, see the [help center](https://support.google.com/a/answer/2537800#asp).
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub last_time_used: i64,
/**
* An application-specific password (ASP) is used with applications that do not accept a verification code when logging into the application on certain devices. The ASP access code is used instead of the login and password you commonly use when accessing an application through a browser. For more information about ASPs and how to create one, see the [help center](https://support.google.com/a/answer/2537800#asp).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub name: String,
/**
* An application-specific password (ASP) is used with applications that do not accept a verification code when logging into the application on certain devices. The ASP access code is used instead of the login and password you commonly use when accessing an application through a browser. For more information about ASPs and how to create one, see the [help center](https://support.google.com/a/answer/2537800#asp).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub user_key: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Asps {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* A list of ASP resources.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub items: Vec<Asp>,
/**
* The type of the API resource. This is always `admin#directory#aspList`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
}

/**
* Message severity
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Severity {
#[serde(rename = "SEVERITY_ERROR")]
SeverityError,
#[serde(rename = "SEVERITY_INFO")]
SeverityInfo,
#[serde(rename = "SEVERITY_UNSPECIFIED")]
SeverityUnspecified,
#[serde(rename = "SEVERITY_WARNING")]
SeverityWarning,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for Severity {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
Severity::SeverityError => "SEVERITY_ERROR",
Severity::SeverityInfo => "SEVERITY_INFO",
Severity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
Severity::SeverityWarning => "SEVERITY_WARNING",
Severity::Noop => "",
Severity::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for Severity {
fn default() -> Severity {
Severity::Noop
}
}
impl Severity {
            pub fn is_noop(&self) -> bool {
                matches!(self, Severity::Noop)
            }
    }


/// Auxiliary message about issues with printers or settings. Example: {message_type:AUXILIARY_MESSAGE_WARNING, field_mask:make_and_model, message:"Given printer is invalid or no longer supported."}
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct AuxiliaryMessage {
/**
* Auxiliary message about issues with printers or settings. Example: {message_type:AUXILIARY_MESSAGE_WARNING, field_mask:make_and_model, message:"Given printer is invalid or no longer supported."}
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub auxiliary_message: String,
/**
* Auxiliary message about issues with printers or settings. Example: {message_type:AUXILIARY_MESSAGE_WARNING, field_mask:make_and_model, message:"Given printer is invalid or no longer supported."}
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub field_mask: String,
/**
* Auxiliary message about issues with printers or settings. Example: {message_type:AUXILIARY_MESSAGE_WARNING, field_mask:make_and_model, message:"Given printer is invalid or no longer supported."}
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub severity: Option<Severity>,
}

/// Request for adding new printers in batch.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct BatchCreatePrintersRequest {
/**
* Request for adding new printers in batch.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub requests: Vec<CreatePrinterRequest>,
}

/// Response for adding new printers in batch.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct BatchCreatePrintersResponse {
/**
* Response for adding new printers in batch.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub failures: Vec<FailureInfo>,
/**
* Response for adding new printers in batch.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub printers: Vec<Printer>,
}

/// Request for deleting existing printers in batch.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct BatchDeletePrintersRequest {
/**
* Request for deleting existing printers in batch.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub printer_ids: Vec<String>,
}

/// Response for deleting existing printers in batch.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct BatchDeletePrintersResponse {
/**
* Response for deleting existing printers in batch.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub failed_printers: Vec<FailureInfo>,
/**
* Response for deleting existing printers in batch.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub printer_ids: Vec<String>,
}

/// Public API: Resources.buildings
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Building {
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub address: Option<BuildingAddress>,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub building_id: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub building_name: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub coordinates: Option<BuildingCoordinates>,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub description: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etags: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub floor_names: Vec<String>,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
}

/// Public API: Resources.buildings
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct BuildingAddress {
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub address_lines: Vec<String>,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub administrative_area: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub language_code: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub locality: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub postal_code: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub region_code: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub sublocality: String,
}

/// Public API: Resources.buildings
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct BuildingCoordinates {
/**
* Public API: Resources.buildings
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f64",
                                    deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
)]
pub latitude: f64,
/**
* Public API: Resources.buildings
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f64",
                                    deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
)]
pub longitude: f64,
}

/// Public API: Resources.buildings
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Buildings {
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub buildings: Vec<Building>,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Public API: Resources.buildings
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
}

/// Public API: Resources.calendars
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct CalendarResource {
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub building_id: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub capacity: i64,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etags: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub feature_instances: Option<serde_json::Value>,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub floor_name: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub floor_section: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub generated_resource_name: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub resource_category: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub resource_description: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub resource_email: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub resource_id: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub resource_name: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub resource_type: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub user_visible_description: String,
}

/// Public API: Resources.calendars
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct CalendarResources {
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub items: Vec<CalendarResource>,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Public API: Resources.calendars
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
}

/// Additional parameters controlling delivery channel behavior. Optional.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Params {
}

/// An notification channel used to watch for resource changes.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Channel {
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub address: String,
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub expiration: i64,
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub id: String,
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub params: Option<Params>,
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub payload: bool,
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub resource_id: String,
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub resource_uri: String,
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub token: String,
/**
* An notification channel used to watch for resource changes.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct ActiveTimeRanges {
/**
* Duration of usage in milliseconds.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub active_time: i64,
/**
* Date of usage
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub date: Option<chrono::NaiveDate>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct CpuTemperatureInfo {
/**
* CPU label
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub label: String,
/**
* Temperature in Celsius degrees.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub temperature: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct CpuStatusReports {
/**
* List of CPU temperature samples.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub cpu_temperature_info: Vec<CpuTemperatureInfo>,
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub cpu_utilization_percentage_info: Vec<i64>,
/**
* Date and time the report was received.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub report_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct DeviceFiles {
/**
* Date and time the file was created
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub create_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* File download URL
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub download_url: String,
/**
* File name
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub name: String,
/**
* File type
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct VolumeInfo {
/**
* Free disk space [in bytes]
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub storage_free: i64,
/**
* Total disk space [in bytes]
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub storage_total: i64,
/**
* Volume id
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub volume_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct DiskVolumeReports {
/**
* Disk volumes
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub volume_info: Vec<VolumeInfo>,
}

/// Information for an ip address.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct LastKnownNetwork {
/**
* Information for an ip address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub ip_address: String,
/**
* Information for an ip address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub wan_ip_address: String,
}

/// List of recent device users, in descending order, by last login time.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct RecentUsers {
/**
* List of recent device users, in descending order, by last login time.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub email: String,
/**
* List of recent device users, in descending order, by last login time.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct SystemRamFreeReports {
/**
* Date and time the report was received.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub report_time: Option<chrono::DateTime<chrono::Utc>>,
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub system_ram_free_info: Vec<i64>,
}

/// Trusted Platform Module (TPM) (Read-only)
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct TpmVersionInfo {
/**
* Trusted Platform Module (TPM) (Read-only)
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub family: String,
/**
* Trusted Platform Module (TPM) (Read-only)
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub firmware_version: String,
/**
* Trusted Platform Module (TPM) (Read-only)
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub manufacturer: String,
/**
* Trusted Platform Module (TPM) (Read-only)
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub spec_level: String,
/**
* Trusted Platform Module (TPM) (Read-only)
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub tpm_model: String,
/**
* Trusted Platform Module (TPM) (Read-only)
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub vendor_specific: String,
}

/// Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct ChromeOsDevice {
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub active_time_ranges: Vec<ActiveTimeRanges>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub annotated_asset_id: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub annotated_location: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub annotated_user: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub auto_update_expiration: i64,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub boot_mode: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub cpu_status_reports: Vec<CpuStatusReports>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub device_files: Vec<DeviceFiles>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub device_id: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub disk_volume_reports: Vec<DiskVolumeReports>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub dock_mac_address: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub ethernet_mac_address: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub ethernet_mac_address_0: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub firmware_version: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub last_enrollment_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub last_known_network: Vec<LastKnownNetwork>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub mac_address: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub manufacture_date: Option<chrono::NaiveDate>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub meid: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub model: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub notes: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub order_number: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub org_unit_path: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub os_version: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub platform_version: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub recent_users: Vec<RecentUsers>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub screenshot_files: Vec<DeviceFiles>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub serial_number: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub status: String,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub support_end_date: Option<chrono::DateTime<chrono::Utc>>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub system_ram_free_reports: Vec<SystemRamFreeReports>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub system_ram_total: i64,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub tpm_version_info: Option<TpmVersionInfo>,
/**
* Google Chrome devices run on the [Chrome OS](https://support.google.com/chromeos). For more information about common API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-chrome-devices).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub will_auto_renew: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct ChromeOsDeviceAction {
/**
* Action to be taken on the Chrome OS device.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub action: String,
/**
* Only used when the action is `deprovision`. With the `deprovision` action, this field is required. \*Note\*: The deprovision reason is audited because it might have implications on licenses for perpetual subscription customers.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub deprovision_reason: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct ChromeOsDevices {
/**
* List of Chrome OS Device objects.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub chromeosdevices: Vec<ChromeOsDevice>,
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Kind of resource this is.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Token used to access the next page of this result. To access the next page, use this token's value in the `pageToken` query string of this request.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct ChromeOsMoveDevicesOu {
/**
* Chrome OS devices to be moved to OU
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub device_ids: Vec<String>,
}

/// Request for adding a new printer.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct CreatePrinterRequest {
/**
* Request for adding a new printer.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub parent: String,
/**
* Request for adding a new printer.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub printer: Option<Printer>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Customer {
/**
* The customer's secondary contact email address. This email address cannot be on the same domain as the `customerDomain`
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub alternate_email: String,
/**
* The customer's creation time (Readonly)
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub customer_creation_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* The customer's primary domain name string. Do not include the `www` prefix when creating a new customer.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub customer_domain: String,
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* The unique ID for the customer's Google Workspace account. (Readonly)
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub id: String,
/**
* Identifies the resource as a customer. Value: `admin#directory#customer`
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* The customer's ISO 639-2 language code. See the [Language Codes](/admin-sdk/directory/v1/languages) page for the list of supported codes. Valid language codes outside the supported set will be accepted by the API but may lead to unexpected behavior. The default value is `en`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub language: String,
/**
* The customer's contact phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub phone_number: String,
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub postal_address: Option<CustomerPostalAddress>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct CustomerPostalAddress {
/**
* A customer's physical address. The address can be composed of one to three lines.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub address_line_1: String,
/**
* Address line 2 of the address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub address_line_2: String,
/**
* Address line 3 of the address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub address_line_3: String,
/**
* The customer contact's name.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub contact_name: String,
/**
* This is a required property. For `countryCode` information see the [ISO 3166 country code elements](https://www.iso.org/iso/country_codes.htm).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub country_code: String,
/**
* Name of the locality. An example of a locality value is the city of `San Francisco`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub locality: String,
/**
* The company or company division name.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub organization_name: String,
/**
* The postal code. A postalCode example is a postal zip code such as `10009`. This is in accordance with - http: //portablecontacts.net/draft-spec.html#address_element.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub postal_code: String,
/**
* Name of the region. An example of a region value is `NY` for the state of New York.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub region: String,
}

/**
* Indicates the command state.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum State {
#[serde(rename = "ACKED_BY_CLIENT")]
AckedByClient,
#[serde(rename = "CANCELLED")]
Cancelled,
#[serde(rename = "EXECUTED_BY_CLIENT")]
ExecutedByClient,
#[serde(rename = "EXPIRED")]
Expired,
#[serde(rename = "PENDING")]
Pending,
#[serde(rename = "SENT_TO_CLIENT")]
SentToClient,
#[serde(rename = "STATE_UNSPECIFIED")]
StateUnspecified,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for State {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
State::AckedByClient => "ACKED_BY_CLIENT",
State::Cancelled => "CANCELLED",
State::ExecutedByClient => "EXECUTED_BY_CLIENT",
State::Expired => "EXPIRED",
State::Pending => "PENDING",
State::SentToClient => "SENT_TO_CLIENT",
State::StateUnspecified => "STATE_UNSPECIFIED",
State::Noop => "",
State::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for State {
fn default() -> State {
State::Noop
}
}
impl State {
            pub fn is_noop(&self) -> bool {
                matches!(self, State::Noop)
            }
    }


/**
* The type of the command.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Type {
#[serde(rename = "COMMAND_TYPE_UNSPECIFIED")]
CommandTypeUnspecified,
#[serde(rename = "REBOOT")]
Reboot,
#[serde(rename = "REMOTE_POWERWASH")]
RemotePowerwash,
#[serde(rename = "SET_VOLUME")]
SetVolume,
#[serde(rename = "TAKE_A_SCREENSHOT")]
TakeAScreenshot,
#[serde(rename = "WIPE_USERS")]
WipeUsers,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for Type {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
Type::CommandTypeUnspecified => "COMMAND_TYPE_UNSPECIFIED",
Type::Reboot => "REBOOT",
Type::RemotePowerwash => "REMOTE_POWERWASH",
Type::SetVolume => "SET_VOLUME",
Type::TakeAScreenshot => "TAKE_A_SCREENSHOT",
Type::WipeUsers => "WIPE_USERS",
Type::Noop => "",
Type::FallthroughString(s) => s,
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


/// Information regarding a command that was issued to a device.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct DirectoryChromeosdevicesCommand {
/**
* Information regarding a command that was issued to a device.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub command_expire_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* Information regarding a command that was issued to a device.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub command_id: i64,
/**
* Information regarding a command that was issued to a device.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub command_result: Option<DirectoryChromeosdevicesCommandResult>,
/**
* Information regarding a command that was issued to a device.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub issue_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* Information regarding a command that was issued to a device.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub payload: String,
/**
* Information regarding a command that was issued to a device.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub state: Option<State>,
/**
* Information regarding a command that was issued to a device.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
rename = "type")]
pub type_: Option<Type>,
}

/**
* The result of the command.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Result {
#[serde(rename = "COMMAND_RESULT_TYPE_UNSPECIFIED")]
CommandResultTypeUnspecified,
#[serde(rename = "FAILURE")]
Failure,
#[serde(rename = "IGNORED")]
Ignored,
#[serde(rename = "SUCCESS")]
Success,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for Result {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
Result::CommandResultTypeUnspecified => "COMMAND_RESULT_TYPE_UNSPECIFIED",
Result::Failure => "FAILURE",
Result::Ignored => "IGNORED",
Result::Success => "SUCCESS",
Result::Noop => "",
Result::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for Result {
fn default() -> Result {
Result::Noop
}
}
impl Result {
            pub fn is_noop(&self) -> bool {
                matches!(self, Result::Noop)
            }
    }


/// The result of executing a command.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct DirectoryChromeosdevicesCommandResult {
/**
* The result of executing a command.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub error_message: String,
/**
* The result of executing a command.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub execute_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* The result of executing a command.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub result: Option<Result>,
}

/**
* The type of command.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CommandType {
#[serde(rename = "COMMAND_TYPE_UNSPECIFIED")]
CommandTypeUnspecified,
#[serde(rename = "REBOOT")]
Reboot,
#[serde(rename = "REMOTE_POWERWASH")]
RemotePowerwash,
#[serde(rename = "SET_VOLUME")]
SetVolume,
#[serde(rename = "TAKE_A_SCREENSHOT")]
TakeAScreenshot,
#[serde(rename = "WIPE_USERS")]
WipeUsers,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for CommandType {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
CommandType::CommandTypeUnspecified => "COMMAND_TYPE_UNSPECIFIED",
CommandType::Reboot => "REBOOT",
CommandType::RemotePowerwash => "REMOTE_POWERWASH",
CommandType::SetVolume => "SET_VOLUME",
CommandType::TakeAScreenshot => "TAKE_A_SCREENSHOT",
CommandType::WipeUsers => "WIPE_USERS",
CommandType::Noop => "",
CommandType::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for CommandType {
fn default() -> CommandType {
CommandType::Noop
}
}
impl CommandType {
            pub fn is_noop(&self) -> bool {
                matches!(self, CommandType::Noop)
            }
    }


/// A request for issuing a command.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct DirectoryChromeosdevicesIssueCommandRequest {
/**
* A request for issuing a command.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub command_type: Option<CommandType>,
/**
* A request for issuing a command.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub payload: String,
}

/// A response for issuing a command.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct DirectoryChromeosdevicesIssueCommandResponse {
/**
* A response for issuing a command.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub command_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct DomainAlias {
/**
* The creation time of the domain alias. (Read-only).
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub creation_time: i64,
/**
* The domain alias name.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub domain_alias_name: String,
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Kind of resource this is.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* The parent domain name that the domain alias is associated with. This can either be a primary or secondary domain name within a customer.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub parent_domain_name: String,
/**
* Indicates the verification state of a domain alias. (Read-only)
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct DomainAliases {
/**
* List of domain alias objects.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub domain_aliases: Vec<DomainAlias>,
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Kind of resource this is.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Domains {
/**
* Creation time of the domain. Expressed in [Unix time](https://en.wikipedia.org/wiki/Epoch_time) format. (Read-only).
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub creation_time: i64,
/**
* List of domain alias objects. (Read-only)
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub domain_aliases: Vec<DomainAlias>,
/**
* The domain name of the customer.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub domain_name: String,
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Indicates if the domain is a primary domain (Read-only).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_primary: bool,
/**
* Kind of resource this is.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Indicates the verification state of a domain. (Read-only).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Domains2 {
/**
* List of domain objects.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub domains: Vec<Domains>,
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Kind of resource this is.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
}

/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Empty {
}

/**
* Canonical code for why the update failed to apply.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ErrorCode {
#[serde(rename = "ABORTED")]
Aborted,
#[serde(rename = "ALREADY_EXISTS")]
AlreadyExists,
#[serde(rename = "CANCELLED")]
Cancelled,
#[serde(rename = "DATA_LOSS")]
DataLoss,
#[serde(rename = "DEADLINE_EXCEEDED")]
DeadlineExceeded,
#[serde(rename = "FAILED_PRECONDITION")]
FailedPrecondition,
#[serde(rename = "INTERNAL")]
Internal,
#[serde(rename = "INVALID_ARGUMENT")]
InvalidArgument,
#[serde(rename = "NOT_FOUND")]
NotFound,
#[serde(rename = "OK")]
Ok,
#[serde(rename = "OUT_OF_RANGE")]
OutOfRange,
#[serde(rename = "PERMISSION_DENIED")]
PermissionDenied,
#[serde(rename = "RESOURCE_EXHAUSTED")]
ResourceExhausted,
#[serde(rename = "UNAUTHENTICATED")]
Unauthenticated,
#[serde(rename = "UNAVAILABLE")]
Unavailable,
#[serde(rename = "UNIMPLEMENTED")]
Unimplemented,
#[serde(rename = "UNKNOWN")]
Unknown,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for ErrorCode {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
ErrorCode::Aborted => "ABORTED",
ErrorCode::AlreadyExists => "ALREADY_EXISTS",
ErrorCode::Cancelled => "CANCELLED",
ErrorCode::DataLoss => "DATA_LOSS",
ErrorCode::DeadlineExceeded => "DEADLINE_EXCEEDED",
ErrorCode::FailedPrecondition => "FAILED_PRECONDITION",
ErrorCode::Internal => "INTERNAL",
ErrorCode::InvalidArgument => "INVALID_ARGUMENT",
ErrorCode::NotFound => "NOT_FOUND",
ErrorCode::Ok => "OK",
ErrorCode::OutOfRange => "OUT_OF_RANGE",
ErrorCode::PermissionDenied => "PERMISSION_DENIED",
ErrorCode::ResourceExhausted => "RESOURCE_EXHAUSTED",
ErrorCode::Unauthenticated => "UNAUTHENTICATED",
ErrorCode::Unavailable => "UNAVAILABLE",
ErrorCode::Unimplemented => "UNIMPLEMENTED",
ErrorCode::Unknown => "UNKNOWN",
ErrorCode::Noop => "",
ErrorCode::FallthroughString(s) => s,
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


/// Info about failures
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct FailureInfo {
/**
* Info about failures
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub error_code: Option<ErrorCode>,
/**
* Info about failures
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub error_message: String,
/**
* Info about failures
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub printer: Option<Printer>,
/**
* Info about failures
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub printer_id: String,
}

/// JSON template for Feature object in Directory API.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Feature {
/**
* JSON template for Feature object in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etags: String,
/**
* JSON template for Feature object in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* JSON template for Feature object in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub name: String,
}

/// JSON template for a feature instance.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct FeatureInstance {
/**
* JSON template for a feature instance.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub feature: Option<Feature>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct FeatureRename {
/**
* New name of the feature.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub new_name: String,
}

/// Public API: Resources.features
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Features {
/**
* Public API: Resources.features
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Public API: Resources.features
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub features: Vec<Feature>,
/**
* Public API: Resources.features
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Public API: Resources.features
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
}

/// Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Group {
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub admin_created: bool,
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub aliases: Vec<String>,
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub description: String,
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub direct_members_count: i64,
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub email: String,
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub id: String,
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub name: String,
/**
* Google Groups provide your users the ability to send messages to groups of people using the group's email address. For more information about common tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-groups).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub non_editable_aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Groups {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* List of group objects.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub groups: Vec<Group>,
/**
* Kind of resource this is.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Token used to access next page of this result.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
}

/// Response for listing allowed printer models.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct ListPrinterModelsResponse {
/**
* Response for listing allowed printer models.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
/**
* Response for listing allowed printer models.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub printer_models: Vec<PrinterModel>,
}

/// Response for listing printers.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct ListPrintersResponse {
/**
* Response for listing printers.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
/**
* Response for listing printers.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub printers: Vec<Printer>,
}

/// A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members).
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Member {
/**
* A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub delivery_settings: String,
/**
* A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub email: String,
/**
* A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub id: String,
/**
* A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub role: String,
/**
* A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub status: String,
/**
* A Google Groups member can be a user or another group. This member can be inside or outside of your account's domains. For more information about common group member tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-group-members).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Members {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Kind of resource this is.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* List of member objects.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub members: Vec<Member>,
/**
* Token used to access next page of this result.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
}

/// JSON template for Has Member response in Directory API.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct MembersHasMember {
/**
* JSON template for Has Member response in Directory API.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_member: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Applications {
/**
* The application's display name. An example is `Browser`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub display_name: String,
/**
* The application's package name. An example is `com.android.browser`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub package_name: String,
/**
* The list of permissions of this application. These can be either a standard Android permission or one defined by the application, and are found in an application's [Android manifest](https://developer.android.com/guide/topics/manifest/uses-permission-element.html). Examples of a Calendar application's permissions are `READ_CALENDAR`, or `MANAGE_ACCOUNTS`.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub permission: Vec<String>,
/**
* The application's version code. An example is `13`.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub version_code: i64,
/**
* The application's version name. An example is `3.2-140714`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub version_name: String,
}

/// Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct MobileDevice {
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub adb_status: bool,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub applications: Vec<Applications>,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub baseband_version: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub bootloader_version: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub brand: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub build_number: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub default_language: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub developer_options_status: bool,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub device_compromised_status: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub device_id: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub device_password_status: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub email: Vec<String>,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub encryption_status: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub first_sync: Option<chrono::DateTime<chrono::Utc>>,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub hardware: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub hardware_id: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub imei: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kernel_version: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub managed_account_is_on_owner_profile: bool,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub manufacturer: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub meid: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub model: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub name: Vec<String>,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub network_operator: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub os: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub other_accounts_info: Vec<String>,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub privilege: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub release_version: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub resource_id: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub security_patch_level: i64,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub serial_number: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub status: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub supports_work_profile: bool,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub unknown_sources_status: bool,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub user_agent: String,
/**
* Google Workspace Mobile Management includes Android, [Google Sync](https://support.google.com/a/answer/135937), and iOS devices. For more information about common group mobile device API tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-mobile-devices.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub wifi_mac_address: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct MobileDeviceAction {
/**
* The action to be performed on the device.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub action: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct MobileDevices {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Kind of resource this is.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* List of Mobile Device objects.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub mobiledevices: Vec<MobileDevice>,
/**
* Token used to access next page of this result.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
}

/// Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct OrgUnit {
/**
* Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub block_inheritance: bool,
/**
* Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub description: String,
/**
* Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub name: String,
/**
* Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub org_unit_id: String,
/**
* Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub org_unit_path: String,
/**
* Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub parent_org_unit_id: String,
/**
* Managing your account's organizational units allows you to configure your users' access to services and custom settings. For more information about common organizational unit tasks, see the [Developer's Guide](/admin-sdk/directory/v1/guides/manage-org-units.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub parent_org_unit_path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct OrgUnits {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* The type of the API resource. For Org Unit resources, the type is `admin#directory#orgUnits`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* List of organizational unit objects.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub organization_units: Vec<OrgUnit>,
}

/// Printer configuration.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Printer {
/**
* Printer configuration.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub auxiliary_messages: Vec<AuxiliaryMessage>,
/**
* Printer configuration.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub create_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* Printer configuration.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub description: String,
/**
* Printer configuration.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub display_name: String,
/**
* Printer configuration.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub id: String,
/**
* Printer configuration.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub make_and_model: String,
/**
* Printer configuration.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub name: String,
/**
* Printer configuration.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub org_unit_id: String,
/**
* Printer configuration.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub uri: String,
/**
* Printer configuration.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub use_driverless_config: bool,
}

/// Printer manufacturer and model
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct PrinterModel {
/**
* Printer manufacturer and model
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub display_name: String,
/**
* Printer manufacturer and model
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub make_and_model: String,
/**
* Printer manufacturer and model
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub manufacturer: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Privilege {
/**
* A list of child privileges. Privileges for a service form a tree. Each privilege can have a list of child privileges; this list is empty for a leaf privilege.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub child_privileges: Vec<Privilege>,
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* If the privilege can be restricted to an organization unit.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_ou_scopable: bool,
/**
* The type of the API resource. This is always `admin#directory#privilege`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* The name of the privilege.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub privilege_name: String,
/**
* The obfuscated ID of the service this privilege is for. This value is returned with [`Privileges.list()`](/admin-sdk/directory/v1/reference/privileges/list).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub service_id: String,
/**
* The name of the service this privilege is for.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub service_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Privileges {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* A list of Privilege resources.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub items: Vec<Privilege>,
/**
* The type of the API resource. This is always `admin#directory#privileges`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct RolePrivileges {
/**
* The name of the privilege.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub privilege_name: String,
/**
* The obfuscated ID of the service this privilege is for. This value is returned with [`Privileges.list()`](/admin-sdk/directory/v1/reference/privileges/list).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub service_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Role {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Returns `true` if the role is a super admin role.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_super_admin_role: bool,
/**
* Returns `true` if this is a pre-defined system role.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_system_role: bool,
/**
* The type of the API resource. This is always `admin#directory#role`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* A short description of the role.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub role_description: String,
/**
* ID of the role.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub role_id: i64,
/**
* Name of the role.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub role_name: String,
/**
* The set of privileges that are granted to this role.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub role_privileges: Vec<RolePrivileges>,
}

/// Defines an assignment of a role.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct RoleAssignment {
/**
* Defines an assignment of a role.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub assigned_to: String,
/**
* Defines an assignment of a role.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Defines an assignment of a role.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Defines an assignment of a role.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub org_unit_id: String,
/**
* Defines an assignment of a role.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub role_assignment_id: i64,
/**
* Defines an assignment of a role.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub role_id: i64,
/**
* Defines an assignment of a role.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub scope_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct RoleAssignments {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* A list of RoleAssignment resources.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub items: Vec<RoleAssignment>,
/**
* The type of the API resource. This is always `admin#directory#roleAssignments`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Roles {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* A list of Role resources.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub items: Vec<Role>,
/**
* The type of the API resource. This is always `admin#directory#roles`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
}

/// The type of API resource. For Schema resources, this is always `admin#directory#schema`.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Schema {
/**
* The type of API resource. For Schema resources, this is always `admin#directory#schema`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub display_name: String,
/**
* The type of API resource. For Schema resources, this is always `admin#directory#schema`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* The type of API resource. For Schema resources, this is always `admin#directory#schema`.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub fields: Vec<SchemaFieldSpec>,
/**
* The type of API resource. For Schema resources, this is always `admin#directory#schema`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* The type of API resource. For Schema resources, this is always `admin#directory#schema`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub schema_id: String,
/**
* The type of API resource. For Schema resources, this is always `admin#directory#schema`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub schema_name: String,
}

/// Indexing spec for a numeric field. By default, only exact match queries will be supported for numeric fields. Setting the `numericIndexingSpec` allows range queries to be supported.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct NumericIndexingSpec {
/**
* Indexing spec for a numeric field. By default, only exact match queries will be supported for numeric fields. Setting the `numericIndexingSpec` allows range queries to be supported.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f64",
                                    deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
)]
pub max_value: f64,
/**
* Indexing spec for a numeric field. By default, only exact match queries will be supported for numeric fields. Setting the `numericIndexingSpec` allows range queries to be supported.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f64",
                                    deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
)]
pub min_value: f64,
}

/// You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct SchemaFieldSpec {
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub display_name: String,
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub field_id: String,
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub field_name: String,
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub field_type: String,
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub indexed: bool,
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub multi_valued: bool,
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub numeric_indexing_spec: Option<NumericIndexingSpec>,
/**
* You can use schemas to add custom fields to user profiles. You can use these fields to store information such as the projects your users work on, their physical locations, their hire dates, or whatever else fits your business needs. For more information, see [Custom User Fields](/admin-sdk/directory/v1/guides/manage-schemas).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub read_access_type: String,
}

/// JSON response template for List Schema operation in Directory API.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Schemas {
/**
* JSON response template for List Schema operation in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* JSON response template for List Schema operation in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* JSON response template for List Schema operation in Directory API.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub schemas: Vec<Schema>,
}

/// JSON template for token resource in Directory API.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Token {
/**
* JSON template for token resource in Directory API.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub anonymous: bool,
/**
* JSON template for token resource in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub client_id: String,
/**
* JSON template for token resource in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub display_text: String,
/**
* JSON template for token resource in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* JSON template for token resource in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* JSON template for token resource in Directory API.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub native_app: bool,
/**
* JSON template for token resource in Directory API.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub scopes: Vec<String>,
/**
* JSON template for token resource in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub user_key: String,
}

/// JSON response template for List tokens operation in Directory API.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Tokens {
/**
* JSON response template for List tokens operation in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* JSON response template for List tokens operation in Directory API.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub items: Vec<Token>,
/**
* JSON response template for List tokens operation in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
}

/// Custom fields of the user.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct CustomSchemas {
}

/// The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct User {
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub addresses: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub agreed_to_terms: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub aliases: Vec<String>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub archived: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub change_password_at_next_login: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub creation_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub custom_schemas: Option<CustomSchemas>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub customer_id: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub deletion_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub emails: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub external_ids: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub gender: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub hash_function: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub id: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub ims: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub include_in_global_address_list: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub ip_whitelisted: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_admin: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_delegated_admin: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_enforced_in_2_sv: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_enrolled_in_2_sv: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub is_mailbox_setup: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub keywords: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub languages: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize",
)]
pub last_login_time: Option<chrono::DateTime<chrono::Utc>>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub locations: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub name: Option<UserName>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub non_editable_aliases: Vec<String>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub notes: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub org_unit_path: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub organizations: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub password: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub phones: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub posix_accounts: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub primary_email: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub recovery_email: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub recovery_phone: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub relations: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub ssh_public_keys: Option<serde_json::Value>,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub suspended: bool,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub suspension_reason: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub thumbnail_photo_etag: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub thumbnail_photo_url: String,
/**
* The Directory API allows you to create and manage your account's users, user aliases, and user Gmail chat profile photos. For more information about common tasks, see the [User Accounts Developer's Guide](/admin-sdk/directory/v1/guides/manage-users.html) and the [User Aliases Developer's Guide](/admin-sdk/directory/v1/guides/manage-user-aliases.html).
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub websites: Option<serde_json::Value>,
}

/// JSON template for About (notes) of a user in Directory API.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserAbout {
/**
* JSON template for About (notes) of a user in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub content_type: String,
/**
* JSON template for About (notes) of a user in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub value: String,
}

/// JSON template for address.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserAddress {
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub country: String,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub country_code: String,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub extended_address: String,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub formatted: String,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub locality: String,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub po_box: String,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub postal_code: String,
/**
* JSON template for address.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub primary: bool,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub region: String,
/**
* JSON template for address.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub source_is_structured: bool,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub street_address: String,
/**
* JSON template for address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

/// JSON template for a set of custom properties (i.e. all fields in a particular schema)
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserCustomProperties {
}

/// JSON template for an email.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserEmail {
/**
* JSON template for an email.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub address: String,
/**
* JSON template for an email.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for an email.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub primary: bool,
/**
* JSON template for an email.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

/// JSON template for an externalId entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserExternalId {
/**
* JSON template for an externalId entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for an externalId entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
/**
* JSON template for an externalId entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserGender {
/**
* AddressMeAs. A human-readable string containing the proper way to refer to the profile owner by humans for example he/him/his or they/them/their.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub address_me_as: String,
/**
* Custom gender.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_gender: String,
/**
* Gender.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

/// JSON template for instant messenger of an user.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserIm {
/**
* JSON template for instant messenger of an user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_protocol: String,
/**
* JSON template for instant messenger of an user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for instant messenger of an user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub im: String,
/**
* JSON template for instant messenger of an user.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub primary: bool,
/**
* JSON template for instant messenger of an user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub protocol: String,
/**
* JSON template for instant messenger of an user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

/// JSON template for a keyword entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserKeyword {
/**
* JSON template for a keyword entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for a keyword entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
/**
* JSON template for a keyword entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub value: String,
}

/// JSON template for a language entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserLanguage {
/**
* JSON template for a language entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_language: String,
/**
* JSON template for a language entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub language_code: String,
}

/// JSON template for a location entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserLocation {
/**
* JSON template for a location entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub area: String,
/**
* JSON template for a location entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub building_id: String,
/**
* JSON template for a location entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for a location entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub desk_code: String,
/**
* JSON template for a location entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub floor_name: String,
/**
* JSON template for a location entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub floor_section: String,
/**
* JSON template for a location entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserMakeAdmin {
/**
* Indicates the administrator status of the user.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub status: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserName {
/**
* The user's last name. Required when creating a user account.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub family_name: String,
/**
* The user's full name formed by concatenating the first and last name values.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub full_name: String,
/**
* The user's first name. Required when creating a user account.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub given_name: String,
}

/// JSON template for an organization entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserOrganization {
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub cost_center: String,
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub department: String,
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub description: String,
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub domain: String,
/**
* JSON template for an organization entry.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub full_time_equivalent: i64,
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub location: String,
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub name: String,
/**
* JSON template for an organization entry.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub primary: bool,
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub symbol: String,
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub title: String,
/**
* JSON template for an organization entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
}

/// JSON template for a phone entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserPhone {
/**
* JSON template for a phone entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for a phone entry.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub primary: bool,
/**
* JSON template for a phone entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
/**
* JSON template for a phone entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserPhoto {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Height of the photo in pixels.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub height: i64,
/**
* The ID the API uses to uniquely identify the user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub id: String,
/**
* The type of the API resource. For Photo resources, this is `admin#directory#user#photo`.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* The MIME type of the photo. Allowed values are `JPEG`, `PNG`, `GIF`, `BMP`, `TIFF`, and web-safe base64 encoding.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub mime_type: String,
/**
* The user photo's upload data in [web-safe Base64](https://en.wikipedia.org/wiki/Base64#URL_applications) format in bytes. This means: \* The slash (/) character is replaced with the underscore (_) character. \* The plus sign (+) character is replaced with the hyphen (-) character. \* The equals sign (=) character is replaced with the asterisk (\*). \* For padding, the period (.) character is used instead of the RFC-4648 baseURL definition which uses the equals sign (=) for padding. This is done to simplify URL-parsing. \* Whatever the size of the photo being uploaded, the API downsizes it to 96x96 pixels.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub photo_data: String,
/**
* The user's primary email address.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub primary_email: String,
/**
* Width of the photo in pixels.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub width: i64,
}

/// JSON template for a POSIX account entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserPosixAccount {
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub account_id: String,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub gecos: String,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub gid: Option<u64>,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub home_directory: String,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub operating_system_type: String,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub primary: bool,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub shell: String,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub system_id: String,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "Option::is_none",
)]
pub uid: Option<u64>,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub username: String,
}

/// JSON template for a relation entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserRelation {
/**
* JSON template for a relation entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for a relation entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
/**
* JSON template for a relation entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub value: String,
}

/// JSON template for a POSIX account entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserSshPublicKey {
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
)]
pub expiration_time_usec: i64,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub fingerprint: String,
/**
* JSON template for a POSIX account entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserUndelete {
/**
* OrgUnit of User
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub org_unit_path: String,
}

/// JSON template for a website entry.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct UserWebsite {
/**
* JSON template for a website entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub custom_type: String,
/**
* JSON template for a website entry.
*/
#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
)]
pub primary: bool,
/**
* JSON template for a website entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
rename = "type")]
pub type_: String,
/**
* JSON template for a website entry.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct Users {
/**
* ETag of the resource.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* Kind of resource this is.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* Token used to access next page of this result.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub next_page_token: String,
/**
* Event that triggered this response (only used in case of Push Response)
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub trigger_event: String,
/**
* List of user objects.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub users: Vec<User>,
}

/// The Directory API allows you to view, generate, and invalidate backup verification codes for a user.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct VerificationCode {
/**
* The Directory API allows you to view, generate, and invalidate backup verification codes for a user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* The Directory API allows you to view, generate, and invalidate backup verification codes for a user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
/**
* The Directory API allows you to view, generate, and invalidate backup verification codes for a user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub user_id: String,
/**
* The Directory API allows you to view, generate, and invalidate backup verification codes for a user.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub verification_code: String,
}

/// JSON response template for List verification codes operation in Directory API.
#[derive(Serialize, Deserialize, Debug, Default, Clone, JsonSchema)]
pub struct VerificationCodes {
/**
* JSON response template for List verification codes operation in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub etag: String,
/**
* JSON response template for List verification codes operation in Directory API.
*/
#[serde(default,
skip_serializing_if = "Vec::is_empty",
)]
pub items: Vec<VerificationCode>,
/**
* JSON response template for List verification codes operation in Directory API.
*/
#[serde(default,
skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
)]
pub kind: String,
}

/**
* V1 error format.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Xgafv {
#[serde(rename = "1")]
One,
#[serde(rename = "2")]
Two,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for Xgafv {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
Xgafv::One => "1",
Xgafv::Two => "2",
Xgafv::Noop => "",
Xgafv::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for Xgafv {
fn default() -> Xgafv {
Xgafv::Noop
}
}
impl Xgafv {
            pub fn is_noop(&self) -> bool {
                matches!(self, Xgafv::Noop)
            }
    }


/**
* Data format for response.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Alt {
#[serde(rename = "json")]
Json,
#[serde(rename = "media")]
Media,
#[serde(rename = "proto")]
Proto,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for Alt {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
Alt::Json => "json",
Alt::Media => "media",
Alt::Proto => "proto",
Alt::Noop => "",
Alt::FallthroughString(s) => s,
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
* Device property to use for sorting results.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrderBy {
#[serde(rename = "annotatedLocation")]
AnnotatedLocation,
#[serde(rename = "annotatedUser")]
AnnotatedUser,
#[serde(rename = "lastSync")]
LastSync,
#[serde(rename = "notes")]
Notes,
#[serde(rename = "serialNumber")]
SerialNumber,
#[serde(rename = "status")]
Status,
#[serde(rename = "supportEndDate")]
SupportEndDate,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for OrderBy {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
OrderBy::AnnotatedLocation => "annotatedLocation",
OrderBy::AnnotatedUser => "annotatedUser",
OrderBy::LastSync => "lastSync",
OrderBy::Notes => "notes",
OrderBy::SerialNumber => "serialNumber",
OrderBy::Status => "status",
OrderBy::SupportEndDate => "supportEndDate",
OrderBy::Noop => "",
OrderBy::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for OrderBy {
fn default() -> OrderBy {
OrderBy::Noop
}
}
impl OrderBy {
            pub fn is_noop(&self) -> bool {
                matches!(self, OrderBy::Noop)
            }
    }


/**
* Restrict information returned to a set of selected fields.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Projection {
#[serde(rename = "BASIC")]
Basic,
#[serde(rename = "FULL")]
Full,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for Projection {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
Projection::Basic => "BASIC",
Projection::Full => "FULL",
Projection::Noop => "",
Projection::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for Projection {
fn default() -> Projection {
Projection::Noop
}
}
impl Projection {
            pub fn is_noop(&self) -> bool {
                matches!(self, Projection::Noop)
            }
    }


/**
* Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SortOrder {
#[serde(rename = "ASCENDING")]
Ascending,
#[serde(rename = "DESCENDING")]
Descending,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for SortOrder {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
SortOrder::Ascending => "ASCENDING",
SortOrder::Descending => "DESCENDING",
SortOrder::Noop => "",
SortOrder::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for SortOrder {
fn default() -> SortOrder {
SortOrder::Noop
}
}
impl SortOrder {
            pub fn is_noop(&self) -> bool {
                matches!(self, SortOrder::Noop)
            }
    }


/**
* Determines whether the response contains the full list of properties or only a subset.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryChromeosdevicesGetProjection {
#[serde(rename = "BASIC")]
Basic,
#[serde(rename = "FULL")]
Full,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryChromeosdevicesGetProjection {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryChromeosdevicesGetProjection::Basic => "BASIC",
DirectoryChromeosdevicesGetProjection::Full => "FULL",
DirectoryChromeosdevicesGetProjection::Noop => "",
DirectoryChromeosdevicesGetProjection::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryChromeosdevicesGetProjection {
fn default() -> DirectoryChromeosdevicesGetProjection {
DirectoryChromeosdevicesGetProjection::Noop
}
}
impl DirectoryChromeosdevicesGetProjection {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryChromeosdevicesGetProjection::Noop)
            }
    }


/**
* Device property to use for sorting results.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryMobiledevicesListOrderBy {
#[serde(rename = "deviceId")]
DeviceId,
#[serde(rename = "email")]
Email,
#[serde(rename = "lastSync")]
LastSync,
#[serde(rename = "model")]
Model,
#[serde(rename = "name")]
Name,
#[serde(rename = "os")]
Os,
#[serde(rename = "status")]
Status,
#[serde(rename = "type")]
Type,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryMobiledevicesListOrderBy {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryMobiledevicesListOrderBy::DeviceId => "deviceId",
DirectoryMobiledevicesListOrderBy::Email => "email",
DirectoryMobiledevicesListOrderBy::LastSync => "lastSync",
DirectoryMobiledevicesListOrderBy::Model => "model",
DirectoryMobiledevicesListOrderBy::Name => "name",
DirectoryMobiledevicesListOrderBy::Os => "os",
DirectoryMobiledevicesListOrderBy::Status => "status",
DirectoryMobiledevicesListOrderBy::Type => "type",
DirectoryMobiledevicesListOrderBy::Noop => "",
DirectoryMobiledevicesListOrderBy::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryMobiledevicesListOrderBy {
fn default() -> DirectoryMobiledevicesListOrderBy {
DirectoryMobiledevicesListOrderBy::Noop
}
}
impl DirectoryMobiledevicesListOrderBy {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryMobiledevicesListOrderBy::Noop)
            }
    }


/**
* Whether to return all sub-organizations or just immediate children.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryOrgunitsListType {
#[serde(rename = "all")]
All,
#[serde(rename = "children")]
Children,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryOrgunitsListType {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryOrgunitsListType::All => "all",
DirectoryOrgunitsListType::Children => "children",
DirectoryOrgunitsListType::Noop => "",
DirectoryOrgunitsListType::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryOrgunitsListType {
fn default() -> DirectoryOrgunitsListType {
DirectoryOrgunitsListType::Noop
}
}
impl DirectoryOrgunitsListType {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryOrgunitsListType::Noop)
            }
    }


/**
* Source from which Building.coordinates are derived.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CoordinatesSource {
#[serde(rename = "CLIENT_SPECIFIED")]
ClientSpecified,
#[serde(rename = "RESOLVED_FROM_ADDRESS")]
ResolvedFromAddress,
#[serde(rename = "SOURCE_UNSPECIFIED")]
SourceUnspecified,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for CoordinatesSource {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
CoordinatesSource::ClientSpecified => "CLIENT_SPECIFIED",
CoordinatesSource::ResolvedFromAddress => "RESOLVED_FROM_ADDRESS",
CoordinatesSource::SourceUnspecified => "SOURCE_UNSPECIFIED",
CoordinatesSource::Noop => "",
CoordinatesSource::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for CoordinatesSource {
fn default() -> CoordinatesSource {
CoordinatesSource::Noop
}
}
impl CoordinatesSource {
            pub fn is_noop(&self) -> bool {
                matches!(self, CoordinatesSource::Noop)
            }
    }


/**
* Column to use for sorting results
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryGroupsListOrderBy {
#[serde(rename = "email")]
Email,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryGroupsListOrderBy {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryGroupsListOrderBy::Email => "email",
DirectoryGroupsListOrderBy::Noop => "",
DirectoryGroupsListOrderBy::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryGroupsListOrderBy {
fn default() -> DirectoryGroupsListOrderBy {
DirectoryGroupsListOrderBy::Noop
}
}
impl DirectoryGroupsListOrderBy {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryGroupsListOrderBy::Noop)
            }
    }


/**
* Whether to return results in ascending or descending order. Only of use when orderBy is also used
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryGroupsListSortOrder {
#[serde(rename = "ASCENDING")]
Ascending,
#[serde(rename = "DESCENDING")]
Descending,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryGroupsListSortOrder {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryGroupsListSortOrder::Ascending => "ASCENDING",
DirectoryGroupsListSortOrder::Descending => "DESCENDING",
DirectoryGroupsListSortOrder::Noop => "",
DirectoryGroupsListSortOrder::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryGroupsListSortOrder {
fn default() -> DirectoryGroupsListSortOrder {
DirectoryGroupsListSortOrder::Noop
}
}
impl DirectoryGroupsListSortOrder {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryGroupsListSortOrder::Noop)
            }
    }


/**
* Event on which subscription is intended (if subscribing)
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Event {
#[serde(rename = "add")]
Add,
#[serde(rename = "delete")]
Delete,
#[serde(rename = "makeAdmin")]
MakeAdmin,
#[serde(rename = "undelete")]
Undelete,
#[serde(rename = "update")]
Update,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for Event {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
Event::Add => "add",
Event::Delete => "delete",
Event::MakeAdmin => "makeAdmin",
Event::Undelete => "undelete",
Event::Update => "update",
Event::Noop => "",
Event::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for Event {
fn default() -> Event {
Event::Noop
}
}
impl Event {
            pub fn is_noop(&self) -> bool {
                matches!(self, Event::Noop)
            }
    }


/**
* Property to use for sorting results.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryUsersListOrderBy {
#[serde(rename = "email")]
Email,
#[serde(rename = "familyName")]
FamilyName,
#[serde(rename = "givenName")]
GivenName,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryUsersListOrderBy {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryUsersListOrderBy::Email => "email",
DirectoryUsersListOrderBy::FamilyName => "familyName",
DirectoryUsersListOrderBy::GivenName => "givenName",
DirectoryUsersListOrderBy::Noop => "",
DirectoryUsersListOrderBy::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryUsersListOrderBy {
fn default() -> DirectoryUsersListOrderBy {
DirectoryUsersListOrderBy::Noop
}
}
impl DirectoryUsersListOrderBy {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryUsersListOrderBy::Noop)
            }
    }


/**
* What subset of fields to fetch for this user.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryUsersListProjection {
#[serde(rename = "basic")]
Basic,
#[serde(rename = "custom")]
Custom,
#[serde(rename = "full")]
Full,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryUsersListProjection {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryUsersListProjection::Basic => "basic",
DirectoryUsersListProjection::Custom => "custom",
DirectoryUsersListProjection::Full => "full",
DirectoryUsersListProjection::Noop => "",
DirectoryUsersListProjection::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryUsersListProjection {
fn default() -> DirectoryUsersListProjection {
DirectoryUsersListProjection::Noop
}
}
impl DirectoryUsersListProjection {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryUsersListProjection::Noop)
            }
    }


/**
* Whether to return results in ascending or descending order.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryUsersListSortOrder {
#[serde(rename = "ASCENDING")]
Ascending,
#[serde(rename = "DESCENDING")]
Descending,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryUsersListSortOrder {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryUsersListSortOrder::Ascending => "ASCENDING",
DirectoryUsersListSortOrder::Descending => "DESCENDING",
DirectoryUsersListSortOrder::Noop => "",
DirectoryUsersListSortOrder::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryUsersListSortOrder {
fn default() -> DirectoryUsersListSortOrder {
DirectoryUsersListSortOrder::Noop
}
}
impl DirectoryUsersListSortOrder {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryUsersListSortOrder::Noop)
            }
    }


/**
* Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin).
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ViewType {
#[serde(rename = "admin_view")]
AdminView,
#[serde(rename = "domain_public")]
DomainPublic,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for ViewType {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
ViewType::AdminView => "admin_view",
ViewType::DomainPublic => "domain_public",
ViewType::Noop => "",
ViewType::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for ViewType {
fn default() -> ViewType {
ViewType::Noop
}
}
impl ViewType {
            pub fn is_noop(&self) -> bool {
                matches!(self, ViewType::Noop)
            }
    }


/**
* Events to watch for.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryUsersWatchEvent {
#[serde(rename = "add")]
Add,
#[serde(rename = "delete")]
Delete,
#[serde(rename = "makeAdmin")]
MakeAdmin,
#[serde(rename = "undelete")]
Undelete,
#[serde(rename = "update")]
Update,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryUsersWatchEvent {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryUsersWatchEvent::Add => "add",
DirectoryUsersWatchEvent::Delete => "delete",
DirectoryUsersWatchEvent::MakeAdmin => "makeAdmin",
DirectoryUsersWatchEvent::Undelete => "undelete",
DirectoryUsersWatchEvent::Update => "update",
DirectoryUsersWatchEvent::Noop => "",
DirectoryUsersWatchEvent::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryUsersWatchEvent {
fn default() -> DirectoryUsersWatchEvent {
DirectoryUsersWatchEvent::Noop
}
}
impl DirectoryUsersWatchEvent {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryUsersWatchEvent::Noop)
            }
    }


/**
* Column to use for sorting results
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryUsersWatchOrderBy {
#[serde(rename = "email")]
Email,
#[serde(rename = "familyName")]
FamilyName,
#[serde(rename = "givenName")]
GivenName,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryUsersWatchOrderBy {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryUsersWatchOrderBy::Email => "email",
DirectoryUsersWatchOrderBy::FamilyName => "familyName",
DirectoryUsersWatchOrderBy::GivenName => "givenName",
DirectoryUsersWatchOrderBy::Noop => "",
DirectoryUsersWatchOrderBy::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryUsersWatchOrderBy {
fn default() -> DirectoryUsersWatchOrderBy {
DirectoryUsersWatchOrderBy::Noop
}
}
impl DirectoryUsersWatchOrderBy {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryUsersWatchOrderBy::Noop)
            }
    }


/**
* Events to watch for.
*/
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum DirectoryUsersAliasesListEvent {
#[serde(rename = "add")]
Add,
#[serde(rename = "delete")]
Delete,
#[serde(rename = "")]
Noop,
FallthroughString(String),
}

impl std::fmt::Display for DirectoryUsersAliasesListEvent {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match &*self {
DirectoryUsersAliasesListEvent::Add => "add",
DirectoryUsersAliasesListEvent::Delete => "delete",
DirectoryUsersAliasesListEvent::Noop => "",
DirectoryUsersAliasesListEvent::FallthroughString(s) => s,
}
.fmt(f)
}
}

impl Default for DirectoryUsersAliasesListEvent {
fn default() -> DirectoryUsersAliasesListEvent {
DirectoryUsersAliasesListEvent::Noop
}
}
impl DirectoryUsersAliasesListEvent {
            pub fn is_noop(&self) -> bool {
                matches!(self, DirectoryUsersAliasesListEvent::Noop)
            }
    }


