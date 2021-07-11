mod template;

use std::collections::{BTreeMap, HashSet};
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use http::status::StatusCode;
use inflector::cases::{snakecase::to_snake_case, titlecase::to_title_case};
use openapiv3::OpenAPI;
use serde::Deserialize;

fn save<P>(p: P, data: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let p = p.as_ref();
    let mut f = OpenOptions::new().create(true).truncate(true).write(true).open(p)?;
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

    if !api.tags.is_empty() {
        println!("tags not presently supported");
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
         * Explicitly allowing "schemas" through.
         */
    }

    /*
     * XXX Ignoring "external_docs" and "extensions" for now, as they seem not
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
                                println!("op {}: security, unsupported", oid);
                            }

                            if o.responses.default.is_some() {
                                bail!("op {}: has response default", oid);
                            }
                        } else {
                            bail!("path {} is missing operation ID", p.0);
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
    fn render_type(&self, name: &str) -> Result<String>;
}

impl ParameterDataExt for openapiv3::ParameterData {
    fn render_type(&self, name: &str) -> Result<String> {
        use openapiv3::{SchemaKind, Type};

        Ok(match &self.format {
            openapiv3::ParameterSchemaOrContent::Schema(s) => {
                if let Ok(s) = s.item() {
                    match &s.schema_kind {
                        SchemaKind::Type(Type::Boolean {}) => "bool".to_string(),
                        SchemaKind::Type(Type::Array(at)) => {
                            // TODO: actually get the array type.
                            println!("XXX array type: {:?}", at);
                            "&[String]".to_string()
                        }
                        SchemaKind::Type(Type::String(st)) => {
                            use openapiv3::{
                                StringFormat::Date,
                                StringFormat::DateTime,
                                VariantOrUnknownOrEmpty::{Empty, Item, Unknown},
                            };

                            if st.pattern.is_some() {
                                bail!("XXX pattern");
                            }

                            if !st.enumeration.is_empty() {
                                if name.is_empty() {
                                    // TODO: fix this.
                                    println!("parameter that enumerates should have a pre-defined type: {:?}", st);
                                } else {
                                    // We have an enum.
                                    // Let's return the correct enum struct name.
                                    let mut sn = struct_name(name);
                                    // TODO: have a more automated way of making sure there aren't
                                    // duplicates of enums.
                                    if sn == "Status" {
                                        sn = format!("{}Param", sn);
                                    }

                                    return Ok(format!("crate::types::{}", sn));
                                }
                            }

                            if st.min_length.is_some() || st.max_length.is_some() {
                                bail!("XXX min/max length");
                            }

                            match &st.format {
                                Item(DateTime) => "DateTime<Utc>".to_string(),
                                Item(Date) => "NaiveDate".to_string(),
                                Empty => "&str".to_string(),
                                Unknown(f) => match f.as_str() {
                                    "float" => "f64".to_string(),
                                    "uri" => "&str".to_string(),
                                    "uri-template" => "&str".to_string(),
                                    "email" => "&str".to_string(),
                                    f => {
                                        bail!("XXX unknown string format {}", f)
                                    }
                                },
                                x => {
                                    bail!("XXX string format {:?}", x);
                                }
                            }
                        }
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
                                    bail!("XXX invalid minimum: {}", min);
                                }
                            }

                            if it.maximum.is_some() {
                                bail!("XXX maximum");
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
                        openapiv3::SchemaKind::OneOf { one_of: _ } => {
                            // TODO: make this smarter, but for now just make it a string.
                            println!("oneof parameter: {:?}", self);
                            "&str".to_string()
                        }
                        x => bail!("unexpected type {:#?}", x),
                    }
                } else {
                    // We have a reference to a type. We could handle it, but for now
                    // easier to return a string.
                    // TODO: handle in the future.
                    "&str".to_string()
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
            bail!("could not find application/json, only found {}", self.content.keys().next().unwrap());
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

                let s = s.item()?;
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
            bail!("could not find application/json, only found {}", self.content.keys().next().unwrap());
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

                let s = s.item()?;
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
#[derive(Debug, PartialEq, Clone)]
enum TypeDetails {
    Unknown,
    Basic,
    NamedType(TypeId, openapiv3::SchemaData),
    Enum(Vec<String>, openapiv3::SchemaData),
    Array(TypeId, openapiv3::SchemaData),
    Optional(TypeId, openapiv3::SchemaData),
    /*
     * Object property names are sorted lexicographically to ensure a stable
     * order in the generated code.
     */
    Object(BTreeMap<String, TypeId>, openapiv3::SchemaData),
}

#[derive(Debug)]
struct TypeEntry {
    id: TypeId,
    name: Option<String>,
    details: TypeDetails,
}

#[derive(Debug, Eq, Clone)]
struct TypeId(u64);

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

#[derive(Debug)]
struct TypeSpace {
    next_id: u64,
    /*
     * Object types generally have a useful name, which we would like to match
     * with anywhere that name appears in the definition document.  Many other
     * types, though, do not; e.g., an array of strings is just going to become
     * Vec<String> without necesssarily having a useful distinct type name.
     */
    name_to_id: BTreeMap<String, TypeId>,
    id_to_entry: BTreeMap<TypeId, TypeEntry>,

    import_chrono: bool,
}

impl TypeSpace {
    fn new() -> TypeSpace {
        TypeSpace {
            next_id: 1,
            name_to_id: BTreeMap::new(),
            id_to_entry: BTreeMap::new(),
            import_chrono: false,
        }
    }

    /**
     * Emit a human-readable diagnostic description for this type ID.
     */
    fn describe(&self, tid: &TypeId) -> String {
        if let Some(te) = self.id_to_entry.get(tid) {
            match &te.details {
                TypeDetails::Basic => {
                    if let Some(n) = &te.name {
                        n.to_string()
                    } else {
                        format!("[BASIC {} !NONAME?]", tid.0)
                    }
                }
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
                TypeDetails::Unknown => {
                    format!("[UNKNOWN {}]", tid.0)
                }
            }
        } else {
            format!("[UNMAPPED {}]", tid.0)
        }
    }

