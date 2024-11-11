//! A fully generated, opinionated API client library for SendGrid.
//!
//!
//! [![docs.rs](https://docs.rs/sendgrid-api/badge.svg)](https://docs.rs/sendgrid-api)
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
//! sendgrid-api = "0.8.0-rc.1"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust
//! use sendgrid_api::Client;
//!
//! let sendgrid = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `SENDGRID_API_KEY`
//!
//! And then you can create a client from the environment.
//!
//! ```rust
//! use sendgrid_api::Client;
//!
//! let sendgrid = Client::new_from_env();
//! ```
//!
#![allow(clippy::derive_partial_eq_without_eq)]
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
pub mod traits;
pub mod transactional_templates;
pub mod transactional_templates_versions;
pub mod types;
pub mod users_api;
#[doc(hidden)]
pub mod utils;
pub mod webhooks;

pub use reqwest::{header::HeaderMap, StatusCode};

#[derive(Debug)]
pub struct Response<T> {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: T,
}

impl<T> Response<T> {
    pub fn new(status: reqwest::StatusCode, headers: reqwest::header::HeaderMap, body: T) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
}

type ClientResult<T> = Result<T, ClientError>;

use thiserror::Error;

/// Errors returned by the client
#[derive(Debug, Error)]
pub enum ClientError {
    /// utf8 convertion error
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    /// URL Parsing Error
    #[error(transparent)]
    UrlParserError(#[from] url::ParseError),
    /// Serde JSON parsing error
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Errors returned by reqwest
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Errors returned by reqwest::header
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    /// Errors returned by reqwest middleware
    #[error(transparent)]
    ReqwestMiddleWareError(#[from] reqwest_middleware::Error),
    /// Generic HTTP Error
    #[error("HTTP Error. Code: {status}, message: {error}")]
    HttpError {
        status: http::StatusCode,
        headers: reqwest::header::HeaderMap,
        error: String,
    },
}

pub const FALLBACK_HOST: &str = "https://api.sendgrid.com/v3";

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

#[derive(Debug, Default)]
pub(crate) struct Message {
    pub body: Option<reqwest::Body>,
    pub content_type: Option<String>,
}

use std::env;

#[derive(Debug, Default, Clone)]
pub struct RootDefaultServer {}

impl RootDefaultServer {
    pub fn default_url(&self) -> &str {
        "https://api.sendgrid.com/v3"
    }
}

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    host_override: Option<String>,
    token: String,

    client: reqwest_middleware::ClientWithMiddleware,
}

impl Client {
    /// Create a new Client struct.
    ///
    /// # Panics
    ///
    /// This function will panic if the internal http client fails to create
    pub fn new<T>(token: T) -> Self
    where
        T: ToString,
    {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build();
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        match client {
            Ok(c) => {
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                        reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                        |req: &reqwest::Request| req.try_clone().is_some(),
                    ))
                    .build();

                let host = RootDefaultServer::default().default_url().to_string();

                Client {
                    host,
                    host_override: None,
                    token: token.to_string(),

                    client,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Override the host for all endpoins in the client.
    pub fn with_host_override<H>(&mut self, host: H) -> &mut Self
    where
        H: ToString,
    {
        self.host_override = Some(host.to_string());
        self
    }

    /// Disables the global host override for the client.
    pub fn remove_host_override(&mut self) -> &mut Self {
        self.host_override = None;
        self
    }

    pub fn get_host_override(&self) -> Option<&str> {
        self.host_override.as_deref()
    }

    pub(crate) fn url(&self, path: &str, host: Option<&str>) -> String {
        format!(
            "{}{}",
            self.get_host_override()
                .or(host)
                .unwrap_or(self.host.as_str()),
            path
        )
    }

    /// Create a new Client struct from environment variables.
    ///
    /// The following environment variables are expected to be set:
    ///   * `SENDGRID_API_KEY`
    ///
    /// # Panics
    ///
    /// This function will panic if the expected environment variables can not be found
    pub fn new_from_env() -> Self {
        let token = env::var("SENDGRID_API_KEY").expect("must set SENDGRID_API_KEY");

        Client::new(token)
    }

    async fn url_and_auth(&self, uri: &str) -> ClientResult<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>()?;
        let auth = format!("Bearer {}", self.token);
        Ok((parsed_url, Some(auth)))
    }

    async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        message: Message,
    ) -> ClientResult<reqwest::Response> {
        let (url, auth) = self.url_and_auth(uri).await?;
        let instance = <&Client>::clone(&self);
        let mut req = instance.client.request(method.clone(), url);
        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(content_type) = &message.content_type {
            req = req.header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_str(content_type).unwrap(),
            );
        } else {
            req = req.header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            );
        }

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }
        if let Some(body) = message.body {
            req = req.body(body);
        }
        Ok(req.send().await?)
    }

    async fn request<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        message: Message,
    ) -> ClientResult<crate::Response<Out>>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, message).await?;

        let status = response.status();
        let headers = response.headers().clone();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok(crate::Response::new(status, headers, parsed_response))
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    headers,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
                    headers,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };

            Err(error)
        }
    }

    async fn request_with_links<Out>(
        &self,
        method: http::Method,
        uri: &str,
        message: Message,
    ) -> ClientResult<(Option<crate::utils::NextLink>, crate::Response<Out>)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, message).await?;

        let status = response.status();
        let headers = response.headers().clone();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| parse_link_header::parse(l).ok())
            .as_ref()
            .and_then(crate::utils::next_link);

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");

            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok((link, crate::Response::new(status, headers, parsed_response)))
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    headers,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
                    headers,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };
            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn post_form<Out>(
        &self,
        uri: &str,
        form: reqwest::multipart::Form,
    ) -> ClientResult<crate::Response<Out>>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let (url, auth) = self.url_and_auth(uri).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(http::Method::POST, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        req = req.multipart(form);

        let response = req.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                let s = String::from_utf8(response_body.to_vec())?;
                serde_json::from_value(serde_json::json!(&s))?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok(crate::Response::new(status, headers, parsed_response))
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    headers,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
                    headers,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_accept_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        accept_mime_type: &str,
    ) -> ClientResult<crate::Response<Out>>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let (url, auth) = self.url_and_auth(uri).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_str(accept_mime_type)?,
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        let response = req.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                let s = String::from_utf8(response_body.to_vec())?;
                serde_json::from_value(serde_json::json!(&s))?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok(crate::Response::new(status, headers, parsed_response))
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    headers,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
                    headers,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
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
    ) -> ClientResult<crate::Response<Out>>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let (url, auth) = self.url_and_auth(uri).await?;

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

        let response = req.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok(crate::Response::new(status, headers, parsed_response))
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    headers,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
                    headers,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };

            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        message: Message,
    ) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let r = self.request(method, uri, message).await?;
        Ok(r)
    }

    #[allow(dead_code)]
    async fn get<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::GET, uri, message).await
    }

    #[allow(dead_code)]
    async fn get_all_pages<D>(&self, uri: &str, _message: Message) -> ClientResult<Response<Vec<D>>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        // TODO: implement this.
        self.unfold(uri).await
    }

    /// "unfold" paginated results of a vector of items
    #[allow(dead_code)]
    async fn unfold<D>(&self, uri: &str) -> ClientResult<crate::Response<Vec<D>>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut response) = self.get_pages(uri).await?;
        let mut link = new_link;
        while !response.body.is_empty() {
            global_items.append(&mut response.body);
            // We need to get the next link.
            if let Some(url) = &link {
                let url = reqwest::Url::parse(&url.0)?;
                let (new_link, new_response) = self.get_pages_url(&url).await?;
                link = new_link;
                response = new_response;
            }
        }

        Ok(Response::new(
            response.status,
            response.headers,
            global_items,
        ))
    }

    #[allow(dead_code)]
    async fn get_pages<D>(
        &self,
        uri: &str,
    ) -> ClientResult<(Option<crate::utils::NextLink>, crate::Response<Vec<D>>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, uri, Message::default())
            .await
    }

    #[allow(dead_code)]
    async fn get_pages_url<D>(
        &self,
        url: &reqwest::Url,
    ) -> ClientResult<(Option<crate::utils::NextLink>, crate::Response<Vec<D>>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, url.as_str(), Message::default())
            .await
    }

    #[allow(dead_code)]
    async fn post<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::POST, uri, message).await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PATCH, uri, message).await
    }

    #[allow(dead_code)]
    async fn put<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PUT, uri, message).await
    }

    #[allow(dead_code)]
    async fn delete<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::DELETE, uri, message)
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
