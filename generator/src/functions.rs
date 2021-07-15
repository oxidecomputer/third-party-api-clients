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

            let mut print_fn = |docs: &str,
                                bounds: &Vec<String>,
                                fn_params_str: &Vec<String>,
                                body_param: &Option<String>,
                                response_type: &str,
                                template: &str,
                                fn_inner: &str,
                                fn_name: &str| {
                // Print the function docs.
                a(docs);

                // For this one function, we need it to be recursive since this is how you get
                // an access token when authenicating on behalf of an app with a JWT.
                if oid == "apps_create_installation_access_token" {
                    a("#[async_recursion::async_recursion]");
                }

                if bounds.is_empty() {
                    a(&format!("pub async fn {}(", fn_name,));
                } else {
                    a(&format!("pub async fn {}<{}>(", fn_name, bounds.join(", ")));
                }
                a("&self,");

                if !fn_params_str.is_empty() {
                    a(&fn_params_str.join(" "));
                }

                if let Some(bp) = &body_param {
                    a(&format!("body: {}", bp));
                }

                a(&format!(") -> Result<{}> {{", response_type));

                a(template);

                a(fn_inner);

                a("}");
                a("");
            };

            let docs = get_fn_docs(o, m, p, parameters, ts)?;

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
                            let id = ts.select(Some(&object_name), s, "")?;
                            let rt = ts.render_type(&id, false)?;
                            (Some(format!("&{}", rt)), Some("json".to_string()))
                        } else {
                            (None, None)
                        }
                    } else if let Some(s) = &mt.schema {
                        let tid = ts.select(None, s, "")?;
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

            /*
             * Get the function parameters.
             */
            let (fn_params_str, query_params) = get_fn_params(ts, o, parameters, false)?;

            /*
             * Generate the URL for the request.
             */
            let tmp = parse(p)?;
            let template = tmp.compile(query_params);

            let fn_inner = get_fn_inner(&oid, m, &body_func)?;

            /*
             * Get the response type.
             */
            let (response_type, tid) = get_response_type(&oid, ts, o)?;
            if let Some(te) = ts.id_to_entry.get(&tid) {
                // If we have a one of, we can generate a few different subfunctions to
                // help as well.
                if let crate::TypeDetails::OneOf(one_of, _) = &te.details {
                    for (rt, itid) in one_of {
                        print_fn(
                            &docs,
                            &bounds,
                            &fn_params_str,
                            &body_param,
                            &ts.render_type(itid, false)?,
                            &template,
                            &fn_inner,
                            &to_snake_case(&struct_name(&format!(
                                "{}_{}",
                                oid.trim_start_matches(&tag).trim_start_matches('_'),
                                to_snake_case(rt)
                            ))),
                        );
                    }
                }
            }

            // Print our standard function.
            print_fn(
                &docs,
                &bounds,
                &fn_params_str,
                &body_param,
                &response_type,
                &template,
                &fn_inner,
                oid.trim_start_matches(&tag).trim_start_matches('_'),
            );

            // If we are returning a list of things and we have page, etc as
            // params, let's get all the pages.
            if response_type.starts_with("Vec<") && http::Method::GET == m {
                let docs = get_fn_docs_all(
                    o,
                    m,
                    p,
                    oid.trim_start_matches(&tag).trim_start_matches('_'),
                )?;

                let (fn_params_str, query_params) = get_fn_params(ts, o, parameters, true)?;

                let tmp = parse(p)?;
                let template = tmp.compile(query_params);

                let fn_inner = "self.client.get_all_pages(&url).await";

                let mut fn_name = oid
                    .replace("_get_", "_get_all_")
                    .replace("_list_", "_list_all_")
                    .trim_start_matches(&tag)
                    .trim_start_matches('_')
                    .to_string();

                if fn_name == "list" {
                    fn_name = "list_all".to_string();
                }
                if fn_name == "get" {
                    fn_name = "get_all".to_string();
                }

                // Now let's print the new function.
                print_fn(
                    &docs,
                    &bounds,
                    &fn_params_str,
                    &body_param,
                    &response_type,
                    &template,
                    fn_inner,
                    &fn_name,
                );
            }

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

fn get_response_type(
    oid: &str,
    ts: &mut TypeSpace,
    o: &openapiv3::Operation,
) -> Result<(String, crate::TypeId)> {
    // Get the first response.
    let first = o.responses.responses.first().unwrap();
    let i = first.1.item()?;

    if i.content.is_empty() {
        // Return empty.
        return Ok(("()".to_string(), crate::TypeId(0)));
    }

    // Get the json response, if it exists.
    if let Some(mt) = i.content.get("application/json") {
        if !mt.encoding.is_empty() {
            bail!("media type encoding not empty: {:#?}", mt);
        }

        if let Some(s) = &mt.schema {
            let object_name = format!("{} response", oid_to_object_name(oid));
            let tid = ts.select(Some(&clean_name(&object_name)), s, "")?;
            let rt = ts.render_type(&tid, false)?;
            return Ok((rt, tid));
        }
    }

    // Get the first response.
    let (ct, mt) = i.content.first().unwrap();
    if ct == "text/plain" || ct == "text/html" || ct == "application/octocat-stream" || ct == "*/*"
    {
        if let Some(s) = &mt.schema {
            let tid = ts.select(None, s, "")?;
            let rt = ts.render_type(&tid, false)?;
            return Ok((rt, tid));
        }
    } else if ct == "application/scim+json" {
        if !mt.encoding.is_empty() {
            bail!("media type encoding not empty: {:#?}", mt);
        }

        if let Some(s) = &mt.schema {
            let object_name = format!("{} response", oid_to_object_name(oid));
            let tid = ts.select(Some(&clean_name(&object_name)), s, "")?;
            let rt = ts.render_type(&tid, false)?;
            return Ok((rt, tid));
        }
    }

    bail!("parsing response got to end with no type");
}

fn get_fn_params(
    ts: &mut TypeSpace,
    o: &openapiv3::Operation,
    parameters: &BTreeMap<String, &openapiv3::Parameter>,
    all_pages: bool,
) -> Result<(Vec<String>, BTreeMap<String, String>)> {
    /*
     * Query parameters are sorted lexicographically to ensure a stable
     * order in the generated code.
     */
    let mut fn_params_str: Vec<String> = Default::default();
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

        let parameter_data = get_parameter_data(item).unwrap();
        let nam = &to_snake_case(&parameter_data.name);
        let typ = parameter_data.render_type(&param_name, ts)?;
        if nam == "ref" || nam == "type" {
            fn_params_str.push(format!("{}_: {},", nam, typ));
        } else if !all_pages || (nam != ("page") && nam != "per_page") {
            if typ == "chrono::DateTime<chrono::Utc>" {
                fn_params_str.push(format!("{}: Option<{}>,", nam, typ));
            } else {
                fn_params_str.push(format!("{}: {},", nam, typ));
            }
        }

        // Check if we have a query.
        // TODO: make this a bool ext.
        if let openapiv3::Parameter::Query {
            parameter_data: _,
            allow_reserved: _,
            style: openapiv3::QueryStyle::Form,
            allow_empty_value,
        } = item
        {
            if let Some(aev) = allow_empty_value {
                if *aev {
                    bail!("allow empty value is a no go");
                }
            }

            if nam == "ref" || nam == "type" {
                query_params.insert(format!("{}_", nam), typ.to_string());
            } else if !all_pages || (nam != ("page") && nam != "per_page") {
                if typ == "chrono::DateTime<chrono::Utc>" {
                    query_params.insert(nam.to_string(), format!("Option<{}>", typ));
                } else {
                    query_params.insert(nam.to_string(), typ.to_string());
                }
            }
        }
    }

    Ok((fn_params_str, query_params))
}

