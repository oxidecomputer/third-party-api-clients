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

    let mut rendered: Vec<String> = Default::default();
    for te in ts.id_to_entry.values() {
        if let Some(sn) = te.name.as_deref() {
            let sn = struct_name(sn);

            if rendered.contains(&sn) {
                // Skip duplicates, this is stupid but since I chose to pick the smaller of the
                // names of the structs, we get duplicates.
                continue;
            }

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
                    rendered.push(sn.to_string());
                }
                TypeDetails::Object(omap, schema_data) => {
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

                            // Try to render the docs.
                            let p = ts.render_docs(tid);
                            if !p.is_empty() && p != desc {
                                a("/**");
                                a(&p);
                                a("*/");
                            }

                            // Render the serde string.
                            if rt == "String" || rt.starts_with("Vec<") || rt.starts_with("Option<")
                            {
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
                            } else if rt == "bool"
                                || rt == "i32"
                                || rt == "i64"
                                || rt == "f32"
                                || rt == "f64"
                                || rt == "u32"
                                || rt == "u64"
                            {
                                a(r#"#[serde(default,"#);
                            } else {
                                a(r#"#[serde("#);
                            }

                            // Close the serde string.
                            if *name != prop {
                                a(&format!(r#"rename = "{}")]"#, name));
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
                    rendered.push(sn.to_string());
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
