mod client;
mod functions;
mod template;
mod types;
mod utils;

use std::{
    collections::{BTreeMap, HashSet},
    ffi::OsStr,
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use inflector::cases::{
    pascalcase::to_pascal_case, snakecase::to_snake_case, titlecase::to_title_case,
};
use openapiv3::OpenAPI;
use serde::Deserialize;

fn save<P>(p: P, data: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let p = p.as_ref();
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(p)?;
    f.write_all(data.as_bytes())?;
    f.flush()?;
    Ok(())
}

fn load<P, T>(p: P) -> Result<T>
where
    P: AsRef<Path>,
    for<'de> T: Deserialize<'de>,
{
    let p = p.as_ref();
    let f = File::open(p)?;
    if let Some(ext) = p.extension() {
        if ext == OsStr::new("yaml") || ext == OsStr::new("yml") {
            return Ok(serde_yaml::from_reader(f)?);
        }
    }
    Ok(serde_json::from_reader(f)?)
}

fn load_api<P>(p: P) -> Result<OpenAPI>
where
    P: AsRef<Path>,
{
    let api: OpenAPI = load(p)?;

    if api.openapi != "3.0.3" {
        /*
         * XXX During development we are being very strict, but this should
         * probably be relaxed.
         */
        println!("unexpected version {}", api.openapi);
    }

    if !api.servers.is_empty() {
        println!("servers not presently supported");
    }

    if api.security.is_some() {
        println!("security not presently supported");
    }

    if let Some(components) = api.components.as_ref() {
        if !components.security_schemes.is_empty() {
            println!("component security schemes not supported");
        }

        if !components.responses.is_empty() {
            println!("component responses not supported");
        }

        if !components.parameters.is_empty() {
            println!("component parameters not supported");
        }

        if !components.request_bodies.is_empty() {
            println!("component request bodies not supported");
        }

        if !components.headers.is_empty() {
            println!("component headers not supported");
        }

        if !components.links.is_empty() {
            println!("component links not supported");
        }

        if !components.callbacks.is_empty() {
            println!("component callbacks not supported");
        }

        /*
         * XXX Ignoring "examples" and "extensions" for now.
         */
    }

    // TODO: add external docs etc to the README/Cargo generation.

    /*
     * XXX Ignoring "extensions" for now, as they seem not
     * to immediately affect our code generation.
     */

    let mut opids = HashSet::new();
    for p in api.paths.iter() {
        match p.1 {
            openapiv3::ReferenceOr::Reference { reference: _ } => {
                bail!("path {} uses reference, unsupported", p.0);
            }
            openapiv3::ReferenceOr::Item(item) => {
                /*
                 * Make sure every operation has an operation ID, and that each
                 * operation ID is only used once in the document.
                 */
                let mut id = |o: Option<&openapiv3::Operation>| -> Result<()> {
                    if let Some(o) = o {
                        if let Some(oid) = o.operation_id.as_ref() {
                            if !opids.insert(oid.to_string()) {
                                bail!("duplicate operation ID: {}", oid);
                            }

                            if !o.servers.is_empty() {
                                println!("op {}: servers, unsupported", oid);
                            }

                            if o.security.is_some() {
                                //println!("op {}: security, unsupported", oid);
                            }

                            if o.responses.default.is_some() {
                                println!("op {}: has response default", oid);
                            }
                        }
                    }

                    Ok(())
                };

                id(item.get.as_ref())?;
                id(item.put.as_ref())?;
                id(item.post.as_ref())?;
                id(item.delete.as_ref())?;
                id(item.options.as_ref())?;
                id(item.head.as_ref())?;
                id(item.patch.as_ref())?;
                id(item.trace.as_ref())?;

                if !item.servers.is_empty() {
                    bail!("path {} has servers; unsupported", p.0);
                }
            }
        }
    }

    Ok(api)
}

trait ParameterDataExt {
    fn render_type(&self, name: &str, ts: &mut TypeSpace) -> Result<String>;
}

impl ParameterDataExt for openapiv3::ParameterData {
    fn render_type(&self, name: &str, ts: &mut TypeSpace) -> Result<String> {
        use openapiv3::{SchemaKind, Type};

        // Cleanup the name.
        let mut n = "".to_string();
        if name.is_empty() {
            n = self.name.to_string();
        }

        let sn = clean_name(&n);

        Ok(match &self.format {
            openapiv3::ParameterSchemaOrContent::Schema(s) => {
                match s {
                    openapiv3::ReferenceOr::Reference { reference } => {
                        let tid = ts.select_ref(Some(name), reference.as_str())?;
                        let t = ts.render_type(&tid, false)?;
                        if t == "String" {
                            return Ok("&str".to_string());
                        }

                        t
                    }
                    openapiv3::ReferenceOr::Item(s) => {
                        match &s.schema_kind {
                            SchemaKind::Type(Type::Boolean {}) => "bool".to_string(),
                            SchemaKind::Type(Type::Array(_at)) => "&[String]".to_string(), /* TODO: make this smarter */
                            SchemaKind::Type(Type::String(st)) => {
                                use openapiv3::{
                                    StringFormat::{Binary, Byte, Date, DateTime, Password},
                                    VariantOrUnknownOrEmpty::{Empty, Item, Unknown},
                                };

                                if st.pattern.is_some() {
                                    bail!("XXX pattern");
                                }

                                if !st.enumeration.is_empty() {
                                    // We have an enum.
                                    // Let's return the correct enum struct name.

                                    // Make sure we actually have a type, we might have
                                    // not added the type since it is a duplicate of another type.
                                    if !name.is_empty() && ts.name_to_id.get(&sn).is_some() {
                                        return Ok(format!("crate::types::{}", struct_name(&sn)));
                                    }

                                    // Create our vector.
                                    let mut enums: Vec<String> = Default::default();
                                    for v in st.enumeration.iter().flatten() {
                                        enums.push(v.to_string());
                                    }
                                    enums.sort_unstable();
                                    enums.dedup();

                                    // Try to find the parameter among our types.
                                    for te in ts.id_to_entry.values() {
                                        if let Some(sn) = te.name.as_deref() {
                                            let sn = struct_name(sn);
                                            if let TypeDetails::Enum(vals, _schema_data) =
                                                &te.details
                                            {
                                                if enums == *vals {
                                                    return Ok(format!("crate::types::{}", sn));
                                                }
                                            }
                                        }
                                    }

                                    let id = ts.select_schema(None, s, "", "").unwrap();
                                    return ts.render_type(&id, false);
                                }

                                if st.min_length.is_some() || st.max_length.is_some() {
                                    println!("XXX min/max length");
                                }

                                match &st.format {
                                    Item(DateTime) => "chrono::DateTime<chrono::Utc>".to_string(),
                                    Item(Date) => "chrono::NaiveDate".to_string(),
                                    Item(Password) => "&str".to_string(),
                                    // TODO: as per the spec this is base64 encoded chars.
                                    Item(Byte) => "&bytes::Bytes".to_string(),
                                    Item(Binary) => "&bytes::Bytes".to_string(),
                                    Empty => "&str".to_string(),
                                    Unknown(f) => match f.as_str() {
                                        "float" => "f64".to_string(),
                                        "int64" => "i64".to_string(),
                                        "uint64" => "u64".to_string(),
                                        "google-fieldmask" => "&str".to_string(),
                                        "google-datetime" => {
                                            "chrono::DateTime<chrono::Utc>".to_string()
                                        }
                                        "ISO 8601 date-time" => {
                                            "chrono::DateTime<chrono::Utc>".to_string()
                                        }
                                        "Promo date-time" => {
                                            "chrono::DateTime<chrono::Utc>".to_string()
                                        }
                                        "dateTime" => "chrono::DateTime<chrono::Utc>".to_string(),
                                        "ipv4" => "std::net::Ipv4Addr".to_string(),
                                        "uri" => "&url::Url".to_string(),
                                        "uri-template" => "&str".to_string(),
                                        "url" => "&url::Url".to_string(),
                                        "email" => "&str".to_string(),
                                        "uuid" => "&str".to_string(),
                                        "hostname" => "&str".to_string(),
                                        "time" => "chrono::NaiveTime".to_string(),
                                        f => {
                                            bail!("XXX unknown string format {}", f)
                                        }
                                    },
                                }
                            }
                            SchemaKind::Type(openapiv3::Type::Number(_)) => "f64".to_string(), /* TODO: make this more exhaustive. */
                            SchemaKind::Type(Type::Integer(it)) => {
                                let mut uint;
                                let width;

                                use openapiv3::VariantOrUnknownOrEmpty::Unknown;
                                if let Unknown(f) = &it.format {
                                    match f.as_str() {
                                        "uint" | "uint32" => {
                                            uint = true;
                                            width = 32;
                                        }
                                        "uint64" => {
                                            uint = true;
                                            width = 32;
                                        }
                                        f => bail!("XXX unknown integer format {}", f),
                                    }
                                } else {
                                    // The format was empty, let's assume it's just a normal
                                    // i64.
                                    uint = false;
                                    width = 64;
                                }

                                if it.multiple_of.is_some() {
                                    bail!("XXX multiple_of");
                                }
                                if it.exclusive_minimum || it.exclusive_maximum {
                                    bail!("XXX exclusive");
                                }

                                if let Some(min) = it.minimum {
                                    if min == 0 {
                                        uint = true;
                                    } else {
                                        // TODO: handle this later
                                        println!("XXX invalid minimum: {}", min);
                                    }
                                }

                                if it.maximum.is_some() {
                                    // TODO: handle this later
                                    println!("XXX maximum is not supported");
                                }
                                if !it.enumeration.is_empty() {
                                    bail!("XXX enumeration {}: {:?}", self.name, it);
                                }
                                if uint {
                                    format!("u{}", width)
                                } else {
                                    format!("i{}", width)
                                }
                            }
                            openapiv3::SchemaKind::OneOf { one_of: _ } => "&str".to_string(), /* TODO: make this smarter. */
                            openapiv3::SchemaKind::Any(_) => "&str".to_string(), /* TODO: make this smarter. */
                            x => bail!("unexpected type {:#?}", x),
                        }
                    }
                }
            }
            x => bail!("XXX param format {:#?}", x),
        })
    }
}

trait ExtractJsonMediaType {
    fn is_binary(&self) -> Result<bool>;
    fn content_json(&self) -> Result<openapiv3::MediaType>;
}

impl ExtractJsonMediaType for openapiv3::Response {
    fn content_json(&self) -> Result<openapiv3::MediaType> {
        // We do not need to check the length of the content because there might be
        // more than one. For example, if xml or some other format is also defined.
        if let Some(mt) = self.content.get("application/json") {
            Ok(mt.clone())
        } else {
            bail!(
                "could not find application/json, only found {}",
                self.content.keys().next().unwrap()
            );
        }
    }

    fn is_binary(&self) -> Result<bool> {
        if self.content.is_empty() {
            /*
             * XXX If there are no content types, I guess it is not binary?
             */
            return Ok(false);
        }

        // We do not need to check the length of the content because there might be
        // more than one. For example, if xml or some other format is also defined.
        if let Some(mt) = self.content.get("application/octet-stream") {
            if !mt.encoding.is_empty() {
                bail!("XXX encoding");
            }

            if let Some(s) = &mt.schema {
                use openapiv3::{SchemaKind, StringFormat, Type, VariantOrUnknownOrEmpty::Item};

                if let Ok(s) = s.item() {
                    if s.schema_data.nullable {
                        bail!("XXX nullable binary?");
                    }
                    if s.schema_data.default.is_some() {
                        bail!("XXX default binary?");
                    }
                    if s.schema_data.discriminator.is_some() {
                        bail!("XXX binary discriminator?");
                    }
                    match &s.schema_kind {
                        SchemaKind::Type(Type::String(st)) => {
                            if st.min_length.is_some() || st.max_length.is_some() {
                                bail!("binary min/max length");
                            }
                            if !matches!(st.format, Item(StringFormat::Binary)) {
                                bail!("expected binary format string, got {:?}", st.format);
                            }
                            if st.pattern.is_some() {
                                bail!("XXX pattern");
                            }
                            if !st.enumeration.is_empty() {
                                bail!("XXX binary enumeration {:?}", st);
                            }
                            return Ok(true);
                        }
                        x => {
                            bail!("XXX schemakind type {:?}", x);
                        }
                    }
                } else {
                    return Ok(false);
                }
            } else {
                bail!("binary thing had no schema?");
            }
        }

        Ok(false)
    }
}

impl ExtractJsonMediaType for openapiv3::RequestBody {
    fn content_json(&self) -> Result<openapiv3::MediaType> {
        // We do not need to check the length of the content because there might be
        // more than one. For example, if xml or some other format is also defined.
        if let Some(mt) = self.content.get("application/json") {
            Ok(mt.clone())
        } else {
            bail!(
                "could not find application/json, only found {}",
                self.content.keys().next().unwrap()
            );
        }
    }

    fn is_binary(&self) -> Result<bool> {
        if self.content.is_empty() {
            /*
             * XXX If there are no content types, I guess it is not binary?
             */
            return Ok(false);
        }

        // We do not need to check the length of the content because there might be
        // more than one. For example, if xml or some other format is also defined.
        if let Some(mt) = self.content.get("application/octet-stream") {
            if !mt.encoding.is_empty() {
                bail!("XXX encoding");
            }

            if let Some(s) = &mt.schema {
                use openapiv3::{SchemaKind, StringFormat, Type, VariantOrUnknownOrEmpty::Item};

                if let Ok(s) = s.item() {
                    if s.schema_data.nullable {
                        bail!("XXX nullable binary?");
                    }
                    if s.schema_data.default.is_some() {
                        bail!("XXX default binary?");
                    }
                    if s.schema_data.discriminator.is_some() {
                        bail!("XXX binary discriminator?");
                    }
                    match &s.schema_kind {
                        SchemaKind::Type(Type::String(st)) => {
                            if st.min_length.is_some() || st.max_length.is_some() {
                                bail!("binary min/max length");
                            }
                            if !matches!(st.format, Item(StringFormat::Binary)) {
                                bail!("expected binary format string, got {:?}", st.format);
                            }
                            if st.pattern.is_some() {
                                bail!("XXX pattern");
                            }
                            if !st.enumeration.is_empty() {
                                bail!("XXX enumeration");
                            }
                            return Ok(true);
                        }
                        x => {
                            bail!("XXX schemakind type {:?}", x);
                        }
                    }
                } else {
                    return Ok(false);
                }
            } else {
                bail!("binary thing had no schema?");
            }
        }

        Ok(false)
    }
}

trait ReferenceOrExt<T> {
    fn item(&self) -> Result<&T>;
}

impl<T> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
    fn item(&self) -> Result<&T> {
        match self {
            openapiv3::ReferenceOr::Item(i) => Ok(i),
            openapiv3::ReferenceOr::Reference { reference } => {
                bail!("reference not supported here: {}", reference);
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TypeDetails {
    Unknown,
    Basic(String, openapiv3::SchemaData),
    NamedType(TypeId, openapiv3::SchemaData),
    Enum(Vec<String>, openapiv3::SchemaData),
    Array(TypeId, openapiv3::SchemaData),
    Optional(TypeId, openapiv3::SchemaData),
    /*
     * Object property names are sorted lexicographically to ensure a stable
     * order in the generated code.
     */
    Object(BTreeMap<String, TypeId>, openapiv3::SchemaData),
    OneOf(Vec<TypeId>, openapiv3::SchemaData),
    AnyOf(Vec<TypeId>, openapiv3::SchemaData),
    AllOf(Vec<TypeId>, openapiv3::SchemaData),
}

#[allow(dead_code)]
impl TypeDetails {
    pub fn is_enum(&self) -> bool {
        if let TypeDetails::Enum(..) = self {
            return true;
        }
        false
    }

    pub fn is_one_of(&self) -> bool {
        if let TypeDetails::OneOf(..) = self {
            return true;
        }
        false
    }

    pub fn is_any_of(&self) -> bool {
        if let TypeDetails::AnyOf(..) = self {
            return true;
        }
        false
    }

    pub fn is_all_of(&self) -> bool {
        if let TypeDetails::AllOf(..) = self {
            return true;
        }
        false
    }

    pub fn is_object(&self) -> bool {
        if let TypeDetails::Object(..) = self {
            return true;
        }
        false
    }

    pub fn is_named_type(&self) -> bool {
        if let TypeDetails::NamedType(..) = self {
            return true;
        }
        false
    }

    pub fn description(&self) -> String {
        let desc = match self {
            TypeDetails::Basic(_, d) => d.description.as_ref(),
            TypeDetails::NamedType(_, d) => d.description.as_ref(),
            TypeDetails::Enum(_, d) => d.description.as_ref(),
            TypeDetails::Array(_, d) => d.description.as_ref(),
            TypeDetails::Optional(_, d) => d.description.as_ref(),
            TypeDetails::Object(_, d) => d.description.as_ref(),
            TypeDetails::OneOf(_, d) => d.description.as_ref(),
            TypeDetails::AnyOf(_, d) => d.description.as_ref(),
            TypeDetails::AllOf(_, d) => d.description.as_ref(),
            TypeDetails::Unknown => None,
        };

        if let Some(n) = desc {
            n.to_string()
        } else {
            "".to_string()
        }
    }
}

impl PartialEq for TypeDetails {
    fn eq(&self, other: &Self) -> bool {
        if self.description() != other.description()
            && !self.is_one_of()
            && !self.is_any_of()
            && self.is_all_of()
        {
            return false;
        }

        match self {
            TypeDetails::Basic(s, _d) => {
                if let TypeDetails::Basic(os, _od) = other {
                    return s == os;
                }
            }
            TypeDetails::NamedType(i, _d) => {
                if let TypeDetails::NamedType(oi, _od) = other {
                    return i == oi;
                }
            }
            TypeDetails::Enum(s, _d) => {
                if let TypeDetails::Enum(os, _od) = other {
                    return s == os;
                }
            }
            TypeDetails::Array(i, _d) => {
                if let TypeDetails::Array(oi, _od) = other {
                    return i == oi;
                }
            }
            TypeDetails::Optional(i, _d) => {
                if let TypeDetails::Optional(oi, _od) = other {
                    return i == oi;
                }
            }
            TypeDetails::Object(s, _d) => {
                if let TypeDetails::Object(os, _od) = other {
                    return s == os;
                }
            }
            TypeDetails::OneOf(s, _d) => {
                if let TypeDetails::OneOf(os, _od) = other {
                    return s == os;
                }
            }
            TypeDetails::AnyOf(s, _d) => {
                if let TypeDetails::AnyOf(os, _od) = other {
                    return s == os;
                }
            }
            TypeDetails::AllOf(s, _d) => {
                if let TypeDetails::AllOf(os, _od) = other {
                    return s == os;
                }
            }
            TypeDetails::Unknown => {
                return self == other;
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
struct TypeEntry {
    id: TypeId,
    name: Option<String>,
    details: TypeDetails,
}

#[derive(Debug, Eq, Clone)]
pub struct TypeId(u64);

impl PartialOrd for TypeId {
    fn partial_cmp(&self, other: &TypeId) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypeId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialEq for TypeId {
    fn eq(&self, other: &TypeId) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone)]
pub struct TypeSpace {
    next_id: u64,
    /*
     * Object types generally have a useful name, which we would like to match
     * with anywhere that name appears in the definition document.  Many other
     * types, though, do not; e.g., an array of strings is just going to become
     * Vec<String> without necesssarily having a useful distinct type name.
     */
    name_to_id: BTreeMap<String, TypeId>,
    id_to_entry: BTreeMap<TypeId, TypeEntry>,
}

impl TypeSpace {
    fn new() -> TypeSpace {
        TypeSpace {
            next_id: 1,
            name_to_id: BTreeMap::new(),
            id_to_entry: BTreeMap::new(),
        }
    }

    /**
     * Emit a human-readable diagnostic description for this type ID.
     */
    fn describe(&self, tid: &TypeId) -> String {
        if let Some(te) = self.id_to_entry.get(tid) {
            match &te.details {
                TypeDetails::Basic(t, ..) => t.to_string(),
                TypeDetails::NamedType(itid, _) => {
                    if let Some(ite) = self.id_to_entry.get(itid) {
                        if let Some(n) = &ite.name {
                            return format!("named type of {} <{}>", n, itid.0);
                        }
                    }

                    /*
                     * If there is no name attached, we should try a
                     * recursive describe.
                     */
                    format!("named type of {}", self.describe(itid))
                }
                TypeDetails::Enum(..) => {
                    if let Some(n) = &te.name {
                        format!("enum {}", n)
                    } else {
                        format!("[ENUM {} !NONAME?]", tid.0)
                    }
                }
                TypeDetails::Array(itid, _) => {
                    if let Some(ite) = self.id_to_entry.get(itid) {
                        if let Some(n) = &ite.name {
                            return format!("array of {} <{}>", n, itid.0);
                        }
                    }

                    /*
                     * If there is no name attached, we should try a
                     * recursive describe.
                     */
                    format!("array of {}", self.describe(itid))
                }
                TypeDetails::Optional(itid, _) => {
                    if let Some(ite) = self.id_to_entry.get(itid) {
                        if let Some(n) = &ite.name {
                            return format!("option of {} <{}>", n, itid.0);
                        }
                    }

                    /*
                     * If there is no name attached, we should try a
                     * recursive describe.
                     */
                    format!("option of {}", self.describe(itid))
                }
                TypeDetails::Object(..) => {
                    if let Some(n) = &te.name {
                        format!("object {}", n)
                    } else {
                        format!("[OBJECT {} !NONAME?]", tid.0)
                    }
                }
                TypeDetails::OneOf(..) => {
                    if let Some(n) = &te.name {
                        format!("one_of {}", n)
                    } else {
                        format!("[ONE_OF {} !NONAME?]", tid.0)
                    }
                }
                TypeDetails::AnyOf(..) => {
                    if let Some(n) = &te.name {
                        format!("any_of {}", n)
                    } else {
                        format!("[ANY_OF {} !NONAME?]", tid.0)
                    }
                }
                TypeDetails::AllOf(..) => {
                    if let Some(n) = &te.name {
                        format!("all_of {}", n)
                    } else {
                        format!("[ALL_OF {} !NONAME?]", tid.0)
                    }
                }
                TypeDetails::Unknown => {
                    format!("[UNKNOWN {}]", tid.0)
                }
            }
        } else {
            format!("[UNMAPPED {}]", tid.0)
        }
    }

    fn get_schema_data_for_id(&self, tid: &TypeId) -> Option<&openapiv3::SchemaData> {
        if let Some(te) = self.id_to_entry.get(tid) {
            match &te.details {
                TypeDetails::Basic(_, schema_data) => Some(schema_data),
                TypeDetails::NamedType(id, schema_data) => {
                    let def: openapiv3::SchemaData = Default::default();
                    if def == *schema_data
                        && schema_data.description.is_none()
                        && !schema_data.nullable
                    {
                        self.get_schema_data_for_id(id)
                    } else {
                        Some(schema_data)
                    }
                }
                TypeDetails::Enum(_, schema_data) => Some(schema_data),
                TypeDetails::Array(_, schema_data) => Some(schema_data),
                TypeDetails::Optional(id, schema_data) => {
                    let def: openapiv3::SchemaData = Default::default();
                    if def == *schema_data {
                        self.get_schema_data_for_id(id)
                    } else {
                        Some(schema_data)
                    }
                }
                TypeDetails::Object(_, schema_data) => Some(schema_data),
                TypeDetails::OneOf(_, schema_data) => Some(schema_data),
                TypeDetails::AnyOf(_, schema_data) => Some(schema_data),
                TypeDetails::AllOf(_, schema_data) => Some(schema_data),
                TypeDetails::Unknown => None,
            }
        } else {
            None
        }
    }

    fn render_docs(&self, tid: &TypeId) -> String {
        let mut out = String::new();

        let mut a = |s: &str| {
            out.push_str(s);
            out.push('\n');
        };

        let schema = self.get_schema_data_for_id(tid);

        if let Some(s) = schema {
            if let Some(description) = &s.description {
                a(&format!(
                    "* {}",
                    description.replace('*', "\\*").replace("\n", "\n*  ")
                ));
            }
            if let Some(external_docs) = &s.external_docs {
                a("*");
                a(&format!("* FROM: <{}>", external_docs.url));
            }
        }

        out.trim().to_string()
    }

    fn render_type(&self, tid: &TypeId, in_mod: bool) -> Result<String> {
        if let Some(te) = self.id_to_entry.get(tid) {
            match &te.details {
                TypeDetails::Basic(t, _) => Ok(t.to_string()),
                TypeDetails::NamedType(itid, _) => self.render_type(itid, in_mod),
                TypeDetails::Enum(..) => {
                    if let Some(n) = &te.name {
                        let struct_name = struct_name(n);
                        if in_mod {
                            Ok(struct_name)
                        } else {
                            /*
                             * Model types are declared in the "types" module,
                             * and must be referenced with that prefix when not
                             * in the module itself.
                             */
                            Ok(format!("crate::types::{}", struct_name))
                        }
                    } else {
                        bail!("enum type {:?} does not have a name?", tid);
                    }
                }
                TypeDetails::OneOf(..) => {
                    if let Some(n) = &te.name {
                        let struct_name = struct_name(n);
                        if in_mod {
                            Ok(struct_name)
                        } else {
                            /*
                             * Model types are declared in the "types" module,
                             * and must be referenced with that prefix when not
                             * in the module itself.
                             */
                            Ok(format!("crate::types::{}", struct_name))
                        }
                    } else {
                        bail!("one_of type {:?} does not have a name?", tid);
                    }
                }
                TypeDetails::AnyOf(..) => {
                    if let Some(n) = &te.name {
                        let struct_name = struct_name(n);
                        if in_mod {
                            Ok(struct_name)
                        } else {
                            /*
                             * Model types are declared in the "types" module,
                             * and must be referenced with that prefix when not
                             * in the module itself.
                             */
                            Ok(format!("crate::types::{}", struct_name))
                        }
                    } else {
                        bail!("any_of type {:?} does not have a name?", tid);
                    }
                }
                TypeDetails::AllOf(..) => {
                    if let Some(n) = &te.name {
                        let struct_name = struct_name(n);
                        if in_mod {
                            Ok(struct_name)
                        } else {
                            /*
                             * Model types are declared in the "types" module,
                             * and must be referenced with that prefix when not
                             * in the module itself.
                             */
                            Ok(format!("crate::types::{}", struct_name))
                        }
                    } else {
                        bail!("all_of type {:?} does not have a name?", tid);
                    }
                }
                TypeDetails::Array(itid, _) => {
                    Ok(format!("Vec<{}>", self.render_type(itid, in_mod)?))
                }
                TypeDetails::Optional(itid, _) => {
                    let rt = self.render_type(itid, in_mod)?;

                    // If it is an enum we should not make it optional.
                    // We have default functions and we should use them.
                    if let Some(te) = self.id_to_entry.get(tid) {
                        if let TypeDetails::Enum(_, sd) = &te.details {
                            if !sd.nullable {
                                return Ok(rt);
                            }
                        }
                    }

                    // We have functions for all these types to have defaults and
                    // custom deserializers.
                    if rt == "String"
                        || rt.starts_with("Vec<")
                        || rt.starts_with("std::collections::HashMap<")
                        || rt == "bool"
                        || rt == "i32"
                        || rt == "i64"
                        || rt == "f32"
                        || rt == "f64"
                        || rt.starts_with("Option<")
                        // This is for ramp, let's hope it doesn't break anything in the future.
                        || rt == "Page"
                        || rt.ends_with("Page")
                        || rt == "crate::types::Page"
                    {
                        Ok(rt)
                    } else {
                        Ok(format!("Option<{}>", rt))
                    }
                }
                TypeDetails::Object(..) => {
                    if let Some(n) = &te.name {
                        let struct_name = struct_name(n);
                        if in_mod {
                            Ok(struct_name)
                        } else {
                            /*
                             * Model types are declared in the "types" module,
                             * and must be referenced with that prefix when not
                             * in the module itself.
                             */
                            Ok(format!("crate::types::{}", struct_name))
                        }
                    } else {
                        bail!("object type {:?} does not have a name?", tid);
                    }
                }
                TypeDetails::Unknown => {
                    bail!("type {:?} is unknown", tid);
                }
            }
        } else {
            panic!("could not resolve type ID {:?}", tid);
        }
    }

    fn assign(&mut self) -> TypeId {
        let id = TypeId(self.next_id);
        self.next_id += 1;
        id
    }

    fn id_for_name(&mut self, name: &str) -> TypeId {
        let id = if let Some(id) = self.name_to_id.get(name) {
            id.clone()
        } else {
            let id = self.assign();
            self.name_to_id.insert(name.to_string(), id.clone());
            id
        };
        id
    }

    fn id_for_optional(&mut self, tid: &TypeId, sd: openapiv3::SchemaData) -> TypeId {
        let mut want = tid.clone();
        for (oid, oent) in self.id_to_entry.iter() {
            match &oent.details {
                TypeDetails::Optional(id, schema_data) => {
                    if *id == want && *schema_data == sd {
                        return oid.clone();
                    }
                }
                _ => continue,
            }
        }

        if let Some(te) = self.id_to_entry.get(tid) {
            if let TypeDetails::Optional(nid, _) = &te.details {
                want = nid.clone();
            }
        }

        let oid = self.assign();
        self.id_to_entry.insert(
            oid.clone(),
            TypeEntry {
                id: oid.clone(),
                name: None,
                details: TypeDetails::Optional(want, sd),
            },
        );
        oid
    }

    fn select_ref(&mut self, _name: Option<&str>, reference: &str) -> Result<TypeId> {
        let ref_ = reference.to_string();

        /*
         * As this is a reference, all we can do for now is determine
         * the type ID.
         */
        if let Some(id) = self.name_to_id.get(&ref_) {
            return Ok(id.clone());
        }

        // This reference has not yet been parsed, so likely what is happening is
        // we are in the middle of populating all the schemas and a schema references
        // a schema that is not yet populated.
        // Let's make some generic details, assign an id and then later in
        // populate ref we can replace this ID with the real one.
        let details = TypeDetails::NamedType(self.assign(), Default::default());
        self.add_if_not_exists(Some(ref_), details, "", true)
    }

    fn add_if_not_exists(
        &mut self,
        name: Option<String>,
        details: TypeDetails,
        parent_name: &str,
        is_reference: bool,
    ) -> Result<TypeId> {
        /*
         * We can have types that are references that are never explicitly called
         * but are duplicated all over. Let's ensure that we don't have a type with a different
         * name that is this exact same type.
         */
        if !is_reference {
            for (tid, te) in self.id_to_entry.iter() {
                if te.details == details {
                    let id = tid.clone();

                    // We have a match! Okay, now we want to keep the shorter
                    // name of the two structs and ensure that is the one we have
                    // in our set.
                    // Only do this if we have an Enum or an Object.
                    let existing_name = if let Some(n) = &te.name {
                        n.to_string()
                    } else {
                        "".to_string()
                    };
                    let new_name = if let Some(n) = &name {
                        n.to_string()
                    } else {
                        "".to_string()
                    };
                    if details.is_object()
                        || details.is_enum()
                        || details.is_one_of()
                        || details.is_any_of()
                        || details.is_all_of()
                    {
                        if existing_name == new_name {
                            // Return early.
                            return Ok(tid.clone());
                        }

                        if !new_name.is_empty() && new_name.len() < existing_name.len() {
                            // Let's make sure we don't already have a type with this name.
                            if let Some(nidd) = self.name_to_id.get(&new_name) {
                                let nid = nidd.clone();

                                // Well we already have an id for this type.
                                // Let's get the details.
                                let nt = self.id_to_entry.get(&nid).unwrap();

                                // Make sure the type we found is not something
                                // we care about.
                                if (nt.details.is_enum()
                                    || nt.details.is_object()
                                    || nt.details.is_named_type()
                                    || nt.details.is_one_of()
                                    || details.is_any_of()
                                    || details.is_all_of())
                                    && nt.details != details
                                {
                                    // Return early we definitely don't want to do any funny business.
                                    return Ok(tid.clone());
                                }
                                let ntdetails = nt.details.clone();

                                // We have some other type...
                                // Let's name it our old name.
                                self.id_to_entry.insert(
                                    nid.clone(),
                                    TypeEntry {
                                        id: nid.clone(),
                                        name: Some(existing_name.to_string()),
                                        details: ntdetails,
                                    },
                                );
                                self.name_to_id.insert(existing_name, nid.clone());
                            }

                            // Let's keep the new_name instead.
                            // This ensure's that since we can't always know the order in which we
                            // parse types, that we will always have the cleaner set of named types.

                            // Update the type entry in our set.
                            self.id_to_entry.insert(
                                id.clone(),
                                TypeEntry {
                                    id: id.clone(),
                                    name: Some(new_name.to_string()),
                                    details,
                                },
                            );
                            self.name_to_id.insert(new_name, id.clone());

                            // Remove the old name from the set.
                            // TODO: somehow don't remove any of the ones we need to keep.
                            // self.name_to_id.remove(&existing_name);
                            return Ok(id);
                        }
                    }

                    return Ok(tid.clone());
                }
            }
        }

        if let Some(name) = &name {
            // First, determine what ID we will use to identify this named type.
            let id = self.id_for_name(name);

            /*
             * If there is already an entry for this type ID, ensure that it
             * matches the entry we have constructed.  If there is not yet an
             * entry, we can just keep this one.
             */
            if let Some(et) = self.id_to_entry.get(&id) {
                if et.details != details {
                    // We can get here if there are two objects with the same name
                    // that have properties that are different.
                    if !parent_name.is_empty() {
                        // We have a parent name, let's append it to the real name.
                        let pname = format!("{} {}", parent_name, name);
                        return self.add_if_not_exists(
                            Some(clean_name(&pname)),
                            details,
                            "",
                            is_reference,
                        );
                    }

                    if !name.contains("data") {
                        // Let's try to append "data" onto the end and see if that helps.
                        let new_name = format!("{} data", name);
                        return self.add_if_not_exists(
                            Some(clean_name(&new_name)),
                            details,
                            "",
                            is_reference,
                        );
                    } else if !name.contains("type") {
                        // Let's try to append "type" onto the end and see if that helps.
                        let new_name = format!("{} type", name);
                        return self.add_if_not_exists(
                            Some(clean_name(&new_name)),
                            details,
                            "",
                            is_reference,
                        );
                    } else if !name.contains("links") {
                        // Let's try to append "type" onto the end and see if that helps.
                        let new_name = format!("{} links", name);
                        return self.add_if_not_exists(
                            Some(clean_name(&new_name)),
                            details,
                            "",
                            is_reference,
                        );
                    } else if !name.contains("object") {
                        // Let's try to append "type" onto the end and see if that helps.
                        let new_name = format!("{} object", name);
                        return self.add_if_not_exists(
                            Some(clean_name(&new_name)),
                            details,
                            "",
                            is_reference,
                        );
                    }

                    // If we don't have anything to append, let's bail.
                    // WE ARE RUNNING OUT OF NAMES AND WE TRIED.
                    bail!(
                        "we ran out of unique names for this thing {}: {:?}",
                        name,
                        details,
                    );
                }
            }

            // We don't have an entry for this type ID so let's add it!
            self.id_to_entry.insert(
                id.clone(),
                TypeEntry {
                    id: id.clone(),
                    name: Some(name.to_string()),
                    details,
                },
            );

            Ok(id)
        } else {
            /*
             * If this type has no name, look for an existing unnamed type with
             * the same shape.
             */
            for (tid, te) in self.id_to_entry.iter() {
                if te.name.is_none() && te.details == details {
                    return Ok(tid.clone());
                }
            }

            /*
             * Otherwise, insert a new entry.
             */
            let tid = self.assign();
            self.id_to_entry.insert(
                tid.clone(),
                TypeEntry {
                    id: tid.clone(),
                    name: None,
                    details,
                },
            );
            Ok(tid)
        }
    }

    fn select_param(
        &mut self,
        name: Option<&str>,
        p: &openapiv3::ReferenceOr<openapiv3::Parameter>,
    ) -> Result<TypeId> {
        match p {
            openapiv3::ReferenceOr::Reference { reference } => {
                self.select_ref(name, reference.as_str())
            }
            openapiv3::ReferenceOr::Item(p) => self.select_parameter(name, p),
        }
    }

    fn populate_ref(
        &mut self,
        name: Option<&str>,
        id: Option<TypeId>,
        type_: &str,
    ) -> Result<TypeId> {
        let nam = if let Some(n) = name {
            n.to_string()
        } else {
            "".to_string()
        };

        let ref_ = format!("#/components/{}s/{}", type_.trim_end_matches('s'), nam);

        let details = if let Some(id) = id {
            TypeDetails::NamedType(id, Default::default())
        } else {
            TypeDetails::NamedType(self.id_for_name("String"), Default::default())
        };

        // Lets check if we already have this reference added.
        // This would have happened if we were parsing all the schemas and something
        // was referenced that had not yet been parsed.
        if let Some(rid) = self.name_to_id.get(&ref_) {
            // Okay we have the id for the reference.
            // Let's update it's named type.
            self.id_to_entry.insert(
                rid.clone(),
                TypeEntry {
                    id: rid.clone(),
                    name: Some(ref_.to_string()),
                    details,
                },
            );

            return Ok(rid.clone());
        }

        self.add_if_not_exists(Some(ref_), details, "", true)
    }

    fn select(
        &mut self,
        name: Option<&str>,
        s: &openapiv3::ReferenceOr<openapiv3::Schema>,
        additional_description: &str,
    ) -> Result<TypeId> {
        match s {
            openapiv3::ReferenceOr::Reference { reference } => {
                self.select_ref(name, reference.as_str())
            }
            openapiv3::ReferenceOr::Item(s) => {
                self.select_schema(name, s, "", additional_description)
            }
        }
    }

    fn select_box(
        &mut self,
        name: Option<&str>,
        s: &openapiv3::ReferenceOr<Box<openapiv3::Schema>>,
        parent_name: &str,
    ) -> Result<TypeId> {
        match s {
            openapiv3::ReferenceOr::Reference { reference } => {
                self.select_ref(name, reference.as_str())
            }
            openapiv3::ReferenceOr::Item(s) => {
                self.select_schema(name, s.as_ref(), parent_name, "")
            }
        }
    }

    fn select_schema(
        &mut self,
        name: Option<&str>,
        s: &openapiv3::Schema,
        parent_name: &str,
        additional_description: &str,
    ) -> Result<TypeId> {
        let (n, details) =
            self.get_type_name_and_details(name, s, parent_name, additional_description)?;

        self.add_if_not_exists(n, details, parent_name, false)
    }

    fn get_type_name_and_details(
        &mut self,
        name: Option<&str>,
        sc: &openapiv3::Schema,
        parent_name: &str,
        additional_description: &str,
    ) -> Result<(Option<String>, TypeDetails)> {
        let nam = if let Some(n) = name {
            n.to_string()
        } else {
            "".to_string()
        };

        let mut s = sc.clone();

        // If we have an additional description and it is better than our original,
        // use it instead.
        if !additional_description.is_empty() {
            let desc = if let Some(d) = &sc.schema_data.description {
                d.to_string()
            } else {
                "".to_string()
            };
            if additional_description.len() > desc.len() {
                s.schema_data.description = Some(additional_description.to_string());
            }
        }

        // Generate a UUID for this type.
        let uid = uuid::Uuid::new_v4();

        match &s.schema_kind {
            openapiv3::SchemaKind::Type(t) => match t {
                openapiv3::Type::Array(at) => {
                    if let Some(items) = &at.items {
                        // Make sure the items type is not empty. If it is empty the
                        // schema kind for the item will be "ANY".
                        if let Ok(item) = items.item() {
                            if let openapiv3::SchemaKind::Any(_) = &item.schema_kind {
                                // The type of array is not defined. Most likey it is a string as has been
                                // observed by the GitHub API spec.
                                let itid = self.add_if_not_exists(
                                    Some(clean_name(&nam)),
                                    TypeDetails::Basic("String".to_string(), Default::default()),
                                    parent_name,
                                    false,
                                )?;
                                return Ok((None, TypeDetails::Array(itid, s.schema_data.clone())));
                            }
                        }

                        // Determine the type of item that will be in this array.
                        let itid = self.select_box(Some(&clean_name(&nam)), items, parent_name)?;
                        Ok((None, TypeDetails::Array(itid, s.schema_data.clone())))
                    } else {
                        // The type of array is not defined. Most likey it is a string as has been
                        // observed by the GitHub API spec.
                        let itid = self.add_if_not_exists(
                            Some(clean_name(&nam)),
                            TypeDetails::Basic("String".to_string(), Default::default()),
                            parent_name,
                            false,
                        )?;
                        Ok((None, TypeDetails::Array(itid, s.schema_data.clone())))
                    }
                }
                openapiv3::Type::Object(o) => {
                    // Object types must have a consistent name.
                    let mut name = clean_name(match (name, s.schema_data.title.as_deref()) {
                        (Some(n), None) => n,
                        (Some(n), Some("")) => n,
                        (None, Some(t)) => t,
                        (Some(""), Some(t)) => t,
                        (Some(n), Some(t)) => {
                            // Check if we already have a type with this name.
                            if n == t
                                || n.ends_with("response")
                                || n.ends_with("request")
                                || self.name_to_id.get(&clean_name(n)).is_some()
                            {
                                t
                            } else if t.ends_with("response")
                                || t.ends_with("request")
                                || self.name_to_id.get(&clean_name(t)).is_some()
                                || n.len() < t.len()
                            {
                                n
                            } else {
                                t
                            }
                        }
                        (None, None) => {
                            if !parent_name.is_empty() {
                                parent_name
                            } else {
                                bail!("object types need a name? {:?} {:?}", name, s)
                            }
                        }
                    });

                    // TODO: this is a horrible fix just for google!
                    if name == "user custom properties" {
                        return Ok((
                            Some(name.to_string()),
                            TypeDetails::Basic(
                                "std::collections::HashMap<String, \
                                 std::collections::HashMap<String, serde_json::Value>>"
                                    .to_string(),
                                s.schema_data.clone(),
                            ),
                        ));
                    }

                    if o.properties.is_empty() {
                        // TODO: make this work for when there is both.
                        if let Some(openapiv3::AdditionalProperties::Schema(ad)) =
                            &o.additional_properties
                        {
                            let desc = if let Some(ref d) = s.schema_data.description {
                                d.to_string()
                            } else {
                                "".to_string()
                            };

                            // If this name already exists add additional properties to it.
                            if self.name_to_id.get(&clean_name(&name)).is_some() {
                                name = format!("{} additional properties", name);
                            }
                            let id = self.select(Some(&name), ad, &desc)?;
                            return Ok((
                                Some(name.to_string()),
                                TypeDetails::NamedType(id, s.schema_data.clone()),
                            ));
                        }
                    }

                    let mut omap = BTreeMap::new();
                    for (n, rb) in o.properties.iter() {
                        if n.is_empty() {
                            println!("XXX n cannot be empty for {}", name);
                            continue;
                        }

                        // If we have a unit struct where there is only one property in
                        // the object, call the object by that property name.
                        // This is Oxide exclusive.
                        // Turn this off it breaks other generators, we, Oxide just need to add
                        // titles to weird shit we do.
                        /*if o.properties.len() == 1 {
                            if self.name_to_id.get(&clean_name(&n)).is_none()
                                && clean_name(&n) != "default"
                            {
                                // Use this name since it doesn't already exist.
                                name = clean_name(n);
                            }
                        }*/

                        let itid = self.select_box(
                            Some(n),
                            rb,
                            &clean_name(&format!("{} {}", &parent_name, name)),
                        )?;

                        if let Some(sd) = &self.get_schema_data_for_id(&itid) {
                            let schema_data = &(*sd).clone();
                            // TODO: "page" is specific to ramp
                            if (schema_data.nullable && name != "page") || (n == "repo") {
                                // This is an optional member.
                                omap.insert(
                                    n.to_string(),
                                    self.id_for_optional(&itid, schema_data.clone()),
                                );
                                continue;
                            }

                            // TODO: "page" is specific to ramp
                            if (o.required.contains(n) || name == "page") && (n != "repo") {
                                omap.insert(n.to_string(), itid.clone());
                            } else {
                                // This is an optional member.
                                omap.insert(
                                    n.to_string(),
                                    self.id_for_optional(&itid, schema_data.clone()),
                                );
                            }
                        }

                        // TODO: "page" is specific to ramp
                        if o.required.contains(n) || name == "page" {
                            omap.insert(n.to_string(), itid);
                        } else {
                            // This is an optional member.
                            omap.insert(
                                n.to_string(),
                                self.id_for_optional(&itid, s.schema_data.clone()),
                            );
                        }
                    }
                    Ok((Some(name), TypeDetails::Object(omap, s.schema_data.clone())))
                }
                openapiv3::Type::String(st) => {
                    use openapiv3::{
                        StringFormat::{Binary, Byte, Date, DateTime, Password},
                        VariantOrUnknownOrEmpty::{Empty, Item, Unknown},
                    };

                    // We have an enumeration.
                    if !st.enumeration.is_empty() {
                        // Enum types must have a consistent name.
                        let name = clean_name(match (name, s.schema_data.title.as_deref()) {
                            (Some(n), None) => n,
                            (Some(n), Some("")) => n,
                            (None, Some(t)) => t,
                            (Some(""), Some(t)) => t,
                            (Some(n), Some(t)) => {
                                // Check if we already have a type with this name.
                                if n == t
                                    || (n.ends_with("response") || n.ends_with("request"))
                                    || self.name_to_id.get(&clean_name(n)).is_some()
                                {
                                    t
                                } else if (t.ends_with("response") || t.ends_with("request"))
                                    || self.name_to_id.get(&clean_name(t)).is_some()
                                    || n.len() < t.len()
                                {
                                    n
                                } else {
                                    t
                                }
                            }
                            (None, None) => {
                                bail!("enumeration types need a name? {:?} {:?}", name, s)
                            }
                        });

                        let mut enums: Vec<String> = Default::default();
                        for v in st.enumeration.iter().flatten() {
                            enums.push(v.to_string());
                        }
                        enums.sort_unstable();
                        enums.dedup();

                        if !enums.is_empty() {
                            return Ok((
                                Some(clean_name(&name)),
                                TypeDetails::Enum(enums, s.schema_data.clone()),
                            ));
                        }
                    }

                    match &st.format {
                        // It is far too risky to not make all the DateTime/Dates optional
                        // otherwise you are just risking a panic when the vendor passes back
                        // a null.
                        Item(DateTime) => Ok((
                            Some(uid.to_string()),
                            TypeDetails::Basic(
                                "Option<chrono::DateTime<chrono::Utc>>".to_string(),
                                s.schema_data.clone(),
                            ),
                        )),
                        Item(Date) => Ok((
                            Some(uid.to_string()),
                            TypeDetails::Basic(
                                "Option<chrono::NaiveDate>".to_string(),
                                s.schema_data.clone(),
                            ),
                        )),
                        Item(Password) => Ok((
                            Some(uid.to_string()),
                            TypeDetails::Basic("String".to_string(), s.schema_data.clone()),
                        )),
                        // TODO: as per the spec this is base64 encoded chars.
                        Item(Byte) => Ok((
                            Some(uid.to_string()),
                            TypeDetails::Basic("bytes::Bytes".to_string(), s.schema_data.clone()),
                        )),
                        Item(Binary) => Ok((
                            Some(uid.to_string()),
                            TypeDetails::Basic("bytes::Bytes".to_string(), s.schema_data.clone()),
                        )),
                        Empty => {
                            // Get the name, we need to find out if its secretly a date.
                            let name = clean_name(match (name, s.schema_data.title.as_deref()) {
                                (Some(n), None) => n,
                                (Some(n), Some("")) => n,
                                (None, Some(t)) => t,
                                (Some(""), Some(t)) => t,
                                (Some(n), Some(_)) => n,
                                (None, None) => "",
                            });

                            if name.starts_with("date ") || name.ends_with(" date") {
                                // Gusto does not set the type as a NaiveDate but it should be so let's fix it.
                                Ok((
                                    Some(uid.to_string()),
                                    TypeDetails::Basic(
                                        "Option<chrono::NaiveDate>".to_string(),
                                        s.schema_data.clone(),
                                    ),
                                ))
                            } else {
                                Ok((
                                    Some(uid.to_string()),
                                    TypeDetails::Basic("String".to_string(), s.schema_data.clone()),
                                ))
                            }
                        }
                        Unknown(f) => match f.as_str() {
                            "float" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic("f64".to_string(), s.schema_data.clone()),
                            )),
                            "int64" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic("i64".to_string(), s.schema_data.clone()),
                            )),
                            "uint64" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic("u64".to_string(), s.schema_data.clone()),
                            )),
                            "google-fieldmask" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic("String".to_string(), s.schema_data.clone()),
                            )),
                            "google-datetime" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic(
                                    "Option<chrono::DateTime<chrono::Utc>>".to_string(),
                                    s.schema_data.clone(),
                                ),
                            )),
                            "ISO 8601 date-time" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic(
                                    "Option<chrono::DateTime<chrono::Utc>>".to_string(),
                                    s.schema_data.clone(),
                                ),
                            )),
                            "Promo date-time" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic(
                                    "Option<chrono::DateTime<chrono::Utc>>".to_string(),
                                    s.schema_data.clone(),
                                ),
                            )),
                            "dateTime" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic(
                                    "Option<chrono::DateTime<chrono::Utc>>".to_string(),
                                    s.schema_data.clone(),
                                ),
                            )),
                            "ipv4" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic(
                                    "std::net::Ipv4Addr".to_string(),
                                    s.schema_data.clone(),
                                ),
                            )),
                            "uri" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic(
                                    "Option<url::Url>".to_string(),
                                    s.schema_data.clone(),
                                ),
                            )),
                            "uri-template" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic("String".to_string(), s.schema_data.clone()),
                            )),
                            "url" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic(
                                    "Option<url::Url>".to_string(),
                                    s.schema_data.clone(),
                                ),
                            )),
                            "email" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic("String".to_string(), s.schema_data.clone()),
                            )),
                            "uuid" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic("String".to_string(), s.schema_data.clone()),
                            )),
                            "hostname" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic("String".to_string(), s.schema_data.clone()),
                            )),
                            "time" => Ok((
                                Some(uid.to_string()),
                                TypeDetails::Basic(
                                    "Option<chrono::NaiveTime>".to_string(),
                                    s.schema_data.clone(),
                                ),
                            )),
                            f => bail!("XXX unknown string format {}", f),
                        },
                    }
                }
                openapiv3::Type::Boolean {} => Ok((
                    Some(uid.to_string()),
                    TypeDetails::Basic("bool".to_string(), s.schema_data.clone()),
                )),
                openapiv3::Type::Number(_) => Ok((
                    Some(uid.to_string()),
                    TypeDetails::Basic("f64".to_string(), s.schema_data.clone()),
                )),
                openapiv3::Type::Integer(_) => Ok((
                    Some(uid.to_string()),
                    TypeDetails::Basic("i64".to_string(), s.schema_data.clone()),
                )),
            },
            openapiv3::SchemaKind::AllOf { all_of } => {
                // TODO: this is a stop gap for now, we should figure out a better solution later.
                // Iterate over each one of and add each of them to our typeset.
                // AllOf types must have a consistent name.
                let mut all_of_name = nam;
                if all_of_name.is_empty() && !parent_name.is_empty() {
                    all_of_name = parent_name.to_string();
                }
                if all_of_name.is_empty() {
                    bail!("all_of name cannot be empty!");
                }
                all_of_name.push_str(" all of");

                let mut omap: Vec<TypeId> = Default::default();
                for one in all_of {
                    let itid = self.select(
                        Some(all_of_name.trim_end_matches("all of").trim()),
                        one,
                        additional_description,
                    )?;

                    // If we only have one value let's just return that
                    // value.
                    if all_of.len() == 1 {
                        if let Some(et) = self.id_to_entry.get(&itid) {
                            if s.schema_data.nullable {
                                return Ok((
                                    Some(all_of_name.trim_end_matches("all of").trim().to_string()),
                                    TypeDetails::Optional(itid, s.schema_data.clone()),
                                ));
                            } else {
                                return Ok((
                                    Some(all_of_name.trim_end_matches("all of").trim().to_string()),
                                    et.details.clone(),
                                ));
                            }
                        }
                    }

                    omap.push(itid);
                }
                omap.sort_unstable();
                omap.dedup();

                Ok((
                    Some(all_of_name),
                    TypeDetails::AllOf(omap, Default::default()),
                ))
            }
            openapiv3::SchemaKind::OneOf { one_of } => {
                // Iterate over each one of and add each of them to our typeset.
                // OneOf types must have a consistent name.
                let mut one_of_name = nam;
                if one_of_name.is_empty() && !parent_name.is_empty() {
                    one_of_name = parent_name.to_string();
                }
                if one_of_name.is_empty() {
                    bail!("one_of name cannot be empty!");
                }
                one_of_name.push_str(" one of");

                let mut omap: Vec<TypeId> = Default::default();
                for one in one_of {
                    let itid = self.select(
                        Some(one_of_name.trim_end_matches("one of").trim()),
                        one,
                        additional_description,
                    )?;

                    // If we only have one value let's just return that
                    // value.
                    if one_of.len() == 1 {
                        if let Some(et) = self.id_to_entry.get(&itid) {
                            if s.schema_data.nullable {
                                return Ok((
                                    Some(one_of_name.trim_end_matches("one of").trim().to_string()),
                                    TypeDetails::Optional(itid, s.schema_data.clone()),
                                ));
                            } else {
                                return Ok((
                                    Some(one_of_name.trim_end_matches("one of").trim().to_string()),
                                    et.details.clone(),
                                ));
                            }
                        }
                    }

                    omap.push(itid);
                }

                omap.sort_unstable();
                omap.dedup();

                Ok((
                    Some(one_of_name),
                    TypeDetails::OneOf(omap, Default::default()),
                ))
            }
            openapiv3::SchemaKind::AnyOf { any_of } => {
                // TODO: This is a stop gap for now, we should figure out a better solution later.
                // Iterate over each one of and add each of them to our typeset.
                // AnyOf types must have a consistent name.
                let mut any_of_name = nam;
                if any_of_name.is_empty() && !parent_name.is_empty() {
                    any_of_name = parent_name.to_string();
                }
                if any_of_name.is_empty() {
                    bail!("any_of name cannot be empty!");
                }
                any_of_name.push_str(" any of");

                let mut omap: Vec<TypeId> = Default::default();
                for one in any_of {
                    let itid = self.select(
                        Some(any_of_name.trim_end_matches("any of").trim()),
                        one,
                        additional_description,
                    )?;

                    // If we only have any value let's just return that
                    // value.
                    if any_of.len() == 1 {
                        if let Some(et) = self.id_to_entry.get(&itid) {
                            if s.schema_data.nullable {
                                return Ok((
                                    Some(any_of_name.trim_end_matches("any of").trim().to_string()),
                                    TypeDetails::Optional(itid, s.schema_data.clone()),
                                ));
                            } else {
                                return Ok((
                                    Some(any_of_name.trim_end_matches("any of").trim().to_string()),
                                    et.details.clone(),
                                ));
                            }
                        }
                    }

                    omap.push(itid);
                }

                omap.sort_unstable();
                omap.dedup();

                Ok((
                    Some(any_of_name),
                    TypeDetails::AnyOf(omap, Default::default()),
                ))
            }
            openapiv3::SchemaKind::Any(any) => {
                // There is at least one occurance where the github api spec gives "items"
                // but not a type array. Let's parse for that.
                // https://github.com/github/rest-api-description/issues/455
                if let Some(items) = &any.items {
                    // Determine the type of item that will be in this array.
                    let itid = self.select_box(Some(&clean_name(&nam)), items, parent_name)?;
                    return Ok((None, TypeDetails::Array(itid, s.schema_data.clone())));
                }

                if let Some(format) = &any.format {
                    // For some reason, ramp has any types with uuids.
                    if format == "uuid" {
                        return Ok((
                            Some(clean_name(&nam)),
                            TypeDetails::Basic("String".to_string(), s.schema_data.clone()),
                        ));
                    }
                }

                if !any.properties.is_empty() && name.is_some() {
                    // TODO: This is an object but it's messed up.
                    // Make this more DRY with the above.
                    // Object types must have a consistent name.
                    let name = clean_name(match (name, s.schema_data.title.as_deref()) {
                        (Some(n), None) => n,
                        (Some(n), Some("")) => n,
                        (None, Some(t)) => t,
                        (Some(""), Some(t)) => t,
                        (Some(n), Some(t)) => {
                            // Check if we already have a type with this name.
                            if n == t
                                || (n.ends_with("response") || n.ends_with("request"))
                                || self.name_to_id.get(&clean_name(n)).is_some()
                            {
                                t
                            } else if (t.ends_with("response") || t.ends_with("request"))
                                || self.name_to_id.get(&clean_name(t)).is_some()
                                || n.len() < t.len()
                            {
                                n
                            } else {
                                t
                            }
                        }
                        (None, None) => {
                            if !parent_name.is_empty() {
                                parent_name
                            } else {
                                bail!("object types need a name? {:?} {:?}", name, s)
                            }
                        }
                    });

                    let mut omap = BTreeMap::new();
                    for (n, rb) in any.properties.iter() {
                        let itid = self.select_box(
                            Some(n),
                            rb,
                            &clean_name(&format!("{} {}", &parent_name, name)),
                        )?;
                        if let Some(sd) = &self.get_schema_data_for_id(&itid) {
                            let schema_data = &(*sd).clone();
                            // TODO: "page" is specific to ramp
                            if schema_data.nullable && name != "page" {
                                // This is an optional member.
                                omap.insert(
                                    n.to_string(),
                                    self.id_for_optional(&itid, schema_data.clone()),
                                );
                                continue;
                            }

                            // TODO: "page" is specific to ramp
                            if any.required.contains(n) || name == "page" {
                                omap.insert(n.to_string(), itid.clone());
                            } else {
                                // This is an optional member.
                                omap.insert(
                                    n.to_string(),
                                    self.id_for_optional(&itid, schema_data.clone()),
                                );
                            }
                        }

                        // TODO: "page" is specific to ramp
                        if any.required.contains(n) || name == "page" {
                            omap.insert(n.to_string(), itid);
                        } else {
                            // This is an optional member.
                            omap.insert(
                                n.to_string(),
                                self.id_for_optional(&itid, s.schema_data.clone()),
                            );
                        }
                    }
                    return Ok((Some(name), TypeDetails::Object(omap, s.schema_data.clone())));
                }

                // We have no idea what this is.
                // Then we use the serde_json type.
                println!(
                    "[warn] got ANY kind: {:?} {} {:?}\n",
                    name, parent_name, any
                );

                Ok((
                    Some(nam),
                    TypeDetails::Basic("serde_json::Value".to_string(), s.schema_data.clone()),
                ))
            }
        }
    }

    fn select_parameter(&mut self, name: Option<&str>, p: &openapiv3::Parameter) -> Result<TypeId> {
        let nam = if let Some(n) = name {
            n.to_string()
        } else {
            "".to_string()
        };

        if let Some(parameter_data) = get_parameter_data(p) {
            if let openapiv3::ParameterSchemaOrContent::Schema(st) = &parameter_data.format {
                let desc = if let Some(d) = &parameter_data.description {
                    d.to_string()
                } else {
                    "".to_string()
                };

                match st {
                    openapiv3::ReferenceOr::Reference { reference } => {
                        self.select_ref(name, reference.as_str())
                    }
                    openapiv3::ReferenceOr::Item(s) => {
                        self.select_schema(Some(&parameter_data.name), s, &nam, &desc)
                    }
                }
            } else {
                bail!("could not get parameter_schema for {:?}: {:?}", name, p);
            }
        } else {
            bail!("could not get parameter_data for {:?}: {:?}", name, p);
        }
    }
}

