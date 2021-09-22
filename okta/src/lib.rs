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
//! okta = "0.2.0"
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
//! - `OKTA_CLIENT_ID`
//! - `OKTA_CLIENT_SECRET`
//! - `OKTA_REDIRECT_URI`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use okta::Client;
//!
//! let okta = Client::new_from_env(String::from("token"), String::from("refresh-token"));
//! ```
//!
//! It is okay to pass empty values for `token` and `refresh_token`. In
//! the initial state of the client, you will not know these values.
//!
//! To start off a fresh client and get a `token` and `refresh_token`, use the following.
//!
//! ```
//! use okta::Client;
//!
//! async fn do_call() {
//!     let mut okta = Client::new_from_env("", "");
//!
//!     // Get the URL to request consent from the user.
//!     // You can optionally pass in scopes. If none are provided, then the
//!     // resulting URL will not have any scopes.
//!     let user_consent_url = okta.user_consent_url(&["some-scope".to_string()]);
//!
//!     // In your redirect URL capture the code sent and our state.
//!     // Send it along to the request for the token.
//!     let code = "thing-from-redirect-url";
//!     let state = "state-from-redirect-url";
//!     let mut access_token = okta.get_access_token(code, state).await.unwrap();
//!
//!     // You can additionally refresh the access token with the following.
//!     // You must have a refresh token to be able to call this function.
//!     access_token = okta.refresh_access_token().await.unwrap();
//! }
//! ```
#![feature(async_stream)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod application;
pub mod authorization_server;
pub mod domain;
pub mod event_hook;
pub mod feature;
pub mod group;
pub mod identity_provider;
pub mod inline_hook;
pub mod linked_object;
pub mod log;
pub mod network_zone;
pub mod policy;
pub mod profile_mapping;
pub mod session;
pub mod template;
#[cfg(test)]
mod tests;
pub mod threat_insight;
pub mod trusted_origin;
pub mod types;
pub mod user;
pub mod user_factor;
pub mod user_schema;
pub mod user_type;
#[doc(hidden)]
pub mod utils;

use anyhow::{anyhow, Error, Result};

pub const DEFAULT_HOST: &str = "https://na4.okta.net";

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

const TOKEN_ENDPOINT: &str = "https://account.okta.com/oauth/token";
const USER_CONSENT_ENDPOINT: &str = "https://account.okta.com/oauth/auth";

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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
        let client_id = env::var("OKTA_CLIENT_ID").expect("must set OKTA_CLIENT_ID");
        let client_secret = env::var("OKTA_CLIENT_SECRET").expect("must set OKTA_CLIENT_SECRET");
        let redirect_uri = env::var("OKTA_REDIRECT_URI").expect("must set OKTA_REDIRECT_URI");

        Client::new(client_id, client_secret, redirect_uri, token, refresh_token)
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
            .await?;

        // Unwrap the response.
        let t: AccessToken = resp.json().await?;

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
            .await?;

        // Unwrap the response.
        let t: AccessToken = resp.json().await?;

        self.token = t.access_token.to_string();
        self.refresh_token = t.refresh_token.to_string();

        Ok(t)
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

        let mut req = instance.client.request(method.clone(), url);

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
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
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

    async fn request_with_links<Out>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<(Option<hyperx::header::Link>, Out)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, body).await?;

        let status = response.status();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| l.parse().ok());

        let response_body = response.bytes().await?;

        if status.is_success() {
            //println!("response payload {}", String::from_utf8_lossy(&response_body));

            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map(|out| (link, out)).map_err(Error::from)
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
    async fn request_with_accept_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        accept_mime_type: &str,
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
            reqwest::header::HeaderValue::from_str(accept_mime_type)?,
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        //println!("Request: {:?}", &req);
        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            //println!("response payload {}", String::from_utf8_lossy(&response_body));
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                serde_json::from_value(serde_json::json!(&String::from_utf8(
                    response_body.to_vec()
                )?))
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
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
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
    async fn get_all_pages<D>(&self, uri: &str, _message: Option<reqwest::Body>) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        // TODO: implement this.
        self.unfold(uri).await
    }

    /// "unfold" paginated results of a vector of items
    #[allow(dead_code)]
    async fn unfold<D>(&self, uri: &str) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut items) = self.get_pages(uri).await?;
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = link.as_ref().and_then(|l| crate::utils::next_link(l)) {
                let url = reqwest::Url::parse(&url)?;
                let (new_link, new_items) = self.get_pages_url(&url).await?;
                link = new_link;
                items = new_items;
            }
        }

        Ok(global_items)
    }

    #[allow(dead_code)]
    async fn get_pages<D>(&self, uri: &str) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, &(DEFAULT_HOST.to_string() + uri), None)
            .await
    }

    #[allow(dead_code)]
    async fn get_pages_url<D>(
        &self,
        url: &reqwest::Url,
    ) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, url.as_str(), None)
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

    pub fn application(&self) -> application::Application {
        application::Application::new(self.clone())
    }

    pub fn authorization_server(&self) -> authorization_server::AuthorizationServer {
        authorization_server::AuthorizationServer::new(self.clone())
    }

    pub fn domain(&self) -> domain::Domain {
        domain::Domain::new(self.clone())
    }

    pub fn event_hook(&self) -> event_hook::EventHook {
        event_hook::EventHook::new(self.clone())
    }

    pub fn feature(&self) -> feature::Feature {
        feature::Feature::new(self.clone())
    }

    pub fn group(&self) -> group::Group {
        group::Group::new(self.clone())
    }

    pub fn identity_provider(&self) -> identity_provider::IdentityProvider {
        identity_provider::IdentityProvider::new(self.clone())
    }

    pub fn inline_hook(&self) -> inline_hook::InlineHook {
        inline_hook::InlineHook::new(self.clone())
    }

    pub fn linked_object(&self) -> linked_object::LinkedObject {
        linked_object::LinkedObject::new(self.clone())
    }

    pub fn log(&self) -> log::Log {
        log::Log::new(self.clone())
    }

    pub fn network_zone(&self) -> network_zone::NetworkZone {
        network_zone::NetworkZone::new(self.clone())
    }

    pub fn policy(&self) -> policy::Policy {
        policy::Policy::new(self.clone())
    }

    pub fn profile_mapping(&self) -> profile_mapping::ProfileMapping {
        profile_mapping::ProfileMapping::new(self.clone())
    }

    pub fn session(&self) -> session::Session {
        session::Session::new(self.clone())
    }

    pub fn template(&self) -> template::Template {
        template::Template::new(self.clone())
    }

    pub fn threat_insight(&self) -> threat_insight::ThreatInsight {
        threat_insight::ThreatInsight::new(self.clone())
    }

    pub fn trusted_origin(&self) -> trusted_origin::TrustedOrigin {
        trusted_origin::TrustedOrigin::new(self.clone())
    }

    pub fn user(&self) -> user::User {
        user::User::new(self.clone())
    }

    pub fn user_factor(&self) -> user_factor::UserFactor {
        user_factor::UserFactor::new(self.clone())
    }

    pub fn user_schema(&self) -> user_schema::UserSchema {
        user_schema::UserSchema::new(self.clone())
    }

    pub fn user_type(&self) -> user_type::UserType {
        user_type::UserType::new(self.clone())
    }
}
