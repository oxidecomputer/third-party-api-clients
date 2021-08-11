use std::collections::BTreeMap;

use anyhow::{anyhow, bail, Context, Result};
use inflector::cases::snakecase::to_snake_case;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Component {
    Constant(String),
    Parameter(String),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Template {
    components: Vec<Component>,
}

impl Template {
    pub fn compile(&self, query_params: BTreeMap<String, String>) -> String {
        let mut out = String::new();

        let mut a = |s: &str| {
            out.push_str(s);
            out.push('\n');
        };

        if !query_params.is_empty() {
            // Format the query params if they exist.
            a("let mut query = String::new();");
            a("let mut query_args: Vec<String> = Default::default();");

            for (nam, value) in &query_params {
                if value == "Option<chrono::DateTime<chrono::Utc>>" {
                    a(&format!(
                        r#"if let Some(date) = {} {{ query_args.push(format!("{}={{}}", &date.to_rfc3339())); }}"#,
                        nam, nam
                    ));
                } else if value == "i64" || value == "i32" {
                    a(&format!(
                        r#"if {} > 0 {{ query_args.push(format!("{}={{}}", {})); }}"#,
                        nam,
                        nam.trim_end_matches('_'),
                        nam
                    ));
                } else if value == "bool" {
                    a(&format!(
                        r#"if {} {{ query_args.push(format!("{}={{}}", {})); }}"#,
                        nam,
                        nam.trim_end_matches('_'),
                        nam
                    ));
                } else if value == "&str" {
                    a(&format!(
                        r#"if !{}.is_empty() {{ query_args.push(format!("{}={{}}", {})); }}"#,
                        nam,
                        nam.trim_end_matches('_'),
                        nam
                    ));
                } else if value == "&[String]" {
                    // TODO: I have no idea how these should be seperated and the docs
                    // don't give any answers either, for an array sent through query
                    // params.
                    // https://docs.github.com/en/rest/reference/migrations
                    a(&format!(
                        r#"if !{}.is_empty() {{ query_args.push(format!("{}={{}}", {}.join(" "))); }}"#,
                        nam,
                        nam.trim_end_matches('_'),
                        nam
                    ));
                } else {
                    a(&format!(
                        r#"query_args.push(format!("{}={{}}", {}));"#,
                        nam.trim_end_matches('_'),
                        nam
                    ));
                }
            }

            a(r#"for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }"#);
        }

        a("let url =");
        if self.components.is_empty() && query_params.is_empty() {
            a(r#""".to_string();"#);

            return out.to_string();
        }

        let mut has_params = false;
        for c in self.components.iter() {
            match c {
                Component::Constant(_) => (),
                Component::Parameter(_) => {
                    has_params = true;
                    break;
                }
            }
        }

        if !has_params && query_params.is_empty() {
            out.push('"');
            for c in self.components.iter() {
                out.push('/');
                match c {
                    Component::Constant(n) => out.push_str(n),
                    Component::Parameter(_) => (),
                }
            }
            out.push_str("\".to_string();");

            return out.to_string();
        }

        out.push_str("format!(\"");
        for c in self.components.iter() {
            out.push('/');
            match c {
                Component::Constant(n) => out.push_str(n),
                Component::Parameter(_) => {
                    out.push_str("{}");
                }
            }
        }

        if !query_params.is_empty() {
            out.push_str("?{}");
        }

        out.push_str("\",\n");
        for c in self.components.iter() {
            if let Component::Parameter(n) = &c {
                if n == "type" || n == "ref" {
                    out.push_str(&format!(
                        "crate::progenitor_support::encode_path(&{}_.to_string()),",
                        to_snake_case(n)
                    ));
                } else {
                    out.push_str(&format!(
                        "crate::progenitor_support::encode_path(&{}.to_string()),",
                        to_snake_case(n)
                    ));
                }
            }
        }

        if !query_params.is_empty() {
            out.push_str("query");
        }

        out.push_str(");\n");

        out
    }
}

pub fn parse(t: &str) -> Result<Template> {
    parse_inner(t).with_context(|| anyhow!("parse failure for template {:?}", t))
}

fn parse_inner(t: &str) -> Result<Template> {
    enum State {
        Start,
        ConstantOrParameter,
        Parameter,
        ParameterSlash,
        Constant,
    }

    let mut s = State::Start;
    let mut a = String::new();
    let mut components = Vec::new();

    for c in t.chars() {
        match s {
            State::Start => {
                if c == '/' {
                    s = State::ConstantOrParameter;
                } else {
                    bail!("path must start with a slash");
                }
            }
            State::ConstantOrParameter => {
                if c == '/' || c == '}' {
                    bail!("expected a constant or parameter");
                } else if c == '{' {
                    s = State::Parameter;
                } else {
                    s = State::Constant;
                    a.push(c);
                }
            }
            State::Constant => {
                if c == '/' {
                    components.push(Component::Constant(a));
                    a = String::new();
                    s = State::ConstantOrParameter;
                } else if c == '{' || c == '}' {
                    bail!("unexpected parameter");
                } else {
                    a.push(c);
                }
            }
            State::Parameter => {
                if c == '}' {
                    components.push(Component::Parameter(a));
                    a = String::new();
                    s = State::ParameterSlash;
                } else if c == '/' || c == '{' {
                    bail!("expected parameter");
                } else {
                    a.push(c);
                }
            }
            State::ParameterSlash => {
                if c == '/' {
                    s = State::ConstantOrParameter;
                } else {
                    bail!("expected a slash after parameter");
                }
            }
        }
    }

    match s {
        State::Start => bail!("empty path"),
        State::ConstantOrParameter | State::ParameterSlash => (),
        State::Constant => components.push(Component::Constant(a)),
        State::Parameter => bail!("unterminated parameter"),
    }

    Ok(Template { components })
}

#[cfg(test)]
mod test {
    use anyhow::{anyhow, Context, Result};

    use super::{parse, Component, Template};

    #[test]
    fn basic() -> Result<()> {
        let trials = vec![
            (
                "/info",
                Template {
                    components: vec![Component::Constant("info".into())],
                },
            ),
            (
                "/measure/{number}",
                Template {
                    components: vec![
                        Component::Constant("measure".into()),
                        Component::Parameter("number".into()),
                    ],
                },
            ),
            (
                "/one/{two}/three",
                Template {
                    components: vec![
                        Component::Constant("one".into()),
                        Component::Parameter("two".into()),
                        Component::Constant("three".into()),
                    ],
                },
            ),
        ];

        for (path, want) in trials.iter() {
            let t = parse(path).with_context(|| anyhow!("path {}", path))?;
            assert_eq!(&t, want);
        }

        Ok(())
    }

    #[test]
    fn compile() -> Result<()> {
        let t = parse("/measure/{number}")?;
        let out = t.compile(Default::default());
        let want = "let url =
format!(\"/measure/{}\",
crate::progenitor_support::encode_path(&number.to_string()),);\n";
        assert_eq!(want, &out);
        Ok(())
    }
}

pub fn generate_docs_github(name: &str, version: &str, proper_name: &str, host: &str) -> String {
    format!(
        r#"//! A fully generated, opinionated API client library for {}.
//!
//! This library is generated from the [{} OpenAPI
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
//! let {} = Client::new(
//!   String::from("user-agent-name"),
//!   Credentials::Token(
//!     String::from("personal-access-token")
//!   ),
//! );
//! ```
//!
//! If you are a GitHub enterprise customer, you will want to create a client with the
//! [Client#host](https://docs.rs/{}/{}/{}/struct.Client.html#method.host) method.
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
//! let {} = Client::custom(
//!     "https://{}",
//!     concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
//!     Credentials::Token(
//!       String::from("personal-access-token")
//!     ),
//!     reqwest::Client::builder().build().unwrap(),
//! );
//!
//! #[cfg(feature = "httpcache")]
//! let {} = Client::custom(
//!     "https://{}",
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
//! // Create the HTTP cache.
//! #[cfg(feature = "httpcache")]
//! let mut dir = dirs::home_dir().expect("Expected a home dir");
//! #[cfg(feature = "httpcache")]
//! dir.push(".cache/github");
//! #[cfg(feature = "httpcache")]
//! let http_cache = Box::new(FileBasedCache::new(dir));
//!
//! let token_generator = InstallationTokenGenerator::new(app_installation_id, jwt);
//!
//! #[cfg(not(feature = "httpcache"))]
//! let {} = Client::custom(
//!     "https://{}",
//!     concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
//!     Credentials::InstallationToken(token_generator),
//!     reqwest::Client::builder().build().unwrap(),
//! );
//!
//! #[cfg(feature = "httpcache")]
//! let {} = Client::custom(
//!     "https://{}",
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
        proper_name,
        proper_name,
        name,
        version,
        name,
        proper_name.to_lowercase(),
        name,
        version,
        name,
        name,
        version,
        name,
        name,
        proper_name.to_lowercase(),
        host,
        proper_name.to_lowercase(),
        host,
        name,
        name,
        proper_name.to_lowercase(),
        host,
        proper_name.to_lowercase(),
        host,
    )
}

pub fn generate_docs_generic_token(name: &str, version: &str, proper_name: &str) -> String {
    format!(
        r#"//! A fully generated, opinionated API client library for {}.
//!
//! This library is generated from the {} OpenAPI
//! specs. This way it will remain
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
//! a user agent string and set of credentials.
//!
//! ```
//! use {}::Client;
//!
//! let {} = Client::new(
//!     String::from("client-id"),
//!     String::from("client-secret"),
//!     String::from("redirect-uri"),
//!     String::from("token"),
//!     String::from("refresh-token")
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `{}_CLIENT_ID`
//! - `{}_CLIENT_SECRET`
//! - `{}_REDIRECT_URI`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use {}::Client;
//!
//! let {} = Client::new_from_env(
//!     String::from("token"),
//!     String::from("refresh-token")
//! );
//! ```
//!
//! It is okay to pass empty values for token and `refresh_token`. In
//! the initial state of the client, you will not know these values.
//!
//! To start off a fresh client and get a `token` and `refresh_token`, use the following.
//!
//! ```
//! use {}::Client;
//!
//! async fn do_call() {{
//!     let {} = Client::new_from_env("", "");
//!
//!     // Get the URL to request consent from the user.
//!     let user_consent_url = {}.user_consent_url();
//!
//!     // In your redirect URL capture the code sent.
//!     // Send it along to the request for the token.
//!     let code = "thing-from-redirect-url";
//!     let mut access_token = {}.get_access_token(code).await.unwrap();
//!
//!     // You can additionally refresh the access token with the following.
//!     // You must have a refresh token to be able to call this function.
//!     access_token = {}.refresh_access_token().await.unwrap();
//! }}
//! ```
//!"#,
        proper_name,
        proper_name,
        name,
        version,
        name,
        proper_name.to_lowercase(),
        proper_name.to_uppercase(),
        proper_name.to_uppercase(),
        proper_name.to_uppercase(),
        name,
        proper_name.to_lowercase(),
        name,
        proper_name.to_lowercase(),
        proper_name.to_lowercase(),
        proper_name.to_lowercase(),
        proper_name.to_lowercase(),
    )
}
