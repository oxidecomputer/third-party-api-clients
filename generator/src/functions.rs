use std::collections::BTreeMap;

use anyhow::{bail, Result};
use inflector::cases::{pascalcase::to_pascal_case, snakecase::to_snake_case};

use crate::{
    clean_fn_name, clean_name, client::generate_servers, get_parameter_data, make_plural,
    oid_to_object_name, path_to_operation_id, struct_name, template::parse, ExtractJsonMediaType,
    ParameterDataExt, ReferenceOrExt, TypeId, TypeSpace,
};

#[derive(Debug, Default)]
pub struct FileOutput {
    pub head: String,
    pub impl_content: String,
}

impl FileOutput {
    fn add_head(&mut self, head: &str) {
        self.head.push_str(head);
    }

    fn add_content(&mut self, content: &str) {
        self.impl_content.push_str(content);
    }
}

/*
 * Generate a function for each Operation.
 */
pub fn generate_files(
    api: &openapiv3::OpenAPI,
    proper_name: &str,
    ts: &mut TypeSpace,
    parameters: &BTreeMap<String, &openapiv3::Parameter>,
) -> Result<BTreeMap<String, FileOutput>> {
    let mut tag_files: BTreeMap<String, FileOutput> = Default::default();

    let mut fn_names: Vec<String> = Default::default();
    for (pn, p) in api.paths.iter() {
        let op = p.item().unwrap_or_else(|e| panic!("bad path: {}", e));

        let mut gen = |p: &str, m: &str, o: Option<&openapiv3::Operation>| -> Result<()> {
            let o = if let Some(o) = o {
                o
            } else {
                return Ok(());
            };

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
                let mut t = vec.first().unwrap().to_string();
                if t == "v1" && vec.len() > 1 {
                    //  Try to get the second part of the path.
                    t = vec[1].to_string();
                }

                if t == "3d_secure" {
                    // This is a special case.
                    t = "three_d_secure".to_string();
                }

                tags.push(t);
            }
            let tag = to_snake_case(&clean_name(&make_plural(
                proper_name,
                tags.first().unwrap(),
            )));

            let oid = clean_fn_name(proper_name, &od, &tag);

            let out = tag_files.entry(tag.clone()).or_default();

            let print_fn = |docs: &str,
                            bounds: &Vec<String>,
                            fn_params_str: &Vec<String>,
                            body_param: &Option<String>,
                            response_type: &str,
                            template: &str,
                            fn_inner: &str,
                            fn_name: &str| {
                let mut content = String::new();

                // Print the function docs.
                content.push_str(docs);

                // For this one function, we need it to be recursive since this is how you get
                // an access token when authenicating on behalf of an app with a JWT.
                if fn_name == "create_installation_access_token" {
                    content.push_str("#[async_recursion::async_recursion]");
                }

                if bounds.is_empty() {
                    content.push_str(&format!("pub async fn {}(", fn_name,));
                } else {
                    content.push_str(&format!("pub async fn {}<{}>(", fn_name, bounds.join(", ")));
                }
                content.push_str("&self,");

                if !fn_params_str.is_empty() {
                    content.push_str(&fn_params_str.join(" "));
                }

                if let Some(bp) = &body_param {
                    content.push_str(&format!("body: {}", bp));
                }

                content.push_str(&format!(") -> ClientResult<{}> {{", response_type));

                content.push_str(template);

                content.push_str(fn_inner);

                content.push('}');
                content.push_str("");

                content
            };

            let docs = get_fn_docs(o, m, p, parameters, ts)?;

            let mut bounds: Vec<String> = Vec::new();

            let mut body_content_type_header = None;

            // println!("{:?} {:?}", o.summary, o.request_body);

            let (body_param, body_func) = if let Some(b) = &o.request_body {
                if let Ok(b) = b.item() {
                    if b.is_binary()? {
                        let (ct, _) = b.content.first().unwrap();
                        body_content_type_header = Some(ct.to_string());

                        bounds.push("B: Into<reqwest::Body>".to_string());
                        (Some("B".to_string()), Some("body".to_string()))
                    } else {
                        let (ct, mt) = b.content.first().unwrap();
                        body_content_type_header = Some(ct.to_string());

                        if ct == "application/json"
                            || ct == "application/octet-stream"
                            || ct.contains("application/json")
                        {
                            if let Some(s) = &mt.schema {
                                let object_name = format!("{} request", oid_to_object_name(&od));
                                let id = ts.select(Some(&object_name), s, "")?;
                                let et = ts.id_to_entry.get(&id).unwrap();
                                if let crate::TypeDetails::Object(p, _) = &et.details {
                                    // We want to make sure we actally have properties
                                    // in our object.
                                    if p.is_empty() {
                                        (None, None)
                                    } else {
                                        let rt = ts.render_type(&id, false)?;
                                        if rt.starts_with("Vec") {
                                            (
                                                Some(format!(
                                                    "&[{}]",
                                                    rt.trim_start_matches("Vec<")
                                                        .trim_end_matches('>')
                                                )),
                                                Some("json".to_string()),
                                            )
                                        } else {
                                            (Some(format!("&{}", rt)), Some("json".to_string()))
                                        }
                                    }
                                } else {
                                    let rt = ts.render_type(&id, false)?;
                                    if rt.starts_with("Vec") {
                                        (
                                            Some(format!(
                                                "&[{}]",
                                                rt.trim_start_matches("Vec<").trim_end_matches('>')
                                            )),
                                            Some("json".to_string()),
                                        )
                                    } else {
                                        (Some(format!("&{}", rt)), Some("json".to_string()))
                                    }
                                }
                            } else {
                                (None, None)
                            }
                        } else if ct == "multipart/form-data" {
                            println!("got multipart/formdata for {}", oid);
                            // Skip it for now.
                            // TODO: fix this later.
                            (None, None)
                        } else if ct == "application/x-www-form-urlencoded" {
                            println!("got application/x-www-form-urlencoded for {}", oid);
                            // Skip it for now.
                            // TODO: fix this later.
                            (None, None)
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
                } else if let openapiv3::ReferenceOr::Reference { reference } = b {
                    // We must have had a reference.
                    let object_name = format!("{} request", oid_to_object_name(&od));
                    let id = ts.select_ref(Some(&clean_name(&object_name)), reference)?;
                    let rt = ts.render_type(&id, false)?;
                    (Some(format!("&{}", rt)), Some("json".to_string()))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            // println!("{body_content_type_header:?}");

            /*
             * Get the function parameters.
             */
            let (fn_params_str, query_params) =
                get_fn_params(ts, o, parameters, false, op.parameters.clone(), proper_name)?;

            // Generate the server to send the request to
            let server_arg = if o.servers.len() == 1 {
                let servers = generate_servers(&o.servers, &to_pascal_case(&op_id));
                let server_type = servers.top_level_type.unwrap();

                if let Some(server_out) = &servers.output {
                    out.add_head(server_out);
                }

                format!("Some({server_type}::default().default_url())")
            } else {
                "None".to_string()
            };

            /*
             * Generate the URL for the request.
             */
            let tmp = parse(p)?;
            let template = tmp.compile(query_params, &server_arg);

            /*
             * Get the response type.
             */
            let (mut response_type, tid, inner_response_type, pagination_property) =
                get_response_type(&od, ts, o)?;

            if proper_name == "GitHub" && response_type == "crate::types::Data" {
                response_type = "()".to_string();
            }
            // We shouldn't ever have an optional response type, thats just annoying.
            if response_type.starts_with("Option<") {
                response_type = response_type
                    .trim_start_matches("Option<")
                    .trim_end_matches('>')
                    .to_string();
            }

            let mut fn_inner = get_fn_inner(
                proper_name,
                &oid,
                m,
                &body_func,
                &response_type,
                &inner_response_type,
                &pagination_property,
                false,
                body_content_type_header.as_deref(),
            )?;

            // TODO: don't special case this.
            if p == "/jobs/{id}/transcript" || p == "/jobs/{id}/captions" {
                fn_inner =
                    r#"self.client.request_with_accept_mime(reqwest::Method::GET, &url, &accept.to_string()).await"#
                        .to_string();
                response_type = "String".to_string();
            }

            if let Some(te) = ts.id_to_entry.get(&tid) {
                // If we have a one of, we can generate a few different subfunctions to
                // help as well.
                if let crate::TypeDetails::OneOf(one_of, _) = &te.details {
                    for itid in one_of {
                        let rt = ts.render_type(itid, false)?;
                        out.add_content(&print_fn(
                            &docs,
                            &bounds,
                            &fn_params_str,
                            &body_param,
                            &rt,
                            &template,
                            &fn_inner,
                            &to_snake_case(&struct_name(&format!(
                                "{}_{}",
                                oid.trim_start_matches(&tag).trim_start_matches('_'),
                                to_snake_case(&rt.replace("crate::types::", ""))
                            ))),
                        ));
                    }
                }
            }

            // Get the function without the function inners.
            // This is specifically for Ramp.
            // We do this directly before we print the other function.
            let mut frt = response_type.to_string();
            if !inner_response_type.is_empty() {
                frt = inner_response_type.to_string();
            }

            let mut fn_name = oid
                .trim_start_matches(&tag)
                .trim_start_matches('_')
                .to_string();
            if proper_name != "GitHub"
                && !frt.starts_with("Vec<")
                && !frt.ends_with("Response")
                && !frt.ends_with("Summary")
                && http::Method::GET == m
                && !fn_name.ends_with("address")
                && !fn_name.ends_with("has")
                && !fn_name.ends_with("access")
            {
                // Make sure we don't add an s where we don't need one.
                // Don't make a function plural where it is not needed.
                fn_name = fn_name.trim_end_matches('s').to_string();
            } else if frt.starts_with("Vec<") && fn_name != "get" && fn_name != "list" {
                fn_name = make_plural(proper_name, &fn_name);
            } else if frt.starts_with("Vec<") && fn_name == "get" {
                fn_name = "get_page".to_string()
            }

            // Fix if we somehow created a function that is actually a keyword.
            if fn_name == "move" {
                fn_name = "mv".to_string();
            }

            // Do this right before printing. Check if we already have this function name.
            // This will ensure we don't have any duplicates.
            if fn_names.contains(&(fn_name.clone() + &tag)) {
                fn_name = format!("{}_{}", fn_name, tag);
            }
            fn_names.push(fn_name.clone() + &tag);

            // Print our standard function.
            out.add_content(&print_fn(
                &docs,
                &bounds,
                &fn_params_str,
                &body_param,
                &frt,
                &template,
                &fn_inner,
                &fn_name,
            ));

            // If we are returning a list of things and we have page, etc as
            // params, let's get all the pages.
            if frt.starts_with("Vec<") && http::Method::GET == m {
                let docs = get_fn_docs_all(
                    o,
                    m,
                    p,
                    oid.trim_start_matches(&tag).trim_start_matches('_'),
                )?;

                let (fn_params_str, query_params) =
                    get_fn_params(ts, o, parameters, true, op.parameters.clone(), proper_name)?;

                let tmp = parse(p)?;
                let template = tmp.compile(query_params, &server_arg);

                let fn_inner = get_fn_inner(
                    proper_name,
                    &oid,
                    m,
                    &body_func,
                    &response_type,
                    &inner_response_type,
                    &pagination_property,
                    true,
                    body_content_type_header.as_deref(),
                )?;

                let mut fn_name = oid
                    .replace("_get_", "_get_all_")
                    .replace("_list_", "_list_all_")
                    .trim_start_matches(&tag)
                    .trim_start_matches('_')
                    .to_string();

                if fn_name == "list" {
                    fn_name = "list_all".to_string();
                } else if fn_name == "get" {
                    fn_name = "get_all".to_string();
                } else if fn_name.starts_with("get_") && !fn_name.starts_with("get_all") {
                    fn_name = fn_name.replace("get_", "get_all_");
                } else if fn_name.starts_with("list_") && !fn_name.starts_with("list_all") {
                    fn_name = fn_name.replace("list_", "list_all_");
                } else if !fn_name.contains("get")
                    && !fn_name.contains("get_all")
                    && !fn_name.contains("list")
                    && !fn_name.contains("list_all")
                {
                    fn_name = format!("get_all_{}", fn_name);
                }

                if fn_name != "get_all"
                    && fn_name != "list_all"
                    && fn_name != "get"
                    && fn_name != "list"
                {
                    fn_name = make_plural(proper_name, &fn_name);
                }

                // Do this right before printing. Check if we already have this function name.
                // This will ensure we don't have any duplicates.
                if fn_names.contains(&(fn_name.clone() + &tag)) {
                    fn_name = format!("{}_all", fn_name);
                }
                fn_names.push(fn_name.clone() + &tag);

                // Now let's print the new function.
                out.add_content(&print_fn(
                    &docs,
                    &bounds,
                    &fn_params_str,
                    &body_param,
                    &frt,
                    &template,
                    &fn_inner,
                    &fn_name,
                ));
            }

            // Add this to our map of functions based on the tag name.
            // tag_files.insert(tag, out.to_string());

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

fn get_response_type_from_object(
    od: &str,
    ts: &mut TypeSpace,
    s: Option<&openapiv3::ReferenceOr<openapiv3::Schema>>,
    r: Option<&openapiv3::ReferenceOr<openapiv3::Response>>,
) -> Result<(
    String,        // original response type
    crate::TypeId, // type id
    String,        // optional vec response type if this struct paginates
    String,        // optional name of vec response property if this struct paginates
)> {
    let object_name = format!("{} response", oid_to_object_name(od));
    let mut tid = TypeId(0);

    if let Some(s) = s {
        tid = ts.select(Some(&clean_name(&object_name)), s, "")?;

        if let openapiv3::ReferenceOr::Reference { reference } = s {
            tid = ts.select_ref(Some(&clean_name(&object_name)), reference)?;
        }
    }

    if let Some(openapiv3::ReferenceOr::Reference { reference }) = r {
        tid = ts.select_ref(Some(&clean_name(&object_name)), reference)?;
    }

    if tid == TypeId(0) {
        bail!("should have gotten type id for {}", od);
    }

    let og_rt = ts.render_type(&tid, false)?;
    let mut et = ts.id_to_entry.get(&tid).unwrap();

    if let crate::TypeDetails::NamedType(id, _) = &et.details {
        et = ts.id_to_entry.get(id).unwrap();
    }

    if let crate::TypeDetails::Object(p, _) = &et.details {
        // For Ramp, the pagination values are passed _in_ the resulting
        // struct, so we want to ignore them and just get the data.
        if let Some(pid) = p.get("page") {
            let rt = ts.render_type(pid, false)?;
            if rt == "crate::types::Page" || rt.ends_with("Page") {
                if let Some(did) = p.get("data") {
                    let rt = ts.render_type(did, false)?;
                    return Ok((og_rt, did.clone(), rt, "data".to_string()));
                } else if p.len() == 2 {
                    // We know for the Ramp API there will only be two fields in
                    // these structs. This should help prevent errors.
                    // Let's find the value of the struct that is the vec!
                    for (n, id) in p {
                        let rt = ts.render_type(id, false)?;
                        if rt.starts_with("Vec<") {
                            return Ok((og_rt, id.clone(), rt, to_snake_case(n)));
                        }
                    }
                }
            }
        }

        // For Ramp, the pagination values are passed _in_ the resulting
        // struct, so we want to ignore them and just get the data.
        if p.get("has_more").is_some() {
            if let Some(did) = p.get("data") {
                let rt = ts.render_type(did, false)?;
                return Ok((og_rt, did.clone(), rt, "data".to_string()));
            } else if p.len() == 2 {
                // We know for the Ramp API there will only be two fields in
                // these structs. This should help prevent errors.
                // Let's find the value of the struct that is the vec!
                for (n, id) in p {
                    let rt = ts.render_type(id, false)?;
                    if rt.starts_with("Vec<") {
                        return Ok((og_rt, id.clone(), rt, to_snake_case(n)));
                    }
                }
            }
        }

        // For Zoom, the pagination values are passed _in_ the resulting
        // struct, so we want to ignore them and just get the data.
        if let Some(pid) = p.get("next_page_token") {
            let rt = ts.render_type(pid, false)?;
            if rt == "String" {
                for (n, id) in p {
                    // Now we must find the property with the vector for this struct.
                    let rt = ts.render_type(id, false)?;
                    if rt.starts_with("Vec<") {
                        return Ok((og_rt, id.clone(), rt, to_snake_case(n)));
                    }
                }
            }
        }

        // For Google, the pagination values are passed _in_ the resulting
        // struct, so we want to ignore them and just get the data.
        if let Some(pid) = p.get("nextPageToken") {
            let rt = ts.render_type(pid, false)?;
            if rt == "String" {
                if let Some(did) = p.get("items") {
                    let rt = ts.render_type(did, false)?;
                    return Ok((og_rt, did.clone(), rt, "items".to_string()));
                } else {
                    for (n, id) in p {
                        // Now we must find the property with the vector for this struct.
                        let rt = ts.render_type(id, false)?;
                        if rt.starts_with("Vec<") {
                            return Ok((og_rt, id.clone(), rt, to_snake_case(n)));
                        }
                    }
                }
            }
        }
    }

    Ok((og_rt, tid, "".to_string(), "".to_string()))
}

fn get_response_type(
    od: &str,
    ts: &mut TypeSpace,
    o: &openapiv3::Operation,
) -> Result<(
    String,        // original response type
    crate::TypeId, // type id
    String,        // optional vec response type if this struct paginates
    String,        // optional name of vec response property if this struct paginates
)> {
    // Get the first response.
    let first = o.responses.responses.first().unwrap();
    if let Ok(i) = first.1.item() {
        if i.content.is_empty() {
            // Return empty.
            return Ok((
                "()".to_string(),
                crate::TypeId(0),
                "".to_string(),
                "".to_string(),
            ));
        }

        // Get the json response, if it exists.
        if let Some(mt) = i.content.get("application/json") {
            if !mt.encoding.is_empty() {
                bail!("media type encoding not empty: {:#?}", mt);
            }

            if let Some(s) = &mt.schema {
                if let Ok(item) = s.item() {
                    // We have an item, we want to check
                    // if its an ANY kind and empty, then we can ignore it.
                    if let openapiv3::SchemaKind::Any(any) = &item.schema_kind {
                        if any.properties.is_empty() && any.format.is_none() && any.items.is_none()
                        {
                            // Return empty.
                            return Ok((
                                "()".to_string(),
                                crate::TypeId(0),
                                "".to_string(),
                                "".to_string(),
                            ));
                        }
                    }
                }

                // Get response type from object.
                return get_response_type_from_object(od, ts, Some(s), None);
            }
        }

        // Get the first response.
        let (ct, mt) = i.content.first().unwrap();
        if ct == "text/plain"
            || ct == "text/html"
            || ct == "application/octocat-stream"
            || ct == "*/*"
        {
            if let Some(s) = &mt.schema {
                let tid = ts.select(None, s, "")?;
                let rt = ts.render_type(&tid, false)?;
                return Ok((rt, tid, "".to_string(), "".to_string()));
            }
        } else if ct == "application/scim+json" {
            if !mt.encoding.is_empty() {
                bail!("media type encoding not empty: {:#?}", mt);
            }

            if let Some(s) = &mt.schema {
                let object_name = format!("{} response", oid_to_object_name(od));
                let tid = ts.select(Some(&clean_name(&object_name)), s, "")?;
                let rt = ts.render_type(&tid, false)?;
                return Ok((rt, tid, "".to_string(), "".to_string()));
            }
        }
    } else if let openapiv3::ReferenceOr::Reference { reference: _ } = first.1 {
        // Get response type from object.
        return get_response_type_from_object(od, ts, None, Some(first.1));
    }

    // Basically if we get here, likely its just an empty struct or something.
    // We never actually hit here before Zoom but then it was just an empty response.
    Ok((
        "()".to_string(),
        crate::TypeId(0),
        "".to_string(),
        "".to_string(),
    ))
}

#[allow(clippy::type_complexity)]
fn get_fn_params(
    ts: &mut TypeSpace,
    o: &openapiv3::Operation,
    parameters: &BTreeMap<String, &openapiv3::Parameter>,
    all_pages: bool,
    global_params: Vec<openapiv3::ReferenceOr<openapiv3::Parameter>>,
    proper_name: &str,
) -> Result<(Vec<String>, BTreeMap<String, (String, String)>)> {
    /*
     * Query parameters are sorted lexicographically to ensure a stable
     * order in the generated code.
     */
    let mut fn_params_str: Vec<String> = Default::default();
    let mut fn_params: Vec<String> = Default::default();
    let mut query_params: BTreeMap<String, (String, String)> = Default::default();
    let mut gp = global_params;
    let mut op = o.parameters.clone();
    gp.append(&mut op);
    for par in gp.iter() {
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

        if !fn_params.contains(nam) && !fn_params.contains(&format!("{}_", nam)) {
            let typ = parameter_data.render_type(&param_name, ts)?;
            if nam == "ref"
                || nam == "type"
                || nam == "foo"
                || nam == "enum"
                || nam == "const"
                || nam == "use"
            {
                fn_params_str.push(format!("{}_: {},", nam, typ));
                fn_params.push(nam.to_string() + "_");
            } else if nam == "i_ds" {
                fn_params_str.push(format!("ids: {},", typ));
                fn_params.push("ids".to_string());
            } else if (!all_pages || !is_page_param(nam, proper_name))
                && (nam != "authorization" || proper_name == "Stripe")
                && !nam.starts_with("authorization_bearer")
                && (!proper_name.starts_with("Google")
                    || !is_google_unnecessary_param(proper_name, nam))
                && (proper_name != "SendGrid" || !is_sendgrid_unnecessary_param(nam))
                && (proper_name != "Slack" || !is_slack_unnecessary_param(nam))
                && (proper_name != "Okta" || !is_okta_unnecessary_param(nam))
                && (proper_name != "ShipBob" || !is_shipbob_unnecessary_param(nam))
                && (proper_name != "Stripe" || !is_stripe_unnecessary_param(nam))
            {
                if typ == "chrono::DateTime<chrono::Utc>" {
                    fn_params_str.push(format!("{}: Option<{}>,", nam, typ));
                    fn_params.push(nam.to_string());
                } else {
                    let p = format!("{}: {},", nam, typ);
                    if !fn_params.contains(nam) {
                        fn_params_str.push(p);
                        fn_params.push(nam.to_string());
                    }
                }
            }

            // Check if we have a query.
            // TODO: make this a bool ext.
            if let openapiv3::Parameter::Query {
                parameter_data: _,
                allow_reserved: _,
                style: openapiv3::QueryStyle::Form,
                // We can ignore the allow empty value, we support this by default and
                // aren't strict about not allowing empty values on other parameters
                // merely because specs cannot be trusted.
                allow_empty_value: _,
            } = item
            {
                if nam == "ref"
                    || nam == "type"
                    || nam == "foo"
                    || nam == "enum"
                    || nam == "const"
                    || nam == "use"
                {
                    query_params.insert(
                        format!("{}_", nam),
                        (typ.to_string(), parameter_data.name.to_string()),
                    );
                } else if nam == "i_ds" {
                    query_params.insert(
                        "ids".to_string(),
                        (typ.to_string(), parameter_data.name.to_string()),
                    );
                } else if (!all_pages || !is_page_param(nam, proper_name))
                    && (nam != "authorization" || proper_name == "Stripe")
                    && !nam.starts_with("authorization_bearer")
                    && (!proper_name.starts_with("Google")
                        || !is_google_unnecessary_param(proper_name, nam))
                    && (proper_name != "SendGrid" || !is_sendgrid_unnecessary_param(nam))
                    && (proper_name != "Slack" || !is_slack_unnecessary_param(nam))
                    && (proper_name != "Okta" || !is_okta_unnecessary_param(nam))
                    && (proper_name != "ShipBob" || !is_shipbob_unnecessary_param(nam))
                    && (proper_name != "Stripe" || !is_stripe_unnecessary_param(nam))
                {
                    if typ == "chrono::DateTime<chrono::Utc>" {
                        query_params.insert(
                            nam.to_string(),
                            (format!("Option<{}>", typ), parameter_data.name.to_string()),
                        );
                    } else {
                        query_params.insert(
                            nam.to_string(),
                            (typ.to_string(), parameter_data.name.to_string()),
                        );
                    }
                }
            }
        }
    }

    Ok((fn_params_str, query_params))
}

/*
 * Perform the function.
 */
// TODO: Fix this
#[allow(clippy::too_many_arguments)]
fn get_fn_inner(
    proper_name: &str,
    oid: &str,
    m: &str,
    body_func: &Option<String>,
    response_type: &str,
    inner_response_type: &str,
    pagination_property: &str,
    all_pages: bool,
    content_type: Option<&str>,
) -> Result<String> {
    let body = if let Some(f) = &body_func {
        if f == "json" {
            "Some(reqwest::Body::from(serde_json::to_vec(body)?))"
        } else {
            "Some(body.into())"
        }
    } else {
        "None"
    };

    let content_type = content_type
        .map(|c| format!(r#"Some("{c}".to_string())"#))
        .unwrap_or_else(|| String::from("None"));

    if all_pages && pagination_property.is_empty() {
        return Ok(format!("self.client.get_all_pages(&url, crate::Message {{ body: {}, content_type: None }}).await", body));
    } else if all_pages && proper_name.starts_with("Stripe") {
        // We will do a custom function here.
        let inner = format!(
            r#"let mut resp: {} = self.client.{}(&url, crate::Message {{ body: {}, content_type: None }}).await?;

            let mut {} = resp.{};
            let mut has_more = resp.has_more;
            let mut page = "".to_string();

            // Paginate if we should.
            while has_more {{
                if !{}.is_empty() {{
                   let last = {}.last().unwrap();
                   let j = serde_json::json!(last);
                   if let serde_json::Value::Object(o) = j {{
                       if let Some(serde_json::Value::String(s)) = o.get("id") {{
                            page = s.to_string();
                       }}
                   }}
                }}

                if !url.contains('?') {{
                    resp = self.client.{}(&format!("{{}}?startng_after={{}}", url, page), crate::Message {{ body: {}, content_type: None }}).await?;
                }} else {{
                    resp = self.client.{}(&format!("{{}}&starting_after={{}}", url, page), crate::Message {{ body: {}, content_type: None }}).await?;
                }}


                {}.append(&mut resp.{});

                has_more = resp.has_more;
            }}

            // Return our response data.
            Ok({}.to_vec())"#,
            response_type,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            pagination_property,
            pagination_property,
            m.to_lowercase(),
            body,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            pagination_property,
        );

        return Ok(inner);
    } else if all_pages && proper_name.starts_with("Google") {
        // We will do a custom function here.
        let inner = format!(
            r#"let mut resp: {} = self.client.{}(&url, crate::Message {{ body: {}, content_type: None }}).await?;

            let mut {} = resp.{};
            let mut page = resp.next_page_token;

            // Paginate if we should.
            while !page.is_empty() {{
                if !url.contains('?') {{
                    resp = self.client.{}(&format!("{{}}?pageToken={{}}", url, page), crate::Message {{ body: {}, content_type: None }}).await?;
                }} else {{
                    resp = self.client.{}(&format!("{{}}&pageToken={{}}", url, page), crate::Message {{ body: {}, content_type: None }}).await?;
                }}


                {}.append(&mut resp.{});

                if !resp.next_page_token.is_empty() && resp.next_page_token != page {{
                    page = resp.next_page_token.to_string();
                }} else {{
                    page = "".to_string();
                }}
            }}

            // Return our response data.
            Ok({})"#,
            response_type,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            m.to_lowercase(),
            body,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            pagination_property,
        );

        return Ok(inner);
    } else if all_pages && proper_name == "Ramp" {
        // We will do a custom function here.
        let inner = format!(
            r#"let resp: {} = self.client.{}(&url, crate::Message {{ body: {}, content_type: None }}).await?;

            let mut {} = resp.{};
            let mut page = resp.page.next.to_string();

            // Paginate if we should.
            while !page.is_empty() {{
                match self.client.{}::<{}>(page.trim_start_matches(&self.client.host), crate::Message {{ body: {}, content_type: None }}).await {{
                    Ok(mut resp) => {{
                        {}.append(&mut resp.{});

                        page = if resp.page.next != page {{
                            resp.page.next.to_string()
                        }} else {{
                        "".to_string()
                        }};
                    }},
                    Err(e) => {{
                        if e.to_string().contains("404 Not Found") {{
                            page = "".to_string();
                        }} else {{
                            return Err(e);
                        }}
                    }}
                }}
            }}

            // Return our response data.
            Ok({})"#,
            response_type,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            m.to_lowercase(),
            response_type,
            body,
            pagination_property,
            pagination_property,
            pagination_property,
        );

        return Ok(inner);
    } else if all_pages && proper_name == "TripActions" {
        // We will do a custom function here.
        let inner = format!(
            r#"
            let mut resp: {} = if !url.contains('?') {{
                self.client.{}(&format!("{{}}?page=0&size=100", url), crate::Message {{ body: {}, content_type: None }}).await?
            }} else {{
                self.client.{}(&format!("{{}}&page=0&size=100", url), crate::Message {{ body: {}, content_type: None }}).await?
            }};

            let mut {} = resp.{};
            let mut page = resp.page.current_page + 1;

            // Paginate if we should.
            while page <= (resp.page.total_pages - 1) {{
                if !url.contains('?') {{
                    resp = self.client.{}(&format!("{{}}?page={{}}&size=100", url, page), crate::Message {{ body: {}, content_type: None }}).await?;
                }} else {{
                    resp = self.client.{}(&format!("{{}}&page={{}}&size=100", url, page), crate::Message {{ body: {}, content_type: None }}).await?;
                }}

                {}.append(&mut resp.{});

                page = resp.page.current_page + 1;
            }}

            // Return our response data.
            Ok({})"#,
            response_type,
            m.to_lowercase(),
            body,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            m.to_lowercase(),
            body,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            pagination_property,
        );

        return Ok(inner);
    } else if all_pages && proper_name == "Zoom" {
        // We will do a custom function here.
        let inner = format!(
            r#"let mut resp: {} = self.client.{}(&url, crate::Message {{ body: {}, content_type: None }}).await?;

            let mut {} = resp.{};
            let mut page = resp.next_page_token;

            // Paginate if we should.
            while !page.is_empty() {{
                // Check if we already have URL params and need to concat the token.
                if !url.contains('?') {{
                    resp = self.client.{}(&format!("{{}}?next_page_token={{}}", url, page), crate::Message {{ body: {}, content_type: None }}).await?;
                }} else {{
                    resp = self.client.{}(&format!("{{}}&next_page_token={{}}", url, page), crate::Message {{ body: {}, content_type: None }}).await?;
                }}

                {}.append(&mut resp.{});

                if !resp.next_page_token.is_empty() && resp.next_page_token != page {{
                    page = resp.next_page_token.to_string();
                }} else {{
                    page = "".to_string();
                }}
            }}

            // Return our response data.
            Ok({})"#,
            response_type,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            m.to_lowercase(),
            body,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            pagination_property,
        );

        return Ok(inner);
    } else if all_pages && !pagination_property.is_empty() {
        bail!(
            "must implement custom pagination function for {} {}",
            proper_name,
            pagination_property
        );
    }

    if (m == http::Method::GET
        || m == http::Method::POST
        || m == http::Method::PATCH
        || m == http::Method::PUT
        || m == http::Method::DELETE)
        && oid != "apps_create_installation_access_token"
    {
        if inner_response_type.is_empty() {
            return Ok(format!(
                "self.client.{}(&url, crate::Message {{ body: {}, content_type: {content_type} }} ).await",
                m.to_lowercase(),
                body
            ));
        }

        let additional = if inner_response_type.starts_with("Vec<") {
            ".to_vec()"
        } else {
            ""
        };

        // Okay we have an inner response type, let's return that instead.
        return Ok(format!(
            r#"let resp: {} = self.client.{}(&url, crate::Message {{ body: {}, content_type: {content_type} }}).await?;

                // Return our response data.
                Ok(resp.{}{})"#,
            response_type,
            m.to_lowercase(),
            body,
            pagination_property,
            additional
        ));
    }

    if oid != "apps_create_installation_access_token" {
        bail!("function {} should be authenticated", oid);
    }

    Ok(format!(
        r#"self.client.post_media(
            &url,
            crate::Message {{
                body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                content_type: {content_type},
            }},
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::JWT,
        ).await"#
    ))
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
                docs = format!(" -- {}.", d.trim_end_matches('.').replace('\n', "\n*   "));
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

        if nam == "ref"
            || nam == "type"
            || nam == "foo"
            || nam == "enum"
            || nam == "const"
            || nam == "use"
        {
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

fn is_page_param(s: &str, proper_name: &str) -> bool {
    s == "page"
        || s == "per_page"
        || s == "per"
        || s == "page_size"
        || s == "size"
        || s == "next_page_token"
        || s == "page_token"
        || s == "max_results"
        || s == "page_number"
        || s == "start"
        || s == "sync_token"
        || s == "limit"
        || s == "ending_before"
        || (s == "after" && proper_name == "Okta")
        || (s == "starting_after" && proper_name == "Stripe")
}

fn is_google_unnecessary_param(proper_name: &str, s: &str) -> bool {
    // These are either dumb or depreciated.
    s == "access_token"
        || s == "oauth_token"
        || s == "pretty_print"
        || s == "xgafv"
        || s == "custom_field_mask"
        || s == "sync_token"
        || s == "user_ip"
        || s == "quota_user"
        || s == "key"
        || s == "fields"
        || s == "callback"
        || s == "upload_protocol"
        || s == "upload_type"
        || s == "always_include_email"
        || s == "enforce_single_parent"
        || s == "corpus"
        || (s == "alt" && proper_name != "Google Groups Settings")
}

fn is_slack_unnecessary_param(s: &str) -> bool {
    s == "token"
}

fn is_sendgrid_unnecessary_param(s: &str) -> bool {
    s == "on_behalf_of" || s == "accept" || s == "x_query_id" || s == "x_cursor"
}

fn is_okta_unnecessary_param(s: &str) -> bool {
    s == "okta_access_gateway_agent"
        || s == "x_forwarded_for"
        || s == "user_agent"
        || s == "accept_language"
}

fn is_shipbob_unnecessary_param(s: &str) -> bool {
    s == "shipbob_channel_id"
}

fn is_stripe_unnecessary_param(s: &str) -> bool {
    s == "expand"
}
