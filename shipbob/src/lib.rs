//! A fully generated, opinionated API client library for ShipBob.
//!
//! [![docs.rs](https://docs.rs/shipbob/badge.svg)](https://docs.rs/shipbob)
//!
//! ## API Details
//!
//! ShipBob Developer API Documentation
//!
//! # Authentication
//!
//! <!-- ReDoc-Inject: <security-definitions> -->
//!
//!
//!
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [ShipBob OpenAPI
//! specs](https://developer.shipbob.com/c196c993-6cf8-4901-84aa-b425f3448df3) based on API spec version `1.0`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! shipbob = "0.3.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use shipbob::Client;
//!
//! let shipbob = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `SHIPBOB_API_KEY`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use shipbob::Client;
//!
//! let shipbob = Client::new_from_env();
//! ```
//!
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Use the Channel Resource to list “channels” which you have access to. You will use this channelId for subsequent API calls made to ShipBob endpoints.
///
/// A channel is a specific installation of an application built by a vendor on top of our API – e.g. Kevin’s Shopify Store #133432.  All write and most read endpoints require a channel to be passed in the header to complete the request. The channel is used to Identify where the data originally came from.  
///
/// Applications that are granted multi-channel permissions will be able to read data from all channels that belong to a user. However, multi-channel applications will only be able to write on behalf of their own channel.
pub mod channels;
/// Use the Inventory Resource to retrieve ShipBob  inventory items and quantities.
///
/// An inventory item is a representation of a physical good, that may or may not have physical stock in ShipBob’s fulfillment centers. Every product will have one or more inventory items mapped to it. A bundle product (a set of products that are sold as one combined package - e.g.  gift or multi packs) is an example of a product that has  2 or more inventory items mapped to it.
///
/// Lot items are physical items that have expiration dates or batch numbers that should be fulfilled in a FIFO (first in, first out) manner. Most food items are lot items. Quantities by lot # and/or expiration date are also listed in the Inventory object.
pub mod inventory;
/// Use this API to interact with the physical locations across ShipBob's fulfillment network.
///
/// An active ShipBob location is operational for fulfillment processes, including receiving inventory and processing returns. It's important to note that some locations, access is granted to all merchants by default, while some locations require special request for merchants to be granted access.
///
/// For each location, determine if it can be leveraged for the user by viewing the access_granted & receiving_enabled fields.
pub mod locations;
/// >  Note: The orderId in the API response will not match the Id displayed in the ShipBob Merchant Portal when you navigate to the Orders page. ShipBob is currently undergoing a schema migration and the Id displayed in the ShipBob Merchant Portal is the shipmentId not the orderId. In the future, the portal will display both orderId(s) and shipmentId(s).
///
/// Use the Orders Resource to create and retrieve orders in ShipBob.
///
/// An order a digital record of a complete purchase that comes from an upstream source (i.e. Shopify) and is intended for ShipBob to fulfill. The order object includes products purchased, shipping address details, shipping method selected etc. Orders are created in ShipBob via a channel.
///
/// When ShipBob fulfills the order, one or more shipments are created for that order. A shipment is a record of the physical package(s) sent out via a carrier. If an order is shipped in multiple packages then 2 or more shipments can be created for that order.
///
/// ### Tips for creating orders in ShipBob via the POST Order endpoint:
///
/// * Populate the referenceId with a unique and immutable order identifier from your upstream system. This field was created to allow you to tieback records in ShipBob with your upstream system.
///
/// * Ensure that the <em> shipping method </em> passed in the API request matches exactly what the user has listed as the <em> shipping method </em> on the <em>  Ship Option Mapping </em> setup page in the ShipBob Merchant Portal. If they don’t match, ShipBob will assume that the user wants to leverage ShipBob’s default shipping method.
///
/// * You can leverage either productId (the ShipBob productId) or the product referenceId (your system's  unique Identifier for products) when creating an order.
///
/// Use the Shipments endpoints to retrieve fulfillment information for shipments or orders.
///
/// A shipment is an object that is the result of a fulfillment of an order. An order can have one or more shipments. Say Shopify order #122323 contains 3 different products, shipped in two separate packages, there would be 2 shipments for that order.
///
///  Serial numbers are unique identifiers for an individual item (e.g. your specific iPhone X that you bought at the Apple Store). No inventory item can possess duplicate serial numbers. Merchants can request “serial scan”, which means ShipBob will capture the serial number(s) upon sending a shipment so the merchant knows which customer received which individual item(s).
pub mod orders;
/// Use the Products Resource to retrieve and create product records in ShipBob.
///
/// A product is a virtual record created in ShipBob’s system via a channel. Say a merchant has two Shopify stores (each store would have its own channel), Kevin’s Shopify Store #133 and Kevin’s Shopify store #134. If the same SKU was sold on both stores, two products would be created for that SKU, one product would be created to represent the SKU sold on Store #133 and one to represent it on Store on #134, with productIds 3884009 and 3884008 respectively.
///
/// While a product is a virtual record, the inventory item is a representation of a physical good. So in the above example, as product 3884009 and product 3884008 represent the same SKU sold on different channels, the same inventory item will be mapped to both products. Every product will have one or more inventory items mapped to it. Bundle products, a set of products that are sold to consumers as one combined package, think gift or multi packs, may have 2 or more inventory items mapped to them.
///
/// ### Tips for creating products in ShipBob via the POST Product endpoints:
///
/// * ShipBob needs products to be created at the lowest level. So if a product has 3 variants, small, medium and large, a separate product needs to be created in ShipBob for all three.
///
/// * Populate the referenceId with a unique and immutable product identifier from your upstream system. This field was created to allow you to tie back records in ShipBob with your upstream system.
///
/// * Use specific and/or unique names to describe each product so they can be easily identified by users in the ShipBob Merchant Portal. In particular, when creating variants, please give them distinguishable names i.e. for a Blue shirt that comes in two sizes, small and medium, strong product names would be Blue shirt size:small and Blue shirt size:medium.
///
/// > **NOTE:** The productId returned in the API response will not match the id displayed in the ShipBob Merchant Portal when you navigate to Inventory > Products. ShipBob is currently undergoing a schema migration and the Id displayed in the ShipBob Merchant Portal is the inventoryId not the productId. In the future, the portal will display both productId(s) and inventoryId(s).
pub mod products;
/// Use the Receiving Resource to retrieve, create and cancel Warehouse Receiving Orders (WROs).
///
/// A WRO is a request form that tells ShipBob's fulfillment centers what inventory should be received and stocked. Some other solutions call this an “ASN” or Advanced Ship Notice. WROs may include multiple inventory items with specific quantities. More details on creating a WRO can be found [here](https://support.shipbob.com/s/article/New-Send-Inventory-to-ShipBob-WRO).
///
/// A WRO can only be **canceled** if it is in the Awaiting status. WROs in Awaiting status are considered to still be in transit to ShipBob FCs. WROs that have Partially Arrived, have been Processed or are Completed, cannot be canceled.
pub mod receiving;
/// **While the Returns API is live, ShipBob's end to end Returns process will not go live until the beginning of March. As a result, any returns arriving at ShipBob's fulfillment centers prior to March 12st, 2020 will NOT be processed**.
///
/// Use the Returns resource to retrieve, create, edit and cancel return records in ShipBob.
///
/// A return is a request for ShipBob to perform an action on inventory that is coming back into our fulfillment centers. Typically, the return is a result of an order being requested to be refunded or exchanged. ShipBob does not handle refunds or exchanges - we simply process the inventory according to the merchant specifications.
///
/// Returns can only be **modified** or **cancelled** when they are in the Awaiting Arrival status. Returns that are being Processed or have been Completed cannot be modified or cancelled.
///
/// ### Tips for creating return orders:
///
/// * Populate the referenceId with a unique and immutable return identifier from your upstream system. This field was created to allow you to tie back records in ShipBob with your upstream system.
///
/// *Include each inventoryId exactly once in the inventory object. If an inventoryId is included more than once, the call will return an error code
///
/// * Provide a tracking # when submitting a return, while it is not a required field, it is the the most surefire way for ShipBob staff to properly and quickly identify the return package when it reaches our fulfillment center.
///
/// * Only include inventory items to the return record that will be returned in the same box i.e. if InventoryId 12232 and InventoryId 12039 will be returned in two seperate boxes, two return orders should be created.
///
/// * ShipBob does not process returns for digital items or bundle inventory items. Return calls that include digital inventory  items (e.g. ebooks) or bundle inventory items (i.e. multipacks, combination of multiple inventory items)  will return an error code.
///
/// * If you choose to provide a requested action (it is an optional field), only provide one requested action per inventory item. So if  you have more than 1 quantity of a given item being returned within the same box, all quantities of the item have to have the same action associated with them. If you don’t provide a requested action, it will default to the action the User set for that inventory item in the ShipBob Merchant portal.
pub mod returns;
pub mod types;
#[doc(hidden)]
pub mod utils;
/// Use the Webhooks Resource to create, view or delete subscriptions for a user.
pub mod webhooks;

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

pub const FALLBACK_HOST: &str = "https://api.shipbob.com/1.0";

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
        "https://api.shipbob.com/1.0/"
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
        let token = env::var("SHIPBOB_API_KEY").expect("must set SHIPBOB_API_KEY");

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

    ///  >  Note: The orderId in the API response will not match the Id displayed in the ShipBob Merchant Portal when you navigate to the Orders page. ShipBob is currently undergoing a schema migration and the Id displayed in the ShipBob Merchant Portal is the shipmentId not the orderId. In the future, the portal will display both orderId(s) and shipmentId(s).
    ///
    /// Use the Orders Resource to create and retrieve orders in ShipBob.
    ///
    /// An order a digital record of a complete purchase that comes from an upstream source (i.e. Shopify) and is intended for ShipBob to fulfill. The order object includes products purchased, shipping address details, shipping method selected etc. Orders are created in ShipBob via a channel.
    ///
    /// When ShipBob fulfills the order, one or more shipments are created for that order. A shipment is a record of the physical package(s) sent out via a carrier. If an order is shipped in multiple packages then 2 or more shipments can be created for that order.
    ///
    /// ### Tips for creating orders in ShipBob via the POST Order endpoint:
    ///
    /// * Populate the referenceId with a unique and immutable order identifier from your upstream system. This field was created to allow you to tieback records in ShipBob with your upstream system.
    ///
    /// * Ensure that the <em> shipping method </em> passed in the API request matches exactly what the user has listed as the <em> shipping method </em> on the <em>  Ship Option Mapping </em> setup page in the ShipBob Merchant Portal. If they don’t match, ShipBob will assume that the user wants to leverage ShipBob’s default shipping method.
    ///
    /// * You can leverage either productId (the ShipBob productId) or the product referenceId (your system's  unique Identifier for products) when creating an order.
    ///
    /// Use the Shipments endpoints to retrieve fulfillment information for shipments or orders.
    ///
    /// A shipment is an object that is the result of a fulfillment of an order. An order can have one or more shipments. Say Shopify order #122323 contains 3 different products, shipped in two separate packages, there would be 2 shipments for that order.
    ///
    ///  Serial numbers are unique identifiers for an individual item (e.g. your specific iPhone X that you bought at the Apple Store). No inventory item can possess duplicate serial numbers. Merchants can request “serial scan”, which means ShipBob will capture the serial number(s) upon sending a shipment so the merchant knows which customer received which individual item(s).
    pub fn orders(&self) -> orders::Orders {
        orders::Orders::new(self.clone())
    }

    /// Use the Products Resource to retrieve and create product records in ShipBob.
    ///
    /// A product is a virtual record created in ShipBob’s system via a channel. Say a merchant has two Shopify stores (each store would have its own channel), Kevin’s Shopify Store #133 and Kevin’s Shopify store #134. If the same SKU was sold on both stores, two products would be created for that SKU, one product would be created to represent the SKU sold on Store #133 and one to represent it on Store on #134, with productIds 3884009 and 3884008 respectively.
    ///
    /// While a product is a virtual record, the inventory item is a representation of a physical good. So in the above example, as product 3884009 and product 3884008 represent the same SKU sold on different channels, the same inventory item will be mapped to both products. Every product will have one or more inventory items mapped to it. Bundle products, a set of products that are sold to consumers as one combined package, think gift or multi packs, may have 2 or more inventory items mapped to them.
    ///
    /// ### Tips for creating products in ShipBob via the POST Product endpoints:
    ///
    /// * ShipBob needs products to be created at the lowest level. So if a product has 3 variants, small, medium and large, a separate product needs to be created in ShipBob for all three.
    ///
    /// * Populate the referenceId with a unique and immutable product identifier from your upstream system. This field was created to allow you to tie back records in ShipBob with your upstream system.
    ///
    /// * Use specific and/or unique names to describe each product so they can be easily identified by users in the ShipBob Merchant Portal. In particular, when creating variants, please give them distinguishable names i.e. for a Blue shirt that comes in two sizes, small and medium, strong product names would be Blue shirt size:small and Blue shirt size:medium.
    ///
    /// > **NOTE:** The productId returned in the API response will not match the id displayed in the ShipBob Merchant Portal when you navigate to Inventory > Products. ShipBob is currently undergoing a schema migration and the Id displayed in the ShipBob Merchant Portal is the inventoryId not the productId. In the future, the portal will display both productId(s) and inventoryId(s).
    pub fn products(&self) -> products::Products {
        products::Products::new(self.clone())
    }

    ///  Use the Inventory Resource to retrieve ShipBob  inventory items and quantities.
    ///
    /// An inventory item is a representation of a physical good, that may or may not have physical stock in ShipBob’s fulfillment centers. Every product will have one or more inventory items mapped to it. A bundle product (a set of products that are sold as one combined package - e.g.  gift or multi packs) is an example of a product that has  2 or more inventory items mapped to it.
    ///
    /// Lot items are physical items that have expiration dates or batch numbers that should be fulfilled in a FIFO (first in, first out) manner. Most food items are lot items. Quantities by lot # and/or expiration date are also listed in the Inventory object.
    pub fn inventory(&self) -> inventory::Inventory {
        inventory::Inventory::new(self.clone())
    }

    /// Use the Channel Resource to list “channels” which you have access to. You will use this channelId for subsequent API calls made to ShipBob endpoints.
    ///
    /// A channel is a specific installation of an application built by a vendor on top of our API – e.g. Kevin’s Shopify Store #133432.  All write and most read endpoints require a channel to be passed in the header to complete the request. The channel is used to Identify where the data originally came from.  
    ///
    /// Applications that are granted multi-channel permissions will be able to read data from all channels that belong to a user. However, multi-channel applications will only be able to write on behalf of their own channel.
    pub fn channels(&self) -> channels::Channels {
        channels::Channels::new(self.clone())
    }

    /// **While the Returns API is live, ShipBob's end to end Returns process will not go live until the beginning of March. As a result, any returns arriving at ShipBob's fulfillment centers prior to March 12st, 2020 will NOT be processed**.
    ///
    /// Use the Returns resource to retrieve, create, edit and cancel return records in ShipBob.
    ///
    /// A return is a request for ShipBob to perform an action on inventory that is coming back into our fulfillment centers. Typically, the return is a result of an order being requested to be refunded or exchanged. ShipBob does not handle refunds or exchanges - we simply process the inventory according to the merchant specifications.
    ///
    /// Returns can only be **modified** or **cancelled** when they are in the Awaiting Arrival status. Returns that are being Processed or have been Completed cannot be modified or cancelled.
    ///
    /// ### Tips for creating return orders:
    ///
    /// * Populate the referenceId with a unique and immutable return identifier from your upstream system. This field was created to allow you to tie back records in ShipBob with your upstream system.
    ///
    /// *Include each inventoryId exactly once in the inventory object. If an inventoryId is included more than once, the call will return an error code
    ///
    /// * Provide a tracking # when submitting a return, while it is not a required field, it is the the most surefire way for ShipBob staff to properly and quickly identify the return package when it reaches our fulfillment center.
    ///
    /// * Only include inventory items to the return record that will be returned in the same box i.e. if InventoryId 12232 and InventoryId 12039 will be returned in two seperate boxes, two return orders should be created.
    ///
    /// * ShipBob does not process returns for digital items or bundle inventory items. Return calls that include digital inventory  items (e.g. ebooks) or bundle inventory items (i.e. multipacks, combination of multiple inventory items)  will return an error code.
    ///
    /// * If you choose to provide a requested action (it is an optional field), only provide one requested action per inventory item. So if  you have more than 1 quantity of a given item being returned within the same box, all quantities of the item have to have the same action associated with them. If you don’t provide a requested action, it will default to the action the User set for that inventory item in the ShipBob Merchant portal.
    pub fn returns(&self) -> returns::Returns {
        returns::Returns::new(self.clone())
    }

    /// Use the Receiving Resource to retrieve, create and cancel Warehouse Receiving Orders (WROs).
    ///
    /// A WRO is a request form that tells ShipBob's fulfillment centers what inventory should be received and stocked. Some other solutions call this an “ASN” or Advanced Ship Notice. WROs may include multiple inventory items with specific quantities. More details on creating a WRO can be found [here](https://support.shipbob.com/s/article/New-Send-Inventory-to-ShipBob-WRO).
    ///
    /// A WRO can only be **canceled** if it is in the Awaiting status. WROs in Awaiting status are considered to still be in transit to ShipBob FCs. WROs that have Partially Arrived, have been Processed or are Completed, cannot be canceled.
    pub fn receiving(&self) -> receiving::Receiving {
        receiving::Receiving::new(self.clone())
    }

    /// Use the Webhooks Resource to create, view or delete subscriptions for a user.
    pub fn webhooks(&self) -> webhooks::Webhooks {
        webhooks::Webhooks::new(self.clone())
    }

    /// Use this API to interact with the physical locations across ShipBob's fulfillment network.
    ///
    /// An active ShipBob location is operational for fulfillment processes, including receiving inventory and processing returns. It's important to note that some locations, access is granted to all merchants by default, while some locations require special request for merchants to be granted access.
    ///
    /// For each location, determine if it can be leveraged for the user by viewing the access_granted & receiving_enabled fields.
    pub fn locations(&self) -> locations::Locations {
        locations::Locations::new(self.clone())
    }
}
