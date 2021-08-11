use std::collections::BTreeMap;

use anyhow::{bail, Result};
use inflector::cases::snakecase::to_snake_case;

use crate::{render_param, struct_name, TypeDetails, TypeSpace};

/*
 * Declare named types we know about:
 */
pub fn generate_types(ts: &mut TypeSpace) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    a("//! The data types sent to and returned from the API client.");
    a("    use schemars::JsonSchema;");
    a("    use serde::{Serialize, Deserialize};");
    a("");

    for te in ts.id_to_entry.values() {
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
                TypeDetails::OneOf(omap, schema_data) => {
                    let desc = if let Some(description) = &schema_data.description {
                        format!("/// {}", description.replace('\n', "\n/// "))
                    } else {
                        "".to_string()
                    };

                    if !desc.is_empty() {
                        a(&desc);
                    }

                    a("#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]");
                    a("#[serde(untagged)]");
                    a(&format!("pub enum {} {{", sn));
                    let mut name_map: BTreeMap<String, String> = Default::default();
                    // Becasue we have so many defaults set on our serde types these enums
                    // sometimes parse the wrong value. It's better to instead use the functions we
                    // inject that force the value to a specific type.
                    for (name, tid) in omap.iter() {
                        // Try to render the docs.
                        let p = ts.render_docs(tid);
                        if !p.is_empty() && p != desc {
                            a("/**");
                            a(&p);
                            a("*/");
                        }

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
                            struct_name(name)
                        };

                        a(&format!("{}({}),", fn_name, name));
                        name_map.insert(fn_name, name.to_string());
                    }
                    a("}");
                    a("");

                    // Render the implementation to easily unpack these things for the end user.
                    a(&format!("impl {} {{", sn));
                    for (fn_name, name) in &name_map {
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
                        a("");
                    }
                    a("}");
                    a("");

                    // Render the implementation to easily unpack these things for the end user.
                    for (fn_name, name) in &name_map {
                        a(&format!(
                            r#"impl From<{}> for {} {{
                                    fn from(f: {}) -> Self {{
                                        {}::{}(f)
                                    }}
                            }}"#,
                            name, sn, name, sn, fn_name,
                        ));
                        a("");
                    }

                    for name in name_map.values() {
                        if name == "i64"
                            || name == "i32"
                            || name == "f64"
                            || name == "f32"
                            || name == "bool"
                        {
                            a(&format!(
                                r#"impl From<{}> for {} {{
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
                        } else {
                            a(&format!(
                                r#"impl From<{}> for {} {{
                                    fn from(f: {}) -> Self {{
                                        f.{}().unwrap().clone()
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
                        }
                        a("");
                    }
                }
                TypeDetails::Object(omap, schema_data) => {
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

                    a("#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]");
                    a(&format!("pub struct {} {{", sn));
                    for (name, tid) in omap.iter() {
                        if let Ok(mut rt) = ts.render_type(tid, true) {
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
                            if rt == "String" || rt.starts_with("Vec<") || rt.starts_with("Option<")
                            {
                                a(r#"#[serde(default,"#);
                                if rt == "String" {
                                    a(r#"skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize","#);
                                } else if rt.starts_with("Vec<") {
                                    a(r#"skip_serializing_if = "Vec::is_empty","#);
                                } else if rt.starts_with("Option<") {
                                    a(r#"skip_serializing_if = "Option::is_none","#);
                                }
                            } else if rt == "bool" {
                                if sn.ends_with("Request") {
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

                            // Close the serde string.
                            if *name != prop {
                                a(&format!(r#"rename = "{}")]"#, name));
                            } else if rt == "Page" && prop == "page" {
                                // Flatten the struct this is for Ramp's pagination.
                                // God willing let's hope no other API breaks this in the future.
                                a(r#"flatten)]"#);
                            } else {
                                a(r#")]"#);
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
