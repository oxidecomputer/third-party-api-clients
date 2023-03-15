//! A fully generated, opinionated API client library for Okta.
//!
//! [![docs.rs](https://docs.rs/okta/badge.svg)](https://docs.rs/okta)
//!
//! ## API Details
//!
//! Allows customers to easily access the Okta API
//!
//! [API Terms of Service](http://developer.okta.com/terms/)
//!
//! ### Contact
//!
//!
//! | name | url | email |
//! |----|----|----|
//! | Okta Developer Team | <http://developer.okta.com/> | devex-public@okta.com |
//!
//! ### License
//!
//!
//! | name | url |
//! |----|----|
//! | Apache-2.0 | <http://www.apache.org/licenses/LICENSE-2.0.html> |
//!
//!
//! ## Client Details
//!
//! This client is generated from the [Okta OpenAPI
//! specs](https://github.com/okta/okta-management-openapi-spec) based on API spec version `2.5.0`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! okta = "0.4.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use okta::Client;
//!
//! let okta = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `OKTA_API_KEY`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use okta::Client;
//!
//! let okta = Client::new_from_env();
//! ```
//!
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod applications;
pub mod authorization_servers;
pub mod domains;
pub mod event_hooks;
pub mod features;
pub mod groups;
pub mod identity_providers;
pub mod inline_hooks;
pub mod linked_objects;
pub mod logs;
pub mod network_zones;
pub mod policies;
pub mod profile_mappings;
pub mod sessions;
pub mod templates;
pub mod threat_insights;
pub mod trusted_origins;
pub mod types;
pub mod user_factors;
pub mod user_schemas;
pub mod user_types;
pub mod users;
#[doc(hidden)]
pub mod utils;

use thiserror::Error;
type ClientResult<T> = Result<T, ClientError>;

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
        error: String,
    },
}

pub const FALLBACK_HOST: &str = "https://na4.okta.net";

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
        "https://your-subdomain.okta.com"
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
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<T>(token: T) -> Self
    where
        T: ToString,
    {
        let client = reqwest::Client::builder().build();
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

    /// Create a new Client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key and your requests will work.
    /// We pass in the token and refresh token to the client so if you are storing
    /// it in a database, you can get it first.
    pub fn new_from_env() -> Self {
        let token = env::var("OKTA_API_KEY").expect("must set OKTA_API_KEY");

        Client::new(token)
    }

    async fn url_and_auth(&self, uri: &str) -> ClientResult<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>()?;
        let auth = format!("SSWS {}", self.token);
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
    ) -> ClientResult<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, message).await?;

        let status = response.status();

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
            Ok(parsed_response)
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
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
    ) -> ClientResult<(Option<crate::utils::NextLink>, Out)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, message).await?;

        let status = response.status();
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
            Ok((link, parsed_response))
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };
            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn post_form<Out>(&self, uri: &str, form: reqwest::multipart::Form) -> ClientResult<Out>
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
            Ok(parsed_response)
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
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
    ) -> ClientResult<Out>
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
            Ok(parsed_response)
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
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
    ) -> ClientResult<Out>
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
            Ok(parsed_response)
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status,
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
    ) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let r = self.request(method, uri, message).await?;
        Ok(r)
    }

    #[allow(dead_code)]
    async fn get<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::GET, uri, message).await
    }

    #[allow(dead_code)]
    async fn get_all_pages<D>(&self, uri: &str, _message: Message) -> ClientResult<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        // TODO: implement this.
        self.unfold(uri).await
    }

    /// "unfold" paginated results of a vector of items
    #[allow(dead_code)]
    async fn unfold<D>(&self, uri: &str) -> ClientResult<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut items) = self.get_pages(uri).await?;
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = link.as_ref() {
                let url = reqwest::Url::parse(&url.0)?;
                let (new_link, new_items) = self.get_pages_url(&url).await?;
                link = new_link;
                items = new_items;
            }
        }

        Ok(global_items)
    }

    #[allow(dead_code)]
    async fn get_pages<D>(
        &self,
        uri: &str,
    ) -> ClientResult<(Option<crate::utils::NextLink>, Vec<D>)>
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
    ) -> ClientResult<(Option<crate::utils::NextLink>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, url.as_str(), Message::default())
            .await
    }

    #[allow(dead_code)]
    async fn post<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::POST, uri, message).await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PATCH, uri, message).await
    }

    #[allow(dead_code)]
    async fn put<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PUT, uri, message).await
    }

    #[allow(dead_code)]
    async fn delete<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::DELETE, uri, message)
            .await
    }

    pub fn applications(&self) -> applications::Applications {
        applications::Applications::new(self.clone())
    }

    pub fn authorization_servers(&self) -> authorization_servers::AuthorizationServers {
        authorization_servers::AuthorizationServers::new(self.clone())
    }

    pub fn domains(&self) -> domains::Domains {
        domains::Domains::new(self.clone())
    }

    pub fn event_hooks(&self) -> event_hooks::EventHooks {
        event_hooks::EventHooks::new(self.clone())
    }

    pub fn features(&self) -> features::Features {
        features::Features::new(self.clone())
    }

    pub fn groups(&self) -> groups::Groups {
        groups::Groups::new(self.clone())
    }

    pub fn identity_providers(&self) -> identity_providers::IdentityProviders {
        identity_providers::IdentityProviders::new(self.clone())
    }

    pub fn inline_hooks(&self) -> inline_hooks::InlineHooks {
        inline_hooks::InlineHooks::new(self.clone())
    }

    pub fn linked_objects(&self) -> linked_objects::LinkedObjects {
        linked_objects::LinkedObjects::new(self.clone())
    }

    pub fn logs(&self) -> logs::Logs {
        logs::Logs::new(self.clone())
    }

    pub fn network_zones(&self) -> network_zones::NetworkZones {
        network_zones::NetworkZones::new(self.clone())
    }

    pub fn policies(&self) -> policies::Policies {
        policies::Policies::new(self.clone())
    }

    pub fn profile_mappings(&self) -> profile_mappings::ProfileMappings {
        profile_mappings::ProfileMappings::new(self.clone())
    }

    pub fn sessions(&self) -> sessions::Sessions {
        sessions::Sessions::new(self.clone())
    }

    pub fn templates(&self) -> templates::Templates {
        templates::Templates::new(self.clone())
    }

    pub fn threat_insights(&self) -> threat_insights::ThreatInsights {
        threat_insights::ThreatInsights::new(self.clone())
    }

    pub fn trusted_origins(&self) -> trusted_origins::TrustedOrigins {
        trusted_origins::TrustedOrigins::new(self.clone())
    }

    pub fn user_factors(&self) -> user_factors::UserFactors {
        user_factors::UserFactors::new(self.clone())
    }

    pub fn user_schemas(&self) -> user_schemas::UserSchemas {
        user_schemas::UserSchemas::new(self.clone())
    }

    pub fn user_types(&self) -> user_types::UserTypes {
        user_types::UserTypes::new(self.clone())
    }

    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }
}