    fn render_type(&self, tid: &TypeId, in_mod: bool) -> Result<String> {
        if let Some(te) = self.id_to_entry.get(tid) {
            match &te.details {
                TypeDetails::Basic => {
                    if let Some(n) = &te.name {
                        Ok(n.to_string())
                    } else {
                        bail!("basic type {:?} does not have a name?", tid);
                    }
                }
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
                            Ok(format!("types::{}", struct_name))
                        }
                    } else {
                        bail!("enum type {:?} does not have a name?", tid);
                    }
                }
                TypeDetails::Array(itid, _) => Ok(format!("Vec<{}>", self.render_type(itid, in_mod)?)),
                TypeDetails::Optional(itid, _) => {
                    let rt = self.render_type(itid, in_mod)?;
                    if rt == "String" || rt.starts_with("Vec<") {
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
                            Ok(format!("types::{}", struct_name))
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
            bail!("could not resolve type ID {:?}", tid);
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

    fn id_for_optional(&mut self, want: &TypeId, sd: openapiv3::SchemaData) -> TypeId {
        for (oid, oent) in self.id_to_entry.iter() {
            match &oent.details {
                TypeDetails::Optional(id, schema_data) if id == want && *schema_data == sd => return oid.clone(),
                _ => continue,
            }
        }

        let oid = self.assign();
        self.id_to_entry.insert(
            oid.clone(),
            TypeEntry {
                id: oid.clone(),
                name: None,
                details: TypeDetails::Optional(want.clone(), sd),
            },
        );
        oid
    }

    fn select_ref(&mut self, _name: Option<&str>, reference: &str) -> Result<TypeId> {
        let r = clean_name(&reference.replace("#/components/schemas/", ""));

        /*
         * As this is a reference, all we can do for now is determine
         * the type ID.
         */
        Ok(if let Some(id) = self.name_to_id.get(&r) {
            // we got the id.
            id.clone()
        } else {
            let id = self.assign();
            self.name_to_id.insert(r.to_string(), id.clone());
            id
        })
    }

    fn select_schema(&mut self, name: Option<&str>, s: &openapiv3::Schema, parent_name: &str, is_schema: bool) -> Result<TypeId> {
        let nam = if let Some(n) = name { n.to_string() } else { "".to_string() };

        // TODO: refactor this function somehow.
        /*if is_schema {
            // If we are on a parent schema that as been defined in the spec, we want to ensure,
            // that type gets a named type. This is so that we can find all these types later on, even
            // arrays and basic types like numbers and strings.
            if let openapiv3::SchemaKind::Type(t) = s.schema_kind {
                if let openapiv3::Type::Object(_) = t {
                    // If it's an object we will always have a named type so it's fine.
                } else {
                    // For everything else ensure we have a named type.
                    let id = self.select_schema(Some(&nam), s, parent_name, false)?;
                    (Some(nam), TypeDetails::NamedType(id))
                }
            }
        }*/

        let (name, details) = match &s.schema_kind {
            openapiv3::SchemaKind::Type(t) => match t {
                openapiv3::Type::Array(at) => {
                    if is_schema {
                        let id = self.select_schema(Some(&nam), s, parent_name, false)?;
                        (Some(nam), TypeDetails::NamedType(id, s.schema_data.clone()))
                    } else {
                        // Determine the type of item that will be in this array.
                        let itid = self.select_box(Some(&clean_name(&nam)), &at.items, parent_name)?;
                        (None, TypeDetails::Array(itid, s.schema_data.clone()))
                    }
                }
                openapiv3::Type::Object(o) => {
                    // Object types must have a consistent name.
                    let name = clean_name(match (name, s.schema_data.title.as_deref()) {
                        (Some(n), None) => n,
                        (Some(n), Some("")) => n,
                        (None, Some(t)) => t,
                        (Some(""), Some(t)) => t,
                        (Some(n), Some(_)) => n,
                        (None, None) => {
                            bail!("types need a name? {:?} {:?}", name, s)
                        }
                    });

                    let mut omap = BTreeMap::new();
                    for (n, rb) in o.properties.iter() {
                        let itid = self.select_box(Some(n), rb, &format!("{} {} {}", parent_name, name, clean_name(n)))?;
                        if o.required.contains(n) {
                            omap.insert(n.to_string(), itid);
                        } else {
                            // This is an optional member.
                            omap.insert(n.to_string(), self.id_for_optional(&itid, s.schema_data.clone()));
                        }
                    }
                    (Some(name), TypeDetails::Object(omap, s.schema_data.clone()))
                }
                openapiv3::Type::String(st) => {
                    use openapiv3::{
                        StringFormat::Date,
                        StringFormat::DateTime,
                        VariantOrUnknownOrEmpty::{Empty, Item, Unknown},
                    };

                    if !st.enumeration.is_empty() {
                        // We have an enumeration.
                        (Some(nam), TypeDetails::Enum(st.enumeration.clone(), s.schema_data.clone()))
                    } else if is_schema {
                        let id = self.select_schema(Some(&nam), s, parent_name, false)?;
                        (Some(nam), TypeDetails::NamedType(id, s.schema_data.clone()))
                    } else {
                        match &st.format {
                            Item(DateTime) => {
                                self.import_chrono = true;
                                (Some("DateTime<Utc>".to_string()), TypeDetails::Basic)
                            }
                            Item(Date) => {
                                self.import_chrono = true;
                                (Some("NaiveDate".to_string()), TypeDetails::Basic)
                            }
                            Empty => (Some("String".to_string()), TypeDetails::Basic),
                            Unknown(f) => match f.as_str() {
                                "float" => (Some("f64".to_string()), TypeDetails::Basic),
                                "uri" => (Some("String".to_string()), TypeDetails::Basic),
                                "uri-template" => (Some("String".to_string()), TypeDetails::Basic),
                                "email" => (Some("String".to_string()), TypeDetails::Basic),
                                f => bail!("XXX unknown string format {}", f),
                            },
                            x => {
                                bail!("XXX string format {:?}", x);
                            }
                        }
                    }
                }
                openapiv3::Type::Boolean {} => {
                    if is_schema {
                        let id = self.select_schema(Some(&nam), s, parent_name, false)?;
                        (Some(nam), TypeDetails::NamedType(id, s.schema_data.clone()))
                    } else {
                        (Some("bool".to_string()), TypeDetails::Basic)
                    }
                }
                openapiv3::Type::Number(_) => {
                    if is_schema {
                        let id = self.select_schema(Some(&nam), s, parent_name, false)?;
                        (Some(nam), TypeDetails::NamedType(id, s.schema_data.clone()))
                    } else {
                        (Some("f64".to_string()), TypeDetails::Basic)
                    }
                }
                openapiv3::Type::Integer(_) => {
                    if is_schema {
                        let id = self.select_schema(Some(&nam), s, parent_name, false)?;
                        (Some(nam), TypeDetails::NamedType(id, s.schema_data.clone()))
                    } else {
                        (Some("i64".to_string()), TypeDetails::Basic)
                    }
                }
            },
            openapiv3::SchemaKind::AllOf { all_of } => {
                // TODO: Actually combine all the types.
                let id = self.select(name, all_of.get(0).unwrap(), is_schema)?;
                if let Some(et) = self.id_to_entry.get(&id) {
                    if let Some(n) = name {
                        if let TypeDetails::Object(..) = et.details {
                            (Some(clean_name(n)), et.details.clone())
                        } else {
                            (Some(n.to_string()), et.details.clone())
                        }
                    } else {
                        bail!("all_of types need a name? {:?} {:?}", name, all_of)
                    }
                } else {
                    bail!("allof schema kind: {:?} {:?} {:?}", id, name, s,);
                }
            }
            openapiv3::SchemaKind::OneOf { one_of } => {
                // Iterate over each one of an select the first one that is not
                // an empty object.
                let mut id = TypeId(0);
                for o in one_of {
                    if let Ok(i) = self.select(name, o, true) {
                        id = i;
                        break;
                    }
                }

                if let Some(et) = self.id_to_entry.get(&id) {
                    if let Some(n) = name {
                        (Some(n.to_string()), et.details.clone())
                    } else {
                        bail!("one_of types need a name? {:?} {:?}", name, one_of)
                    }
                } else {
                    bail!("oneof schema kind: {:?} {:?}\n{:?}", name, s, one_of);
                }
            }
            openapiv3::SchemaKind::AnyOf { any_of } => {
                // TODO: Actually combine all the types.
                let id = self.select(name, any_of.get(0).unwrap(), is_schema)?;
                if let Some(et) = self.id_to_entry.get(&id) {
                    if let Some(n) = name {
                        (Some(n.to_string()), et.details.clone())
                    } else {
                        bail!("any_of types need a name? {:?} {:?}", name, any_of)
                    }
                } else {
                    bail!("anyof schema kind: {:?} {:?}\n{:?}", name, s, any_of);
                }
            }
            openapiv3::SchemaKind::Any(_a) => {
                // Then we use the serde_json type.
                if is_schema {
                    let id = self.select_schema(Some(&nam), s, parent_name, false)?;
                    (Some(nam), TypeDetails::NamedType(id, s.schema_data.clone()))
                } else {
                    (Some("serde_json::Value".to_string()), TypeDetails::Basic)
                }
            }
        };

        if let Some(name) = &name {
            /*
             * First, determine what ID we will use to identify this named type.
             */
            let id = self.id_for_name(name.as_str());

            /*
             * If there is already an entry for this type ID, ensure that it
             * matches the entry we have constructed.  If there is not yet an
             * entry, we can just keep this one.
             */
            if let Some(et) = self.id_to_entry.get(&id) {
                if et.details != details {
                    // We can get here if there are two objects with the same name
                    // that have properties that are different.
                    // Let's check if we already have an object with the parent name.
                    let mut pn = parent_name.to_string();
                    if parent_name.is_empty() {
                        // Append "Data" to the name.
                        pn = format!("{} Data", name);
                    }
                    let parent_id = self.id_for_name(&pn);
                    if let Some(pet) = self.id_to_entry.get(&parent_id) {
                        // We already have an item with the parent name!
                        if pet.details != details {
                            // We can get here if there are two objects with the same name
                            // that have properties that are different.
                            // Let's rename the new object with the parent name.
                            println!("object details for {} do not match: {:?} != {:?}", pn, pet.details, details,);
                        }
                    } else {
                        // Let's rename the new object with the parent name.
                        // Insert the new one with the parent name.
                        self.id_to_entry.insert(
                            parent_id.clone(),
                            TypeEntry {
                                id: parent_id.clone(),
                                name: Some(pn),
                                details,
                            },
                        );
                    }
                }
            } else {
                self.id_to_entry.insert(
                    id.clone(),
                    TypeEntry {
                        id: id.clone(),
                        name: Some(name.clone()),
                        details,
                    },
                );
            }

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

    fn select(&mut self, name: Option<&str>, s: &openapiv3::ReferenceOr<openapiv3::Schema>, is_schema: bool) -> Result<TypeId> {
        match s {
            openapiv3::ReferenceOr::Reference { reference } => self.select_ref(name, reference.as_str()),
            openapiv3::ReferenceOr::Item(s) => self.select_schema(name, s, "", is_schema),
        }
    }

    fn select_box(&mut self, name: Option<&str>, s: &openapiv3::ReferenceOr<Box<openapiv3::Schema>>, parent_name: &str) -> Result<TypeId> {
        match s {
            openapiv3::ReferenceOr::Reference { reference } => self.select_ref(name, reference.as_str()),
            openapiv3::ReferenceOr::Item(s) => self.select_schema(name, s.as_ref(), parent_name, false),
        }
    }
}

fn get_parameter_data(param: &openapiv3::Parameter) -> Option<&openapiv3::ParameterData> {
    match param {
        openapiv3::Parameter::Path {
            parameter_data,
            style: openapiv3::PathStyle::Simple,
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

fn render_raw_param(n: &str, param: &openapiv3::Parameter) -> String {
    if let Some(parameter_data) = get_parameter_data(param) {
        if let openapiv3::ParameterSchemaOrContent::Schema(s) = &parameter_data.format {
            if let Ok(s) = s.item() {
                if let openapiv3::SchemaKind::Type(openapiv3::Type::String(st)) = &s.schema_kind {
                    let mut desc = "".to_string();
                    if let Some(d) = &parameter_data.description {
                        desc = d.to_string();
                    }
                    return render_param(n, &st.enumeration.clone(), parameter_data.required, &desc);
                }
            }
        }
    }

    "".to_string()
}

fn render_param(n: &str, enums: &[String], required: bool, description: &str) -> String {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    if enums.is_empty() {
        return out.to_string();
    }

    if !description.is_empty() {
        a(&format!("/// {}", description.replace('\n', "\n/// ")));
    }

    let mut sn = struct_name(n);
    // TODO: have a more automated way of making sure there aren't
    // duplicates of enums.
    /*if sn == "Status" {
        sn = format!("{}Param", sn);
    }*/

    a("#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]");
    a(r#"#[serde(rename_all = "snake_case")]"#);
    a(&format!("pub enum {} {{", sn));
    for e in enums {
        if struct_name(e).is_empty() {
            // TODO: do something for empty(?)
            continue;
        }
        a(&format!("{},", struct_name(e)));
    }
    if !required {
        a("Noop,");
    }
    a("}");
    a("");

    a(&format!("impl std::fmt::Display for {} {{", sn));
    a(r#"fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {"#);
    a(r#"match *self {"#);
    for e in enums {
        if struct_name(e).is_empty() {
            // TODO: do something for empty(?)
            continue;
        }
        a(&format!(r#"{}::{} => "{}","#, sn, struct_name(e), e));
    }
    if !required {
        a(&format!(r#"{}::Noop => "","#, sn,));
    }
    a("}");
    a(".fmt(f)");
    a("}");
    a("}");
    a("");

    // Add a default for the enum if it is not required.
    // TODO: use the default that can be passed to the OpenAPI,
    // github is not using that currently but we might want to
    // in the future.
    if !required {
        a(&format!("impl Default for {} {{", sn));
        a(&format!("fn default() -> {} {{", sn));
        a(&format!("{}::Noop", sn));
        a("}");
        a("}");
    }

    out.to_string()
}

fn gen(api: &OpenAPI, ts: &mut TypeSpace, parameters: BTreeMap<String, &openapiv3::Parameter>, n: &str, version: &str) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    /*
     * Deal with any dependencies we require to produce this client.
     */
    a(&format!(
        r#"//! A fully generated, opinionated API client library for GitHub.
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
//! {} = "{}"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of `auth::Credentials`.
//!
//! ```
//! use {}::{{auth::Credentials, Client}};
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
//! [Client#host](struct.Client.html#method.host) method.
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
//! {} = {{ version = "{}", features = ["httpcache"] }}
//! ```
//!
//! Then use the `Client::custom` constructor to provide a cache implementation.
//!
//! Here is an example:
//!
//! ```
//! use {}::{{auth::Credentials, Client}};
//! #[cfg(feature = "httpcache")]
//! use {}::http_cache::HttpCache;
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
//! use {}::{{Client, auth::{{Credentials, InstallationTokenGenerator, JWTCredentials}}}};
//! #[cfg(feature = "httpcache")]
//! use {}::http_cache::FileBasedCache;
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
//! #[cfg(feature = "httpcache")]
//! {{
//!     // Create the HTTP cache.
//!     let mut dir = dirs::home_dir().expect("Expected a home dir");
//!     dir.push(".cache/github");
//!     let http_cache = Box::new(FileBasedCache::new(dir));
//! }}
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
//!"#,
        n, version, n, n, version, n, n, n, n
    ));
    a("#![feature(async_stream)]");
    a("#![allow(clippy::too_many_arguments)]");
    a("#![allow(missing_docs)]"); // TODO: Make this a deny.
    a("");
    a("pub mod auth;");
    a(r#"#[cfg(feature = "httpcache")]"#);
    a("pub mod http_cache;");
    a("#[doc(hidden)]");
    a("pub mod utils;");
    a("");

    a("use anyhow::{anyhow, Error, Result};");
    a("use async_recursion::async_recursion;");
    a("use chrono::{DateTime, Utc};");
    a("");

    a(r#"const DEFAULT_HOST: &str = "https://api.github.com";"#);
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
    a("    pub(crate) fn encode_path(pc: &str) -> String {");
    a("        utf8_percent_encode(pc, PATH_SET).to_string()");
    a("    }");
    a("}");
    a("");

    /*
     * Declare named types we know about:
     */
    a("/// The data types sent to and returned from the API client.");
    a("pub mod types {");
    a("    use chrono::{DateTime, Utc, NaiveDate};");
    a("    use schemars::JsonSchema;");
    a("    use serde::{Serialize, Deserialize};");
    a("");

    // Let's iterate over the parameters and create all the enums.
    for (name, param) in &parameters {
        let p = render_raw_param(name.as_str(), &(*param).clone());
        a(&p);
    }

    for te in ts.id_to_entry.values() {
        if let Some(sn) = te.name.as_deref() {
            let sn = struct_name(sn);
            match &te.details {
                TypeDetails::Enum(vals, schema_data) => {
                    // Check if it already exists in params and skip it if so.
                    if let Some(t) = parameters.get(&sn) {
                        if let Some(parameter_data) = get_parameter_data(t) {
                            if let openapiv3::ParameterSchemaOrContent::Schema(s) = &parameter_data.format {
                                if let Ok(s) = s.item() {
                                    if let openapiv3::SchemaKind::Type(openapiv3::Type::String(st)) = &s.schema_kind {
                                        if st.enumeration == *vals {
                                            println!("enum {} already exists:\n\texisting -> {:?}\n\tnew -> {:?}", sn, st.enumeration, vals);
                                            continue;
                                        } else {
                                            bail!("enum {} already exists:\n\texisting -> {:?}\n\tnew -> {:?}", sn, st.enumeration, vals);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let mut desc = "".to_string();
                    if let Some(d) = &schema_data.description {
                        desc = d.to_string();
                    }
                    let p = render_param(sn.as_str(), vals, false, &desc);
                    a(&p);
                }
                TypeDetails::Object(omap, _) => {
                    a("#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]");
                    a(&format!("pub struct {} {{", sn));
                    for (name, tid) in omap.iter() {
                        if let Ok(rt) = ts.render_type(tid, true) {
                            let mut prop = name.to_string();
                            if name == "ref" || name == "type" || name == "self" {
                                prop = format!("{}_", name);
                            } else if name == "$ref" {
                                prop = format!("{}_", name.replace('$', ""));
                            } else if name == "+1" {
                                prop = "plus_one".to_string()
                            } else if name == "-1" {
                                prop = "minus_one".to_string()
                            } else if name.starts_with('@') {
                                prop = name.replace('@', "");
                            }

                            if rt == "String" || rt.starts_with("Vec<") || rt.starts_with("Option<") {
                                a(r#"#[serde(default,"#);
                                if rt == "String" {
                                    a(
                                        r#"skip_serializing_if = "String::is_empty", deserialize_with = "crate::utils::deserialize_null_string::deserialize","#,
                                    );
                                } else if rt.starts_with("Vec<") {
                                    a(r#"skip_serializing_if = "Vec::is_empty","#);
                                } else if rt.starts_with("Option<") {
                                    a(r#"skip_serializing_if = "Option::is_none","#);
                                }

                                if *name != prop {
                                    a(&format!(r#"rename = "{}")]"#, name));
                                } else {
                                    a(r#")]"#);
                                }
                            } else if *name != prop {
                                a(&format!(r#"#[serde(rename = "{}")]"#, name));
                            }

                            if !prop.ends_with('_') {
                                prop = to_snake_case(&prop);
                            }
                            a(&format!("pub {}: {},", prop, rt));
                        } else {
                            bail!("rendering type {} {:?} failed", name, tid);
                        }
                    }
                    a("}");
                    a("");
                }
                TypeDetails::Basic => {}
                TypeDetails::Unknown => {}
                TypeDetails::NamedType(..) => {}
                TypeDetails::Array(..) => {}
                TypeDetails::Optional(..) => {}
            }
        }
    }

    a("}");
    a("");

    /*
     * Declare the client object:
     */
    a("/// Entrypoint for interacting with the API client.");
    a(r#"pub struct Client {
        host: String,
        agent: String,
        client: reqwest::Client,
        credentials: Option<crate::auth::Credentials>,
        #[cfg(feature = "httpcache")]
        http_cache: crate::http_cache::BoxedHttpCache,
    }"#);
    a("");

    a(r#"impl Client {
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

                    let token = self.apps_create_installation_access_token(apptoken.installation_id as i64,
                    &types::CreateInstallationAccessTokenAppRequest{
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
        println!("Request: {:?}", &req);
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
            println!("response payload {}",
                String::from_utf8_lossy(&response_body)
            );
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
            println!("error status: {:?}, response payload: {}",
                status,
                String::from_utf8_lossy(&response_body),
            );
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
        items.reverse();
        while !items.is_empty() {
            match items.pop() {
                Some(item) => global_items.push(item),
                // We need to get the next link.
                None => if let Some(url) = link.as_ref().and_then(|l| crate::utils::next_link(l)) {
                    let url = reqwest::Url::parse(&url).unwrap();
                    let (new_link, new_items) = self.get_pages_url(&url).await?;
                    link = new_link;
                    items = new_items;
                },
            }
        }

        Ok(global_items)
    }"#);
    a("");

    /*
     * Generate a function for each Operation.
     *
     * XXX We should probably be producing an intermediate object for each of
     * these, which can link in to the type space, instead of doing this inline
     * here.
     */
    for (pn, p) in api.paths.iter() {
        let op = p.item()?;

        let mut gen = |p: &str, m: &str, o: Option<&openapiv3::Operation>| -> Result<()> {
            let o = if let Some(o) = o {
                o
            } else {
                return Ok(());
            };

            let mut oid = o.operation_id.as_deref().unwrap().to_string();
            oid = oid.replace("-", "_").replace("/", "_");
            a("/**");
            if let Some(summary) = &o.summary {
                a(&format!("* {}.", summary.trim_end_matches('.')));
                a("*");
            }
            a(&format!("* This function performs a `{}` to the `{}` endpoint.", m, p));
            if let Some(description) = &o.description {
                a("*");
                a(&format!("* {}", description.replace('\n', "\n* ")));
            }
            if let Some(external_docs) = &o.external_docs {
                a("*");
                a(&format!("* FROM: <{}>", external_docs.url));
            }
            a("*/");

            let mut bounds: Vec<String> = Vec::new();

            let (body_param, body_func) = if let Some(b) = &o.request_body {
                let b = b.item()?;
                if b.is_binary()? {
                    bounds.push("B: Into<reqwest::Body>".to_string());
                    (Some("B".to_string()), Some("body".to_string()))
                } else {
                    let (ct, mt) = b.content.first().unwrap();
                    if !mt.encoding.is_empty() {
                        bail!("media type encoding not empty: {:#?}", mt);
                    }

                    if ct == "application/json" {
                        if let Some(s) = &mt.schema {
                            let tid = match s {
                                openapiv3::ReferenceOr::Reference { reference } => ts.select_ref(None, reference.as_str())?,
                                openapiv3::ReferenceOr::Item(item) => {
                                    let object_name = format!("{} request", summary_to_object_name("", o.summary.as_ref().unwrap()));
                                    ts.select_schema(Some(&object_name), item, "", false)?
                                }
                            };
                            (Some(format!("&{}", ts.render_type(&tid, false)?)), Some("json".to_string()))
                        } else {
                            bail!("media type encoding, no schema: {:#?}", mt);
                        }
                    } else if ct == "text/plain" || ct == "*/*" {
                        if let Some(s) = &mt.schema {
                            let tid = ts.select(None, s, false)?;
                            let rt = ts.render_type(&tid, false)?;
                            bounds.push("T: Into<reqwest::Body>".to_string());
                            if rt == "String" {
                                (Some("T".to_string()), Some("body".to_string()))
                            } else {
                                (Some(rt), Some("body".to_string()))
                            }
                        } else {
                            bail!("media type encoding, no schema: {:#?}", mt);
                        }
                    } else {
                        bail!("unhandled request content type: {}", ct);
                    }
                }
            } else {
                (None, None)
            };

            if oid == "apps_create_installation_access_token" {
                a("#[async_recursion]");
            }

            if bounds.is_empty() {
                a(&format!("    pub async fn {}(", oid));
            } else {
                a(&format!("    pub async fn {}<{}>(", oid, bounds.join(", ")));
            }
            a("        &self,");

            let mut query_params_str: Vec<String> = Default::default();
            /*
             * Query parameters are sorted lexicographically to ensure a stable
             * order in the generated code.
             */
            let mut query_params: BTreeMap<String, String> = Default::default();
            for par in o.parameters.iter() {
                let mut param_name = "".to_string();
                let item = match par {
                    openapiv3::ReferenceOr::Reference { reference } => {
                        param_name = struct_name(&reference.replace("#/components/parameters/", ""));
                        // Get the parameter from our BTreeMap.
                        if let Some(param) = parameters.get(&param_name) {
                            param
                        } else {
                            bail!("could not find parameter with reference: {}", reference);
                        }
                    }
                    openapiv3::ReferenceOr::Item(item) => item,
                };

                match item {
                    openapiv3::Parameter::Path {
                        parameter_data,
                        style: openapiv3::PathStyle::Simple,
                    } => {
                        let nam = &to_snake_case(&parameter_data.name);
                        let typ = parameter_data.render_type(&param_name)?;
                        if nam == "ref" || nam == "type" {
                            a(&format!("{}_: {},", nam, typ));
                        } else {
                            a(&format!("{}: {},", nam, typ));
                        }
                    }
                    openapiv3::Parameter::Query {
                        parameter_data,
                        allow_reserved: _,
                        style: openapiv3::QueryStyle::Form,
                        allow_empty_value,
                    } => {
                        if let Some(aev) = allow_empty_value {
                            if *aev {
                                bail!("allow empty value is a no go");
                            }
                        }

                        let nam = &to_snake_case(&parameter_data.name);
                        let typ = parameter_data.render_type(&param_name)?;
                        if nam == "ref" || nam == "type" {
                            a(&format!("        {}_: {},", nam, typ));
                            query_params_str.push(format!(r#"("{}", {}_.to_string())"#, nam, nam));
                            query_params.insert(nam.to_string(), format!("{}_", nam));
                        } else {
                            a(&format!("        {}: {},", nam, typ));
                            if typ == "DateTime<Utc>" {
                                query_params_str.push(format!(r#"("{}", {}.to_rfc3339())"#, nam, nam));
                                query_params.insert(nam.to_string(), format!("{}.to_rfc3339()", nam));
                            } else if typ == "i64" || typ == "bool" {
                                query_params_str.push(format!(r#"("{}", format!("{{}}", {}))"#, nam, nam));
                                query_params.insert(nam.to_string(), format!(r#"format!("{{}}", {})"#, nam));
                            } else if typ == "&str" {
                                query_params_str.push(format!(r#"("{}", {}.to_string())"#, nam, nam));
                                query_params.insert(nam.to_string(), format!("{}.to_string()", nam));
                            } else if typ == "&[String]" {
                                // TODO: I have no idea how these should be seperated and the docs
                                // don't give any answers either, for "exclude".
                                // https://docs.github.com/en/rest/reference/migrations
                                query_params_str.push(format!(r#"("{}", {}.join(" "))"#, nam, nam));
                                query_params.insert(nam.to_string(), format!("{}.join(\" \")", nam));
                            } else {
                                query_params_str.push(format!(r#"("{}", {})"#, nam, nam));
                                query_params.insert(nam.to_string(), nam.to_string());
                            }
                        }
                    }
                    x => bail!("unhandled parameter type: {:#?}", x),
                }
            }

            if let Some(bp) = &body_param {
                a(&format!("        body: {},", bp));
            }

            // Only do the first.
            let is_vector = if let Some(only) = o.responses.responses.first() {
                match only.0 {
                    openapiv3::StatusCode::Code(n) => {
                        // 302 is the code returned from /orgs/{org}/migrations/{migration_id}/archive GET
                        if *n < 200 || *n > 303 {
                            bail!("code? {:#?}", only);
                        }
                    }
                    _ => bail!("code? {:#?}", only),
                }

                let i = only.1.item()?;
                if !i.headers.is_empty() {
                    // TODO: do response headers.
                }

                if !i.links.is_empty() {
                    // TODO: do response links
                }
                /*
                 * XXX ignoring extensions.
                 */

                /*
                 * Look at the response content.  For now, support a single
                 * JSON-formatted response.
                 */
                if i.content.is_empty() {
                    a("    ) -> Result<()> {");
                    false
                } else {
                    match i.content.get("application/json") {
                        Some(mt) => {
                            if !mt.encoding.is_empty() {
                                bail!("media type encoding not empty: {:#?}", mt);
                            }

                            if let Some(s) = &mt.schema {
                                let tid = match s {
                                    openapiv3::ReferenceOr::Reference { reference } => ts.select_ref(None, reference.as_str())?,
                                    openapiv3::ReferenceOr::Item(item) => {
                                        if let openapiv3::StatusCode::Code(c) = only.0 {
                                            let status_code = StatusCode::from_u16(*c).unwrap();
                                            let object_name = format!(
                                                "{} {} Response",
                                                summary_to_object_name(m, o.summary.as_ref().unwrap()),
                                                status_code.canonical_reason().unwrap().to_lowercase()
                                            );
                                            ts.select_schema(Some(&object_name), item, "", false)?
                                        } else {
                                            bail!("got a range and not a code for {:?}", only.0);
                                        }
                                    }
                                };
                                if let Ok(rt) = ts.render_type(&tid, false) {
                                    a(&format!("    ) -> Result<{}> {{", rt));

                                    rt.starts_with("Vec<")
                                } else {
                                    bail!("rendering type {:?}: {:?} failed", tid, s);
                                }
                            } else {
                                bail!("media type encoding, no schema: {:#?}", mt);
                            }
                        }
                        None => {
                            let (ct, mt) = i.content.first().unwrap();
                            if ct == "text/plain" || ct == "text/html" || ct == "application/octocat-stream" || ct == "*/*" {
                                if let Some(s) = &mt.schema {
                                    let tid = ts.select(None, s, false)?;
                                    let rt = ts.render_type(&tid, false)?;

                                    a(&format!("    ) -> Result<{}> {{", rt));
                                    rt.starts_with("Vec<")
                                } else {
                                    bail!("media type encoding, no schema: {:#?}", mt);
                                }
                            } else if ct == "application/scim+json" {
                                if !mt.encoding.is_empty() {
                                    bail!("media type encoding not empty: {:#?}", mt);
                                }

                                if let Some(s) = &mt.schema {
                                    let tid = match s {
                                        openapiv3::ReferenceOr::Reference { reference } => ts.select_ref(None, reference.as_str())?,
                                        openapiv3::ReferenceOr::Item(item) => {
                                            if let openapiv3::StatusCode::Code(c) = only.0 {
                                                let status_code = StatusCode::from_u16(*c).unwrap();
                                                let object_name = format!(
                                                    "{} {} Response",
                                                    summary_to_object_name(m, o.summary.as_ref().unwrap()),
                                                    status_code.canonical_reason().unwrap().to_lowercase()
                                                );
                                                ts.select_schema(Some(&object_name), item, "", false)?
                                            } else {
                                                bail!("got a range and not a code for {:?}", only.0);
                                            }
                                        }
                                    };
                                    if let Ok(rt) = ts.render_type(&tid, false) {
                                        a(&format!("    ) -> Result<{}> {{", rt));

                                        rt.starts_with("Vec<")
                                    } else {
                                        bail!("rendering type {:?} failed", tid);
                                    }
                                } else {
                                    bail!("media type encoding, no schema: {:#?}", mt);
                                }
                            } else {
                                bail!("unhandled response content type: {}", ct);
                            }
                        }
                    }
                }
            } else {
                bail!("responses? {:#?}", o.responses);
            };

            /*
             * Generate the URL for the request.
             */
            let tmp = template::parse(p)?;
            a(&tmp.compile(query_params));

            /*
             * Perform the request.
             */
            if m == http::Method::GET {
                if is_vector {
                    a("self.get_all_pages(&url).await");
                } else {
                    a(&format!("self.{}(&url).await", m.to_lowercase()));
                }
            } else if (m == http::Method::POST || m == http::Method::PATCH || m == http::Method::PUT || m == http::Method::DELETE)
                && oid != "apps_create_installation_access_token"
            {
                let body = if let Some(f) = &body_func {
                    if f == "json" {
                        "Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))"
                    } else {
                        "Some(body.into())"
                    }
                } else {
                    "None"
                };
                a(&format!("self.{}(&url, {}).await", m.to_lowercase(), body));
            } else {
                if oid != "apps_create_installation_access_token" {
                    panic!("function {} should be authenticated", oid);
                }

                a(r#"self.post_media(
                        &url,
                        Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
                        crate::utils::MediaType::Json,
                        crate::auth::AuthenticationConstraint::JWT,
                    ).await"#);
            }
            a("}");
            a("");

            Ok(())
        };

        gen(pn.as_str(), "GET", op.get.as_ref())?;
        gen(pn.as_str(), "PUT", op.put.as_ref())?;
        gen(pn.as_str(), "POST", op.post.as_ref())?;
        gen(pn.as_str(), "DELETE", op.delete.as_ref())?;
        gen(pn.as_str(), "OPTIONS", op.options.as_ref())?;
        gen(pn.as_str(), "HEAD", op.head.as_ref())?;
        gen(pn.as_str(), "PATCH", op.patch.as_ref())?;
        gen(pn.as_str(), "TRACE", op.trace.as_ref())?;
    }

    a("}");

    Ok(out)
}

fn struct_name(s: &str) -> String {
    to_title_case(&clean_name(s)).replace(" ", "").replace("Self", "SelfData")
}

fn clean_name(t: &str) -> String {
    let mut s = t;
    if t == "/" {
        s = "root";
    }

    to_snake_case(
        &s.replace("+1", "plus_one")
            .replace("-1", "minus_one")
            .replace("100644", "file_blob")
            .replace("100755", "executable_blob")
            .replace("040000", "subdirectory_tree")
            .replace("160000", "submodule_commit")
            .replace("120000", "symlink_path_blob")
            .replace("/", " ")
            .replace("-", " "),
    )
    .replace("_", " ")
    .trim()
    .to_string()
}

fn summary_to_object_name(m: &str, s: &str) -> String {
    let cleaned = s
        .to_lowercase()
        .replace('.', "")
        .replace('_', " ")
        .replace(" an ", " ")
        .replace(" or ", " ")
        .replace(" for ", " ")
        .replace(" to ", " ")
        .replace(" your ", " ")
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

    if m.is_empty() || cleaned.starts_with(&m.to_lowercase()) {
        cleaned
    } else {
        format!("{} {}", m.to_lowercase(), cleaned)
    }
}

fn main() -> Result<()> {
    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.reqopt("i", "", "OpenAPI definition document (JSON | YAML)", "INPUT");
    opts.reqopt("o", "", "Generated Rust crate directory", "OUTPUT");
    opts.reqopt("n", "", "Target Rust crate name", "CRATE");
    opts.reqopt("v", "", "Target Rust crate version", "VERSION");
    opts.reqopt("d", "", "Target Rust crate description", "DESCRIPTION");

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

    /*
     * Grab all the types defined by schemas and parameters.
     */
    let mut ts = TypeSpace::new();
    let mut parameters: BTreeMap<String, &openapiv3::Parameter> = BTreeMap::new();

    if let Some(components) = &api.components {
        // Populate a type to describe each entry in the schemas section.
        for (i, (sn, s)) in components.schemas.iter().enumerate() {
            let name = clean_name(sn);
            println!("SCHEMA {}/{}: {}", i + 1, components.schemas.len(), name);

            let id = ts.select(Some(name.as_str()), s, true)?;
            println!("    -> {:?}", id);
            println!();
        }

        // Populate a type to describe each entry in the parameters section.
        for (i, (pn, p)) in components.parameters.iter().enumerate() {
            println!("PARAMETER {}/{}: {}", i + 1, components.parameters.len(), pn);

            if let openapiv3::ReferenceOr::Item(item) = p {
                parameters.insert(struct_name(&pn.to_string()), item);
            } else {
                bail!("parameter {} uses reference, unsupported: {:?}", pn, p);
            }
            println!();
        }
    }

    /*
     * In addition to types defined in schemas, types may be defined inline in
     * request and response bodies.
     */
    for (pn, p) in api.paths.iter() {
        let op = p.item()?;

        let grab = |pn: &str, m: &str, o: Option<&openapiv3::Operation>, ts: &mut TypeSpace| -> Result<()> {
            if let Some(o) = o {
                /*
                 * Get the request body type, if this operation has one:
                 */
                if let Some(openapiv3::ReferenceOr::Item(body)) = &o.request_body {
                    if !body.is_binary()? {
                        if let Ok(mt) = body.content_json().with_context(|| anyhow!("{} {} request", m, pn)) {
                            if let Some(s) = &mt.schema {
                                let object_name = format!("{} request", summary_to_object_name("", o.summary.as_ref().unwrap()));
                                let id = ts.select(Some(&object_name), s, false)?;
                                println!("    {} {} request body -> '{}' {:?}", pn, m, object_name, id);
                            }
                        } else if let Some((ct, mt)) = body.content.first() {
                            if ct == "text/plain" || ct == "text/html" {
                                println!("    {} {} request body -> &str", pn, m,);
                            } else if let Some(s) = &mt.schema {
                                println!("    {} {} request body -> {:?}", pn, m, s);
                            } else {
                                bail!("unknown request content: {} {} {:?}", pn, m, body.content);
                            }
                        } else {
                            bail!("unknown request content: {} {} {:?}", pn, m, body.content);
                        }
                    }
                } else if let Some(openapiv3::ReferenceOr::Reference { reference }) = &o.request_body {
                    let id = ts.select_ref(None, reference.as_str())?;
                    println!("    {} {} request body -> {:?}", pn, m, id);
                }

                /*
                 * Get the response body type for each status code:
                 */
                for (code, r) in o.responses.responses.iter() {
                    match r {
                        openapiv3::ReferenceOr::Item(ri) => {
                            if !ri.is_binary()? && !ri.content.is_empty() {
                                if let Ok(mt) = ri.content_json().with_context(|| anyhow!("{} {} {}", m, pn, code)) {
                                    if let Some(s) = &mt.schema {
                                        if let openapiv3::StatusCode::Code(c) = code {
                                            let status_code = StatusCode::from_u16(*c).unwrap();
                                            let object_name = format!(
                                                "{} {} Response",
                                                summary_to_object_name(m, o.summary.as_ref().unwrap()),
                                                status_code.canonical_reason().unwrap().to_lowercase()
                                            );
                                            let id = ts.select(Some(&object_name), s, false)?;
                                            println!("    {} {} {} response body -> {:?}", pn, m, code, id);
                                        } else {
                                            bail!("got a range and not a code for {:?}", code);
                                        }
                                    }
                                } else if let Some((ct, mt)) = ri.content.first() {
                                    if ct == "text/plain" || ct == "text/html" {
                                        println!("    {} {} {} response body -> String", pn, m, code,);
                                    } else if let Some(s) = &mt.schema {
                                        println!("    {} {} {} response body -> {:?}", pn, m, code, s);
                                    } else {
                                        bail!("unknown response content: {} {} {} {:?}", pn, m, code, ri.content);
                                    }
                                } else {
                                    bail!("unknown response content: {} {} {} {:?}", pn, m, code, ri.content);
                                }
                            }
                        }
                        openapiv3::ReferenceOr::Reference { reference } => {
                            let id = ts.select_ref(None, reference.as_str())?;
                            println!("    {} {} {} response body -> {:?}", pn, m, code, id);
                        }
                    }
                }
            }
            Ok(())
        };

        grab(pn, "GET", op.get.as_ref(), &mut ts)?;
        grab(pn, "POST", op.post.as_ref(), &mut ts)?;
        grab(pn, "PUT", op.put.as_ref(), &mut ts)?;
        grab(pn, "DELETE", op.delete.as_ref(), &mut ts)?;
        grab(pn, "OPTIONS", op.options.as_ref(), &mut ts)?;
        grab(pn, "HEAD", op.head.as_ref(), &mut ts)?;
        grab(pn, "PATCH", op.patch.as_ref(), &mut ts)?;
        grab(pn, "TRACE", op.trace.as_ref(), &mut ts)?;
    }

    let name = args.opt_str("n").unwrap();
    let version = args.opt_str("v").unwrap();

    let fail = match gen(&api, &mut ts, parameters, &name.replace("-", "_"), &version) {
        Ok(out) => {
            let description = args.opt_str("d").unwrap();

            /*
             * Create the top-level crate directory:
             */
            let root = PathBuf::from(args.opt_str("o").unwrap());
            std::fs::create_dir_all(&root)?;

            /*
             * Write the Cargo.toml file:
             */
            let mut toml = root.clone();
            toml.push("Cargo.toml");
            let tomlout = format!(
                r#"[package]
name = "{}"
description = "{}"
version = "{}"
documentation = "https://docs.rs/{}/"
repository = "https://github.com/oxidecomputer/{}"
readme = "README.md"
edition = "2018"

[dependencies]
anyhow = "1"
async-recursion = "^0.3.2"
chrono = {{ version = "0.4", features = ["serde"] }}
dirs = {{ version = "^3.0.2", optional = true }}
http = "^0.2.4"
hyperx = "1"
jsonwebtoken = "7"
mime = "0.3"
percent-encoding = "2.1"
reqwest = {{ version = "0.11", features = ["json"] }}
schemars = {{ version = "0.8", features = ["chrono", "uuid"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"

[dev-dependencies]
base64 = "^0.12"
dirs = "^3.0.2"
nom_pem = "4"

[features]
# enable etag-based http_cache functionality
httpcache = ["dirs"]
"#,
                name, description, version, name, name
            );
            save(&toml, tomlout.as_str())?;

            /*
             * Create the src/ directory:
             */
            let mut src = root;
            src.push("src");
            std::fs::create_dir_all(&src)?;

            /*
             * Create the Rust source file containing the generated client:
             */
            let mut librs = src;
            librs.push("lib.rs");
            save(librs, out.as_str())?;
            false
        }
        Err(e) => {
            println!("gen fail: {:?}", e);
            true
        }
    };

    println!("-----------------------------------------------------");
    println!(" TYPE SPACE");
    println!("-----------------------------------------------------");
    for te in ts.id_to_entry.values() {
        let n = ts.describe(&te.id);
        println!("{:>4}  {}", te.id.0, n);
    }
    println!("-----------------------------------------------------");
    println!();

    if fail {
        bail!("generation experienced errors");
    }

    Ok(())
}
