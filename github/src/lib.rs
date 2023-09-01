//! A fully generated, opinionated API client library for GitHub.
//!
//!
//! [![docs.rs](https://docs.rs/octorust/badge.svg)](https://docs.rs/octorust)
//!
//! ## API Details
//!
//! GitHub's v3 REST API.
//!
//! [API Terms of Service](https://docs.github.com/articles/github-terms-of-service)
//!
//! ### Contact
//!
//!
//! | name | url |
//! |----|----|
//! | Support | <https://support.github.com/contact?tags=rest-api> |
//!
//! ### License
//!
//!
//! | name | url |
//! |----|----|
//! | MIT | <https://spdx.org/licenses/MIT> |
//!
//!
//! ## Client Details
//!
//! This client is generated from the [GitHub OpenAPI
//! specs](https://github.com/github/rest-api-description) based on API spec version `1.1.4`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! octorust = "0.7.1"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of `auth::Credentials`.
//!
//! ```rust
//! use octorust::{auth::Credentials, Client};
//!
//! let github = Client::new(
//!   String::from("user-agent-name"),
//!   Credentials::Token(
//!     String::from("personal-access-token")
//!   ),
//! );
//! ```
//!
//! If you are a GitHub enterprise customer, you will want to create a client with the
//! [Client#host_override](https://docs.rs/octorust/0.7.1/octorust/struct.Client.html#method.host_override) method.
//!
//! ## Feature flags
//!
//! ### httpcache
//!
//! Github supports conditional HTTP requests using etags to checksum responses
//! Experimental support for utilizing this to cache responses locally with the
//! `httpcache` feature flag.
//!
//! To enable this, add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! octorust = { version = "0.7.1", features = ["httpcache"] }
//! ```
//!
//! Then use the `Client::custom` constructor to provide a cache implementation.
//!
//! Here is an example:
//!
//! ```rust
//! use octorust::{auth::Credentials, Client};
//! #[cfg(feature = "httpcache")]
//! use octorust::http_cache::HttpCache;
//!
//! #[cfg(feature = "httpcache")]
//! let http_cache = HttpCache::in_home_dir();
//!
//! #[cfg(not(feature = "httpcache"))]
//! let github = Client::custom(
//!     concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
//!     Credentials::Token(
//!       String::from("personal-access-token")
//!     ),
//!     reqwest::Client::builder().build().unwrap(),
//! );
//!
//! #[cfg(feature = "httpcache")]
//! let github = Client::custom(
//!     concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
//!     Credentials::Token(
//!       String::from("personal-access-token")
//!     ),
//!     reqwest::Client::builder().build().unwrap(),
//!     http_cache
//! );
//! ```
//! ## Authenticating GitHub apps
//!
//! You can also authenticate via a GitHub app.
//!
//! Here is an example:
//!
//! ```rust
//! use std::env;
//!
//! use octorust::{Client, auth::{Credentials, InstallationTokenGenerator, JWTCredentials}};
//! #[cfg(feature = "httpcache")]
//! use octorust::http_cache::FileBasedCache;
//! use base64::{Engine, engine::general_purpose::STANDARD};
//!
//! let app_id_str = env::var("GH_APP_ID").unwrap();
//! let app_id = app_id_str.parse::<u64>().unwrap();
//!
//! let app_installation_id_str = env::var("GH_INSTALLATION_ID").unwrap();
//! let app_installation_id = app_installation_id_str.parse::<u64>().unwrap();
//!
//! let encoded_private_key = env::var("GH_PRIVATE_KEY").unwrap();
//! let private_key = STANDARD.decode(encoded_private_key).unwrap();
//!
//! // Decode the key.
//! let key = nom_pem::decode_block(&private_key).unwrap();
//!
//! // Get the JWT credentials.
//! let jwt = JWTCredentials::new(app_id, key.data).unwrap();
//!
//! // Create the HTTP cache.
//! #[cfg(feature = "httpcache")]
//! let mut dir = dirs::home_dir().expect("Expected a home dir");
//! #[cfg(feature = "httpcache")]
//! dir.push(".cache/github");
//! #[cfg(feature = "httpcache")]
//! let http_cache = Box::new(FileBasedCache::new(dir));
//!
//! let token_generator = InstallationTokenGenerator::new(app_installation_id, jwt);
//!
//! #[cfg(not(feature = "httpcache"))]
//! let github = Client::custom(
//!     concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
//!     Credentials::InstallationToken(token_generator),
//!     reqwest::Client::builder().build().unwrap(),
//! );
//!
//! #[cfg(feature = "httpcache")]
//! let github = Client::custom(
//!     concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
//!     Credentials::InstallationToken(token_generator),
//!     reqwest::Client::builder().build().unwrap(),
//!     http_cache,
//! );
//! ```
//!
//! ## Acknowledgements
//!
//! Shout out to [hubcaps](https://github.com/softprops/hubcaps) for paving the
//! way here. This extends that effort in a generated way so the library is
//! always up to the date with the OpenAPI spec and no longer requires manual
//! contributions to add new endpoints.
//!
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Endpoints to manage GitHub Actions using the REST API.
pub mod actions;
/// Activity APIs provide access to notifications, subscriptions, and timelines.
pub mod activity;
/// Information for integrations and installations.
pub mod apps;
pub mod auth;
/// Monitor charges and usage from Actions and Packages.
pub mod billing;
/// Rich interactions with checks run by your integrations.
pub mod checks;
/// Retrieve code scanning alerts from a repository.
pub mod code_scanning;
/// Insight into codes of conduct for your communities.
pub mod codes_of_conduct;
/// List emojis available to use on GitHub.
pub mod emojis;
/// Administer a GitHub enterprise.
pub mod enterprise_admin;
/// View, modify your gists.
pub mod gists;
/// Raw Git functionality.
pub mod git;
/// View gitignore templates.
pub mod gitignore;
#[cfg(feature = "httpcache")]
#[cfg_attr(docsrs, doc(cfg(feature = "httpcache")))]
pub mod http_cache;
/// Owner or admin management of users interactions.
pub mod interactions;
/// Interact with GitHub Issues.
pub mod issues;
/// View various OSS licenses.
pub mod licenses;
/// Render Github flavored markdown.
pub mod markdown;
/// Endpoints that give information about the API.
pub mod meta;
/// Move projects to or from GitHub.
pub mod migrations;
/// Manage access of OAuth applications.
pub mod oauth_authorizations;
/// Interact with GitHub Orgs.
pub mod orgs;
/// Manage packages for authenticated users and organizations.
pub mod packages;
/// Interact with GitHub Projects.
pub mod projects;
/// Interact with GitHub Pull Requests.
pub mod pulls;
/// Check your current rate limit status.
pub mod rate_limit;
/// Interact with reactions to various GitHub entities.
pub mod reactions;
/// Interact with GitHub Repos.
pub mod repos;
/// Provisioning of GitHub organization membership for SCIM-enabled providers.
pub mod scim;
/// Look for stuff on GitHub.
pub mod search;
/// Retrieve secret scanning alerts from a repository.
pub mod secret_scanning;
/// Interact with GitHub Teams.
pub mod teams;
pub mod types;
/// Interact with and view information about users and also current user.
pub mod users;
#[doc(hidden)]
pub mod utils;

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
    // Github only
    /// Ratelimited
    #[error("Rate limited for the next {duration} seconds")]
    RateLimited { duration: u64 },
    /// JWT errors from auth.rs
    #[error(transparent)]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    /// IO Errors
    #[cfg(feature = "httpcache")]
    #[error(transparent)]
    #[cfg(feature = "httpcache")]
    IoError(#[from] std::io::Error),
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

pub const FALLBACK_HOST: &str = "https://api.github.com";

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

#[derive(Debug, Default, Clone)]
pub struct RootDefaultServer {}

impl RootDefaultServer {
    pub fn default_url(&self) -> &str {
        "https://api.github.com"
    }
}

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    host_override: Option<String>,
    agent: String,
    client: reqwest_middleware::ClientWithMiddleware,
    credentials: Option<crate::auth::Credentials>,
    #[cfg(feature = "httpcache")]
    http_cache: crate::http_cache::BoxedHttpCache,
}

