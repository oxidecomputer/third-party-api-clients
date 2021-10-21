//! A fully generated, opinionated API client library for Rev.ai.
//!
//! [![docs.rs](https://docs.rs/revai/badge.svg)](https://docs.rs/revai)
//!
//! ## API Details
//!
//! Rev.ai provides quality speech-text recognition via a RESTful API. All public methods and objects are documented here for developer reference.
//! For a real-time speech to text solution, use Rev.ai's [Streaming API](/docs/streaming).
//!
//! # Base Endpoint
//!
//! The base url for this version of the API is
//!
//! > `https://api.rev.ai/speechtotext/v1`
//!
//! All endpoints described in this documentation are relative to this base url.
//!
//! # Quick Start
//!
//! Follow the [getting started checklist](https://www.rev.ai/getting_started)
//!
//! ## Get your Access Token
//!
//! You can generate your [access token](#section/Authentication/Access-Token) on the [settings page](https://www.rev.ai/access_token) of your account. This access token only needs to be generated once and never expires. You can re-generate your token, however this will invalidate the previous token.
//!
//! ## Submit a File
//!
//! To submit an audio file for transcription to Rev.ai:
//!
//! ```
//! curl -X POST "https://api.rev.ai/speechtotext/v1/jobs" -H "Authorization: Bearer $REV_ACCESS_TOKEN" -H "Content-Type: application/json" -d "{\"media_url\":\"https://www.rev.ai/FTC_Sample_1.mp3\",\"metadata\":\"This is a sample submit jobs option\"}"
//! ```
//!
//! You’ll receive a response like this:
//!
//! ~~~
//! {
//!   "id": "Umx5c6F7pH7r",
//!   "created_on": "2018-09-15T05:14:38.13",
//!   "name": "sample.mp3",
//!   "metadata": "This is a sample submit jobs option for multipart",
//!   "status": "in_progress"
//! }
//! ~~~
//!
//! The `id` (in this case `Umx5c6F7pH7r`) will allow you to retrieve your transcript.
//!
//! ## Get Your Transcript
//!
//! Once a transcription job's `status` becomes `transcribed`, you can retrieve the transcript in JSON format by running:
//!
//! ```
//! curl -X GET "https://api.rev.ai/speechtotext/v1/jobs/{id}/transcript" -H "Authorization: Bearer $REV_ACCESS_TOKEN" -H "Accept: application/vnd.rev.transcript.v1.0+json"
//! ```
//!
//! Alternatively you can get the plain text version by running:
//!
//! ```
//! curl -X GET "https://api.rev.ai/speechtotext/v1/jobs/{id}/transcript" -H "Authorization: Bearer $REV_ACCESS_TOKEN" -H "Accept: text/plain"
//! ```
//!
//! You can poll for the `status` of your job by querying for the job periodically:
//!
//! ```
//! curl -X GET https://api.rev.ai/speechtotext/v1/jobs/{id} -H "Authorization: Bearer $REV_ACCESS_TOKEN"
//! ```
//!
//! **Note:** Polling is NOT recommended in a production server. Rather, use [webhooks](#section/Webhooks) to asynchronously receive notifications once the transcription job completes
//!
//! If you have any further questions, contact us at <support@rev.ai>
//!
//! # Submitting Files
//!
//! Two `POST` request formats can be used to submit a file: `application/json` or `multipart/form-data`.
//!
//! ## JSON
//!
//! This is the preferred method of file submission. Uses the `media_url` property to provide a direct download URL to the Rev.ai server.
//! This method supports the use of pre-signed URLs. Links to videos hosted on platforms like Youtube are not valid because they are not direct download links.
//!
//! **Important note on presigned URLs:**
//! Signed URLs usually have an expiration time which is configurable. To ensure the Rev.ai server can access the link, make sure the expiration time is set to 2 hours or more.
//! In the event you plan on resending the same file, make sure to generate a new presigned URL.
//!
//! ## FormData
//!
//! Used to send a local file to the Rev.ai server. This allows the customer to send the file directly from the host machine.
//! Certain limits apply to this format, see the [Async API Limits section](#section/Async-API-Limits) for more detals.
//!
//! # Turnaround Time and Chunking
//!
//! Often, especially for shorter files, your transcript will be ready in 5 minutes or less. It generally takes no longer than 15 minutes to return longer audios. If you require faster turn around time please contact <support@rev.ai>
//!
//! Chunking is the act of breaking audio files into smaller segments. Rev.ai uses this method to decrease turnaround time of audios greater than 3 minutes in length.
//!
//! # Webhooks
//!
//! If the optional `callback_url` is provided, the API will make an HTTP POST request to the `callback_url` with the
//! following request body when the job either completes successfully or fails.
//!
//! ## Sample Webhook
//!
//! **On Successful Transcription Job**
//!
//! ```
//! {
//!   "job": {
//!     "id": "Umx5c6F7pH7r",
//!     "status": "transcribed",
//!     "created_on": "2018-05-05T23:23:22.29Z",
//!     "callback_url": "https://www.example.com/callback",
//!     "duration_seconds": 356.24,
//!     "media_url": "https://www.rev.ai/FTC_Sample_1.mp3"
//!   }
//! }
//! ```
//!
//! **On Failed Transcription Job**
//!
//! ```
//! {
//!   "job": {
//!     "id": "Umx5c6F7pH7r",
//!     "status": "failed",
//!     "created_on": "2018-05-05T23:23:22.29Z",
//!     "callback_url": "https://www.example.com/callback",
//!     "failure": "download_failure",
//!     "failure_detail": "Failed to download media file. Please check your url and file type"
//!   }
//! }
//! ```
//!
//! **Important notes for using webhooks:**
//! The API will make a POST request, not a GET request, to the `callback_url`. The request body is the job details.
//! You can unsubscribe from a webhook by responding to the webhook request with a 200 response.
//! If a webhook invocation does not receive a 200 Rev.ai will retry the `callback_url` every 30 minutes until either 24 hours have passed or we receive a 200 response.
//!
//! For initial webhook testing, you can try using a third party webhook testing tool such as [https://webhook.site/](https://webhook.site/).
//!
//! # Async API Limits
//!
//! The following default limits apply per user, per endpoint and are configurable by Rev.ai support. If you have any further questions, contact us at <support@rev.ai>
//!
//! - 10,000 transcription requests submitted every 10 minutes
//! - 500 transcriptions processed every 10 minutes
//! - Multi-part/form-data requests to the /jobs endpoint have a concurrency limit of 10 and a file size limit of 2GB
//! - POST requests to the /jobs endpoint that use the media_url property do not have a concurrency limit or file restriction. They are only limited by the first two bullet points
//!
//! # Error Handling
//!
//! The API indicates failure with 4xx and 5xx HTTP status codes. 4xx status codes indicate an error due to the request provided (e.g., a required parameter was omitted). 5xx error indicate an error with Rev.ai's servers.
//!
//! When an 4xx error occurs during invocation of a request, the API responds with a [problem details](https://tools.ietf.org/html/rfc7807) as HTTP response payload.
//!
//! The problem details information is represented as a JSON object with the following optional properties:
//!
//! | Property   | Description                                   |
//! |------------|-----------------------------------------------|
//! | type       | a URI representing the type for the error     |
//! | title      | a short human readable description of type    |
//! | details    | additional details of the error               |
//! | status     | HTTP status code of the error                 |
//!
//! In addition to the properties listed above, the problem details object may list additional properties that help to troubleshoot the problem.
//!
//! ## Example Errors
//!
//! ```
//! // Bad Submit Job Request
//! {
//!   "parameter": {
//!     "media_url": [
//!       "The media_url field is required"
//!     ]
//!   },
//!   "type": "https://www.rev.ai/api/v1/errors/invalid-parameters",
//!   "title": "Your request parameters didn't validate",
//!   "status": 400
//! }
//!
//! // Invalid Transcript State
//! {
//!   "allowed_values": [
//!     "transcribed"
//!   ],
//!   "current_value": "in_progress",
//!   "type": "https://rev.ai/api/v1/errors/invalid-job-state",
//!   "title": "Job is in invalid state",
//!   "detail": "Job is in invalid state to obtain the transcript",
//!   "status": 409
//! }
//! ```
//!
//! ## Retrying Failed Requests
//!
//! Some errors can be resolved simply by retrying the request. The following error codes are likely to be resolved with successive retries.
//!
//! | Status Code | Error |
//! |---|:---|
//! | 429 | Too Many Requests |
//! | 502 | Bad Gateway |
//! | 503 | Service Unavailable |
//! | 504 | Gateway Timeout |
//!
//! Note: With the exception of the 429 status code, it is recommended that the maximum number of retries be limited to 5 attempts per request. The number of retries can be higher
//! for 429 errors but if you notice consistent throttling please contact us at <support@rev.ai>.
//!
//!
//!
//!
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [Rev.ai OpenAPI
//! specs](https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/rev.ai/v1/openapi.yaml) based on API spec version `v1`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! revai = "0.2.3"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use revai::Client;
//!
//! let rev.ai = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `REV.AI_API_KEY`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use revai::Client;
//!
//! let rev.ai = Client::new_from_env();
//! ```
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod account;
pub mod captions;
pub mod jobs;
#[cfg(test)]
mod tests;
pub mod traits;
pub mod transcript;
pub mod types;
#[doc(hidden)]
pub mod utils;

use anyhow::{anyhow, Error, Result};

pub const DEFAULT_HOST: &str = "https://api.rev.ai/speechtotext/v1";

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

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    token: String,

    client: reqwest::Client,
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
        match client {
            Ok(c) => Client {
                host: DEFAULT_HOST.to_string(),
                token: token.to_string(),

                client: c,
            },
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Override the default host for the client.
    pub fn with_host<H>(&self, host: H) -> Self
    where
        H: ToString,
    {
        let mut c = self.clone();
        c.host = host.to_string();
        c
    }

    /// Create a new Client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key and your requests will work.
    /// We pass in the token and refresh token to the client so if you are storing
    /// it in a database, you can get it first.
    pub fn new_from_env() -> Self {
        let token = env::var("REVAI_API_KEY").expect("must set REVAI_API_KEY");

        Client::new(token)
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
            (self.host.clone() + uri).to_string()
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
            log::debug!(
                "body: {:?}",
                String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap()
            );
            req = req.body(body);
        }
        log::debug!("request: {:?}", &req);
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
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
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
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );

            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map(|out| (link, out)).map_err(Error::from)
        } else {
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
    async fn post_form<Out>(&self, uri: &str, form: reqwest::multipart::Form) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(http::Method::POST, url);

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

        log::debug!("form: {:?}", form);
        req = req.multipart(form);

        log::debug!("request: {:?}", &req);
        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
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
            (self.host.clone() + uri).to_string()
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

        log::debug!("request: {:?}", &req);
        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
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
            (self.host.clone() + uri).to_string()
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

        log::debug!("request: {:?}", &req);
        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
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
        self.request_entity(http::Method::GET, &(self.host.to_string() + uri), message)
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
        self.request_with_links(http::Method::GET, &(self.host.to_string() + uri), None)
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
        self.request_entity(http::Method::POST, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PATCH, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PUT, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(self.host.to_string() + uri),
            message,
        )
        .await
    }

    pub fn account(&self) -> account::Account {
        account::Account::new(self.clone())
    }

    pub fn captions(&self) -> captions::Captions {
        captions::Captions::new(self.clone())
    }

    pub fn jobs(&self) -> jobs::Jobs {
        jobs::Jobs::new(self.clone())
    }

    pub fn transcript(&self) -> transcript::Transcript {
        transcript::Transcript::new(self.clone())
    }
}
