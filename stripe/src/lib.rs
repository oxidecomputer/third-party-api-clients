//! A fully generated, opinionated API client library for Stripe.
//!
//! [![docs.rs](https://docs.rs/dolladollabills/badge.svg)](https://docs.rs/dolladollabills)
//!
//! ## API Details
//!
//! The Stripe REST API. Please see https://stripe.com/docs/api for more details.
//!
//! [API Terms of Service](https://stripe.com/us/terms/)
//!
//! ### Contact
//!
//!
//! | name | url | email |
//! |----|----|----|
//! | Stripe Dev Platform Team | <https://stripe.com> | dev-platform@stripe.com |
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [Stripe OpenAPI
//! specs](https://raw.githubusercontent.com/stripe/openapi/master/openapi/spec3.json) based on API spec version `2020-08-27`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! dolladollabills = "0.4.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use dolladollabills::Client;
//!
//! let stripe = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `STRIPE_API_KEY`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use dolladollabills::Client;
//!
//! let stripe = Client::new_from_env();
//! ```
//!
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod account;
pub mod account_links;
pub mod accounts;
pub mod apple_pay;
pub mod application_fees;
pub mod balance;
pub mod balance_transactions;
pub mod billing_portal;
pub mod bitcoin;
pub mod charges;
pub mod checkout;
pub mod country_specs;
pub mod coupons;
pub mod credit_notes;
pub mod customers;
pub mod disputes;
pub mod ephemeral_keys;
pub mod events;
pub mod exchange_rates;
pub mod file_links;
pub mod files;
pub mod identity;
pub mod invoiceitems;
pub mod invoices;
pub mod issuer_fraud_records;
pub mod issuing;
pub mod mandates;
pub mod order_returns;
pub mod orders;
pub mod payment_intents;
pub mod payment_links;
pub mod payment_methods;
pub mod payouts;
pub mod plans;
pub mod prices;
pub mod products;
pub mod promotion_codes;
pub mod quotes;
pub mod radar;
pub mod recipients;
pub mod refunds;
pub mod reporting;
pub mod reviews;
pub mod setup_attempts;
pub mod setup_intents;
pub mod shipping_rates;
pub mod sigma;
pub mod skus;
pub mod sources;
pub mod subscription_items;
pub mod subscription_schedules;
pub mod subscriptions;
pub mod tax_codes;
pub mod tax_rates;
pub mod terminal;
pub mod test_helpers;
pub mod three_d_secure;
pub mod tokens;
pub mod topups;
pub mod transfers;
pub mod types;
#[doc(hidden)]
pub mod utils;
pub mod webhook_endpoints;

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

pub const FALLBACK_HOST: &str = "https://api.stripe.com/v1";

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
        "https://api.stripe.com/"
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
        let token = env::var("STRIPE_API_KEY").expect("must set STRIPE_API_KEY");

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

    pub fn account(&self) -> account::Account {
        account::Account::new(self.clone())
    }

    pub fn account_links(&self) -> account_links::AccountLinks {
        account_links::AccountLinks::new(self.clone())
    }

    pub fn accounts(&self) -> accounts::Accounts {
        accounts::Accounts::new(self.clone())
    }

    pub fn apple_pay(&self) -> apple_pay::ApplePay {
        apple_pay::ApplePay::new(self.clone())
    }

    pub fn application_fees(&self) -> application_fees::ApplicationFees {
        application_fees::ApplicationFees::new(self.clone())
    }

    pub fn balance(&self) -> balance::Balance {
        balance::Balance::new(self.clone())
    }

    pub fn balance_transactions(&self) -> balance_transactions::BalanceTransactions {
        balance_transactions::BalanceTransactions::new(self.clone())
    }

    pub fn billing_portal(&self) -> billing_portal::BillingPortal {
        billing_portal::BillingPortal::new(self.clone())
    }

    pub fn bitcoin(&self) -> bitcoin::Bitcoin {
        bitcoin::Bitcoin::new(self.clone())
    }

    pub fn charges(&self) -> charges::Charges {
        charges::Charges::new(self.clone())
    }

    pub fn checkout(&self) -> checkout::Checkout {
        checkout::Checkout::new(self.clone())
    }

    pub fn country_specs(&self) -> country_specs::CountrySpecs {
        country_specs::CountrySpecs::new(self.clone())
    }

    pub fn coupons(&self) -> coupons::Coupons {
        coupons::Coupons::new(self.clone())
    }

    pub fn credit_notes(&self) -> credit_notes::CreditNotes {
        credit_notes::CreditNotes::new(self.clone())
    }

    pub fn customers(&self) -> customers::Customers {
        customers::Customers::new(self.clone())
    }

    pub fn disputes(&self) -> disputes::Disputes {
        disputes::Disputes::new(self.clone())
    }

    pub fn ephemeral_keys(&self) -> ephemeral_keys::EphemeralKeys {
        ephemeral_keys::EphemeralKeys::new(self.clone())
    }

    pub fn events(&self) -> events::Events {
        events::Events::new(self.clone())
    }

    pub fn exchange_rates(&self) -> exchange_rates::ExchangeRates {
        exchange_rates::ExchangeRates::new(self.clone())
    }

    pub fn file_links(&self) -> file_links::FileLinks {
        file_links::FileLinks::new(self.clone())
    }

    pub fn files(&self) -> files::Files {
        files::Files::new(self.clone())
    }

    pub fn identity(&self) -> identity::Identity {
        identity::Identity::new(self.clone())
    }

    pub fn invoiceitems(&self) -> invoiceitems::Invoiceitems {
        invoiceitems::Invoiceitems::new(self.clone())
    }

    pub fn invoices(&self) -> invoices::Invoices {
        invoices::Invoices::new(self.clone())
    }

    pub fn issuer_fraud_records(&self) -> issuer_fraud_records::IssuerFraudRecords {
        issuer_fraud_records::IssuerFraudRecords::new(self.clone())
    }

    pub fn issuing(&self) -> issuing::Issuing {
        issuing::Issuing::new(self.clone())
    }

    pub fn mandates(&self) -> mandates::Mandates {
        mandates::Mandates::new(self.clone())
    }

    pub fn order_returns(&self) -> order_returns::OrderReturns {
        order_returns::OrderReturns::new(self.clone())
    }

    pub fn orders(&self) -> orders::Orders {
        orders::Orders::new(self.clone())
    }

    pub fn payment_intents(&self) -> payment_intents::PaymentIntents {
        payment_intents::PaymentIntents::new(self.clone())
    }

    pub fn payment_links(&self) -> payment_links::PaymentLinks {
        payment_links::PaymentLinks::new(self.clone())
    }

    pub fn payment_methods(&self) -> payment_methods::PaymentMethods {
        payment_methods::PaymentMethods::new(self.clone())
    }

    pub fn payouts(&self) -> payouts::Payouts {
        payouts::Payouts::new(self.clone())
    }

    pub fn plans(&self) -> plans::Plans {
        plans::Plans::new(self.clone())
    }

    pub fn prices(&self) -> prices::Prices {
        prices::Prices::new(self.clone())
    }

    pub fn products(&self) -> products::Products {
        products::Products::new(self.clone())
    }

    pub fn promotion_codes(&self) -> promotion_codes::PromotionCodes {
        promotion_codes::PromotionCodes::new(self.clone())
    }

    pub fn quotes(&self) -> quotes::Quotes {
        quotes::Quotes::new(self.clone())
    }

    pub fn radar(&self) -> radar::Radar {
        radar::Radar::new(self.clone())
    }

    pub fn recipients(&self) -> recipients::Recipients {
        recipients::Recipients::new(self.clone())
    }

    pub fn refunds(&self) -> refunds::Refunds {
        refunds::Refunds::new(self.clone())
    }

    pub fn reporting(&self) -> reporting::Reporting {
        reporting::Reporting::new(self.clone())
    }

    pub fn reviews(&self) -> reviews::Reviews {
        reviews::Reviews::new(self.clone())
    }

    pub fn setup_attempts(&self) -> setup_attempts::SetupAttempts {
        setup_attempts::SetupAttempts::new(self.clone())
    }

    pub fn setup_intents(&self) -> setup_intents::SetupIntents {
        setup_intents::SetupIntents::new(self.clone())
    }

    pub fn shipping_rates(&self) -> shipping_rates::ShippingRates {
        shipping_rates::ShippingRates::new(self.clone())
    }

    pub fn sigma(&self) -> sigma::Sigma {
        sigma::Sigma::new(self.clone())
    }

    pub fn skus(&self) -> skus::Skus {
        skus::Skus::new(self.clone())
    }

    pub fn sources(&self) -> sources::Sources {
        sources::Sources::new(self.clone())
    }

    pub fn subscription_items(&self) -> subscription_items::SubscriptionItems {
        subscription_items::SubscriptionItems::new(self.clone())
    }

    pub fn subscription_schedules(&self) -> subscription_schedules::SubscriptionSchedules {
        subscription_schedules::SubscriptionSchedules::new(self.clone())
    }

    pub fn subscriptions(&self) -> subscriptions::Subscriptions {
        subscriptions::Subscriptions::new(self.clone())
    }

    pub fn tax_codes(&self) -> tax_codes::TaxCodes {
        tax_codes::TaxCodes::new(self.clone())
    }

    pub fn tax_rates(&self) -> tax_rates::TaxRates {
        tax_rates::TaxRates::new(self.clone())
    }

    pub fn terminal(&self) -> terminal::Terminal {
        terminal::Terminal::new(self.clone())
    }

    pub fn test_helpers(&self) -> test_helpers::TestHelpers {
        test_helpers::TestHelpers::new(self.clone())
    }

    pub fn three_d_secure(&self) -> three_d_secure::ThreeDSecure {
        three_d_secure::ThreeDSecure::new(self.clone())
    }

    pub fn tokens(&self) -> tokens::Tokens {
        tokens::Tokens::new(self.clone())
    }

    pub fn topups(&self) -> topups::Topups {
        topups::Topups::new(self.clone())
    }

    pub fn transfers(&self) -> transfers::Transfers {
        transfers::Transfers::new(self.clone())
    }

    pub fn webhook_endpoints(&self) -> webhook_endpoints::WebhookEndpoints {
        webhook_endpoints::WebhookEndpoints::new(self.clone())
    }
}
