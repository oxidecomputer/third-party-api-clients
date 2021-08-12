
#![feature(async_stream)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(test)]
mod tests;
pub mod types;
#[doc(hidden)]
pub mod utils;
/// Account operations.
pub mod accounts;
pub mod archiving;
/// Billing operations.
pub mod billing;
pub mod chat_channels;
pub mod chat_channels_account_level;
/// Chat Messages operations.
pub mod chat_messages;
pub mod chatbot_messages;
/// Contacts operations.
pub mod contacts;
/// Cloud Recording operations.
pub mod cloud_recording;
/// Dashboard operations.
pub mod dashboards;
pub mod data_compliance;
/// H323 Device operations.
pub mod devices;
/// Group operations.
pub mod groups;
/// IM Chat operations.
pub mod im_chat;
/// IM Group operations.
pub mod im_groups;
/// Meeting operations.
pub mod meetings;
/// PAC operations.
pub mod pac;
pub mod phone;
pub mod phone_auto_receptionists;
pub mod phone_blocked_list;
pub mod phone_call_queues;
pub mod phone_devices;
pub mod phone_reports;
pub mod phone_shared_line_groups;
pub mod phone_site;
pub mod common_area_phones;
/// Report operations.
pub mod reports;
/// User Roles.
pub mod roles;
/// ZoomRooms operations.
pub mod rooms;
pub mod rooms_account;
pub mod rooms_location;
pub mod sip_phone;
/// Tracking Field operations.
pub mod tracking_field;
/// TSP operations.
pub mod tsp;
/// User operations.
pub mod users;
/// Webhook operations.
pub mod webhooks;
/// Webinar operations.
pub mod webinars;
/// APIs available to the master account for a Zoom account.
pub mod master_account_ap_is;
pub mod rooms_devices;
pub mod sip_connected_audio;
pub mod deprecated_api_endpoints;

use anyhow::{anyhow, Error, Result};

pub const DEFAULT_HOST: &str = "https://api.zoom.us/v2";

mod progenitor_support {
    use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

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

    #[allow(dead_code)]
    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}



/// Account operations.
               pub fn accounts(&self) -> accounts::Accounts {
                    accounts::Accounts::new(self.clone())
               }

/// Return a reference to an interface that provides access to Archiving operations.
               pub fn archiving(&self) -> archiving::Archiving {
                    archiving::Archiving::new(self.clone())
               }

/// Billing operations.
               pub fn billing(&self) -> billing::Billing {
                    billing::Billing::new(self.clone())
               }

/// Return a reference to an interface that provides access to Chat Channels operations.
               pub fn chat_channels(&self) -> chat_channels::ChatChannels {
                    chat_channels::ChatChannels::new(self.clone())
               }

/// Return a reference to an interface that provides access to Chat Channels (Account-level) operations.
               pub fn chat_channels_account_level(&self) -> chat_channels_account_level::ChatChannelsAccountLevel {
                    chat_channels_account_level::ChatChannelsAccountLevel::new(self.clone())
               }

/// Chat Messages operations.
               pub fn chat_messages(&self) -> chat_messages::ChatMessages {
                    chat_messages::ChatMessages::new(self.clone())
               }

/// Return a reference to an interface that provides access to Chatbot Messages operations.
               pub fn chatbot_messages(&self) -> chatbot_messages::ChatbotMessages {
                    chatbot_messages::ChatbotMessages::new(self.clone())
               }

/// Contacts operations.
               pub fn contacts(&self) -> contacts::Contacts {
                    contacts::Contacts::new(self.clone())
               }

/// Cloud Recording operations.
               pub fn cloud_recording(&self) -> cloud_recording::CloudRecording {
                    cloud_recording::CloudRecording::new(self.clone())
               }

/// Dashboard operations.
               pub fn dashboards(&self) -> dashboards::Dashboards {
                    dashboards::Dashboards::new(self.clone())
               }

/// Return a reference to an interface that provides access to Data Compliance operations.
               pub fn data_compliance(&self) -> data_compliance::DataCompliance {
                    data_compliance::DataCompliance::new(self.clone())
               }

/// H323 Device operations.
               pub fn devices(&self) -> devices::Devices {
                    devices::Devices::new(self.clone())
               }

/// Group operations.
               pub fn groups(&self) -> groups::Groups {
                    groups::Groups::new(self.clone())
               }

/// IM Chat operations.
               pub fn im_chat(&self) -> im_chat::ImChat {
                    im_chat::ImChat::new(self.clone())
               }

/// IM Group operations.
               pub fn im_groups(&self) -> im_groups::ImGroups {
                    im_groups::ImGroups::new(self.clone())
               }

/// Meeting operations.
               pub fn meetings(&self) -> meetings::Meetings {
                    meetings::Meetings::new(self.clone())
               }

/// PAC operations.
               pub fn pac(&self) -> pac::Pac {
                    pac::Pac::new(self.clone())
               }

