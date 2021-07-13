use std::collections::BTreeMap;

use anyhow::{bail, Result};
use inflector::cases::snakecase::to_snake_case;

use crate::{
    clean_name, get_parameter_data, oid_to_object_name, struct_name, template::parse,
    ExtractJsonMediaType, ParameterDataExt, ReferenceOrExt, TypeSpace,
};

/*
 * Generate a function for each Operation.
 */
pub fn generate_files(
    api: &openapiv3::OpenAPI,
    ts: &mut TypeSpace,
    parameters: &BTreeMap<String, &openapiv3::Parameter>,
) -> Result<BTreeMap<String, String>> {
    let mut tag_files: BTreeMap<String, String> = Default::default();

    for (pn, p) in api.paths.iter() {
        let op = p.item()?;

        let mut gen = |p: &str, m: &str, o: Option<&openapiv3::Operation>| -> Result<()> {
            let o = if let Some(o) = o {
                o
            } else {
                return Ok(());
            };

            let oid = to_snake_case(o.operation_id.as_deref().unwrap());

            // Make sure we have exactly 1 tag. This likely needs to change in the
            // future but for now it seems fairly consistent.
            if o.tags.len() != 1 {
                bail!("invalid number of tags for op {}: {}", oid, o.tags.len());
            }
            let tag = to_snake_case(o.tags.first().unwrap());

            let mut out = String::new();
            if let Some(o) = tag_files.get(&tag) {
                out = o.to_string();
            }

            let mut a = |s: &str| {
                out.push_str(s);
                out.push('\n');
            };

            a("/**");
            if let Some(summary) = &o.summary {
                a(&format!("* {}.", summary.trim_end_matches('.')));
                a("*");
            }
            a(&format!(
                "* This function performs a `{}` to the `{}` endpoint.",
                m, p
            ));
            if let Some(description) = &o.description {
                a("*");
                a(&format!("* {}", description.replace('\n', "\n* ")));
            }
            if let Some(external_docs) = &o.external_docs {
                a("*");
                a(&format!("* FROM: <{}>", external_docs.url));
            }
            if !o.parameters.is_empty() {
                a("*");
                a("* **Parameters:**");
                a("*");
            }
            // Iterate over the function parameters and add any data those had as well.
            for par in o.parameters.iter() {
                let mut param_name = "".to_string();
                let item = match par {
                    openapiv3::ReferenceOr::Reference { reference } => {
                        param_name =
                            struct_name(&reference.replace("#/components/parameters/", ""));
                        // Get the parameter from our BTreeMap.
                        if let Some(param) = parameters.get(&param_name) {
                            param
                        } else {
                            bail!("could not find parameter with reference: {}", reference);
                        }
                    }
                    openapiv3::ReferenceOr::Item(item) => item,
                };

                let parameter_data = get_parameter_data(item).unwrap();

                let pid = ts.select_param(None, par, false)?;
                let mut docs = ts.render_docs(&pid);
                if let Some(d) = &parameter_data.description {
                    if !d.is_empty() && d.len() > docs.len() {
                        docs = format!(" -- {}.", d.trim_end_matches('.').replace("\n", "\n*   "));
                    } else if !docs.is_empty() {
                        docs = format!(
                            " -- {}.",
                            docs.trim_start_matches('*').trim_end_matches('.').trim()
                        );
                    }
                } else if !docs.is_empty() {
                    docs = format!(
                        " -- {}.",
                        docs.trim_start_matches('*').trim_end_matches('.').trim()
                    );
                }

                let nam = &to_snake_case(&clean_name(&parameter_data.name));
                let typ = parameter_data.render_type(&param_name, ts)?;

                if nam == "ref" || nam == "type" {
                    a(&format!("* * `{}_: {}`{}", nam, typ, docs));
                } else {
                    a(&format!("* * `{}: {}`{}", nam, typ, docs));
                }
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
                    if ct == "application/json" {
                        if let Some(s) = &mt.schema {
                            let object_name = format!("{} request", oid_to_object_name(&oid));
                            let id = ts.select(Some(&object_name), s, false, "")?;
                            let rt = ts.render_type(&id, false)?;
                            (Some(format!("&{}", rt)), Some("json".to_string()))
                        } else {
                            (None, None)
                        }
                    } else if let Some(s) = &mt.schema {
                        let tid = ts.select(None, s, false, "")?;
                        let rt = ts.render_type(&tid, false)?;
                        bounds.push("T: Into<reqwest::Body>".to_string());
                        if rt == "String" {
                            (Some("T".to_string()), Some("body".to_string()))
                        } else {
                            (Some(rt), Some("body".to_string()))
                        }
                    } else {
                        (None, None)
                    }
                }
            } else {
                (None, None)
            };

            // For this one function, we need it to be recursive since this is how you get
            // an access token when authenicating on behalf of an app with a JWT.
            if oid == "apps_create_installation_access_token" {
                a("#[async_recursion::async_recursion]");
            }

            if bounds.is_empty() {
                a(&format!(
                    "pub async fn {}(",
                    oid.trim_start_matches(&tag).trim_start_matches('_')
                ));
            } else {
                a(&format!(
                    "pub async fn {}<{}>(",
                    oid.trim_start_matches(&tag).trim_start_matches('_'),
                    bounds.join(", ")
                ));
            }
            a("&self,");

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
                        param_name =
                            struct_name(&reference.replace("#/components/parameters/", ""));
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
                        let typ = parameter_data.render_type(&param_name, ts)?;
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
                        let typ = parameter_data.render_type(&param_name, ts)?;
                        if nam == "ref" || nam == "type" {
                            a(&format!("        {}_: {},", nam, typ));
                            query_params_str.push(format!(r#"("{}", {}_.to_string())"#, nam, nam));
                            query_params.insert(nam.to_string(), format!("{}_", nam));
                        } else {
                            a(&format!("        {}: {},", nam, typ));
                            if typ == "DateTime<Utc>" {
                                query_params_str
                                    .push(format!(r#"("{}", {}.to_rfc3339())"#, nam, nam));
                                query_params
                                    .insert(nam.to_string(), format!("{}.to_rfc3339()", nam));
                            } else if typ == "i64" || typ == "bool" {
                                query_params_str
                                    .push(format!(r#"("{}", format!("{{}}", {}))"#, nam, nam));
                                query_params.insert(
                                    nam.to_string(),
                                    format!(r#"format!("{{}}", {})"#, nam),
                                );
                            } else if typ == "&str" {
                                query_params_str
                                    .push(format!(r#"("{}", {}.to_string())"#, nam, nam));
                                query_params
                                    .insert(nam.to_string(), format!("{}.to_string()", nam));
                            } else if typ == "&[String]" {
                                // TODO: I have no idea how these should be seperated and the docs
                                // don't give any answers either, for an array sent through query
                                // params.
                                // https://docs.github.com/en/rest/reference/migrations
                                query_params_str.push(format!(r#"("{}", {}.join(" "))"#, nam, nam));
                                query_params
                                    .insert(nam.to_string(), format!("{}.join(\" \")", nam));
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
                a(&format!("body: {},", bp));
            }

            let response_type = get_response_type(&oid, ts, o)?;
            a(&format!(") -> Result<{}> {{", response_type));

            /*
             * Generate the URL for the request.
             */
            let tmp = parse(p)?;
            a(&tmp.compile(query_params));

            /*
             * Perform the request.
             */
            if m == http::Method::GET {
                a(&format!("self.client.{}(&url).await", m.to_lowercase(),));
            } else if (m == http::Method::POST
                || m == http::Method::PATCH
                || m == http::Method::PUT
                || m == http::Method::DELETE)
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
                a(&format!(
                    "self.client.{}(&url, {}).await",
                    m.to_lowercase(),
                    body
                ));
            } else {
                if oid != "apps_create_installation_access_token" {
                    panic!("function {} should be authenticated", oid);
                }

                a(r#"self.client.post_media(
                        &url,
                        Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
                        crate::utils::MediaType::Json,
                        crate::auth::AuthenticationConstraint::JWT,
                    ).await"#);
            }
            a("}");
            a("");

            // Add this to our map of functions based on the tag name.
            tag_files.insert(tag, out.to_string());

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

    Ok(tag_files)
}

fn get_response_type(oid: &str, ts: &mut TypeSpace, o: &openapiv3::Operation) -> Result<String> {
    // Get the first response.
    let first = o.responses.responses.first().unwrap();
    let i = first.1.item()?;

    if i.content.is_empty() {
        // Return empty.
        return Ok("()".to_string());
    }

    // Get the json response, if it exists.
    if let Some(mt) = i.content.get("application/json") {
        if !mt.encoding.is_empty() {
            bail!("media type encoding not empty: {:#?}", mt);
        }

        if let Some(s) = &mt.schema {
            let object_name = format!("{} response", oid_to_object_name(oid));
            let tid = ts.select(Some(&clean_name(&object_name)), s, false, "")?;
            let rt = ts.render_type(&tid, false)?;
            return Ok(rt);
        }
    }

    // Get the first response.
    let (ct, mt) = i.content.first().unwrap();
    if ct == "text/plain" || ct == "text/html" || ct == "application/octocat-stream" || ct == "*/*"
    {
        if let Some(s) = &mt.schema {
            let tid = ts.select(None, s, false, "")?;
            let rt = ts.render_type(&tid, false)?;
            return Ok(rt);
        }
    } else if ct == "application/scim+json" {
        if !mt.encoding.is_empty() {
            bail!("media type encoding not empty: {:#?}", mt);
        }

        if let Some(s) = &mt.schema {
            let object_name = format!("{} response", oid_to_object_name(oid));
            let tid = ts.select(Some(&clean_name(&object_name)), s, false, "")?;
            let rt = ts.render_type(&tid, false)?;
            return Ok(rt);
        }
    }

    bail!("parsing response got to end with no type");
}
