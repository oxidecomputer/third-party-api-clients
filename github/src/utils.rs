use std::{fmt, str::FromStr};

use serde::de::{self, Visitor};

//const X_GITHUB_REQUEST_ID: &str = "x-github-request-id";
//const X_RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
const X_RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";
const X_RATELIMIT_RESET: &str = "x-ratelimit-reset";

pub fn next_link(l: &hyperx::header::Link) -> Option<String> {
    l.values().iter().find_map(|value| {
        value.rel().and_then(|rels| {
            if rels
                .iter()
                .any(|rel| rel == &hyperx::header::RelationType::Next)
            {
                Some(value.link().into())
            } else {
                None
            }
        })
    })
}

#[cfg(not(feature = "httpcache"))]
type HeaderValues = (Option<u32>, Option<u32>);
#[cfg(feature = "httpcache")]
type HeaderValues = (Option<u32>, Option<u32>, Option<Vec<u8>>);

pub fn get_header_values(
    headers: &http::header::HeaderMap<http::header::HeaderValue>,
) -> HeaderValues {
    /*if let Some(value) = headers.get(X_GITHUB_REQUEST_ID) {
        println!("x-github-request-id: {:?}", value)
    }
    if let Some(value) = headers.get(X_RATELIMIT_LIMIT) {
        println!("x-rate-limit-limit: {:?}", value)
    }*/
    let remaining = headers
        .get(X_RATELIMIT_REMAINING)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u32>().ok());
    let reset = headers
        .get(X_RATELIMIT_RESET)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u32>().ok());
    /*if let Some(value) = remaining {
        println!("x-rate-limit-remaining: {}", value)
    }
    if let Some(value) = reset {
        println!("x-rate-limit-reset: {}", value)
    }*/
    #[cfg(feature = "httpcache")]
    let etag = headers.get(http::header::ETAG);
    /*if let Some(value) = etag {
        println!("etag: {:?}", value)
    }*/

    #[cfg(feature = "httpcache")]
    {
        let etag = etag.map(|etag| etag.as_bytes().to_vec());
        (remaining, reset, etag)
    }
    #[cfg(not(feature = "httpcache"))]
    (remaining, reset)
}

/// GitHub defined Media types
/// See [this doc](https://developer.github.com/v3/media/) for more for more information
#[derive(Clone, Copy)]
pub enum MediaType {
    /// Return json (the default)
    Json,
    /// Return json in preview form
    Preview(&'static str),
}

impl Default for MediaType {
    fn default() -> MediaType {
        MediaType::Json
    }
}

impl From<MediaType> for mime::Mime {
    fn from(media: MediaType) -> mime::Mime {
        match media {
            MediaType::Json => "application/vnd.github.v3+json".parse().unwrap(),
            MediaType::Preview(codename) => {
                format!("application/vnd.github.{}-preview+json", codename)
                    .parse()
                    .unwrap_or_else(|_| {
                        panic!("could not parse media type for preview {}", codename)
                    })
            }
        }
    }
}

// TODO: we should add a function for deserializing a null vector.
pub mod deserialize_null_string {
    use serde::{self, Deserialize, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).unwrap_or_default();

        Ok(s)
    }
}

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match FromStr::from_str(value) {
            Ok(s) => Ok(s),
            Err(_) => Err(de::Error::invalid_value(
                de::Unexpected::Str(value),
                &"bool",
            )),
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match FromStr::from_str(&value) {
            Ok(s) => Ok(s),
            Err(_) => Err(de::Error::invalid_value(
                de::Unexpected::Str(&value),
                &"bool",
            )),
        }
    }
}

pub mod deserialize_null_boolean {
    use serde::{self, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = deserializer
            .deserialize_bool(crate::utils::BoolVisitor)
            .unwrap_or_default();

        Ok(s)
    }
}

struct I32Visitor;

impl<'de> Visitor<'de> for I32Visitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::i32;
        if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
            Ok(value as i32)
        } else {
            Err(E::custom(format!("i32 out of range: {}", value)))
        }
    }
}

pub mod deserialize_null_i32 {
    use serde::{self, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = deserializer
            .deserialize_i32(crate::utils::I32Visitor)
            .unwrap_or_default();

        Ok(s)
    }
}

struct I64Visitor;

impl<'de> Visitor<'de> for I64Visitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }
}

pub mod deserialize_null_i64 {
    use serde::{self, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = deserializer
            .deserialize_i64(crate::utils::I64Visitor)
            .unwrap_or_default();

        Ok(s)
    }
}

struct F32Visitor;

impl<'de> Visitor<'de> for F32Visitor {
    type Value = f32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a float between -2^31 and 2^31")
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::f32;
        if value >= f64::from(f32::MIN) && value <= f64::from(f32::MAX) {
            Ok(value as f32)
        } else {
            Err(E::custom(format!("f32 out of range: {}", value)))
        }
    }
}

pub mod deserialize_null_f32 {
    use serde::{self, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = deserializer
            .deserialize_f32(crate::utils::F32Visitor)
            .unwrap_or_default();

        Ok(s)
    }
}

struct F64Visitor;

impl<'de> Visitor<'de> for F64Visitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a float")
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }
}

pub mod deserialize_null_f64 {
    use serde::{self, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = deserializer
            .deserialize_f64(crate::utils::F64Visitor)
            .unwrap_or_default();

        Ok(s)
    }
}

pub fn zero_i32(num: &i32) -> bool {
    *num == 0
}

pub fn zero_i64(num: &i64) -> bool {
    *num == 0
}

pub fn zero_f32(num: &f32) -> bool {
    *num == 0.0
}

pub fn zero_f64(num: &f64) -> bool {
    *num == 0.0
}