/*
 * Perform the function.
 */
fn get_fn_inner(oid: &str, m: &str, body_func: &Option<String>) -> Result<String> {
    if m == http::Method::GET {
        return Ok(format!("self.client.{}(&url).await", m.to_lowercase()));
    }

    if (m == http::Method::POST
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

        return Ok(format!(
            "self.client.{}(&url, {}).await",
            m.to_lowercase(),
            body
        ));
    }

    if oid != "apps_create_installation_access_token" {
        bail!("function {} should be authenticated", oid);
    }

    Ok(r#"self.client.post_media(
            &url,
            Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::JWT,
        ).await"#
        .to_string())
}

fn get_fn_docs(
    o: &openapiv3::Operation,
    m: &str,
    p: &str,
    parameters: &BTreeMap<String, &openapiv3::Parameter>,
    ts: &mut TypeSpace,
) -> Result<String> {
    let mut out = String::new();

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

        let parameter_data = get_parameter_data(item).unwrap();

        let pid = ts.select_param(None, par)?;
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

    Ok(out.trim().to_string())
}

fn get_fn_docs_all(o: &openapiv3::Operation, m: &str, p: &str, fn_name: &str) -> Result<String> {
    let mut out = String::new();

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
    a("*");
    a(&format!(
        "* As opposed to `{}`, this function returns all the pages of the request at once.",
        fn_name
    ));
    if let Some(description) = &o.description {
        a("*");
        a(&format!("* {}", description.replace('\n', "\n* ")));
    }
    if let Some(external_docs) = &o.external_docs {
        a("*");
        a(&format!("* FROM: <{}>", external_docs.url));
    }
    a("*/");

    Ok(out.trim().to_string())
}