fn get_parameter_data(param: &openapiv3::Parameter) -> Option<&openapiv3::ParameterData> {
    match param {
        openapiv3::Parameter::Path {
            parameter_data,
            style: openapiv3::PathStyle::Simple,
        } => return Some(parameter_data),
        openapiv3::Parameter::Header {
            parameter_data,
            style: openapiv3::HeaderStyle::Simple,
        } => return Some(parameter_data),
        openapiv3::Parameter::Cookie {
            parameter_data,
            style: openapiv3::CookieStyle::Form,
        } => return Some(parameter_data),
        openapiv3::Parameter::Query {
            parameter_data,
            allow_reserved: _,
            style: openapiv3::QueryStyle::Form,
            allow_empty_value: _,
        } => {
            return Some(parameter_data);
        }
        _ => (),
    }

    None
}

fn render_param(
    sn: &str,
    en: &[String],
    required: bool,
    description: &str,
    default: Option<&serde_json::Value>,
) -> String {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    if en.is_empty() {
        return out.to_string();
    }

    let mut enumsd = en.to_vec();
    enumsd.sort_unstable();
    enumsd.dedup();

    let mut enums: Vec<String> = Default::default();
    for e in &enumsd {
        // Find any duplicates that are capitalized differently.
        if enums.contains(e)
            || enums.contains(&to_snake_case(e))
            || enums.contains(&to_title_case(e))
        {
            continue;
        }
        enums.push(e.to_string());
    }

    if !description.is_empty() {
        a("/**");
        a(&format!("* {}", description.replace("\n", "\n*   ")));
        a("*/");
    }

    a("#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]");

    a(&format!("pub enum {} {{", sn));
    for e in &enums {
        if struct_name(e).is_empty() {
            // TODO: do something for empty(?)
            continue;
        }
        a(&format!(r#"#[serde(rename = "{}")]"#, e));
        a(&format!("{},", struct_name(e)));
    }
    if !required && default.is_none() {
        a(r#"#[serde(rename = "")]"#);
        a("Noop,");
    }

    // Let's add the wildcard.
    a("#[serde(other)]");
    a("FallthroughString");

    a("}");
    a("");

    a(&format!("impl std::fmt::Display for {} {{", sn));
    a(r#"fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {"#);
    a(r#"match &*self {"#);
    for e in &enums {
        if struct_name(e).is_empty() {
            // TODO: do something for empty(?)
            continue;
        }
        a(&format!(r#"{}::{} => "{}","#, sn, struct_name(e), e));
    }
    if !required && default.is_none() {
        a(&format!(r#"{}::Noop => "","#, sn));
    }

    // Let's add the display format for the wildcard.
    a(&format!(r#"{}::FallthroughString => "*","#, sn));

    a("}");
    a(".fmt(f)");
    a("}");
    a("}");
    a("");

    // Add a default for the enum if it is not required.
    if !required || default.is_some() {
        a(&format!("impl Default for {} {{", sn));
        a(&format!("fn default() -> {} {{", sn));
        if let Some(d) = default {
            // Use the default that can be passed to the OpenAPI,
            // github is not using that currently for everything but we might want to
            // in the future.
            a(&format!(
                "{}::{}",
                sn,
                struct_name(&d.to_string().replace('"', ""))
            ));
        } else {
            a(&format!("{}::Noop", sn));
        }
        a("}");
        a("}");
    }

    // Add a method to check if it is empty if it has this Noop state.
    if !required && default.is_none() {
        a(&format!(
            r#"impl {} {{
            pub fn is_noop(&self) -> bool {{
                matches!(self, {}::Noop)
            }}
    }}"#,
            sn, sn
        ));
        a("");
    }

    out.to_string()
}

fn gen(
    api: &OpenAPI,
    proper_name: &str,
    host: &str,
    tags: Vec<String>,
    token_endpoint: &str,
    user_consent_endpoint: &str,
    add_post_header: &str,
) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    /*
     * Deal with any dependencies we require to produce this client.
     */
    a("#![allow(clippy::too_many_arguments)]");
    a("#![allow(clippy::nonstandard_macro_braces)]");
    a("#![allow(clippy::large_enum_variant)]");
    a("#![allow(clippy::tabs_in_doc_comments)]");
    a("#![allow(missing_docs)]"); // TODO: Make this a deny.
    a("#![cfg_attr(docsrs, feature(doc_cfg))]");
    a("");
    if proper_name == "GitHub" {
        a("pub mod auth;");
        a(r#"#[cfg(feature = "httpcache")]"#);
        a(r#"#[cfg_attr(docsrs, doc(cfg(feature = "httpcache")))]"#);
        a("pub mod http_cache;");
    }
    if proper_name == "Google Drive"
        || proper_name == "Google Sheets"
        || proper_name == "SendGrid"
        || proper_name == "Rev.ai"
    {
        a("pub mod traits;");
    }
    a("#[cfg(test)]");
    a("mod tests;");
    // Hopefully there is never a "tag" named after these reserved libs.
    a("pub mod types;");
    a("#[doc(hidden)]");
    a("pub mod utils;");

    /*
     * Import the module for each tag.
     * Tags are how functions are grouped.
     */
    for tag in api.tags.iter() {
        if !tags.contains(&to_snake_case(&clean_name(&tag.name)))
            && (proper_name == "Zoom" || proper_name == "DocuSign")
        {
            // Return early do nothing!
            // This fixes Zoom and DocuSign where they list tags that have no associated functions.
            continue;
        }

        let mut docs = "".to_string();
        if let Some(d) = &tag.description {
            docs = format!("{}.", d.trim_end_matches('.'));
        }
        if let Some(e) = &tag.external_docs {
            if !e.url.is_empty() {
                docs = format!("{}\n\nFROM: {}", docs, e.url);
            }
        }
        docs = docs.trim().to_string();

        if !docs.is_empty() {
            a(&format!("/// {}", docs.replace("\n", "\n///"),));
        }
        a(&format!(
            "pub mod {};",
            to_snake_case(&clean_name(&tag.name))
        ));
    }
    if api.tags.is_empty() {
        // If the spec didn't call out tags explicitly, we need to use the
        // ones we found ourselves.
        for tag in tags.iter() {
            if !tag.is_empty() {
                a(&format!("pub mod {};", to_snake_case(&clean_name(tag))));
            }
        }
    }

    a("");
    if proper_name.starts_with("Google") {
        a("use std::io::Write;");
        a("");
    }

    a("use anyhow::{anyhow, Error, Result};");
    a("");

    a(&format!(
        r#"pub const DEFAULT_HOST: &str = "https://{}";"#,
        host.trim_start_matches("https://")
    ));
    a("");

    a("mod progenitor_support {");
    a("    use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};");
    a("");
    /*
     * The percent-encoding crate abrogates its responsibility for providing
     * useful percent-encoding sets, so we must provide one for path components
     * here.
     */
    a("    const PATH_SET: &AsciiSet = &CONTROLS");
    /*
     * The query percent-encode set is the C0 control percent-encode set and
     * U+0020 SPACE, U+0022 ("), U+0023 (#), U+003C (<), and U+003E (>).
     */
    a("        .add(b' ')");
    a("        .add(b'\"')");
    a("        .add(b'#')");
    a("        .add(b'<')");
    a("        .add(b'>')");
    /*
     * The path percent-encode set is the query percent-encode set and U+003F
     * (?), U+0060 (`), U+007B ({), and U+007D (}).
     */
    a("        .add(b'?')");
    a("        .add(b'`')");
    a("        .add(b'{')");
    a("        .add(b'}');");
    a("");
    a("    #[allow(dead_code)]");
    a("    pub(crate) fn encode_path(pc: &str) -> String {");
    a("        utf8_percent_encode(pc, PATH_SET).to_string()");
    a("    }");
    a("}");
    a("");

    a("");

    // Print the client template.
    if proper_name == "GitHub" {
        a(crate::client::GITHUB_TEMPLATE);
    } else if proper_name == "SendGrid"
        || proper_name == "Giphy"
        || proper_name == "Rev.ai"
        || proper_name == "Okta"
    {
        a(&crate::client::generate_client_generic_api_key(
            proper_name,
            add_post_header,
        ));
    } else if proper_name == "TripActions" {
        a(&crate::client::generate_client_generic_client_credentials(
            proper_name,
            token_endpoint,
            add_post_header,
        ));
    } else {
        a(&crate::client::generate_client_generic_token(
            proper_name,
            token_endpoint,
            user_consent_endpoint,
            add_post_header,
        ));
    }

    a("");

    /*
     * Generate a function for each tag.
     * Tags are how functions are grouped.
     */
    for tag in api.tags.iter() {
        if !tags.contains(&to_snake_case(&tag.name))
            && (proper_name == "Zoom" || proper_name == "DocuSign")
        {
            // Return early do nothing!
            // This fixes Zoom and DocuSign where they list tags that have no associated functions.
            continue;
        }

        let mut docs = format!(
            "Return a reference to an interface that provides access to {} operations.",
            tag.name
        );
        if let Some(d) = &tag.description {
            docs = format!("{}.", d.trim_end_matches('.'));
        }
        if let Some(e) = &tag.external_docs {
            if !e.url.is_empty() {
                docs = format!("{}\n\nFROM: {}", docs, e.url);
            }
        }

        a(&format!(
            r#"/// {}
               pub fn {}(&self) -> {}::{} {{
                    {}::{}::new(self.clone())
               }}"#,
            docs.replace("\n", "\n///"),
            to_snake_case(&clean_name(&tag.name)),
            to_snake_case(&clean_name(&tag.name)),
            struct_name(&tag.name),
            to_snake_case(&clean_name(&tag.name)),
            struct_name(&tag.name),
        ));
        a("");
    }
    if api.tags.is_empty() {
        // If the spec didn't call out tags explicitly, we need to use the
        // ones we found ourselves.
        for tag in tags.iter() {
            if !tag.is_empty() {
                a(&format!(
                    r#"pub fn {}(&self) -> {}::{} {{
                    {}::{}::new(self.clone())
               }}"#,
                    to_snake_case(&clean_name(tag)),
                    to_snake_case(&clean_name(tag)),
                    struct_name(tag),
                    to_snake_case(&clean_name(tag)),
                    struct_name(tag),
                ));
                a("");
            }
        }
    }

    a("}");

    Ok(out)
}

pub fn make_plural(proper_name: &str, s: &str) -> String {
    // Only fix the ramp names.
    if proper_name != "Ramp" && proper_name != "Okta" {
        return s.to_string();
    }

    if s.ends_with("ss") && !s.ends_with("access") {
        return format!("{}es", s);
    } else if s.ends_with('s') || s.ends_with("_all") {
        return s.to_string();
    } else if s.ends_with('y') {
        return format!("{}ies", s.trim_end_matches('y'));
    }

    format!("{}s", s)
}

fn struct_name(s: &str) -> String {
    let t = to_title_case(&clean_name(s)).replace(" ", "");

    // Check if we actually have a number.
    if let Ok(i) = t.parse::<i64>() {
        // Change the number to words for the enum, etc.
        // This fixes Zoom and hopefully does not break anyone else.
        to_pascal_case(&english_numbers::convert_all_fmt(i))
    } else if t == "Option" || t == "Self" {
        // Fix any reserved words.
        format!("{}Data", t)
    } else {
        t
    }
}

fn clean_name(t: &str) -> String {
    let mut s = t;
    if t == "/" {
        s = "root";
    }

    let mut st = to_snake_case(
        &s.replace("+1", "plus_one")
            .replace("-1", "minus_one")
            .replace("2fa", "two_fa")
            .replace(".v1", "")
            .replace("100644", "file_blob")
            .replace("100755", "executable_blob")
            .replace("040000", "subdirectory_tree")
            .replace("160000", "submodule_commit")
            .replace("120000", "symlink_path_blob")
            .replace("shipbob", "")
            .replace("ShipBob", "")
            .replace("public api models", "")
            .replace("api view models model", "")
            .replace(" an ", " ")
            .replace(" or ", " ")
            .replace(" for ", " ")
            .replace(" to ", " ")
            .replace(" your ", " ")
            .replace(" is ", " ")
            .replace(" and ", " ")
            .replace(" the ", " ")
            .replace("/", " ")
            .replace("-", " "),
    )
    .replace("_i_ds", "_ids")
    .replace("v_1_", "")
    .replace("_v_2_", "_")
    .replace("_v_3_", "_")
    .replace("s_uuid", "")
    .replace("_id_or_uuid", "")
    .replace("_uuid", "")
    .replace("_id_", "_")
    .replace("companies_company_", "company_")
    .replace("employees_employee_", "employee_")
    .replace("benefits_benefit_", "benefit_")
    .replace("compensations_compensation_", "compensation_")
    .replace("garnishments_garnishment_", "garnishment_")
    .replace("types_type_", "type_")
    .replace("contractors_contractor_", "comntractor_")
    .replace("jobs_job_", "job_")
    .replace("payrolls_payroll_", "payroll_")
    .replace("locations_location_", "location_")
    .replace("applicants_applicant_", "applicant_")
    .replace("_public_api_view_models", "_")
    .replace("_public_common_models", "_")
    .replace("_common_models", "_")
    .replace("_public_common", "_")
    .replace("_api_view_models", "_")
    .replace("_api_models_public", "_")
    .replace("_presentation_view_models", "_")
    .replace("_presentation_models", "_")
    .replace("channels_channel", "channel")
    .replace("orders_order", "order")
    .replace("webhooks_webhook", "webhook")
    .replace("products_public", "products")
    .replace("products_product", "product")
    .replace("returns_return", "return")
    .replace("_view_model", "_")
    .trim_start_matches('_')
    .trim_end_matches('_')
    .replace("_", " ")
    .trim()
    .to_string();

    if st == "i_ds" {
        st = "ids".to_string();
    }

    let mut words: Vec<String> = Default::default();
    // Only get a string with unique words.
    let split = st.split(' ');
    for s in split {
        if words.contains(&s.to_string()) {
            continue;
        }
        words.push(s.to_string());
    }

    words.join(" ")
}

pub fn path_to_operation_id(path: &str, method: &str) -> String {
    let new = format!(
        "{}_{}",
        method.to_lowercase(),
        path.replace("/", "-")
            .trim_start_matches('-')
            .replace('{', "_by_")
            .replace('}', "")
            .replace("shippingmethod", "shipping_method")
            .replace("statushistory", "status_history")
            .replace("cancelbulk", "cancel_bulk")
    );

    if path == "/order/{orderId}/shipment" {
        return "get_order_shipments".to_string();
    } else if path == "/order/{orderId}/shipment/{shipmentId}/logs" {
        return "get_order_shipment_logs".to_string();
    }

    new
}

pub fn clean_fn_name(proper_name: &str, oid: &str, tag: &str) -> String {
    if proper_name == "GitHub" {
        return to_snake_case(oid).trim_start_matches('_').to_string();
    }

    let mut clean_name = "_".to_string();
    if proper_name.starts_with("Google") {
        clean_name = format!(
            "{}_",
            to_snake_case(
                proper_name
                    .replace("Google", "")
                    .replace("Admin", "Directory")
                    .trim()
            )
        );
    }

    let mut o = oid.to_string();
    if o == "listimmessages" {
        o = "list im messages".to_string();
    } else if o == "sendimmessages" {
        o = "send im messages".to_string();
    }

    let mut st = to_snake_case(&o)
        .replace("v_1_", "")
        .replace("_in_", "_")
        .replace("_id_", "_")
        .replace("_a_", "_")
        .replace("_to_", "_")
        .replace("_id_", "_")
        .replace("_with_", "_")
        .replace("_by_", "_")
        .replace("_v_2_", "_")
        .replace("_v_3_", "_")
        .replace("s_uuid", "")
        .replace("_id_or_uuid", "")
        .replace("_uuid", "")
        .replace("cloudresourcemanager_", "_")
        .replace("_csrs_csr", "_csr")
        .replace("_idps_idp", "_idp")
        .replace("shippingmethod_", "shipping_method_")
        .replace("companies_company_", "company_")
        .replace("employees_employee_", "employee_")
        .replace("jobs_job_", "job_")
        .replace("applicants_applicant_", "applicant_")
        .trim_start_matches('_')
        .trim_start_matches("in_")
        .trim_start_matches("by_")
        .trim_start_matches("with_")
        .trim_start_matches("to_")
        .trim_start_matches("a_")
        .trim_end_matches('_')
        .trim_end_matches("_a")
        .trim_end_matches("_in")
        .trim_end_matches("_id")
        .trim_end_matches("_by")
        .trim_end_matches("_with")
        .trim_end_matches("_to")
        .trim_start_matches(&clean_name)
        .to_string();

    if st.starts_with("post_") {
        // This should be singular.
        st = st.trim_end_matches('s').to_string();
    }

    if !st.contains("api_key") {
        st = st.replace("_api_", "_");
    }

    let mut words: Vec<String> = Default::default();
    // Only get a string with unique words.
    let split = st.split('_');
    for s in split {
        if words.contains(&s.to_string()) {
            continue;
        }
        words.push(s.to_string());
    }

    let mut f = words.join("_");

    if to_snake_case(tag) == f {
        return "get".to_string();
    }

    f = f
        .replace(&tag, "")
        .replace(&format!("_{}", tag.trim_end_matches('s')), "")
        .replace("_apps_app", "_app")
        .replace("__", "_")
        .trim_end_matches('_')
        .trim_end_matches("_s")
        .replace("_s_", "_")
        .trim_start_matches(&format!("{}_", tag.trim_end_matches('s')))
        .to_string();

    // Fix if we somehow created a function that is actually a keyword.
    if f == "move" {
        return "mv".to_string();
    } else if f == "return" {
        return "return_".to_string();
    }

    f
}

fn oid_to_object_name(s: &str) -> String {
    let cleaned = s
        .to_lowercase()
        .replace('.', "")
        .replace('_', " ")
        .replace(" an ", " ")
        .replace(" or ", " ")
        .replace(" for ", " ")
        .replace(" to ", " ")
        .replace(" your ", " ")
        .replace(" is ", " ")
        .replace(" and ", " ")
        .replace(" the ", " ")
        .replace("(beta)", "")
        .replace("(legacy)", "")
        .replace("-", " ")
        .replace(" a ", " ")
        .replace("'", "")
        .replace(" of ", " ")
        .replace("authenticated user", "")
        .replace("  ", " ")
        .trim()
        .to_string();

    cleaned
}

fn main() -> Result<()> {
    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.reqopt(
        "i",
        "",
        "OpenAPI definition document (JSON | YAML)",
        "INPUT",
    );
    opts.reqopt("o", "", "Generated Rust crate directory", "OUTPUT");
    opts.reqopt("n", "", "Target Rust crate name", "CRATE");
    opts.reqopt("v", "", "Target Rust crate version", "VERSION");
    opts.reqopt("d", "", "Target Rust crate description", "DESCRIPTION");
    opts.reqopt("", "host", "Target default host", "DEFAULT_HOST");
    opts.reqopt(
        "",
        "proper-name",
        "Target client proper name",
        "PROPER_NAME",
    );
    opts.reqopt("", "spec-link", "Link to the spec", "SPEC_LINK");
    opts.optopt(
        "",
        "token-endpoint",
        "Target token endpoint",
        "TOKEN_ENDPOINT",
    );
    opts.optopt(
        "",
        "user-consent-endpoint",
        "Target user consent endpoint",
        "USER_CONSENT_ENDPOINT",
    );
    opts.optopt(
        "",
        "add-post-header",
        "A header to add to post requests",
        "ADD_POST_HEADER",
    );
    opts.optflag("", "debug", "Print debug output");

    let args = match opts.parse(std::env::args().skip(1)) {
        Ok(args) => {
            if !args.free.is_empty() {
                eprintln!("{}", opts.usage("progenitor"));
                bail!("unexpected positional arguments");
            }
            args
        }
        Err(e) => {
            eprintln!("{}", opts.usage("progenitor"));
            bail!(e);
        }
    };

    let api = load_api(&args.opt_str("i").unwrap())?;

    let debug = |s: &str| {
        if args.opt_present("debug") {
            println!("{}", s);
        }
    };

    /*
     * Grab all the types defined by schemas and parameters.
     */
    let mut ts = TypeSpace::new();
    let mut parameters: BTreeMap<String, &openapiv3::Parameter> = BTreeMap::new();

    if let Some(components) = &api.components {
        // Populate a type to describe each entry in the schemas section.
        for (i, (sn, s)) in components.schemas.iter().enumerate() {
            let name = clean_name(sn);
            debug(&format!(
                "SCHEMA {}/{}: {}",
                i + 1,
                components.schemas.len(),
                name
            ));

            let id = ts.select(Some(name.as_str()), s, "")?;
            debug(&format!("    -> {:?}", id));
            debug("");

            // Insert the named type for our reference.
            // DO NOT CLEAN THE NAME HERE.
            ts.populate_ref(Some(sn.as_str()), Some(id), "schema")?;
        }

        // Populate a type to describe each entry in the parameters section.
        for (i, (pn, p)) in components.parameters.iter().enumerate() {
            let name = clean_name(pn);
            debug(&format!(
                "PARAMETER {}/{}: {}",
                i + 1,
                components.parameters.len(),
                name
            ));

            let id = ts.select_param(Some(name.as_str()), p)?;

            // Insert the named type for our reference.
            // DO NOT CLEAN THE NAME HERE.
            ts.populate_ref(Some(pn.as_str()), Some(id.clone()), "parameter")?;

            debug(&format!("    -> {:?}", id));
            debug("");
            if let openapiv3::ReferenceOr::Item(item) = p {
                parameters.insert(struct_name(&pn.to_string()), item);
            } else {
                bail!("parameter {} uses reference, unsupported: {:?}", pn, p);
            }
            debug("");
        }

        // Populate a type to describe each entry in the requestBodies section.
        for (i, (rn, r)) in components.request_bodies.iter().enumerate() {
            let name = clean_name(rn);
            debug(&format!(
                "REQUEST BODY {}/{}: {}",
                i + 1,
                components.request_bodies.len(),
                name
            ));

            let content = &r.item().unwrap().content;

            for (ct, mt) in content {
                // TODO: have a better way of handling multipart/form-data
                if ct == "application/json"
                    || ((ct == "multipart/form-data" || ct == "application/x-www-form-urlencoded")
                        && content.len() == 1)
                {
                    if let Some(s) = &mt.schema {
                        let object_name = format!("{} request", name);
                        let id = ts.select(Some(&clean_name(&object_name)), s, "")?;

                        // Insert the named type for our reference.
                        // DO NOT CLEAN THE NAME HERE.
                        ts.populate_ref(Some(rn.as_str()), Some(id.clone()), "requestBodies")?;

                        debug(&format!("    -> {:?}", id));
                        debug("");
                    }
                }
            }

            if content.is_empty() {
                // Populate an empty reference.
                ts.populate_ref(Some(rn.as_str()), None, "requestBodies")?;
            }
        }

        // Populate a type to describe each entry in the responses section.
        for (i, (rn, r)) in components.responses.iter().enumerate() {
            let name = clean_name(rn);
            debug(&format!(
                "RESPONSE {}/{}: {}",
                i + 1,
                components.responses.len(),
                name
            ));

            let content = &r.item().unwrap().content;

            for (ct, mt) in content {
                if ct == "application/json" {
                    if let Some(s) = &mt.schema {
                        if let Ok(item) = s.item() {
                            // We have an item, we want to check
                            // if its an ANY kind and empty, then we can ignore it.
                            if let openapiv3::SchemaKind::Any(any) = &item.schema_kind {
                                if any.properties.is_empty()
                                    && any.format.is_none()
                                    && any.items.is_none()
                                {
                                    // Continue early here.
                                    continue;
                                }
                            }
                        }

                        let object_name = format!("{} response", name);
                        let id = ts.select(Some(&clean_name(&object_name)), s, "")?;

                        // Insert the named type for our reference.
                        // DO NOT CLEAN THE NAME HERE.
                        ts.populate_ref(Some(rn.as_str()), Some(id.clone()), "response")?;

                        debug(&format!("    -> {:?}", id));
                        debug("");
                    }
                }
            }

            if content.is_empty() {
                // Populate an empty reference.
                ts.populate_ref(Some(rn.as_str()), None, "response")?;
            }
        }
    }

    /*
     * In addition to types defined in schemas, types may be defined inline in
     * request and response bodies.
     */
    let proper_name = args.opt_str("proper-name").unwrap();
    let mut tags: Vec<String> = Default::default();
    for (pn, p) in api.paths.iter() {
        let op = p.item()?;

        /*
         * Get the request parameters, those might have lingering enums.
         */
        for par in op.parameters.iter() {
            // The name will be filled in by the parameter data.
            ts.select_param(None, par)?;
        }

        let grab = |pn: &str,
                    m: &str,
                    o: Option<&openapiv3::Operation>,
                    ts: &mut TypeSpace|
         -> Result<String> {
            if let Some(o) = o {
                let op_id = if o.operation_id.is_none() {
                    // Make the operation id, the function.
                    path_to_operation_id(pn, m)
                } else {
                    o.operation_id.as_ref().unwrap().to_string()
                };
                let od = to_snake_case(&op_id);

                // Make sure we have exactly 1 tag. This likely needs to change in the
                // future but for now it seems fairly consistent.
                let mut tags = o.tags.clone();
                if tags.is_empty() {
                    // This "x-tags" bullshit is for Gusto.
                    if let Some(x) = o.extensions.get("x-tags") {
                        let xtags: Vec<String> = serde_json::from_value(x.clone()).unwrap();
                        tags = xtags;
                    }
                }
                if tags.is_empty() {
                    // If we still have no tags.... Oxide, parse it from
                    // the path.
                    let split = pn.trim_start_matches('/').split('/');
                    let vec = split.collect::<Vec<&str>>();

                    tags.push(vec.first().unwrap().to_string());
                }
                let tag = to_snake_case(&clean_name(&make_plural(
                    &proper_name,
                    tags.first().unwrap(),
                )));

                let oid = clean_fn_name(&proper_name, &od, &tag);

                debug("");
                debug(&oid);

                /*
                 * Get the request body type, if this operation has one.
                 */
                let mut req: Vec<String> = Default::default();
                if let Some(openapiv3::ReferenceOr::Item(body)) = &o.request_body {
                    for (ct, mt) in &body.content {
                        if ct == "application/json" {
                            if let Some(s) = &mt.schema {
                                let object_name = format!("{} request", oid_to_object_name(&od));
                                let id = ts.select(Some(&object_name), s, "")?;
                                let rt = ts.render_type(&id, true)?;
                                req.push(format!("{} {:?}", rt, id));
                            }
                        } else {
                            req.push(ct.to_string());
                        }
                    }
                } else if let Some(openapiv3::ReferenceOr::Reference { reference }) =
                    &o.request_body
                {
                    let id = ts.select_ref(None, reference.as_str())?;
                    req.push(format!("{:?}", id));
                }
                if !req.is_empty() {
                    debug(&format!(
                        "\t{} {} request body -> {}",
                        pn,
                        m,
                        req.join(" | ")
                    ));
                }

                /*
                 * Get the request parameters, those might have lingering enums.
                 */
                for par in o.parameters.iter() {
                    // The name will be filled in by the parameter data.
                    ts.select_param(Some(&oid_to_object_name(&od)), par)?;
                }

                /*
                 * Get the response body type for each status code:
                 */
                let mut res: Vec<String> = Default::default();
                for (_, r) in o.responses.responses.iter() {
                    match r {
                        openapiv3::ReferenceOr::Item(ri) => {
                            for (ct, mt) in &ri.content {
                                if ct == "application/json" {
                                    if let Some(s) = &mt.schema {
                                        if let Ok(item) = s.item() {
                                            // We have an item, we want to check
                                            // if its an ANY kind and empty, then we can ignore it.
                                            if let openapiv3::SchemaKind::Any(any) =
                                                &item.schema_kind
                                            {
                                                if any.properties.is_empty()
                                                    && any.format.is_none()
                                                    && any.items.is_none()
                                                {
                                                    // Continue early here.
                                                    continue;
                                                }
                                            }
                                        }

                                        // Otherwise we can parse the object.
                                        let object_name =
                                            format!("{} response", oid_to_object_name(&od));
                                        let id =
                                            ts.select(Some(&clean_name(&object_name)), s, "")?;
                                        let rt = ts.render_type(&id, false)?;
                                        res.push(format!("{} {:?}", rt, id));
                                    }
                                } else {
                                    res.push(ct.to_string());
                                }
                            }
                        }
                        openapiv3::ReferenceOr::Reference { reference } => {
                            let id = ts.select_ref(None, reference.as_str())?;
                            res.push(format!("{:?}", id));
                        }
                    }
                }
                debug(&format!(
                    "\t{} {} response body -> {}",
                    pn,
                    m,
                    res.join(" | ")
                ));

                return Ok(tag);
            }

            Ok("".to_string())
        };

        tags.push(grab(pn, "GET", op.get.as_ref(), &mut ts)?);
        tags.push(grab(pn, "POST", op.post.as_ref(), &mut ts)?);
        tags.push(grab(pn, "PUT", op.put.as_ref(), &mut ts)?);
        tags.push(grab(pn, "DELETE", op.delete.as_ref(), &mut ts)?);
        tags.push(grab(pn, "OPTIONS", op.options.as_ref(), &mut ts)?);
        tags.push(grab(pn, "HEAD", op.head.as_ref(), &mut ts)?);
        tags.push(grab(pn, "PATCH", op.patch.as_ref(), &mut ts)?);
        tags.push(grab(pn, "TRACE", op.trace.as_ref(), &mut ts)?);
    }
    debug("");

    let name = args.opt_str("n").unwrap();
    let version = args.opt_str("v").unwrap();
    let host = args.opt_str("host").unwrap();
    let output_dir = args.opt_str("o").unwrap();
    let spec_link = args.opt_str("spec-link").unwrap();
    let token_endpoint = if let Some(te) = args.opt_str("token-endpoint") {
        te
    } else {
        String::new()
    };
    let user_consent_endpoint = if let Some(uce) = args.opt_str("user-consent-endpoint") {
        uce
    } else {
        String::new()
    };

    // Sort our tags and de-duplicate them.
    tags.sort_unstable();
    tags.dedup();

    let add_post_header = if let Some(ph) = args.opt_str("add-post-header") {
        ph
    } else {
        String::new()
    };
    let fail = match gen(
        &api,
        &proper_name,
        &host,
        tags,
        &token_endpoint,
        &user_consent_endpoint,
        &add_post_header,
    ) {
        Ok(out) => {
            let description = args.opt_str("d").unwrap();

            /*
             * Create the top-level crate directory:
             */
            let root = PathBuf::from(&output_dir);
            std::fs::create_dir_all(&root)?;

            /*
             * Write the Cargo.toml file:
             */
            let mut uuid_lib = "".to_string();
            let mut yup_oauth2_lib = "".to_string();
            if proper_name != "GitHub" {
                uuid_lib = r#"
bytes = { version = "1", features = ["serde"] }
async-trait = "^0.1.51"
urlencoding = "^1.3.3"
uuid = { version = "^0.8", features = ["serde", "v4"] }"#
                    .to_string();
            }

            if proper_name.starts_with("Google") {
                yup_oauth2_lib = r#"
base64 = "^0.12"
yup-oauth2 = "^5""#
                    .to_string();
            }

            let mut toml = root.clone();
            toml.push("Cargo.toml");
            let tomlout = format!(
                r#"[package]
name = "{}"
description = "{}"
version = "{}"
documentation = "https://docs.rs/{}/"
repository = "https://github.com/oxidecomputer/third-party-api-clients/tree/main/{}"
readme = "README.md"
edition = "2018"
license = "MIT"

[dependencies]
anyhow = "1"
async-recursion = "^0.3.2"
chrono = {{ version = "0.4", features = ["serde"] }}
dirs = {{ version = "^3.0.2", optional = true }}
http = "^0.2.4"
hyperx = "1"
jsonwebtoken = "7"
log = {{ version = "^0.4", features = ["serde"] }}
mime = "0.3"
percent-encoding = "2.1"
reqwest = {{ version = "0.11", features = ["json", "multipart"] }}
schemars = {{ version = "0.8", features = ["bytes", "chrono", "url", "uuid"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
serde_urlencoded = "^0.7"
url = {{ version = "2", features = ["serde"] }}{}{}

[dev-dependencies]
base64 = "^0.12"
dirs = "^3.0.2"
nom_pem = "4"
tokio = {{ version = "1.8.0", features = ["full"] }}

[features]
# enable etag-based http_cache functionality
httpcache = ["dirs"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
"#,
                name, description, version, name, output_dir, uuid_lib, yup_oauth2_lib
            );
            save(&toml, tomlout.as_str())?;

            /*
             * Generate our documentation for the library.
             */
            let docs = if proper_name == "GitHub" {
                template::generate_docs_github(
                    &api,
                    &to_snake_case(&name),
                    &version,
                    &proper_name,
                    host.trim_start_matches("https://"),
                    &spec_link,
                )
            } else if proper_name == "SendGrid"
                || proper_name == "Giphy"
                || proper_name == "Rev.ai"
                || proper_name == "Okta"
            {
                template::generate_docs_generic_api_key(
                    &api,
                    &to_snake_case(&name),
                    &version,
                    &proper_name,
                    &spec_link,
                )
            } else if proper_name == "TripActions" {
                template::generate_docs_generic_client_credentials(
                    &api,
                    &to_snake_case(&name),
                    &version,
                    &proper_name,
                    &spec_link,
                )
            } else {
                template::generate_docs_generic_token(
                    &api,
                    &to_snake_case(&name),
                    &version,
                    &proper_name,
                    &spec_link,
                    &add_post_header,
                )
            };
            let mut readme = root.clone();
            readme.push("README.md");
            save(
                readme,
                // Add a title to the README.md so it looks nicer in GitHub.
                &format!(
                    "# `{}`\n\n{}",
                    name,
                    docs.replace("//! ", "").replace("//!", "").as_str()
                ),
            )?;

            /*
             * Create the src/ directory:
             */
            let mut src = root;
            src.push("src");
            std::fs::create_dir_all(&src)?;

            /*
             * Create the Rust source file containing the generated client:
             */
            let lib = format!("{}\n{}", docs, out);
            let mut librs = src.clone();
            librs.push("lib.rs");
            save(librs, lib.as_str())?;

            /*
             * Create the Rust utils module:
             */
            let utils = utils::generate_utils(&proper_name);
            let mut utilsrs = src.clone();
            utilsrs.push("utils.rs");
            save(utilsrs, utils.as_str())?;

            /*
             * Create the Rust source types file containing the generated types:
             */
            let types = types::generate_types(&mut ts, &proper_name)?;
            let mut typesrs = src.clone();
            typesrs.push("types.rs");
            save(typesrs, types.as_str())?;

            /*
             * Create the Rust source files for each of the tags functions:
             */
            let fail = match functions::generate_files(&api, &proper_name, &mut ts, &parameters) {
                Ok(files) => {
                    // We have a map of our files, let's write to them.
                    for (f, content) in files {
                        let mut tagrs = src.clone();
                        tagrs.push(format!("{}.rs", to_snake_case(&clean_name(&f))));

                        let output = format!(
                            r#"use anyhow::Result;

use crate::Client;

pub struct {} {{
    pub client: Client,
}}

impl {} {{
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {{
        {} {{
            client,
        }}
    }}

    {}
}}"#,
                            struct_name(&f),
                            struct_name(&f),
                            struct_name(&f),
                            content,
                        );
                        save(tagrs, output.as_str())?;
                    }

                    false
                }
                Err(e) => {
                    println!("generate_files fail: {:?}", e);
                    true
                }
            };

            fail
        }
        Err(e) => {
            println!("gen fail: {:?}", e);
            true
        }
    };

    debug("-----------------------------------------------------");
    debug(" TYPE SPACE");
    debug("-----------------------------------------------------");
    for te in ts.id_to_entry.values() {
        let n = ts.describe(&te.id);
        debug(&format!("{:>4}  {}", te.id.0, n));
    }
    debug("-----------------------------------------------------");
    debug("");

    if fail {
        bail!("generation experienced errors");
    }

    Ok(())
}
