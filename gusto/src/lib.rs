//! A fully generated, opinionated API client library for GitHub.
//!
//! This library is generated from the [GitHub OpenAPI
//! specs](https://github.com/github/rest-api-description). This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! gusto_api = "0.2.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of `auth::Credentials`.
//!
//! ```
//! use gusto_api::{auth::Credentials, Client};
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
//! [Client#host](https://docs.rs/gusto_api/0.2.0/gusto_api/struct.Client.html#method.host) method.
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
//! gusto_api = { version = "0.2.0", features = ["httpcache"] }
//! ```
//!
//! Then use the `Client::custom` constructor to provide a cache implementation.
//!
//! Here is an example:
//!
//! ```
//! use gusto_api::{auth::Credentials, Client};
//! #[cfg(feature = "httpcache")]
//! use gusto_api::http_cache::HttpCache;
//!
//! #[cfg(feature = "httpcache")]
//! let http_cache = HttpCache::in_home_dir();
//!
//! #[cfg(not(feature = "httpcache"))]
//! let github = Client::custom(
//!     "https://api.github.com",
//!     concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
//!     Credentials::Token(
//!       String::from("personal-access-token")
//!     ),
//!     reqwest::Client::builder().build().unwrap(),
//! );
//!
//! #[cfg(feature = "httpcache")]
//! let github = Client::custom(
//!     "https://api.github.com",
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
//! use gusto_api::{Client, auth::{Credentials, InstallationTokenGenerator, JWTCredentials}};
//! #[cfg(feature = "httpcache")]
//! use gusto_api::http_cache::FileBasedCache;
//!
//! let app_id_str = env::var("GH_APP_ID").unwrap();
//! let app_id = app_id_str.parse::<u64>().unwrap();
//!
//! let app_installation_id_str = env::var("GH_INSTALLATION_ID").unwrap();
//! let app_installation_id = app_installation_id_str.parse::<u64>().unwrap();
//!
//! let encoded_private_key = env::var("GH_PRIVATE_KEY").unwrap();
//! let private_key = base64::decode(encoded_private_key).unwrap();
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
//!     "https://api.github.com",
//!     concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
//!     Credentials::InstallationToken(token_generator),
//!     reqwest::Client::builder().build().unwrap(),
//! );
//!
//! #[cfg(feature = "httpcache")]
//! let github = Client::custom(
//!     "https://api.github.com",
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
#![feature(async_stream)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod auth;
#[cfg(feature = "httpcache")]
#[cfg_attr(docsrs, doc(cfg(feature = "httpcache")))]
pub mod http_cache;
pub mod types;
#[doc(hidden)]
pub mod utils;
pub mod current_user;
pub mod companies;
pub mod employees;
pub mod contractors;
pub mod payroll;
pub mod contractor_payments;
pub mod company_bank_accounts_beta;
pub mod benefits;
pub mod locations;
pub mod jobs;
pub mod job_applicants_beta;
pub mod compensations;
pub mod pay_schedules;
pub mod garnishments;
pub mod time_off_requests;
pub mod earning_type;
pub mod terminations;
pub mod custom_fields;
pub mod admins_beta;

use anyhow::{anyhow, Error, Result};

const DEFAULT_HOST: &str = "https://api.github.com";

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

    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}


/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    agent: String,
    client: reqwest::Client,
    credentials: Option<crate::auth::Credentials>,
    #[cfg(feature = "httpcache")]
    http_cache: crate::http_cache::BoxedHttpCache,
}

impl Client {
    pub fn new<A, C>(agent: A, credentials: C) -> Result<Self>
    where
        A: Into<String>,
        C: Into<Option<crate::auth::Credentials>>,
    {
        Self::host(DEFAULT_HOST, agent, credentials)
    }

    pub fn host<H, A, C>(host: H, agent: A, credentials: C) -> Result<Self>
    where
        H: Into<String>,
        A: Into<String>,
        C: Into<Option<crate::auth::Credentials>>,
    {
        let http = reqwest::Client::builder().build()?;
        #[cfg(feature = "httpcache")]
        {
            Ok(Self::custom(
                host,
                agent,
                credentials,
                http,
                <dyn crate::http_cache::HttpCache>::noop(),
            ))
        }
        #[cfg(not(feature = "httpcache"))]
        {
            Ok(Self::custom(host, agent, credentials, http))
        }
    }