/// Return a reference to an interface that provides access to Phone operations.
               pub fn phone(&self) -> phone::Phone {
                    phone::Phone::new(self.clone())
               }

/// Return a reference to an interface that provides access to Phone Auto Receptionists operations.
               pub fn phone_auto_receptionists(&self) -> phone_auto_receptionists::PhoneAutoReceptionists {
                    phone_auto_receptionists::PhoneAutoReceptionists::new(self.clone())
               }

/// Return a reference to an interface that provides access to Phone Blocked List operations.
               pub fn phone_blocked_list(&self) -> phone_blocked_list::PhoneBlockedList {
                    phone_blocked_list::PhoneBlockedList::new(self.clone())
               }

/// Return a reference to an interface that provides access to Phone Call Queues operations.
               pub fn phone_call_queues(&self) -> phone_call_queues::PhoneCallQueues {
                    phone_call_queues::PhoneCallQueues::new(self.clone())
               }

/// Return a reference to an interface that provides access to Phone Devices operations.
               pub fn phone_devices(&self) -> phone_devices::PhoneDevices {
                    phone_devices::PhoneDevices::new(self.clone())
               }

/// Return a reference to an interface that provides access to Phone Reports operations.
               pub fn phone_reports(&self) -> phone_reports::PhoneReports {
                    phone_reports::PhoneReports::new(self.clone())
               }

/// Return a reference to an interface that provides access to Phone Shared Line Groups operations.
               pub fn phone_shared_line_groups(&self) -> phone_shared_line_groups::PhoneSharedLineGroups {
                    phone_shared_line_groups::PhoneSharedLineGroups::new(self.clone())
               }

/// Return a reference to an interface that provides access to Phone Site operations.
               pub fn phone_site(&self) -> phone_site::PhoneSite {
                    phone_site::PhoneSite::new(self.clone())
               }

/// Return a reference to an interface that provides access to Common Area Phones operations.
               pub fn common_area_phones(&self) -> common_area_phones::CommonAreaPhones {
                    common_area_phones::CommonAreaPhones::new(self.clone())
               }

/// Report operations.
               pub fn reports(&self) -> reports::Reports {
                    reports::Reports::new(self.clone())
               }

/// User Roles.
               pub fn roles(&self) -> roles::Roles {
                    roles::Roles::new(self.clone())
               }

/// ZoomRooms operations.
               pub fn rooms(&self) -> rooms::Rooms {
                    rooms::Rooms::new(self.clone())
               }

/// Return a reference to an interface that provides access to Rooms Account operations.
               pub fn rooms_account(&self) -> rooms_account::RoomsAccount {
                    rooms_account::RoomsAccount::new(self.clone())
               }

/// Return a reference to an interface that provides access to Rooms Location operations.
               pub fn rooms_location(&self) -> rooms_location::RoomsLocation {
                    rooms_location::RoomsLocation::new(self.clone())
               }

/// Return a reference to an interface that provides access to SIP Phone operations.
               pub fn sip_phone(&self) -> sip_phone::SipPhone {
                    sip_phone::SipPhone::new(self.clone())
               }

/// Tracking Field operations.
               pub fn tracking_field(&self) -> tracking_field::TrackingField {
                    tracking_field::TrackingField::new(self.clone())
               }

/// TSP operations.
               pub fn tsp(&self) -> tsp::Tsp {
                    tsp::Tsp::new(self.clone())
               }

/// User operations.
               pub fn users(&self) -> users::Users {
                    users::Users::new(self.clone())
               }

/// Webhook operations.
               pub fn webhooks(&self) -> webhooks::Webhooks {
                    webhooks::Webhooks::new(self.clone())
               }

/// Webinar operations.
               pub fn webinars(&self) -> webinars::Webinars {
                    webinars::Webinars::new(self.clone())
               }

/// APIs available to the master account for a Zoom account.
               pub fn master_account_ap_is(&self) -> master_account_ap_is::MasterAccountApIs {
                    master_account_ap_is::MasterAccountApIs::new(self.clone())
               }

/// Return a reference to an interface that provides access to Rooms Devices operations.
               pub fn rooms_devices(&self) -> rooms_devices::RoomsDevices {
                    rooms_devices::RoomsDevices::new(self.clone())
               }

/// Return a reference to an interface that provides access to SIP Connected Audio operations.
               pub fn sip_connected_audio(&self) -> sip_connected_audio::SipConnectedAudio {
                    sip_connected_audio::SipConnectedAudio::new(self.clone())
               }

/// Return a reference to an interface that provides access to Deprecated API Endpoints operations.
               pub fn deprecated_api_endpoints(&self) -> deprecated_api_endpoints::DeprecatedApiEndpoints {
                    deprecated_api_endpoints::DeprecatedApiEndpoints::new(self.clone())
               }

}
