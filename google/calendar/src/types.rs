//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Acl {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * List of rules on the access control list.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AclRule>,
    /**
     * Type of the collection ("calendar#acl").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page_token: String,
    /**
     * Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sync_token: String,
}

/// The extent to which calendar access is granted by this ACL rule.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Scope {
    /**
     * The extent to which calendar access is granted by this ACL rule.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * The extent to which calendar access is granted by this ACL rule.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AclRule {
    /**
     * ETag of the resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * Identifier of the Access Control List (ACL) rule. See Sharing calendars.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Type of the resource ("calendar#aclRule").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * The role assigned to the scope. Possible values are:  
     *  - "none" - Provides no access.
     *  - "freeBusyReader" - Provides read access to free/busy information.
     *  - "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden.
     *  - "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible.
     *  - "owner" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
    /**
     * The extent to which calendar access is granted by this ACL rule.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Calendar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_properties: Option<ConferenceProperties>,
    /**
     * Description of the calendar. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * ETag of the resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * Identifier of the calendar. To retrieve IDs call the calendarList.list() method.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Type of the resource ("calendar#calendar").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Geographic location of the calendar as free-form text. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    /**
     * Title of the calendar.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CalendarList {
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * Calendars that are present on the user's calendar list.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CalendarListEntry>,
    /**
     * Type of the collection ("calendar#calendarList").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page_token: String,
    /**
     * Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sync_token: String,
}

/// The notifications that the authenticated user is receiving for this calendar.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct NotificationSettings {
    /**
     * The notifications that the authenticated user is receiving for this calendar.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notifications: Vec<CalendarNotification>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CalendarListEntry {
    /**
     * The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  
     *  - "freeBusyReader" - Provides read access to free/busy information.
     *  - "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden.
     *  - "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible.
     *  - "owner" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_role: String,
    /**
     * The main color of the calendar in the hexadecimal format "#0088aa". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub background_color: String,
    /**
     * The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_properties: Option<ConferenceProperties>,
    /**
     * The default reminders that the authenticated user has for this calendar.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_reminders: Vec<EventReminder>,
    /**
     * Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub deleted: bool,
    /**
     * Description of the calendar. Optional. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * ETag of the resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * The foreground color of the calendar in the hexadecimal format "#ffffff". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub foreground_color: String,
    /**
     * Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub hidden: bool,
    /**
     * Identifier of the calendar.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Type of the resource ("calendar#calendarListEntry").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Geographic location of the calendar as free-form text. Optional. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    /**
     * The notifications that the authenticated user is receiving for this calendar.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<NotificationSettings>,
    /**
     * Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub primary: bool,
    /**
     * Whether the calendar content shows up in the calendar UI. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub selected: bool,
    /**
     * Title of the calendar. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * The summary that the authenticated user has set for this calendar. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary_override: String,
    /**
     * The time zone of the calendar. Optional. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CalendarNotification {
    /**
     * The method used to deliver the notification. The possible value is:  
     *  - "email" - Notifications are sent via email.  
     *  Required when adding a notification.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub method: String,
    /**
     * The type of notification. Possible values are:  
     *  - "eventCreation" - Notification sent when a new event is put on the calendar.
     *  - "eventChange" - Notification sent when an event is changed.
     *  - "eventCancellation" - Notification sent when an event is cancelled.
     *  - "eventResponse" - Notification sent when an attendee responds to the event invitation.
     *  - "agenda" - An agenda with the events of the day (sent out in the morning).  
     *  Required when adding a notification.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// Additional parameters controlling delivery channel behavior. Optional.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Params {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Channel {
    /**
     * The address where notifications are delivered for this channel.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    /**
     * Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub expiration: i64,
    /**
     * A UUID or similar unique string that identifies this channel.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel".
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Additional parameters controlling delivery channel behavior. Optional.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
    /**
     * A Boolean value to indicate whether payload is wanted. Optional.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub payload: bool,
    /**
     * An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource_id: String,
    /**
     * A version-specific identifier for the watched resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource_uri: String,
    /**
     * An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    /**
     * The type of delivery mechanism used for this channel. Valid values are "web_hook" (or "webhook"). Both values refer to a channel where Http requests are used to deliver messages.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ColorDefinition {
    /**
     * The background color associated with this color definition.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub background: String,
    /**
     * The foreground color that can be used to write on top of a background with 'background' color.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub foreground: String,
}

/// A global palette of calendar colors, mapping from the color ID to its definition. A calendarListEntry resource refers to one of these color IDs in its colorId field. Read-only.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ColorsCalendar {}

/// A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Event {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Colors {
    /**
     * A global palette of calendar colors, mapping from the color ID to its definition. A calendarListEntry resource refers to one of these color IDs in its colorId field. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<ColorsCalendar>,
    /**
     * A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    /**
     * Type of the resource ("calendar#colors").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Last modification time of the color palette (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceData {
    /**
     * The ID of the conference.
     *  Can be used by developers to keep track of conferences, should not be displayed to users.
     *  The ID value is formed differently for each conference solution type:  
     *  - eventHangout: ID is not set. (This conference type is deprecated.)
     *  - eventNamedHangout: ID is the name of the Hangout. (This conference type is deprecated.)
     *  - hangoutsMeet: ID is the 10-letter meeting code, for example aaa-bbbb-ccc.
     *  - addOn: ID is defined by the third-party provider.  Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conference_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_solution: Option<ConferenceSolution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_request: Option<CreateConferenceRequest>,
    /**
     * Information about individual conference entry points, such as URLs or phone numbers.
     *  All of them must belong to the same conference.
     *  Either conferenceSolution and at least one entryPoint, or createRequest is required.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_points: Vec<EntryPoint>,
    /**
     * Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notes: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ConferenceParameters>,
    /**
     * The signature of the conference data.
     *  Generated on server side. Must be preserved while copying the conference data between events, otherwise the conference data will not be copied.
     *  Unset for a conference with a failed create request.
     *  Optional for a conference with a pending create request.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_on_parameters: Option<ConferenceParametersAddOn>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Parameters {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceParametersAddOn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceProperties {
    /**
     * The types of conference solutions that are supported for this calendar.
     *  The possible values are:  
     *  - "eventHangout"
     *  - "eventNamedHangout"
     *  - "hangoutsMeet"  Optional.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_conference_solution_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceRequestStatus {
    /**
     * The current status of the conference create request. Read-only.
     *  The possible values are:  
     *  - "pending": the conference create request is still being processed.
     *  - "success": the conference create request succeeded, the entry points are populated.
     *  - "failure": the conference create request failed, there are no entry points.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceSolution {
    /**
     * The user-visible icon for this solution.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub icon_uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<ConferenceSolutionKey>,
    /**
     * The user-visible name of this solution. Not localized.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConferenceSolutionKey {
    /**
     * The conference solution type.
     *  If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
     *  The possible values are:  
     *  - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
     *  - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
     *  - "hangoutsMeet" for Google Meet (http://meet.google.com)
     *  - "addOn" for 3P conference providers
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CreateConferenceRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_solution_key: Option<ConferenceSolutionKey>,
    /**
     * The client-generated unique ID for this request.
     *  Clients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub request_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ConferenceRequestStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EntryPoint {
    /**
     * The access code to access the conference. The maximum length is 128 characters.
     *  When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
     *  Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_code: String,
    /**
     * Features of the entry point, such as being toll or toll-free. One entry point can have multiple features. However, toll and toll-free cannot be both set on the same entry point.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_point_features: Vec<String>,
    /**
     * The type of the conference entry point.
     *  Possible values are:  
     *  - "video" - joining a conference over HTTP. A conference can have zero or one video entry point.
     *  - "phone" - joining a conference by dialing a phone number. A conference can have zero or more phone entry points.
     *  - "sip" - joining a conference over SIP. A conference can have zero or one sip entry point.
     *  - "more" - further conference joining instructions, for example additional phone numbers. A conference can have zero or one more entry point. A conference with only a more entry point is not a valid conference.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub entry_point_type: String,
    /**
     * The label for the URI. Visible to end users. Not localized. The maximum length is 512 characters.
     *  Examples:  
     *  - for video: meet.google.com/aaa-bbbb-ccc
     *  - for phone: +1 123 268 2601
     *  - for sip: 12345678@altostrat.com
     *  - for more: should not be filled  
     *  Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    /**
     * The meeting code to access the conference. The maximum length is 128 characters.
     *  When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
     *  Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub meeting_code: String,
    /**
     * The passcode to access the conference. The maximum length is 128 characters.
     *  When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub passcode: String,
    /**
     * The password to access the conference. The maximum length is 128 characters.
     *  When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
     *  Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub password: String,
    /**
     * The PIN to access the conference. The maximum length is 128 characters.
     *  When creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.
     *  Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pin: String,
    /**
     * The CLDR/ISO 3166 region code for the country associated with this phone access. Example: "SE" for Sweden.
     *  Calendar backend will populate this field only for EntryPointType.PHONE.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub region_code: String,
    /**
     * The URI of the entry point. The maximum length is 1300 characters.
     *  Format:  
     *  - for video, http: or https: schema is required.
     *  - for phone, tel: schema is required. The URI should include the entire dial sequence (e.g., tel:+12345678900,,,123456789;1234).
     *  - for sip, sip: schema is required, e.g., sip:12345678@myprovider.com.
     *  - for more, http: or https: schema is required.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Error {
    /**
     * Domain, or broad category, of the error.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,
    /**
     * Specific reason for the error. Some of the possible values are:  
     *  - "groupTooBig" - The group of users requested is too large for a single query.
     *  - "tooManyCalendarsRequested" - The number of calendars requested is too large for a single query.
     *  - "notFound" - The requested resource was not found.
     *  - "internalError" - The API service has encountered an internal error.  Additional error types may be added in the future, so clients should gracefully handle additional error statuses not included in this list.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
}

/// The creator of the event. Read-only.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Creator {
    /**
     * The creator of the event. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * The creator of the event. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The creator of the event. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The creator of the event. Read-only.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "self"
    )]
    pub self_: bool,
}

/// Properties that are private to the copy of the event that appears on this calendar.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Private {}

/// Properties that are shared between copies of the event on other attendees' calendars.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Shared {}

/// Extended properties of the event.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ExtendedProperties {
    /**
     * Extended properties of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<Private>,
    /**
     * Extended properties of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared: Option<Shared>,
}

/// Preferences.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Preferences {}

/// A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Gadget {
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub height: i64,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub icon_link: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub link: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub width: i64,
}

/// The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Organizer {
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "self"
    )]
    pub self_: bool,
}

/// Information about the event's reminders for the authenticated user.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Reminders {
    /**
     * Information about the event's reminders for the authenticated user.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overrides: Vec<EventReminder>,
    /**
     * Information about the event's reminders for the authenticated user.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub use_default: bool,
}

/// Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Source {
    /**
     * Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventData {
    /**
     * Whether anyone can invite themselves to the event (deprecated). Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub anyone_can_add_self: bool,
    /**
     * File attachments for the event. Currently only Google Drive attachments are supported.
     *  In order to modify attachments the supportsAttachments request parameter should be set to true.
     *  There can be at most 25 attachments per event,
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<EventAttachment>,
    /**
     * The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attendees: Vec<EventAttendee>,
    /**
     * Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub attendees_omitted: bool,
    /**
     * The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_data: Option<ConferenceData>,
    /**
     * Creation time of the event (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The creator of the event. Read-only.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /**
     * Description of the event. Can contain HTML. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<EventDateTime>,
    /**
     * Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub end_time_unspecified: bool,
    /**
     * ETag of the resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * Specific type of the event. Read-only. Possible values are:  
     *  - "default" - A regular event or not further specified.
     *  - "outOfOffice" - An out-of-office event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event_type: String,
    /**
     * Extended properties of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<ExtendedProperties>,
    /**
     * A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gadget: Option<Gadget>,
    /**
     * Whether attendees other than the organizer can invite others to the event. Optional. The default is True.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub guests_can_invite_others: bool,
    /**
     * Whether attendees other than the organizer can modify the event. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub guests_can_modify: bool,
    /**
     * Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub guests_can_see_other_guests: bool,
    /**
     * An absolute link to the Google Hangout associated with this event. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hangout_link: String,
    /**
     * An absolute link to this event in the Google Calendar Web UI. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_link: String,
    /**
     * Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.
     *  Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub i_cal_uid: String,
    /**
     * Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  
     *  - characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938
     *  - the length of the ID must be between 5 and 1024 characters
     *  - the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.
     *  If you do not specify an ID, it will be automatically generated by the server.
     *  Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Type of the resource ("calendar#event").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Geographic location of the event as free-form text. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    /**
     * Whether this is a locked event copy where no changes can be made to the main event fields "summary", "description", "location", "start", "end" or "recurrence". The default is False. Read-Only.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub locked: bool,
    /**
     * The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizer: Option<Organizer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_time: Option<EventDateTime>,
    /**
     * If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub private_copy: bool,
    /**
     * List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recurrence: Vec<String>,
    /**
     * For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub recurring_event_id: String,
    /**
     * Information about the event's reminders for the authenticated user.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Reminders>,
    /**
     * Sequence number as per iCalendar.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub sequence: i64,
    /**
     * Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<EventDateTime>,
    /**
     * Status of the event. Optional. Possible values are:  
     *  - "confirmed" - The event is confirmed. This is the default status.
     *  - "tentative" - The event is tentatively confirmed.
     *  - "cancelled" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.
     *  A cancelled status represents two different states depending on the event type:  
     *  - Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.
     *  Cancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  
     *  - All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.
     *  Deleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.
     *  If an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    /**
     * Title of the event.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * Whether the event blocks time on the calendar. Optional. Possible values are:  
     *  - "opaque" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI.
     *  - "transparent" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub transparency: String,
    /**
     * Last modification time of the event (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Visibility of the event. Optional. Possible values are:  
     *  - "default" - Uses the default visibility for events on the calendar. This is the default value.
     *  - "public" - The event is public and event details are visible to all readers of the calendar.
     *  - "private" - The event is private and only event attendees may view event details.
     *  - "confidential" - The event is private. This value is provided for compatibility reasons.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventAttachment {
    /**
     * ID of the attached file. Read-only.
     *  For Google Drive files, this is the ID of the corresponding Files resource entry in the Drive API.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub file_id: String,
    /**
     * URL link to the attachment.
     *  For adding Google Drive file attachments use the same format as in alternateLink property of the Files resource in the Drive API.
     *  Required when adding an attachment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub file_url: String,
    /**
     * URL link to the attachment's icon. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub icon_link: String,
    /**
     * Internet media type (MIME type) of the attachment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mime_type: String,
    /**
     * Attachment title.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventAttendee {
    /**
     * Number of additional guests. Optional. The default is 0.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub additional_guests: i64,
    /**
     * The attendee's response comment. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comment: String,
    /**
     * The attendee's name, if available. Optional.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * The attendee's email address, if available. This field must be present when adding an attendee. It must be a valid email address as per RFC5322.
     *  Required when adding an attendee.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The attendee's Profile ID, if available.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Whether this is an optional attendee. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub optional: bool,
    /**
     * Whether the attendee is the organizer of the event. Read-only. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub organizer: bool,
    /**
     * Whether the attendee is a resource. Can only be set when the attendee is added to the event for the first time. Subsequent modifications are ignored. Optional. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub resource: bool,
    /**
     * The attendee's response status. Possible values are:  
     *  - "needsAction" - The attendee has not responded to the invitation.
     *  - "declined" - The attendee has declined the invitation.
     *  - "tentative" - The attendee has tentatively accepted the invitation.
     *  - "accepted" - The attendee has accepted the invitation.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub response_status: String,
    /**
     * Whether this entry represents the calendar on which this copy of the event appears. Read-only. The default is False.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "self"
    )]
    pub self_: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventDateTime {
    /**
     * The date, in the format "yyyy-mm-dd", if this is an all-day event.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::NaiveDate>,
    /**
     * The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub date_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventReminder {
    /**
     * The method used by this reminder. Possible values are:  
     *  - "email" - Reminders are sent via email.
     *  - "popup" - Reminders are sent via a UI popup.  
     *  Required when adding a reminder.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub method: String,
    /**
     * Number of minutes before the start of the event when the reminder should trigger. Valid values are between 0 and 40320 (4 weeks in minutes).
     *  Required when adding a reminder.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub minutes: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Events {
    /**
     * The user's access role for this calendar. Read-only. Possible values are:  
     *  - "none" - The user has no access.
     *  - "freeBusyReader" - The user has read access to free/busy information.
     *  - "reader" - The user has read access to the calendar. Private events will appear to users with reader access, but event details will be hidden.
     *  - "writer" - The user has read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible.
     *  - "owner" - The user has ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_role: String,
    /**
     * The default reminders on the calendar for the authenticated user. These reminders apply to all events on this calendar that do not explicitly override them (i.e. do not have reminders.useDefault set to True).
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_reminders: Vec<EventReminder>,
    /**
     * Description of the calendar. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * ETag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * List of events on the calendar.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EventData>,
    /**
     * Type of the collection ("calendar#events").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page_token: String,
    /**
     * Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sync_token: String,
    /**
     * Title of the calendar. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * The time zone of the calendar. Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
    /**
     * Last modification time of the calendar (as a RFC3339 timestamp). Read-only.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyCalendar {
    /**
     * List of time ranges during which this calendar should be regarded as busy.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub busy: Vec<TimePeriod>,
    /**
     * Optional error(s) (if computation for the calendar failed).
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Error>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyGroup {
    /**
     * List of calendars' identifiers within a group.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calendars: Vec<String>,
    /**
     * Optional error(s) (if computation for the group failed).
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Error>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyRequest {
    /**
     * Maximal number of calendars for which FreeBusy information is to be provided. Optional. Maximum value is 50.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub calendar_expansion_max: i64,
    /**
     * Maximal number of calendar identifiers to be provided for a single group. Optional. An error is returned for a group with more members than this value. Maximum value is 100.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub group_expansion_max: i64,
    /**
     * List of calendars and/or groups to query.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<FreeBusyRequestItem>,
    /**
     * The end of the interval for the query formatted as per RFC3339.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_max: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The start of the interval for the query formatted as per RFC3339.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_min: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Time zone used in the response. Optional. The default is UTC.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyRequestItem {
    /**
     * The identifier of a calendar or a group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
}

/// List of free/busy information for calendars.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Calendars {}

/// Expansion of groups.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Groups {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FreeBusyResponse {
    /**
     * List of free/busy information for calendars.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendars: Option<Calendars>,
    /**
     * Expansion of groups.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Groups>,
    /**
     * Type of the resource ("calendar#freeBusy").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * The end of the interval.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_max: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The start of the interval.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_min: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Setting {
    /**
     * ETag of the resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * The id of the user setting.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Type of the resource ("calendar#setting").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Value of the user setting. The format of the value depends on the ID of the setting. It must always be a UTF-8 string of length up to 1024 characters.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Settings {
    /**
     * Etag of the collection.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub etag: String,
    /**
     * List of user settings.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Setting>,
    /**
     * Type of the collection ("calendar#settings").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page_token: String,
    /**
     * Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_sync_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimePeriod {
    /**
     * The (exclusive) end of the time period.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The (inclusive) start of the time period.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub start: Option<chrono::DateTime<chrono::Utc>>,
}

/**
 * Data format for the response.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Alt {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for Alt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Alt::Json => "json",
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
 * The order of the events returned in the result. Optional. The default is an unspecified, stable order.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrderBy {
    #[serde(rename = "startTime")]
    StartTime,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrderBy::StartTime => "startTime",
            OrderBy::Updated => "updated",
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
 * Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum SendUpdates {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "externalOnly")]
    ExternalOnly,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for SendUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SendUpdates::All => "all",
            SendUpdates::ExternalOnly => "externalOnly",
            SendUpdates::None => "none",
            SendUpdates::Noop => "",
            SendUpdates::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for SendUpdates {
    fn default() -> SendUpdates {
        SendUpdates::Noop
    }
}
impl SendUpdates {
    pub fn is_noop(&self) -> bool {
        matches!(self, SendUpdates::Noop)
    }
}

/**
 * Guests who should receive notifications about the creation of the new event.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CalendarEventsQuickAddSendUpdates {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "externalOnly")]
    ExternalOnly,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for CalendarEventsQuickAddSendUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CalendarEventsQuickAddSendUpdates::All => "all",
            CalendarEventsQuickAddSendUpdates::ExternalOnly => "externalOnly",
            CalendarEventsQuickAddSendUpdates::None => "none",
            CalendarEventsQuickAddSendUpdates::Noop => "",
            CalendarEventsQuickAddSendUpdates::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for CalendarEventsQuickAddSendUpdates {
    fn default() -> CalendarEventsQuickAddSendUpdates {
        CalendarEventsQuickAddSendUpdates::Noop
    }
}
impl CalendarEventsQuickAddSendUpdates {
    pub fn is_noop(&self) -> bool {
        matches!(self, CalendarEventsQuickAddSendUpdates::Noop)
    }
}

/**
 * Guests who should receive notifications about the event update (for example, title changes, etc.).
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CalendarEventsUpdateSendUpdates {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "externalOnly")]
    ExternalOnly,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for CalendarEventsUpdateSendUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CalendarEventsUpdateSendUpdates::All => "all",
            CalendarEventsUpdateSendUpdates::ExternalOnly => "externalOnly",
            CalendarEventsUpdateSendUpdates::None => "none",
            CalendarEventsUpdateSendUpdates::Noop => "",
            CalendarEventsUpdateSendUpdates::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for CalendarEventsUpdateSendUpdates {
    fn default() -> CalendarEventsUpdateSendUpdates {
        CalendarEventsUpdateSendUpdates::Noop
    }
}
impl CalendarEventsUpdateSendUpdates {
    pub fn is_noop(&self) -> bool {
        matches!(self, CalendarEventsUpdateSendUpdates::Noop)
    }
}

/**
 * Guests who should receive notifications about the deletion of the event.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CalendarEventsDeleteSendUpdates {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "externalOnly")]
    ExternalOnly,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for CalendarEventsDeleteSendUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CalendarEventsDeleteSendUpdates::All => "all",
            CalendarEventsDeleteSendUpdates::ExternalOnly => "externalOnly",
            CalendarEventsDeleteSendUpdates::None => "none",
            CalendarEventsDeleteSendUpdates::Noop => "",
            CalendarEventsDeleteSendUpdates::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for CalendarEventsDeleteSendUpdates {
    fn default() -> CalendarEventsDeleteSendUpdates {
        CalendarEventsDeleteSendUpdates::Noop
    }
}
impl CalendarEventsDeleteSendUpdates {
    pub fn is_noop(&self) -> bool {
        matches!(self, CalendarEventsDeleteSendUpdates::Noop)
    }
}

/**
 * Guests who should receive notifications about the change of the event's organizer.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum CalendarEventsMoveSendUpdates {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "externalOnly")]
    ExternalOnly,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for CalendarEventsMoveSendUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CalendarEventsMoveSendUpdates::All => "all",
            CalendarEventsMoveSendUpdates::ExternalOnly => "externalOnly",
            CalendarEventsMoveSendUpdates::None => "none",
            CalendarEventsMoveSendUpdates::Noop => "",
            CalendarEventsMoveSendUpdates::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for CalendarEventsMoveSendUpdates {
    fn default() -> CalendarEventsMoveSendUpdates {
        CalendarEventsMoveSendUpdates::Noop
    }
}
impl CalendarEventsMoveSendUpdates {
    pub fn is_noop(&self) -> bool {
        matches!(self, CalendarEventsMoveSendUpdates::Noop)
    }
}

/**
 * The minimum access role for the user in the returned entries. Optional. The default is no restriction.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum MinAccessRole {
    #[serde(rename = "freeBusyReader")]
    FreeBusyReader,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "reader")]
    Reader,
    #[serde(rename = "writer")]
    Writer,
    #[serde(rename = "")]
    Noop,
    FallthroughString(String),
}

impl std::fmt::Display for MinAccessRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MinAccessRole::FreeBusyReader => "freeBusyReader",
            MinAccessRole::Owner => "owner",
            MinAccessRole::Reader => "reader",
            MinAccessRole::Writer => "writer",
            MinAccessRole::Noop => "",
            MinAccessRole::FallthroughString(s) => s,
        }
        .fmt(f)
    }
}

impl Default for MinAccessRole {
    fn default() -> MinAccessRole {
        MinAccessRole::Noop
    }
}
impl MinAccessRole {
    pub fn is_noop(&self) -> bool {
        matches!(self, MinAccessRole::Noop)
    }
}
