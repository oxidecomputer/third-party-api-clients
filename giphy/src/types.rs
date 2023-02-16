//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// All of the following types are flattened into one object:
///
/// - `Image`
/// - `serde_json::Value`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LoopingAllOf {
    #[serde(flatten)]
    pub image: Image,
    /**
     * Data surrounding a version of this GIF downsized to be under 2mb.
     */
    #[serde(flatten)]
    pub value: serde_json::Value,
}

/// An object containing data for various available formats and sizes of this GIF.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Images {
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downsized: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downsized_large: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downsized_medium: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downsized_small: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downsized_still: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_height: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_height_downsampled: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_height_small: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_height_small_still: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_height_still: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_width: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_width_downsampled: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_width_small: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_width_small_still: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_width_still: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub looping: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_still: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<LoopingAllOf>,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_gif: Option<LoopingAllOf>,
}

/**
* Type of the gif. By default, this is almost always gif
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Type {
    #[serde(rename = "gif")]
    Gif,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Gif => "gif",
            Type::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::Gif
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Gif {
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bitly_url: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_url: String,
    /**
     * The date this GIF was added to the GIPHY database.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub create_datetime: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub embded_url: String,
    /**
     * An array of featured tags for this GIF (Note: Not available when using the Public Beta Key)
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub featured_tags: Vec<String>,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * An object containing data for various available formats and sizes of this GIF.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<Images>,
    /**
     * The date this GIF was added to the GIPHY database.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub import_datetime: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rating: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub source: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub source_post_url: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub source_tld: String,
    /**
     * An array of featured tags for this GIF (Note: Not available when using the Public Beta Key)
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<String>,
    /**
     * The date this GIF was added to the GIPHY database.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub trending_datetime: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Type of the gif. By default, this is almost always gif
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
    /**
     * The date this GIF was added to the GIPHY database.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub update_datetime: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * The User Object contains information about the user associated with a GIF and URLs to assets such as that user's avatar image, profile, and more.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Image {
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub frames: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub height: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "mp4"
    )]
    pub mp_4: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "mp4_size"
    )]
    pub mp_4_size: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub size: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub webp: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub webp_size: String,
    /**
     * The unique bit.ly URL for this GIF
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub width: String,
}

/// The Meta Object contains basic information regarding the request, whether it was successful, and the response given by the API.  Check `responses` to see a description of types of response codes the API might give you under different cirumstances.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Meta {
    /**
     * The Meta Object contains basic information regarding the request, whether it was successful, and the response given by the API.  Check `responses` to see a description of types of response codes the API might give you under different cirumstances.
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub msg: String,
    /**
     * The Meta Object contains basic information regarding the request, whether it was successful, and the response given by the API.  Check `responses` to see a description of types of response codes the API might give you under different cirumstances.
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub response_id: String,
    /**
     * The Meta Object contains basic information regarding the request, whether it was successful, and the response given by the API.  Check `responses` to see a description of types of response codes the API might give you under different cirumstances.
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub status: i64,
}

/// The Pagination Object contains information relating to the number of total results available as well as the number of results fetched and their relative positions.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Pagination {
    /**
     * The Pagination Object contains information relating to the number of total results available as well as the number of results fetched and their relative positions.
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,
    /**
     * The Pagination Object contains information relating to the number of total results available as well as the number of results fetched and their relative positions.
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub offset: i64,
    /**
     * The Pagination Object contains information relating to the number of total results available as well as the number of results fetched and their relative positions.
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_count: i64,
}

/// The User Object contains information about the user associated with a GIF and URLs to assets such as that user's avatar image, profile, and more.
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct User {
    /**
     * The User Object contains information about the user associated with a GIF and URLs to assets such as that user's avatar image, profile, and more.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    /**
     * The User Object contains information about the user associated with a GIF and URLs to assets such as that user's avatar image, profile, and more.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub banner_url: String,
    /**
     * The User Object contains information about the user associated with a GIF and URLs to assets such as that user's avatar image, profile, and more.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * The User Object contains information about the user associated with a GIF and URLs to assets such as that user's avatar image, profile, and more.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub profile_url: String,
    /**
     * The User Object contains information about the user associated with a GIF and URLs to assets such as that user's avatar image, profile, and more.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter: String,
    /**
     * The User Object contains information about the user associated with a GIF and URLs to assets such as that user's avatar image, profile, and more.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetGifsByResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub data: Vec<Gif>,
    /**
     * The Meta Object contains basic information regarding the request, whether it was successful, and the response given by the API.  Check `responses` to see a description of types of response codes the API might give you under different cirumstances.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    /**
     * The Pagination Object contains information relating to the number of total results available as well as the number of results fetched and their relative positions.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RandomGifResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Gif>,
    /**
     * The Meta Object contains basic information regarding the request, whether it was successful, and the response given by the API.  Check `responses` to see a description of types of response codes the API might give you under different cirumstances.
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}
