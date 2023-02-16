//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Rev.ai Account Model
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Account {
    /**
     * Rev.ai Account Model
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub balance_seconds: i64,
    /**
     * Rev.ai Account Model
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DescriptionlessJobOptions {
    /**
     * Email of developer account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DescriptionlessJobOptionsData {
    /**
     * Email of developer account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub callback_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Language {
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "cmn")]
    Cmn,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tr")]
    Tr,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Ar => "ar",
            Language::Bg => "bg",
            Language::Ca => "ca",
            Language::Cmn => "cmn",
            Language::Cs => "cs",
            Language::Da => "da",
            Language::De => "de",
            Language::El => "el",
            Language::En => "en",
            Language::Es => "es",
            Language::Fi => "fi",
            Language::Fr => "fr",
            Language::Hi => "hi",
            Language::Hr => "hr",
            Language::Hu => "hu",
            Language::It => "it",
            Language::Ja => "ja",
            Language::Ko => "ko",
            Language::Lt => "lt",
            Language::Lv => "lv",
            Language::Ms => "ms",
            Language::Nl => "nl",
            Language::No => "no",
            Language::Pl => "pl",
            Language::Pt => "pt",
            Language::Ro => "ro",
            Language::Ru => "ru",
            Language::Sk => "sk",
            Language::Sl => "sl",
            Language::Sv => "sv",
            Language::Tr => "tr",
            Language::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::En
    }
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DescriptionlessJobOptionsDataType {
    /**
     * Email of developer account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub custom_vocabulary_id: String,
    /**
     * Amount of Rev.ai API credits remaining in seconds
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub delete_after_seconds: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub filter_profanity: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /**
     * Email of developer account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub media_url: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub remove_disfluencies: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub skip_diarization: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub skip_punctuation: bool,
    /**
     * Amount of Rev.ai API credits remaining in seconds
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub speaker_channels_count: i64,
}

/// All of the following types are flattened into one object:
///
/// - `DescriptionlessJobOptions`
/// - `DescriptionlessJobOptionsData`
/// - `DescriptionlessJobOptionsDataType`
///
#[derive(Serialize, Deserialize, Default, PartialEq, Debug, Clone, JsonSchema)]
pub struct DescriptionlessJobOptionsAllOf {
    #[serde(flatten)]
    pub descriptionless_job_options: DescriptionlessJobOptions,
    #[serde(flatten)]
    pub descriptionless_job_options_data: DescriptionlessJobOptionsData,
    #[serde(flatten)]
    pub descriptionless_job_options_data_type: DescriptionlessJobOptionsDataType,
}

/**
* Simple reason of why the transcription job failed. Check `failure_detail` for specific details and solutions
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Failure {
    #[serde(rename = "download_failure")]
    DownloadFailure,
    #[serde(rename = "duration_exceeded")]
    DurationExceeded,
    #[serde(rename = "duration_too_short")]
    DurationTooShort,
    #[serde(rename = "empty_media")]
    EmptyMedia,
    #[serde(rename = "insufficient_balance")]
    InsufficientBalance,
    #[serde(rename = "internal_processing")]
    InternalProcessing,
    #[serde(rename = "invalid_media")]
    InvalidMedia,
    #[serde(rename = "invoicing_limit_exceeded")]
    InvoicingLimitExceeded,
    #[serde(rename = "transcription")]
    Transcription,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Failure::DownloadFailure => "download_failure",
            Failure::DurationExceeded => "duration_exceeded",
            Failure::DurationTooShort => "duration_too_short",
            Failure::EmptyMedia => "empty_media",
            Failure::InsufficientBalance => "insufficient_balance",
            Failure::InternalProcessing => "internal_processing",
            Failure::InvalidMedia => "invalid_media",
            Failure::InvoicingLimitExceeded => "invoicing_limit_exceeded",
            Failure::Transcription => "transcription",
            Failure::Noop => "",
            Failure::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Failure {
    fn default() -> Failure {
        Failure::Noop
    }
}
impl Failure {
    pub fn is_noop(&self) -> bool {
        matches!(self, Failure::Noop)
    }
}

/**
* Current status of the job
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Status {
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "transcribed")]
    Transcribed,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Failed => "failed",
            Status::InProgress => "in_progress",
            Status::Transcribed => "transcribed",
            Status::Noop => "",
            Status::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Status {
    fn default() -> Status {
        Status::Noop
    }
}
impl Status {
    pub fn is_noop(&self) -> bool {
        matches!(self, Status::Noop)
    }
}

/**
* Type of speech recognition performed. Currently the only supported values are 'async' for asynchronous jobs and `stream` for streaming jobs
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Type {
    #[serde(rename = "async")]
    Async,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Async => "async",
            Type::Noop => "",
            Type::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::Noop
    }
}
impl Type {
    pub fn is_noop(&self) -> bool {
        matches!(self, Type::Noop)
    }
}

/// Rev.ai Transcription Job
/// ***
/// Note: properties are not displayed in the returned object if they are null
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Job {
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<serde_json::Value>,
    /**
     * The date and time the job was completed, whether successfully or failing, in ISO-8601 UTC form
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub completed_on: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The date and time the job was completed, whether successfully or failing, in ISO-8601 UTC form
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_on: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_vocabulary_id: Option<serde_json::Value>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_after_seconds: Option<serde_json::Value>,
    /**
     * Duration of the file in seconds. Null if the file could not be retrieved or there was not a valid media file
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub duration_seconds: f64,
    /**
     * Simple reason of why the transcription job failed. Check `failure_detail` for specific details and solutions
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<Failure>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub failure_detail: String,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_profanity: Option<serde_json::Value>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<serde_json::Value>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_url: Option<serde_json::Value>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_disfluencies: Option<serde_json::Value>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_diarization: Option<serde_json::Value>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_punctuation: Option<serde_json::Value>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaker_channels_count: Option<serde_json::Value>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
}

/// All of the following types are flattened into one object:
///
/// - `DescriptionlessJobOptionsAllOf`
/// - `Job`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct JobAllOf {
    #[serde(flatten)]
    pub descriptionless_job_options_all_of: DescriptionlessJobOptionsAllOf,
    /**
     * Rev.ai Transcription Job
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *
     */
    #[serde(flatten)]
    pub job: Job,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubmitJobMediaUrlOptions {
    /**
     * Email of developer account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub media_url: String,
}

/// All of the following types are flattened into one object:
///
/// - `SubmitJobMediaUrlOptions`
/// - `SubmitJobOptionsAllOf`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubmitJobMediaUrlOptionsAllOf {
    #[serde(flatten)]
    pub submit_job_media_url_options: SubmitJobMediaUrlOptions,
    #[serde(flatten)]
    pub submit_job_options_all_of: SubmitJobOptionsAllOf,
}

/// Rev.ai Job Options Object Model
#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubmitJobOptions {
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<serde_json::Value>,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_vocabulary_id: Option<serde_json::Value>,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_after_seconds: Option<serde_json::Value>,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_profanity: Option<serde_json::Value>,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<serde_json::Value>,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_disfluencies: Option<serde_json::Value>,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_diarization: Option<serde_json::Value>,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_punctuation: Option<serde_json::Value>,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaker_channels_count: Option<serde_json::Value>,
}

/// Contains a collection of phrases. Custom vocabulary informs and biases the speech recognition to find those phrases (at the cost of slightly slower transcription).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CustomVocabularies {
    /**
     * Array of phrases not found in normal dictionary. Add technical jargon, proper nouns and uncommon phrases as strings in this array to add them to the lexicon for this job.
     *  
     *  A phrase must contain at least 1 alpha character but may contain any non-numeric character from the Basic Latin set. A phrase can contain up to 12 words. Each word can contain up to 34 characters.
     *  
     *  \*\*Note\*\*: Only 6000 phrases can be used per transcription job. For more details, check [Custom Vocabularies](https://www.rev.ai/docs/overview#section/Features/Custom-Vocabularies).
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub phrases: Vec<String>,
}

#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubmitJobOptionsData {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub custom_vocabularies: Vec<CustomVocabularies>,
}

/// All of the following types are flattened into one object:
///
/// - `DescriptionlessJobOptionsAllOf`
/// - `SubmitJobOptions`
/// - `SubmitJobOptionsData`
///
#[derive(Serialize, Deserialize, Default, PartialEq, Debug, Clone, JsonSchema)]
pub struct SubmitJobOptionsAllOf {
    #[serde(flatten)]
    pub descriptionless_job_options_all_of: DescriptionlessJobOptionsAllOf,
    /**
     * Rev.ai Job Options Object Model
     */
    #[serde(flatten)]
    pub submit_job_options: SubmitJobOptions,
    #[serde(flatten)]
    pub submit_job_options_data: SubmitJobOptionsData,
}

/**
* Type of transcript element. If Rev.ai was unable to determine the spoken word, the `type` will be `unknown`.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TranscriptMonologuesElementsType {
    #[serde(rename = "punct")]
    Punct,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TranscriptMonologuesElementsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranscriptMonologuesElementsType::Punct => "punct",
            TranscriptMonologuesElementsType::Text => "text",
            TranscriptMonologuesElementsType::Unknown => "unknown",
            TranscriptMonologuesElementsType::Noop => "",
            TranscriptMonologuesElementsType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TranscriptMonologuesElementsType {
    fn default() -> TranscriptMonologuesElementsType {
        TranscriptMonologuesElementsType::Noop
    }
}
impl TranscriptMonologuesElementsType {
    pub fn is_noop(&self) -> bool {
        matches!(self, TranscriptMonologuesElementsType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Elements {
    /**
     * Duration of the file in seconds. Null if the file could not be retrieved or there was not a valid media file
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub confidence: f64,
    /**
     * Duration of the file in seconds. Null if the file could not be retrieved or there was not a valid media file
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub ts: f64,
    /**
     * Duration of the file in seconds. Null if the file could not be retrieved or there was not a valid media file
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub ts_end: f64,
    /**
     * Type of transcript element. If Rev.ai was unable to determine the spoken word, the `type` will be `unknown`.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<TranscriptMonologuesElementsType>,
    /**
     * Email of developer account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Monologues {
    /**
     * Array of transcript elements
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub elements: Vec<Elements>,
    /**
     * Amount of Rev.ai API credits remaining in seconds
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub speaker: i64,
}

/// Rev.ai Transcript Model
/// ***
/// Note: properties are not displayed in the returned object if they are null
///
/// Jobs with skip_diarization set to true will only show a single speaker for the entire duration of the transcript.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Transcript {
    /**
     * Rev.ai Transcript Model
     *  \*\*\*
     *  Note: properties are not displayed in the returned object if they are null
     *  
     *  Jobs with skip_diarization set to true will only show a single speaker for the entire duration of the transcript.
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub monologues: Vec<Monologues>,
}

/**
* MIME type specifying the caption output format
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Accept {
    #[serde(rename = "application/x-subrip")]
    ApplicationXSubrip,
    #[serde(rename = "text/vtt")]
    TextVtt,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Accept {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accept::ApplicationXSubrip => "application/x-subrip",
            Accept::TextVtt => "text/vtt",
            Accept::Noop => "",
            Accept::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Accept {
    fn default() -> Accept {
        Accept::Noop
    }
}
impl Accept {
    pub fn is_noop(&self) -> bool {
        matches!(self, Accept::Noop)
    }
}

/**
* MIME type specifying the transcription output format
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AcceptTranscript {
    #[serde(rename = "application/vnd.rev.transcript.v1.0+json")]
    ApplicationVndRevTranscript0Json,
    #[serde(rename = "text/plain")]
    TextPlain,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AcceptTranscript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcceptTranscript::ApplicationVndRevTranscript0Json => {
                "application/vnd.rev.transcript.v1.0+json"
            }
            AcceptTranscript::TextPlain => "text/plain",
            AcceptTranscript::Noop => "",
            AcceptTranscript::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AcceptTranscript {
    fn default() -> AcceptTranscript {
        AcceptTranscript::Noop
    }
}
impl AcceptTranscript {
    pub fn is_noop(&self) -> bool {
        matches!(self, AcceptTranscript::Noop)
    }
}