    #[cfg(feature = "httpcache")]
    pub fn custom<H, A, CR>(
        host: H,
        agent: A,
        credentials: CR,
        http: reqwest::Client,
        http_cache: crate::http_cache::BoxedHttpCache,
    ) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
            http_cache,
        }
    }

    #[cfg(not(feature = "httpcache"))]
    pub fn custom<H, A, CR>(host: H, agent: A, credentials: CR, http: reqwest::Client) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
        }
    }

    pub fn set_credentials<CR>(&mut self, credentials: CR)
    where
        CR: Into<Option<crate::auth::Credentials>>,
    {
        self.credentials = credentials.into();
    }

    fn credentials(&self, authentication: crate::auth::AuthenticationConstraint) -> Option<&crate::auth::Credentials> {
        match (authentication, self.credentials.as_ref()) {
            (crate::auth::AuthenticationConstraint::Unconstrained, creds) => creds,
            (crate::auth::AuthenticationConstraint::JWT, creds @ Some(&crate::auth::Credentials::JWT(_))) => creds,
            (
                crate::auth::AuthenticationConstraint::JWT,
                Some(&crate::auth::Credentials::InstallationToken(ref apptoken)),
            ) => Some(apptoken.jwt()),
            (crate::auth::AuthenticationConstraint::JWT, creds) => {
                println!(
                    "Request needs JWT authentication but only {:?} available",
                    creds
                );
                None
            }
        }
    }

    async fn url_and_auth(
        &self,
        uri: &str,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();

        match self.credentials(authentication) {
            Some(&crate::auth::Credentials::Client(ref id, ref secret)) => parsed_url
                .map(|mut u| {
                    u.query_pairs_mut()
                        .append_pair("client_id", id)
                        .append_pair("client_secret", secret);
                    (u, None)
                })
                .map_err(Error::from),
            Some(&crate::auth::Credentials::Token(ref token)) => {
                let auth = format!("token {}", token);
                parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
            }
            Some(&crate::auth::Credentials::JWT(ref jwt)) => {
                let auth = format!("Bearer {}", jwt.token());
                parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
            }
            Some(&crate::auth::Credentials::InstallationToken(ref apptoken)) => {
                if let Some(token) = apptoken.token() {
                    let auth = format!("token {}", token);
                    parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
                } else {
                    println!("App token is stale, refreshing");
                    let token_ref = apptoken.access_key.clone();

                    let token = self.apps().create_installation_access_token(apptoken.installation_id as i64,
                    &types::AppsCreateInstallationAccessTokenRequest{
                        permissions: Default::default(),
                        repositories: Default::default(),
                        repository_ids: Default::default(),
                    }).await.unwrap();
                    let auth = format!("token {}", &token.token);
                    *token_ref.lock().unwrap() = Some(token.token);
                    parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
                }
            }
            None => parsed_url.map(|u| (u, None)).map_err(Error::from),
        }
    }

    async fn request<Out>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<(Option<hyperx::header::Link>, Out)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        #[cfg(feature = "httpcache")]
        let uri2 = uri.to_string();

        let (url, auth) = self.url_and_auth(uri, authentication).await?;

        let instance = <&Client>::clone(&self);

        #[cfg(not(feature = "httpcache"))]
        let mut req = instance.client.request(method, url);

        #[cfg(feature = "httpcache")]
        let mut req = {
            let mut req = instance.client.request(method.clone(), url);
            if method == http::Method::GET {
                if let Ok(etag) = instance.http_cache.lookup_etag(&uri2) {
                    req = req.header(http::header::IF_NONE_MATCH, etag);
                }
            }
            req
        };

        req = req.header(http::header::USER_AGENT, &*instance.agent);
        req = req.header(
            http::header::ACCEPT,
            &*format!("{}", hyperx::header::qitem::<mime::Mime>(From::from(media_type))),
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

        #[cfg(feature = "httpcache")]
        let instance2 = <&Client>::clone(&self);

        #[cfg(feature = "httpcache")]
        let uri3 = uri.to_string();

        #[cfg(not(feature = "httpcache"))]
        let (remaining, reset) = crate::utils::get_header_values(response.headers());

        #[cfg(feature = "httpcache")]
        let (remaining, reset, etag) = crate::utils::get_header_values(response.headers());

        let status = response.status();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| l.parse().ok());

        let response_body = response.bytes().await?;

        if status.is_success() {
            //println!("response payload {}", String::from_utf8_lossy(&response_body));
            #[cfg(feature = "httpcache")]
            {
                if let Some(etag) = etag {
                    let next_link = link.as_ref().and_then(|l| crate::utils::next_link(l));
                    if let Err(e) = instance2.http_cache.cache_response(
                        &uri3,
                        &response_body,
                        &etag,
                        &next_link,
                    ) {
                        // failing to cache isn't fatal, so just log & swallow the error
                        println!("Failed to cache body & etag: {}", e);
                    }
                }
            }
            let parsed_response = if status == http::StatusCode::NO_CONTENT { serde_json::from_str("null") } else { serde_json::from_slice::<Out>(&response_body) };
            parsed_response.map(|out| (link, out)).map_err(Error::from)
        } else if status == http::StatusCode::NOT_MODIFIED {
                // only supported case is when client provides if-none-match
                // header when cargo builds with --cfg feature="httpcache"
                #[cfg(feature = "httpcache")]
                {
                    let body = instance2.http_cache.lookup_body(&uri3).unwrap();
                    let out = serde_json::from_str::<Out>(&body).unwrap();
                    let link = match link {
                        Some(link) => Ok(Some(link)),
                        None => instance2.http_cache.lookup_next_link(&uri3)
                                    .map(|next_link| next_link.map(|next| {
                                        let next = hyperx::header::LinkValue::new(next).push_rel(hyperx::header::RelationType::Next);
                                        hyperx::header::Link::new(vec![next])
                                    }))
                    };
                    link.map(|link| (link, out))
                }
                #[cfg(not(feature = "httpcache"))]
                {
                    unreachable!("this should not be reachable without the httpcache feature enabled")
                }
        } else {
            /*println!("error status: {:?}, response payload: {}",
                status,
                String::from_utf8_lossy(&response_body),
            );*/
            let error = match (remaining, reset) {
                (Some(remaining), Some(reset)) if remaining == 0 => {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    anyhow!("rate limit exceeded, will reset in {} seconds", u64::from(reset) - now)
                },
                _ => {
                    if response_body.is_empty() {
                        anyhow!("code: {}, empty response", status)
                    } else {
                        anyhow!("code: {}, error: {:?}", status, serde_json::from_slice(&response_body)?)
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
        body: Option<reqwest::Body>,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let (_ , r) = self.request(method, uri, body, media_type, authentication).await?;
        Ok(r)
    }

    async fn get<D>(&self, uri: &str) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.get_media(uri, crate::utils::MediaType::Json).await
    }

    async fn get_media<D>(&self, uri: &str, media: crate::utils::MediaType) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::GET,
            &(self.host.clone() + uri),
            None,
            media,
            self::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn get_all_pages<D>(&self, uri: &str) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.unfold(uri).await
    }

    async fn get_pages<D>(&self, uri: &str) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request(
            http::Method::GET,
            &(self.host.clone() + uri),
            None,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn get_pages_url<D>(&self, url: &reqwest::Url) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request(
            http::Method::GET,
            url.as_str(),
            None,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.post_media(
            uri,
            message,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn post_media<D>(
        &self,
        uri: &str,
        message: Option<reqwest::Body>,
        media: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::POST,
            &(self.host.clone() + uri),
            message,
            media,
            authentication,
        ).await
    }

    async fn patch_media<D>(&self, uri: &str, message: Option<reqwest::Body>, media: crate::utils::MediaType) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PATCH,
            &(self.host.clone() + uri),
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.patch_media(uri, message, crate::utils::MediaType::Json).await
    }

    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.put_media(uri, message, crate::utils::MediaType::Json).await
    }

    async fn put_media<D>(&self, uri: &str, message: Option<reqwest::Body>, media: crate::utils::MediaType) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PUT,
            &(self.host.clone() + uri),
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(self.host.clone() + uri),
            message,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    /// "unfold" paginated results of a vector of items
    async fn unfold<D>(
        &self,
        uri: &str,
    ) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut items) = self.get_pages(uri).await.unwrap();
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = link.as_ref().and_then(|l| crate::utils::next_link(l)) {
                let url = reqwest::Url::parse(&url).unwrap();
                let (new_link, new_items) = self.get_pages_url(&url).await?;
                link = new_link;
                items = new_items;
            }
        }

        Ok(global_items)
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
               pub fn company_bank_accounts_beta(&self) -> company_bank_accounts_beta::CompanyBankAccountsBeta {
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
