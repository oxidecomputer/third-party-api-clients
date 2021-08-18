//! A fully generated, opinionated API client library for SendGrid.
//!
//! ## API Details
//!
//! The Beta endpoints for the new Email Activity APIs - functionality is subject to change without notice. You may not have access to this Beta endpoint.
//!
//! Email Activity offers filtering and search by event type for two days worth of data. There is an optional add-on to store 60 days worth of data. This add-on also gives you access to the ability to download a CSV of the 60 days worth of email event data. The Beta endpoints for the new Email Activity APIs - functionality is subject to change without notice. You may not have access to this Beta endpoint.
//!
//! Email Activity offers filtering and search by event type for two days worth of data. There is an optional add-on to store 60 days worth of data. This add-on also gives you access to the ability to download a CSV of the 60 days worth of email event data.
//!
//!
//!
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [SendGrid OpenAPI
//! specs](https://raw.githubusercontent.com/sendgrid/sendgrid-oai/main/oai.json) based on API spec version ``. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! sendgrid_api = "0.2.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use sendgrid_api::Client;
//!
//! let sendgrid = Client::new(
//!     String::from("client-id"),
//!     String::from("client-secret"),
//!     String::from("redirect-uri"),
//!     String::from("token"),
//!     String::from("refresh-token"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `SENDGRID_CLIENT_ID`
//! - `SENDGRID_CLIENT_SECRET`
//! - `SENDGRID_REDIRECT_URI`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use sendgrid_api::Client;
//!
//! let sendgrid = Client::new_from_env(String::from("token"), String::from("refresh-token"));
//! ```
//!
//! It is okay to pass empty values for `token` and `refresh_token`. In
//! the initial state of the client, you will not know these values.
//!
//! To start off a fresh client and get a `token` and `refresh_token`, use the following.
//!
//! ```
//! use sendgrid_api::Client;
//!
//! async fn do_call() {
//!     let mut sendgrid = Client::new_from_env("", "");
//!
//!     // Get the URL to request consent from the user.
//!     // You can optionally pass in scopes. If none are provided, then the
//!     // resulting URL will not have any scopes.
//!     let user_consent_url = sendgrid.user_consent_url(&["some-scope".to_string()]);
//!
//!     // In your redirect URL capture the code sent and our state.
//!     // Send it along to the request for the token.
//!     let code = "thing-from-redirect-url";
//!     let state = "state-from-redirect-url";
//!     let mut access_token = sendgrid.get_access_token(code, state).await.unwrap();
//!
//!     // You can additionally refresh the access token with the following.
//!     // You must have a refresh token to be able to call this function.
//!     access_token = sendgrid.refresh_access_token().await.unwrap();
//! }
//! ```
#![feature(async_stream)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod alerts;
pub mod api_key_permissions;
pub mod api_keys;
pub mod blocks_api;
pub mod bounces_api;
pub mod campaigns_api;
pub mod cancel_scheduled_sends;
pub mod categories;
pub mod certificates;
pub mod contacts;
pub mod contacts_api_custom_fields;
pub mod contacts_api_lists;
pub mod contacts_api_recipients;
pub mod contacts_api_segments;
pub mod csv_ui_only;
pub mod custom_fields;
pub mod designs_api;
pub mod domain_authentication;
pub mod email_address_validation;
pub mod email_cname_records;
pub mod invalid_emails_api;
pub mod ip_access_management;
pub mod ip_addresses;
pub mod ip_pools;
pub mod ip_warmup;
pub mod link_branding;
pub mod lists;
pub mod mail_send;
pub mod marketing_campaigns_stats;
pub mod query;
pub mod reverse_dns;
pub mod segmenting_contacts;
pub mod segmenting_contacts_beta;
pub mod send_test_email;
pub mod sender_identities_api;
pub mod sender_verification;
pub mod senders;
pub mod settings_enforced_tls;
pub mod settings_inbound_parse;
pub mod settings_mail;
pub mod settings_partner;
pub mod settings_tracking;
pub mod single_sends;
pub mod single_sign_on_settings;
pub mod single_sign_on_teammates;
pub mod spam_reports_api;
pub mod stats;
pub mod subuser_monitor_settings;
pub mod subuser_statistics;
pub mod subusers_api;
pub mod suppressions;
pub mod suppressions_global;
pub mod suppressions_unsubscribe_groups;
pub mod teammates;
#[cfg(test)]
mod tests;
pub mod transactional_templates;
pub mod transactional_templates_versions;
pub mod types;
pub mod users_api;
#[doc(hidden)]
pub mod utils;
pub mod webhooks;

use anyhow::{anyhow, Error, Result};

pub const DEFAULT_HOST: &str = "https://api.sendgrid.us/v2";

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

    #[allow(dead_code)]
    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}

