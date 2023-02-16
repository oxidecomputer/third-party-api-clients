use std::collections::BTreeMap;

use anyhow::{bail, Result};
use inflector::cases::snakecase::to_snake_case;

use crate::{render_param, struct_name, TypeDetails, TypeSpace};

/*
 * Declare named types we know about:
 */
pub fn generate_types(ts: &mut TypeSpace, proper_name: &str) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    a("//! The data types sent to and returned from the API client.");
    a("    use schemars::JsonSchema;");
    a("    use serde::{Serialize, Deserialize};");
    a("");

    for te in ts.clone().id_to_entry.values() {
        if let Some(sn) = te.name.as_deref() {
            let sn = struct_name(sn);

            match &te.details {
                TypeDetails::Enum(vals, schema_data) => {
                    let mut desc = "".to_string();
                    if let Some(d) = &schema_data.description {
                        desc = d.to_string();
                    }
                    let p = render_param(
                        sn.as_str(),
                        vals,
                        false,
                        &desc,
                        schema_data.default.as_ref(),
                    );
                    a(&p);
                }
                TypeDetails::OneOf(omap, _) => a(&do_one_of_type(ts, omap, sn)),
                TypeDetails::AnyOf(omap, _) => {
                    // When Stripe uses anyof what they really mean is one of.
                    // Because something can not both be a string and an object.
                    if proper_name == "Stripe" {
                        a(&do_one_of_type(ts, omap, sn));
                    } else {
                        a(&do_all_of_type(ts, omap, sn));
                    }
                }
                TypeDetails::AllOf(omap, _) => a(&do_all_of_type(ts, omap, sn)),
                TypeDetails::Object(omap, schema_data) => {
                    // panic!("{:#?} {:#?} {:#?}", te.details, omap, schema_data);
                    /*
                     * TODO: This breaks things so ignore for now.
                     * Eventually this should work, we should ignore empty structs.
                    if omap.is_empty() {
                        // Continue early.
                        // We don't care about empty structs.
                        continue;
                    }*/

                    let desc = if let Some(description) = &schema_data.description {
                        format!("/// {}", description.replace('\n', "\n/// "))
                    } else {
                        "".to_string()
                    };

                    if !desc.is_empty() {
                        a(&desc);
                    }

                    // TODO: just make everything a default,
                    // this is gated by the oneof types cooperating.
                    if sn == "Page"
                        || sn.ends_with("Page")
                        || sn == "PagesSourceHash"
                        || sn == "PagesHttpsCertificate"
                        || sn == "ErrorDetails"
                        || sn == "EnvelopeDefinition"
                        || (sn == "Event" && proper_name != "Stripe")
                        || sn == "User"
                        || sn == "Group"
                        || sn == "CalendarResource"
                        || sn == "Building"
                        || sn == "Repo"
                        || sn == "Payload"
                        || sn == "Actor"
                        || sn == "File"
                        || sn == "PostMailSendRequest"
                        || sn == "FromEmailObject"
                        || sn == "Personalizations"
                        || sn == "DescriptionlessJobOptions"
                        || sn == "DescriptionlessJobOptionsData"
                        || sn == "DescriptionlessJobOptionsDataType"
                        || sn == "SubmitJobOptions"
                        || sn == "SubmitJobOptionsData"
                        || sn == "MinimalRepository"
                        || sn == "WorkflowRun"
                        || sn == "CheckAnnotation"
                    {
                        a(
                            "#[derive(Serialize, Default, Deserialize, PartialEq, Debug, Clone, \
                             JsonSchema)]",
                        );
                    } else {
                        a("#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, \
                             JsonSchema)]");
                    }
                    a(&format!("pub struct {} {{", sn));
                    for (name, tid) in omap.iter() {
                        if let Ok(mut rt) = ts.render_type(tid, true) {
                            // Stripe has some really weird recursive types.
                            if rt.ends_with("AnyOf") && proper_name == "Stripe" {
                                // Stripe uses anyof, but we want oneof.
                                rt = format!("Box<{}>", rt);
                            } else if ((rt.ends_with("AnyOf>") && rt.starts_with("Option<"))
                                || rt == "Option<ApiErrors>"
                                || rt == "Option<PaymentIntent>")
                                && proper_name == "Stripe"
                            {
                                // Stripe uses anyof, but we want oneof.
                                rt = format!(
                                    "Box<Option<{}>>",
                                    rt.trim_start_matches("Option<").trim_end_matches('>')
                                );
                            } else if rt.ends_with("AnyOf>")
                                && rt.starts_with("Vec<")
                                && proper_name == "Stripe"
                            {
                                // Stripe uses anyof, but we want oneof.
                                rt = format!(
                                    "Box<Vec<{}>>",
                                    rt.trim_start_matches("Vec<").trim_end_matches('>')
                                );
                            }
                            let mut prop = name.trim().to_string();
                            if prop == "next" {
                                rt = "String".to_string();
                            }
                            if prop == "ref"
                                || prop == "type"
                                || prop == "self"
                                || prop == "box"
                                || prop == "match"
                                || prop == "foo"
                                || prop == "enum"
                                || prop == "const"
                                || prop == "use"
                            {
                                prop = format!("{}_", name);
                            } else if name == "$ref" {
                                prop = format!("{}_", name.replace('$', ""));
                            } else if name == "$type" {
                                prop = format!("{}__", name.replace('$', ""));
                            } else if name == "+1" {
                                prop = "plus_one".to_string()
                            } else if name == "-1" {
                                prop = "minus_one".to_string()
                            } else if name.starts_with('@') {
                                prop = name.trim_start_matches('@').to_string();
                            } else if name.starts_with('_') {
                                prop = name.trim_start_matches('_').to_string();
                            }

                            // Try to render the docs.
                            let p = ts.render_docs(tid);
                            if !p.is_empty() && p != desc {
                                a("/**");
                                a(&p);
                                a("*/");
                            }

                            let te = ts.id_to_entry.get(tid).unwrap();

                            // Render the serde string.
                            if rt == "String"
                                || rt.starts_with("Vec<")
                                || rt.starts_with("Option<")
                                || rt.starts_with("HashMap<")
                            {
                                a(r#"#[serde(default,"#);
                                if rt == "String" {
                                    a(r#"skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize","#);
                                } else if rt.starts_with("Vec<") {
                                    a(r#"skip_serializing_if = "Vec::is_empty",
                                      deserialize_with = "crate::utils::deserialize_null_vector::deserialize","#);
                                } else if rt.starts_with("std::collections::HashMap<") {
                                    a(
                                        r#"skip_serializing_if = "std::collections::HashMap::is_empty","#,
                                    );
                                } else if rt.starts_with("Option<url::Url") {
                                    a(r#"skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::deserialize_empty_url::deserialize","#);
                                } else if rt.starts_with("Option<chrono::NaiveDate") {
                                    a(r#"skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_format::deserialize","#);
                                } else if rt.starts_with("Option<chrono::DateTime") {
                                    a(r#"skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize","#);

                                    // Google Calendar is weird and requires a custom format.
                                    if proper_name == "Google Calendar" {
                                        // We need to serialize with the right format!
                                        a(
                                            r#"serialize_with = "crate::utils::google_calendar_date_time_format::serialize","#,
                                        );
                                    }
                                } else if rt.starts_with("Option<") {
                                    if (prop == "required_pull_request_reviews"
                                        || prop == "required_status_checks"
                                        || prop == "restrictions")
                                        && proper_name == "GitHub"
                                    {
                                    } else {
                                        a(r#"skip_serializing_if = "Option::is_none","#);
                                    }
                                }
                            } else if rt == "bool" {
                                if sn.ends_with("Request") || proper_name == "Google Drive" {
                                    // We have a request, we want to make sure our bools are
                                    // options so we don't have to always provide them.
                                    a(
                                        r#"#[serde(default, skip_serializing_if = "Option::is_none","#,
                                    );
                                    rt = "Option<bool>".to_string();
                                } else {
                                    a(r#"#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize","#);
                                }
                            } else if rt == "i32" {
                                a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i32",
                                    deserialize_with = "crate::utils::deserialize_null_i32::deserialize","#);
                            } else if rt == "i64" {
                                a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize","#);
                            } else if rt == "f32" {
                                a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f32",
                                    deserialize_with = "crate::utils::deserialize_null_f32::deserialize","#);
                            } else if rt == "f64" {
                                a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f64",
                                    deserialize_with = "crate::utils::deserialize_null_f64::deserialize","#);
                            } else if rt == "u32" || rt == "u64" {
                                a(r#"#[serde(default,"#);
                            } else if let TypeDetails::Enum(_, sd) = &te.details {
                                // We for sure have a default for every single enum, even
                                // if the default is a noop.
                                a(r#"#[serde(default,"#);
                                // Figure out if its a no op and skip serializing if it is.
                                if sd.default.is_none() {
                                    a(&format!(r#"skip_serializing_if = "{}::is_noop","#, rt));
                                }
                            } else {
                                a(r#"#[serde("#);
                            }

                            if !prop.ends_with('_') {
                                prop = to_snake_case(&prop);
                            }

                            // DO this again.
                            // I know this is shit sue me, but sometimes we change the prop
                            // so much it becomes one of these, ie. in the case of shipbob.
                            if prop == "ref"
                                || prop == "type"
                                || prop == "self"
                                || prop == "box"
                                || prop == "match"
                                || prop == "foo"
                                || prop == "enum"
                                || prop == "const"
                                || prop == "use"
                            {
                                prop = format!("{}_", prop);
                            }

                            // Close the serde string.
                            if *name != prop {
                                a(&format!(r#"rename = "{}")]"#, name));
                            } else if rt == "Page" && prop == "page" || rt.ends_with("Page") {
                                a(r#"default)]"#);
                            } else {
                                a(r#")]"#);
                            }

                            if prop == "type" {
                                println!("{} {}", sn, prop);
                            }

                            a(&format!("pub {}: {},", prop, rt));
                        } else {
                            bail!("rendering type {} {:?} failed", name, tid);
                        }
                    }
                    a("}");
                    a("");
                }
                TypeDetails::Basic(..) => {}
                TypeDetails::Unknown => {}
                TypeDetails::NamedType(..) => {}
                TypeDetails::Array(..) => {}
                TypeDetails::Optional(..) => {}
            }
        }
    }

    Ok(out.to_string())
}

fn do_one_of_type(ts: &mut TypeSpace, omap: &[crate::TypeId], sn: String) -> String {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    // Get the description.
    let mut description = "All of the following types:\n\n".to_string();

    let mut flatten = true;
    for (i, itid) in omap.iter().enumerate() {
        let rt = ts.render_type(itid, true).unwrap();
        description.push_str(&format!("- `{}`\n", rt));

        // Determine if we can do anything fancy with the resulting enum and flatten it.
        let et = ts.id_to_entry.get(itid).unwrap();

        if i == 0 && !et.details.is_enum() {
            // If we don't have an enum for the first type we can't do anything
            // fancy and flatten the enum.
            flatten = false;
        } else if let TypeDetails::Object(o, _) = &et.details {
            if o.is_empty() || o.len() > 1 {
                // We don't have a unit struct so we can't do anything fancy.
                flatten = false;
            }
        } else {
            // We don't have an object so let's just call it not fancy.
            flatten = false;
        }
    }
    description.push_str(
        "\nYou can easily convert this enum to the inner value with `From` and `Into`, as both \
         are implemented for each type.\n",
    );
    description = format!("/// {}", description.replace('\n', "\n/// "));
    a(&description);

    a("#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]");
    if !flatten {
        a("#[serde(untagged)]");
    }
    a(&format!("pub enum {} {{", sn));
    let mut name_map: BTreeMap<String, String> = Default::default();
    // Becasue we have so many defaults set on our serde types these enums
    // sometimes parse the wrong value. It's better to instead use the functions we
    // inject that force the value to a specific type.
    let mut fns: Vec<String> = Default::default();
    for tid in omap.iter() {
        let name = ts.render_type(tid, true).unwrap();

        let fn_name = if name.starts_with("Vec<") {
            format!(
                "{}Vector",
                name.trim_start_matches("Vec<")
                    .trim_end_matches('>')
                    .replace("serde_json::", "")
            )
        } else if name.starts_with("serde_json") {
            "Value".to_string()
        } else {
            struct_name(&name)
        };

        if !fns.contains(&fn_name) {
            // Try to render the docs.
            let p = ts.render_docs(tid);
            if !p.is_empty() && p != description {
                a("/**");
                a(&p);
                a("*/");
            }

            a(&format!("{}({}),", fn_name, name));
            name_map.insert(fn_name.to_string(), name.to_string());
            fns.push(fn_name);
        }
    }
    a("}");
    a("");

    // Render the implementation to easily unpack these things for the end user.
    a(&format!("impl {} {{", sn));
    for (fn_name, name) in &name_map {
        if name_map.len() > 1 {
            a(&format!(
                r#"pub fn {}(&self) -> Option<&{}> {{
                            if let {}::{}(ref_) = self {{
                                return Some(ref_);
                            }}
                            None
                        }}"#,
                to_snake_case(name)
                    .replace("f_64", "f64")
                    .replace("f_32", "f32")
                    .replace("i_64", "i64")
                    .replace("i_32", "i32"),
                name,
                sn,
                fn_name,
            ));
        } else {
            a(&format!(
                r#"pub fn {}(&self) -> Option<&{}> {{
                            let {}::{}(ref_) = self;
                            Some(ref_)

                        }}"#,
                to_snake_case(name)
                    .replace("f_64", "f64")
                    .replace("f_32", "f32")
                    .replace("i_64", "i64")
                    .replace("i_32", "i32"),
                name,
                sn,
                fn_name,
            ));
        }
        a("");
    }
    a("}");
    a("");

    // TODO: Implement defaults for these then turn on defaults for
    // the objects above.

    // Render the implementation to easily unpack these things for the end user.
    for (fn_name, name) in &name_map {
        if name == "i64"
            || name == "i32"
            || name == "f64"
            || name == "f32"
            || name == "bool"
            || name == "String"
            || name.starts_with("Vec<")
        {
            a(&format!(
                r#"impl std::convert::From<{}> for {} {{
                                    fn from(f: {}) -> Self {{
                                        {}::{}(f)
                                    }}
                            }}"#,
                name, sn, name, sn, fn_name,
            ));
            a("");
        }
    }

    for name in name_map.values() {
        if name == "i64" || name == "i32" || name == "f64" || name == "f32" || name == "bool" {
            a(&format!(
                r#"impl std::convert::From<{}> for {} {{
                                    fn from(f: {}) -> Self {{
                                        *f.{}().unwrap()
                                    }}
                            }}"#,
                sn,
                name,
                sn,
                to_snake_case(name)
                    .replace("f_64", "f64")
                    .replace("f_32", "f32")
                    .replace("i_64", "i64")
                    .replace("i_32", "i32"),
            ));
        } else if name == "String" || name.starts_with("Vec<") {
            a(&format!(
                r#"impl std::convert::From<{}> for {} {{
                                    fn from(f: {}) -> Self {{
                                        f.{}().unwrap().clone()
                                    }}
                            }}"#,
                sn,
                name,
                sn,
                to_snake_case(name)
            ));
        }
        a("");
    }

    out
}

fn do_all_of_type(ts: &mut TypeSpace, omap: &[crate::TypeId], sn: String) -> String {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    // Get the description.
    let mut description =
        "All of the following types are flattened into one object:\n\n".to_string();

    for itid in omap {
        let rt = ts.render_type(itid, true).unwrap();
        description.push_str(&format!("- `{}`\n", rt));
    }
    description = format!("/// {}", description.replace('\n', "\n/// "));
    a(&description);

    if sn == "SubmitJobOptionsAllOf" || sn == "DescriptionlessJobOptionsAllOf" {
        a("#[derive(Serialize, Deserialize, Default, PartialEq, Debug, Clone, JsonSchema)]");
    } else {
        a("#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]");
    }
    a(&format!("pub struct {} {{", sn));
    let mut name_map: BTreeMap<String, String> = Default::default();
    // Becasue we have so many defaults set on our serde types these enums
    // sometimes parse the wrong value. It's better to instead use the functions we
    // inject that force the value to a specific type.
    let mut fns: Vec<String> = Default::default();
    for tid in omap.iter() {
        let name = ts.render_type(tid, true).unwrap();

        let fn_name = if name.starts_with("Vec<") {
            format!(
                "{}Vector",
                name.trim_start_matches("Vec<")
                    .trim_end_matches('>')
                    .replace("serde_json::", "")
            )
        } else if name.starts_with("serde_json") {
            "Value".to_string()
        } else {
            struct_name(&name)
        };

        if !fns.contains(&fn_name) {
            // Try to render the docs.
            let p = ts.render_docs(tid);
            if !p.is_empty() && p != description {
                a("/**");
                a(&p);
                a("*/");
            }

            a("#[serde(flatten)]");
            a(&format!("pub {}: {},", to_snake_case(&fn_name), name));
            name_map.insert(fn_name.to_string(), name.to_string());
            fns.push(fn_name);
        }
    }
    a("}");
    a("");

    out
}