impl Client {
    pub fn new<A, C>(agent: A, credentials: C) -> ClientResult<Self>
    where
        A: Into<String>,
        C: Into<Option<crate::auth::Credentials>>,
    {
        let http = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        let client = reqwest_middleware::ClientBuilder::new(http)
            // Trace HTTP requests. See the tracing crate to make use of these traces.
            .with(reqwest_tracing::TracingMiddleware::default())
            // Retry failed requests.
            .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                |req: &reqwest::Request| req.try_clone().is_some(),
            ))
            .build();

        #[cfg(feature = "httpcache")]
        {
            Ok(Self::custom(
                agent,
                credentials,
                client,
                <dyn crate::http_cache::HttpCache>::noop(),
            ))
        }
        #[cfg(not(feature = "httpcache"))]
        {
            Ok(Self::custom(agent, credentials, client))
        }
    }

    #[cfg(feature = "httpcache")]
    pub fn custom<A, CR>(
        agent: A,
        credentials: CR,
        http: reqwest_middleware::ClientWithMiddleware,
        http_cache: crate::http_cache::BoxedHttpCache,
    ) -> Self
    where
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: RootDefaultServer::default().default_url().to_string(),
            host_override: None,
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
            http_cache,
        }
    }

    #[cfg(not(feature = "httpcache"))]
    pub fn custom<A, CR>(
        agent: A,
        credentials: CR,
        http: reqwest_middleware::ClientWithMiddleware,
    ) -> Self
    where
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: RootDefaultServer::default().default_url().to_string(),
            host_override: None,
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
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

    pub fn set_credentials<CR>(&mut self, credentials: CR)
    where
        CR: Into<Option<crate::auth::Credentials>>,
    {
        self.credentials = credentials.into();
    }

    fn credentials(
        &self,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Option<&crate::auth::Credentials> {
        match (authentication, self.credentials.as_ref()) {
            (crate::auth::AuthenticationConstraint::Unconstrained, creds) => creds,
            (
                crate::auth::AuthenticationConstraint::JWT,
                creds @ Some(&crate::auth::Credentials::JWT(_)),
            ) => creds,
            (
                crate::auth::AuthenticationConstraint::JWT,
                Some(crate::auth::Credentials::InstallationToken(apptoken)),
            ) => Some(apptoken.jwt()),
            (crate::auth::AuthenticationConstraint::JWT, _) => {
                log::info!(
                    "Request needs JWT authentication but only a mismatched method is available"
                );
                None
            }
        }
    }

    async fn url_and_auth(
        &self,
        uri: &str,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> ClientResult<(reqwest::Url, Option<String>)> {
        let mut parsed_url = uri.parse::<reqwest::Url>()?;

        match self.credentials(authentication) {
            Some(crate::auth::Credentials::Client(id, secret)) => {
                parsed_url
                    .query_pairs_mut()
                    .append_pair("client_id", id)
                    .append_pair("client_secret", secret);
                Ok((parsed_url, None))
            }
            Some(crate::auth::Credentials::Token(token)) => {
                let auth = format!("token {}", token);
                Ok((parsed_url, Some(auth)))
            }
            Some(crate::auth::Credentials::JWT(jwt)) => {
                let auth = format!("Bearer {}", jwt.token());
                Ok((parsed_url, Some(auth)))
            }
            Some(crate::auth::Credentials::InstallationToken(apptoken)) => {
                let token = if let Some(token) = apptoken.token().await {
                    token
                } else {
                    let mut token_guard = apptoken.access_key.write().await;
                    if let Some(token) = token_guard.as_ref().and_then(|t| t.token()) {
                        token.to_owned()
                    } else {
                        log::debug!("app token is stale, refreshing");

                        let created_at = tokio::time::Instant::now();
                        let token = self
                            .apps()
                            .create_installation_access_token(
                                apptoken.installation_id,
                                &types::AppsCreateInstallationAccessTokenRequest {
                                    permissions: Default::default(),
                                    repositories: Default::default(),
                                    repository_ids: Default::default(),
                                },
                            )
                            .await?;
                        *token_guard = Some(crate::auth::ExpiringInstallationToken::new(
                            token.body.token.clone(),
                            created_at,
                        ));
                        token.body.token
                    }
                };
                let auth = format!("token {}", token);
                Ok((parsed_url, Some(auth)))
            }
            None => Ok((parsed_url, None)),
        }
    }

    async fn make_request(
        &self,
        method: http::Method,
        uri: &str,
        message: Message,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> ClientResult<reqwest_middleware::RequestBuilder> {
        let (url, auth) = self.url_and_auth(uri, authentication).await?;

        let mut req = self.client.request(method, url);

        if let Some(content_type) = &message.content_type {
            req = req.header(http::header::CONTENT_TYPE, content_type.clone());
        }

        req = req.header(http::header::USER_AGENT, &*self.agent);
        req = req.header(http::header::ACCEPT, &media_type.to_string());

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if let Some(body) = message.body {
            req = req.body(body);
        }

        Ok(req)
    }

    async fn request<Out>(
        &self,
        method: http::Method,
        uri: &str,
        message: Message,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> ClientResult<(Option<crate::utils::NextLink>, crate::Response<Out>)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        #[cfg(not(feature = "httpcache"))]
        let req = self
            .make_request(method.clone(), uri, message, media_type, authentication)
            .await?;

        #[cfg(feature = "httpcache")]
        let req = {
            let mut req = self
                .make_request(method.clone(), uri, message, media_type, authentication)
                .await?;

            if method == http::Method::GET {
                if let Ok(etag) = self.http_cache.lookup_etag(&uri) {
                    req = req.header(http::header::IF_NONE_MATCH, etag);
                }
            }

            req
        };

        let response = req.send().await?;

        #[cfg(not(feature = "httpcache"))]
        let (remaining, reset) = crate::utils::get_header_values(response.headers());

        #[cfg(feature = "httpcache")]
        let (remaining, reset, etag) = crate::utils::get_header_values(response.headers());

        let status = response.status();
        let headers = response.headers().clone();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| parse_link_header::parse(l).ok());
        let next_link = link.as_ref().and_then(crate::utils::next_link);

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            #[cfg(feature = "httpcache")]
            {
                if let Some(etag) = etag {
                    if let Err(e) = self.http_cache.cache_response(
                        &uri,
                        &response_body,
                        &etag,
                        &next_link.as_ref().map(|n| n.0.clone()),
                    ) {
                        // failing to cache isn't fatal, so just log & swallow the error
                        log::info!("failed to cache body & etag: {}", e);
                    }
                }
            }

            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok((
                next_link,
                crate::Response::new(status, headers, parsed_response),
            ))
        } else if status.is_redirection() {
            match status {
                http::StatusCode::NOT_MODIFIED => {
                    // only supported case is when client provides if-none-match
                    // header when cargo builds with --cfg feature="httpcache"
                    #[cfg(feature = "httpcache")]
                    {
                        let body = self.http_cache.lookup_body(&uri).unwrap();
                        let out = serde_json::from_str::<Out>(&body).unwrap();
                        let link = match next_link {
                            Some(next_link) => Ok(Some(next_link)),
                            None => self
                                .http_cache
                                .lookup_next_link(&uri)
                                .map(|next_link| next_link.map(crate::utils::NextLink)),
                        };
                        link.map(|link| (link, Response::new(status, headers, out)))
                    }
                    #[cfg(not(feature = "httpcache"))]
                    {
                        unreachable!(
                            "this should not be reachable without the httpcache feature enabled"
                        )
                    }
                }
                _ => {
                    // The body still needs to be parsed. Except in the case of 304 (handled above),
                    // returning a body in the response is allowed.
                    let body = if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>() {
                        serde_json::from_str("null")?
                    } else {
                        serde_json::from_slice::<Out>(&response_body)?
                    };

                    Ok((None, crate::Response::new(status, headers, body)))
                }
            }
        } else {
            let error = match (remaining, reset) {
                (Some(remaining), Some(reset)) if remaining == 0 => {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    ClientError::RateLimited {
                        duration: u64::from(reset).saturating_sub(now),
                    }
                }
                _ => {
                    if response_body.is_empty() {
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
                    }
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
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let (_, r) = self
            .request(method, uri, message, media_type, authentication)
            .await?;
        Ok(r)
    }

    async fn get<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.get_media(uri, crate::utils::MediaType::Json, message)
            .await
    }

    async fn get_media<D>(
        &self,
        uri: &str,
        media: crate::utils::MediaType,
        message: Message,
    ) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::GET,
            uri,
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        )
        .await
    }

    async fn get_all_pages<D>(
        &self,
        uri: &str,
        _message: Message,
    ) -> ClientResult<crate::Response<Vec<D>>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.unfold(uri).await
    }

    async fn get_pages<D>(
        &self,
        uri: &str,
    ) -> ClientResult<(Option<crate::utils::NextLink>, crate::Response<Vec<D>>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request(
            http::Method::GET,
            uri,
            Message::default(),
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        )
        .await
    }

    async fn get_pages_url<D>(
        &self,
        url: &reqwest::Url,
    ) -> ClientResult<(Option<crate::utils::NextLink>, crate::Response<Vec<D>>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request(
            http::Method::GET,
            url.as_str(),
            Message::default(),
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        )
        .await
    }

    async fn post<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.post_media(
            uri,
            message,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        )
        .await
    }

    async fn post_media<D>(
        &self,
        uri: &str,
        message: Message,
        media: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::POST, uri, message, media, authentication)
            .await
    }

    async fn patch_media<D>(
        &self,
        uri: &str,
        message: Message,
        media: crate::utils::MediaType,
    ) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PATCH,
            uri,
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        )
        .await
    }

    async fn patch<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.patch_media(uri, message, crate::utils::MediaType::Json)
            .await
    }

    async fn put<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.put_media(uri, message, crate::utils::MediaType::Json)
            .await
    }

    async fn put_media<D>(
        &self,
        uri: &str,
        message: Message,
        media: crate::utils::MediaType,
    ) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PUT,
            uri,
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        )
        .await
    }

    async fn delete<D>(&self, uri: &str, message: Message) -> ClientResult<crate::Response<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            uri,
            message,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        )
        .await
    }

    /// "unfold" paginated results of a vector of items
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

    /// Endpoints to manage GitHub Actions using the REST API.
    pub fn actions(&self) -> actions::Actions {
        actions::Actions::new(self.clone())
    }

    /// Activity APIs provide access to notifications, subscriptions, and timelines.
    pub fn activity(&self) -> activity::Activity {
        activity::Activity::new(self.clone())
    }

    /// Information for integrations and installations.
    pub fn apps(&self) -> apps::Apps {
        apps::Apps::new(self.clone())
    }

    /// Monitor charges and usage from Actions and Packages.
    pub fn billing(&self) -> billing::Billing {
        billing::Billing::new(self.clone())
    }

    /// Rich interactions with checks run by your integrations.
    pub fn checks(&self) -> checks::Checks {
        checks::Checks::new(self.clone())
    }

    /// Retrieve code scanning alerts from a repository.
    pub fn code_scanning(&self) -> code_scanning::CodeScanning {
        code_scanning::CodeScanning::new(self.clone())
    }

    /// Insight into codes of conduct for your communities.
    pub fn codes_of_conduct(&self) -> codes_of_conduct::CodesOfConduct {
        codes_of_conduct::CodesOfConduct::new(self.clone())
    }

    /// List emojis available to use on GitHub.
    pub fn emojis(&self) -> emojis::Emojis {
        emojis::Emojis::new(self.clone())
    }

    /// Administer a GitHub enterprise.
    pub fn enterprise_admin(&self) -> enterprise_admin::EnterpriseAdmin {
        enterprise_admin::EnterpriseAdmin::new(self.clone())
    }

    /// View, modify your gists.
    pub fn gists(&self) -> gists::Gists {
        gists::Gists::new(self.clone())
    }

    /// Raw Git functionality.
    pub fn git(&self) -> git::Git {
        git::Git::new(self.clone())
    }

    /// View gitignore templates.
    pub fn gitignore(&self) -> gitignore::Gitignore {
        gitignore::Gitignore::new(self.clone())
    }

    /// Owner or admin management of users interactions.
    pub fn interactions(&self) -> interactions::Interactions {
        interactions::Interactions::new(self.clone())
    }

    /// Interact with GitHub Issues.
    pub fn issues(&self) -> issues::Issues {
        issues::Issues::new(self.clone())
    }

    /// View various OSS licenses.
    pub fn licenses(&self) -> licenses::Licenses {
        licenses::Licenses::new(self.clone())
    }

    /// Render Github flavored markdown.
    pub fn markdown(&self) -> markdown::Markdown {
        markdown::Markdown::new(self.clone())
    }

    /// Endpoints that give information about the API.
    pub fn meta(&self) -> meta::Meta {
        meta::Meta::new(self.clone())
    }

    /// Move projects to or from GitHub.
    pub fn migrations(&self) -> migrations::Migrations {
        migrations::Migrations::new(self.clone())
    }

    /// Manage access of OAuth applications.
    pub fn oauth_authorizations(&self) -> oauth_authorizations::OauthAuthorizations {
        oauth_authorizations::OauthAuthorizations::new(self.clone())
    }

    /// Interact with GitHub Orgs.
    pub fn orgs(&self) -> orgs::Orgs {
        orgs::Orgs::new(self.clone())
    }

    /// Manage packages for authenticated users and organizations.
    pub fn packages(&self) -> packages::Packages {
        packages::Packages::new(self.clone())
    }

    /// Interact with GitHub Projects.
    pub fn projects(&self) -> projects::Projects {
        projects::Projects::new(self.clone())
    }

    /// Interact with GitHub Pull Requests.
    pub fn pulls(&self) -> pulls::Pulls {
        pulls::Pulls::new(self.clone())
    }

    /// Check your current rate limit status.
    pub fn rate_limit(&self) -> rate_limit::RateLimit {
        rate_limit::RateLimit::new(self.clone())
    }

    /// Interact with reactions to various GitHub entities.
    pub fn reactions(&self) -> reactions::Reactions {
        reactions::Reactions::new(self.clone())
    }

    /// Interact with GitHub Repos.
    pub fn repos(&self) -> repos::Repos {
        repos::Repos::new(self.clone())
    }

    /// Provisioning of GitHub organization membership for SCIM-enabled providers.
    pub fn scim(&self) -> scim::Scim {
        scim::Scim::new(self.clone())
    }

    /// Look for stuff on GitHub.
    pub fn search(&self) -> search::Search {
        search::Search::new(self.clone())
    }

    /// Retrieve secret scanning alerts from a repository.
    pub fn secret_scanning(&self) -> secret_scanning::SecretScanning {
        secret_scanning::SecretScanning::new(self.clone())
    }

    /// Interact with GitHub Teams.
    pub fn teams(&self) -> teams::Teams {
        teams::Teams::new(self.clone())
    }

    /// Interact with and view information about users and also current user.
    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }
}
