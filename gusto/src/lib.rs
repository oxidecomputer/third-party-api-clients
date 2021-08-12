//! A fully generated, opinionated API client library for Gusto.
//!
//! ## API Details
//!
//! Welcome to Gusto's API documentation.
//!
//! [API Terms of Service](https://gusto.com/about/terms/developer-terms-of-service)
//!
//! ### Contact
//!
//!
//! | name | email |
//! |----|----|
//! | Developer Relations | developer@gusto.com |
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [Gusto OpenAPI
//! specs](https://github.com/Gusto-API/api.gusto.dev) based on OpenAPI spec version `1.0`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//! //! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! gusto_api = "0.2.10"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use gusto_api::Client;
//!
//! let gusto = Client::new(
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
//! - `GUSTO_CLIENT_ID`
//! - `GUSTO_CLIENT_SECRET`
//! - `GUSTO_REDIRECT_URI`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use gusto_api::Client;
//!
//! let gusto = Client::new_from_env(String::from("token"), String::from("refresh-token"));
//! ```
//!
//! It is okay to pass empty values for `token` and `refresh_token`. In
//! the initial state of the client, you will not know these values.
//!
//! To start off a fresh client and get a `token` and `refresh_token`, use the following.
//!
//! ```
//! use gusto_api::Client;
//!
//! async fn do_call() {
//!     let mut gusto = Client::new_from_env("", "");
//!
//!     // Get the URL to request consent from the user.
//!     // You can optionally pass in scopes. If none are provided, then the
//!     // resulting URL will not have any scopes.
//!     let user_consent_url = gusto.user_consent_url(&["some-scope".to_string()]);
//!
//!     // In your redirect URL capture the code sent and our state.
//!     // Send it along to the request for the token.
//!     let code = "thing-from-redirect-url";
//!     let state = "state-from-redirect-url";
//!     let mut access_token = gusto.get_access_token(code, state).await.unwrap();
//!
//!     // You can additionally refresh the access token with the following.
//!     // You must have a refresh token to be able to call this function.
//!     access_token = gusto.refresh_access_token().await.unwrap();
//! }
//! ```
#![feature(async_stream)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod admins_beta;
pub mod benefits;
pub mod companies;
pub mod company_bank_accounts_beta;
pub mod compensations;
pub mod contractor_payments;
pub mod contractors;
pub mod current_user;
pub mod custom_fields;
pub mod earning_type;
pub mod employees;
pub mod garnishments;
pub mod job_applicants_beta;
pub mod jobs;
pub mod locations;
pub mod pay_schedules;
pub mod payroll;
pub mod terminations;
#[cfg(test)]
mod tests;
pub mod time_off_requests;
pub mod types;
#[doc(hidden)]
pub mod utils;

use anyhow::{anyhow, Error, Result};

pub const DEFAULT_HOST: &str = "https://api.gusto.com";

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

const TOKEN_ENDPOINT: &str = "https://api.gusto.com/oauth/token";
const USER_CONSENT_ENDPOINT: &str = "https://api.gusto.com/oauth/authorize";

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
    pub access_token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token_type: String,
    #[serde(default)]
    pub expires_in: i64,
    #[serde(default, alias = "x_refresh_token_expires_in")]
    pub refresh_token_expires_in: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub refresh_token: String,
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
        let client_id = env::var("GUSTO_CLIENT_ID").unwrap();
        let client_secret = env::var("GUSTO_CLIENT_SECRET").unwrap();
        let redirect_uri = env::var("GUSTO_REDIRECT_URI").unwrap();

        Client::new(client_id, client_secret, redirect_uri, token, refresh_token)
    }

    async fn url_and_auth(&self, uri: &str) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();

        let auth = format!("Bearer {}", self.token);
        parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
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

    /// Return a reference to an interface that provides access to Current User operations.
    pub fn current_user(&self) -> current_user::CurrentUser {
        current_user::CurrentUser::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Companies operations.
    pub fn companies(&self) -> companies::Companies {
        companies::Companies::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Employees operations.
    pub fn employees(&self) -> employees::Employees {
        employees::Employees::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Contractors operations.
    pub fn contractors(&self) -> contractors::Contractors {
        contractors::Contractors::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Payroll operations.
    pub fn payroll(&self) -> payroll::Payroll {
        payroll::Payroll::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Contractor Payments operations.
    pub fn contractor_payments(&self) -> contractor_payments::ContractorPayments {
        contractor_payments::ContractorPayments::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Company Bank Accounts (Beta) operations.
    pub fn company_bank_accounts_beta(
        &self,
    ) -> company_bank_accounts_beta::CompanyBankAccountsBeta {
        company_bank_accounts_beta::CompanyBankAccountsBeta::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Benefits operations.
    pub fn benefits(&self) -> benefits::Benefits {
        benefits::Benefits::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Locations operations.
    pub fn locations(&self) -> locations::Locations {
        locations::Locations::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Jobs operations.
    pub fn jobs(&self) -> jobs::Jobs {
        jobs::Jobs::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Job Applicants (Beta) operations.
    pub fn job_applicants_beta(&self) -> job_applicants_beta::JobApplicantsBeta {
        job_applicants_beta::JobApplicantsBeta::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Compensations operations.
    pub fn compensations(&self) -> compensations::Compensations {
        compensations::Compensations::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Pay Schedules operations.
    pub fn pay_schedules(&self) -> pay_schedules::PaySchedules {
        pay_schedules::PaySchedules::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Garnishments operations.
    pub fn garnishments(&self) -> garnishments::Garnishments {
        garnishments::Garnishments::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Time Off Requests operations.
    pub fn time_off_requests(&self) -> time_off_requests::TimeOffRequests {
        time_off_requests::TimeOffRequests::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Earning Type operations.
    pub fn earning_type(&self) -> earning_type::EarningType {
        earning_type::EarningType::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Terminations operations.
    pub fn terminations(&self) -> terminations::Terminations {
        terminations::Terminations::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Custom Fields operations.
    pub fn custom_fields(&self) -> custom_fields::CustomFields {
        custom_fields::CustomFields::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Admins (Beta) operations.
    pub fn admins_beta(&self) -> admins_beta::AdminsBeta {
        admins_beta::AdminsBeta::new(self.clone())
    }
}
