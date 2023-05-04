//const X_GITHUB_REQUEST_ID: &str = "x-github-request-id";
//const X_RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
const X_RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";
const X_RATELIMIT_RESET: &str = "x-ratelimit-reset";

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
#[derive(Clone, Copy, Default)]
pub enum MediaType {
    /// Return json (the default)
    #[default]
    Json,
    /// Return json in preview form
    Preview(&'static str),
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MediaType::Json => write!(f, "application/vnd.github.v3+json"),
            MediaType::Preview(codename) => {
                write!(f, "application/vnd.github.{}-preview+json", codename)
            }
        }
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

#[cfg(test)]
mod github_tests {
    use super::MediaType;

    #[test]
    fn test_hyperx_qitem_compat() {
        let tests = [
            (MediaType::Json, "application/vnd.github.v3+json"),
            (
                MediaType::Preview("test-value"),
                "application/vnd.github.test-value-preview+json",
            ),
        ];

        for (media_type, header) in tests {
            assert_eq!(header, media_type.to_string(),)
        }
    }
}

use std::{fmt, str::FromStr};

use parse_link_header::LinkMap;
use serde::de::{self, Visitor};

pub struct NextLink(pub String);

pub fn next_link(l: &LinkMap) -> Option<NextLink> {
    l.get(&Some("next".to_string()))
        .map(|link| link.raw_uri.to_string())
        .map(NextLink)
}

pub mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            if s.is_empty() {
                Ok(None)
            } else {
                // This is standard.
                match serde_json::from_str::<NaiveDate>(&format!("\"{}\"", s)) {
                    Ok(t) => Ok(Some(t)),
                    Err(e) => Err(serde::de::Error::custom(format!(
                        "deserializing {} as NaiveDate failed: {}",
                        s, e
                    ))),
                }
            }
        } else {
            Ok(None)
        }
    }
}

pub mod date_time_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    // The date format Ramp returns looks like this: "2021-04-24T01:03:21"
    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%:z";

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(mut s) = s {
            // This is standard.
            match serde_json::from_str::<DateTime<Utc>>(&format!("\"{}\"", s)) {
                Ok(t) => Ok(Some(t)),
                Err(_) => {
                    // This is google calendar.
                    match Utc.datetime_from_str(&s, "%Y-%m-%dT%H:%M:%S%.3fZ") {
                        Ok(t) => Ok(Some(t)),
                        Err(_) => match Utc.datetime_from_str(&s, FORMAT) {
                            Ok(t) => Ok(Some(t)),
                            Err(_) => match Utc.datetime_from_str(&s, "%+") {
                                Ok(t) => Ok(Some(t)),
                                Err(_) => match chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                                    Ok(d) => Ok(Some(DateTime::<Utc>::from_utc(
                                        chrono::NaiveDateTime::new(
                                            d,
                                            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                                        ),
                                        Utc,
                                    ))),
                                    Err(_) => {
                                        s = format!("{}+00:00", s);
                                        match Utc.datetime_from_str(&s, FORMAT) {
                                            Ok(r) => Ok(Some(r)),
                                            Err(_) => match Utc.datetime_from_str(&s, "%+") {
                                                Ok(d) => Ok(Some(d)),
                                                Err(e) => Err(serde::de::Error::custom(format!(
                                                    "deserializing {} as DateTime<Utc> failed: {}",
                                                    s, e
                                                ))),
                                            },
                                        }
                                    }
                                },
                            },
                        },
                    }
                }
            }
        } else {
            Ok(None)
        }
    }
}

pub mod deserialize_empty_url {
    use serde::{self, Deserialize, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<url::Url>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Ok(s) = String::deserialize(deserializer) {
            if s.is_empty() {
                return Ok(None);
            }

            match url::Url::parse(&s) {
                Ok(u) => return Ok(Some(u)),
                Err(e) => {
                    return Err(serde::de::Error::custom(format!(
                        "error url parsing {}: {}",
                        s, e
                    )))
                }
            }
        }

        Ok(None)
    }
}

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

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as i32)
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as i32)
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

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as i32)
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

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as i64)
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as i64)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as i64)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as i64)
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

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f32)
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f32)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f32)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::i32;
        if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
            Ok(value as f32)
        } else {
            Err(E::custom(format!("f32 out of range: {}", value)))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f32)
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

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f64)
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f64)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f64)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f64)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f64)
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as f64)
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

pub mod google_calendar_date_time_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Serializer};

    // Google Calendar doesn't accept actual normal date formats they want this.
    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if date.is_some() {
            let s = format!("{}", date.unwrap().format(FORMAT));
            return serializer.serialize_str(&s);
        }

        serializer.serialize_none()
    }
}

struct VectorVisitor<T>(std::marker::PhantomData<fn() -> Vec<T>>);

impl<'de, T> Visitor<'de> for VectorVisitor<T>
where
    T: de::Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special vector")
    }

    fn visit_seq<A: de::SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut things: Vec<T> = Default::default();

        // While there are entries remaining in the input, add them
        // into our vector.
        while let Some(t) = access.next_element::<T>()? {
            things.push(t);
        }

        Ok(things)
    }
}

pub mod deserialize_null_vector {
    use serde::{self, Deserialize, Deserializer};

    use super::VectorVisitor;

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        if let Ok(r) = deserializer.deserialize_seq(VectorVisitor::<T>(std::marker::PhantomData)) {
            return Ok(r);
        }

        Ok(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::next_link;

    #[test]
    fn test_hyperx_next_link_compat() {
        let value = "<https://previous-link>; rel=\"prev\", <https://next-link>; rel=\"next\", <https://last-link>; rel=\"last\", <https://first-link>; rel=\"first\"";

        let link = parse_link_header::parse(value).unwrap();
        let next = next_link(&link).unwrap().0;

        assert_eq!("https://next-link", next);
    }
}