use std::env;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const TOKEN_ENDPOINT: &str = "https://sendgrid.us/oauth/token";
const USER_CONSENT_ENDPOINT: &str = "https://sendgrid.us/oauth/authorize";

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    token: String,
    // This will expire within a certain amount of time as determined by the
    // expiration date passed back in the initial request.
    refresh_token: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,

    client: reqwest::Client,
}

#[derive(Debug, JsonSchema, Clone, Default, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token_type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
    #[serde(default)]
    pub expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub refresh_token: String,
    #[serde(default, alias = "x_refresh_token_expires_in")]
    pub refresh_token_expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scope: String,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<I, K, R, T, Q>(
        client_id: I,
        client_secret: K,
        redirect_uri: R,
        token: T,
        refresh_token: Q,
    ) -> Self
    where
        I: ToString,
        K: ToString,
        R: ToString,
        T: ToString,
        Q: ToString,
    {
        let client = reqwest::Client::builder().build();
        match client {
            Ok(c) => {
                // We do not refresh the access token here since we leave that up to the
                // user to do so they can re-save it to their database.
                // TODO: But in the future we should save the expires in date and refresh it
                // if it needs to be refreshed.
                //
                Client {
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    redirect_uri: redirect_uri.to_string(),
                    token: token.to_string(),
                    refresh_token: refresh_token.to_string(),

                    client: c,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Create a new Client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key and your requests will work.
    /// We pass in the token and refresh token to the client so if you are storing
    /// it in a database, you can get it first.
    pub fn new_from_env<T, R>(token: T, refresh_token: R) -> Self
    where
        T: ToString,
        R: ToString,
    {
        let client_id = env::var("SENDGRID_CLIENT_ID").unwrap();
        let client_secret = env::var("SENDGRID_CLIENT_SECRET").unwrap();
        let redirect_uri = env::var("SENDGRID_REDIRECT_URI").unwrap();

        Client::new(client_id, client_secret, redirect_uri, token, refresh_token)
    }

    async fn url_and_auth(&self, uri: &str) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();

        let auth = format!("Bearer {}", self.token);
        parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
    }

    async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::Response> {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (DEFAULT_HOST.to_string() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if let Some(body) = body {
            //println!("Body: {:?}", String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap());
            req = req.body(body);
        }
        //println!("Request: {:?}", &req);
        Ok(req.send().await?)
    }

    async fn request<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, body).await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            //println!("response payload {}", String::from_utf8_lossy(&response_body));
            let parsed_response = if status == http::StatusCode::NO_CONTENT {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            /*println!("error status: {:?}, response payload: {}",
                status,
                String::from_utf8_lossy(&response_body),
            );*/

            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        content: &[u8],
        mime_type: &str,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (DEFAULT_HOST.to_string() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_bytes(mime_type.as_bytes()).unwrap(),
        );
        // We are likely uploading a file so add the right headers.
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-type"),
            reqwest::header::HeaderValue::from_static("application/octet-stream"),
        );
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-length"),
            reqwest::header::HeaderValue::from_bytes(format!("{}", content.len()).as_bytes())
                .unwrap(),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if content.len() > 1 {
            let b = bytes::Bytes::copy_from_slice(content);
            // We are uploading a file so add that as the body.
            req = req.body(b);
        }

        //println!("Request: {:?}", &req);
        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            //println!("response payload {}", String::from_utf8_lossy(&response_body));
            let parsed_response = if status == http::StatusCode::NO_CONTENT {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            /*println!("error status: {:?}, response payload: {}",
                status,
                String::from_utf8_lossy(&response_body),
            );*/

            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let r = self.request(method, uri, body).await?;
        Ok(r)
    }

    /// Return a user consent url with an optional set of scopes.
    /// If no scopes are provided, they will not be passed in the url.
    pub fn user_consent_url(&self, scopes: &[String]) -> String {
        let state = uuid::Uuid::new_v4();

        let url = format!(
            "{}?client_id={}&response_type=code&redirect_uri={}&state={}",
            USER_CONSENT_ENDPOINT, self.client_id, self.redirect_uri, state
        );

        if scopes.is_empty() {
            return url;
        }

        // Add the scopes.
        format!("{}&scope={}", url, scopes.join(" "))
    }

    /// Refresh an access token from a refresh token. Client must have a refresh token
    /// for this to work.
    pub async fn refresh_access_token(&mut self) -> Result<AccessToken> {
        if self.refresh_token.is_empty() {
            anyhow!("refresh token cannot be empty");
        }

        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh_token),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri),
        ];
        let client = reqwest::Client::new();
        let resp = client
            .post(TOKEN_ENDPOINT)
            .headers(headers)
            .form(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .unwrap();

        // Unwrap the response.
        let t: AccessToken = resp.json().await.unwrap();

        self.token = t.access_token.to_string();
        self.refresh_token = t.refresh_token.to_string();

        Ok(t)
    }

    /// Get an access token from the code returned by the URL paramter sent to the
    /// redirect URL.
    pub async fn get_access_token(&mut self, code: &str, state: &str) -> Result<AccessToken> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri),
            ("state", state),
        ];
        let client = reqwest::Client::new();
        let resp = client
            .post(TOKEN_ENDPOINT)
            .headers(headers)
            .form(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .unwrap();

        // Unwrap the response.
        let t: AccessToken = resp.json().await.unwrap();

        self.token = t.access_token.to_string();
        self.refresh_token = t.refresh_token.to_string();

        Ok(t)
    }

    #[allow(dead_code)]
    async fn get<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::GET,
            &(DEFAULT_HOST.to_string() + uri),
            message,
        )
        .await
    }

    #[allow(dead_code)]
    async fn get_all_pages<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        // TODO: implement this.
        self.request_entity(
            http::Method::GET,
            &(DEFAULT_HOST.to_string() + uri),
            message,
        )
        .await
    }

    #[allow(dead_code)]
    async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::POST,
            &(DEFAULT_HOST.to_string() + uri),
            message,
        )
        .await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PATCH,
            &(DEFAULT_HOST.to_string() + uri),
            message,
        )
        .await
    }

    #[allow(dead_code)]
    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PUT,
            &(DEFAULT_HOST.to_string() + uri),
            message,
        )
        .await
    }

    #[allow(dead_code)]
    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(DEFAULT_HOST.to_string() + uri),
            message,
        )
        .await
    }

    pub fn alerts(&self) -> alerts::Alerts {
        alerts::Alerts::new(self.clone())
    }

    pub fn api_key_permissions(&self) -> api_key_permissions::ApiKeyPermissions {
        api_key_permissions::ApiKeyPermissions::new(self.clone())
    }

    pub fn api_keys(&self) -> api_keys::ApiKeys {
        api_keys::ApiKeys::new(self.clone())
    }

    pub fn blocks_api(&self) -> blocks_api::BlocksApi {
        blocks_api::BlocksApi::new(self.clone())
    }

    pub fn bounces_api(&self) -> bounces_api::BouncesApi {
        bounces_api::BouncesApi::new(self.clone())
    }

    pub fn campaigns_api(&self) -> campaigns_api::CampaignsApi {
        campaigns_api::CampaignsApi::new(self.clone())
    }

    pub fn cancel_scheduled_sends(&self) -> cancel_scheduled_sends::CancelScheduledSends {
        cancel_scheduled_sends::CancelScheduledSends::new(self.clone())
    }

    pub fn categories(&self) -> categories::Categories {
        categories::Categories::new(self.clone())
    }

    pub fn certificates(&self) -> certificates::Certificates {
        certificates::Certificates::new(self.clone())
    }

    pub fn contacts(&self) -> contacts::Contacts {
        contacts::Contacts::new(self.clone())
    }

    pub fn contacts_api_custom_fields(
        &self,
    ) -> contacts_api_custom_fields::ContactsApiCustomFields {
        contacts_api_custom_fields::ContactsApiCustomFields::new(self.clone())
    }

    pub fn contacts_api_lists(&self) -> contacts_api_lists::ContactsApiLists {
        contacts_api_lists::ContactsApiLists::new(self.clone())
    }

    pub fn contacts_api_recipients(&self) -> contacts_api_recipients::ContactsApiRecipients {
        contacts_api_recipients::ContactsApiRecipients::new(self.clone())
    }

    pub fn contacts_api_segments(&self) -> contacts_api_segments::ContactsApiSegments {
        contacts_api_segments::ContactsApiSegments::new(self.clone())
    }

    pub fn csv_ui_only(&self) -> csv_ui_only::CsvUiOnly {
        csv_ui_only::CsvUiOnly::new(self.clone())
    }

    pub fn custom_fields(&self) -> custom_fields::CustomFields {
        custom_fields::CustomFields::new(self.clone())
    }

    pub fn designs_api(&self) -> designs_api::DesignsApi {
        designs_api::DesignsApi::new(self.clone())
    }

    pub fn domain_authentication(&self) -> domain_authentication::DomainAuthentication {
        domain_authentication::DomainAuthentication::new(self.clone())
    }

    pub fn email_address_validation(&self) -> email_address_validation::EmailAddressValidation {
        email_address_validation::EmailAddressValidation::new(self.clone())
    }

    pub fn email_cname_records(&self) -> email_cname_records::EmailCnameRecords {
        email_cname_records::EmailCnameRecords::new(self.clone())
    }

    pub fn invalid_emails_api(&self) -> invalid_emails_api::InvalidEmailsApi {
        invalid_emails_api::InvalidEmailsApi::new(self.clone())
    }

    pub fn ip_access_management(&self) -> ip_access_management::IpAccessManagement {
        ip_access_management::IpAccessManagement::new(self.clone())
    }

    pub fn ip_addresses(&self) -> ip_addresses::IpAddresses {
        ip_addresses::IpAddresses::new(self.clone())
    }

    pub fn ip_pools(&self) -> ip_pools::IpPools {
        ip_pools::IpPools::new(self.clone())
    }

    pub fn ip_warmup(&self) -> ip_warmup::IpWarmup {
        ip_warmup::IpWarmup::new(self.clone())
    }

    pub fn link_branding(&self) -> link_branding::LinkBranding {
        link_branding::LinkBranding::new(self.clone())
    }

    pub fn lists(&self) -> lists::Lists {
        lists::Lists::new(self.clone())
    }

    pub fn mail_send(&self) -> mail_send::MailSend {
        mail_send::MailSend::new(self.clone())
    }

    pub fn marketing_campaigns_stats(&self) -> marketing_campaigns_stats::MarketingCampaignsStats {
        marketing_campaigns_stats::MarketingCampaignsStats::new(self.clone())
    }

    pub fn query(&self) -> query::Query {
        query::Query::new(self.clone())
    }

    pub fn reverse_dns(&self) -> reverse_dns::ReverseDns {
        reverse_dns::ReverseDns::new(self.clone())
    }

    pub fn segmenting_contacts(&self) -> segmenting_contacts::SegmentingContacts {
        segmenting_contacts::SegmentingContacts::new(self.clone())
    }

    pub fn segmenting_contacts_beta(&self) -> segmenting_contacts_beta::SegmentingContactsBeta {
        segmenting_contacts_beta::SegmentingContactsBeta::new(self.clone())
    }

    pub fn send_test_email(&self) -> send_test_email::SendTestEmail {
        send_test_email::SendTestEmail::new(self.clone())
    }

    pub fn sender_identities_api(&self) -> sender_identities_api::SenderIdentitiesApi {
        sender_identities_api::SenderIdentitiesApi::new(self.clone())
    }

    pub fn sender_verification(&self) -> sender_verification::SenderVerification {
        sender_verification::SenderVerification::new(self.clone())
    }

    pub fn senders(&self) -> senders::Senders {
        senders::Senders::new(self.clone())
    }

    pub fn settings_enforced_tls(&self) -> settings_enforced_tls::SettingsEnforcedTls {
        settings_enforced_tls::SettingsEnforcedTls::new(self.clone())
    }

    pub fn settings_inbound_parse(&self) -> settings_inbound_parse::SettingsInboundParse {
        settings_inbound_parse::SettingsInboundParse::new(self.clone())
    }

    pub fn settings_mail(&self) -> settings_mail::SettingsMail {
        settings_mail::SettingsMail::new(self.clone())
    }

    pub fn settings_partner(&self) -> settings_partner::SettingsPartner {
        settings_partner::SettingsPartner::new(self.clone())
    }

    pub fn settings_tracking(&self) -> settings_tracking::SettingsTracking {
        settings_tracking::SettingsTracking::new(self.clone())
    }

    pub fn single_sends(&self) -> single_sends::SingleSends {
        single_sends::SingleSends::new(self.clone())
    }

    pub fn single_sign_on_settings(&self) -> single_sign_on_settings::SingleSignOnSettings {
        single_sign_on_settings::SingleSignOnSettings::new(self.clone())
    }

    pub fn single_sign_on_teammates(&self) -> single_sign_on_teammates::SingleSignOnTeammates {
        single_sign_on_teammates::SingleSignOnTeammates::new(self.clone())
    }

    pub fn spam_reports_api(&self) -> spam_reports_api::SpamReportsApi {
        spam_reports_api::SpamReportsApi::new(self.clone())
    }

    pub fn stats(&self) -> stats::Stats {
        stats::Stats::new(self.clone())
    }

    pub fn subuser_monitor_settings(&self) -> subuser_monitor_settings::SubuserMonitorSettings {
        subuser_monitor_settings::SubuserMonitorSettings::new(self.clone())
    }

    pub fn subuser_statistics(&self) -> subuser_statistics::SubuserStatistics {
        subuser_statistics::SubuserStatistics::new(self.clone())
    }

    pub fn subusers_api(&self) -> subusers_api::SubusersApi {
        subusers_api::SubusersApi::new(self.clone())
    }

    pub fn suppressions(&self) -> suppressions::Suppressions {
        suppressions::Suppressions::new(self.clone())
    }

    pub fn suppressions_global(&self) -> suppressions_global::SuppressionsGlobal {
        suppressions_global::SuppressionsGlobal::new(self.clone())
    }

    pub fn suppressions_unsubscribe_groups(
        &self,
    ) -> suppressions_unsubscribe_groups::SuppressionsUnsubscribeGroups {
        suppressions_unsubscribe_groups::SuppressionsUnsubscribeGroups::new(self.clone())
    }

    pub fn teammates(&self) -> teammates::Teammates {
        teammates::Teammates::new(self.clone())
    }

    pub fn transactional_templates(&self) -> transactional_templates::TransactionalTemplates {
        transactional_templates::TransactionalTemplates::new(self.clone())
    }

    pub fn transactional_templates_versions(
        &self,
    ) -> transactional_templates_versions::TransactionalTemplatesVersions {
        transactional_templates_versions::TransactionalTemplatesVersions::new(self.clone())
    }

    pub fn users_api(&self) -> users_api::UsersApi {
        users_api::UsersApi::new(self.clone())
    }

    pub fn webhooks(&self) -> webhooks::Webhooks {
        webhooks::Webhooks::new(self.clone())
    }
}
