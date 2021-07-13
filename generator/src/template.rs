use std::collections::BTreeMap;

use anyhow::{anyhow, bail, Context, Result};

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
        let mut out = "let url = ".to_string();
        if self.components.is_empty() && query_params.is_empty() {
            out.push_str(r#""".to_string();"#);
        } else {
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
            } else {
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
                    out.push('?');
                    for (i, key) in query_params.keys().enumerate() {
                        if i > 0 {
                            out.push('&');
                        }
                        out.push_str(&format!("{}={{}}", key));
                    }
                }
                out.push_str("\",\n");
                for c in self.components.iter() {
                    if let Component::Parameter(n) = &c {
                        if n == "type" || n == "ref" {
                            out.push_str(&format!(
                                "crate::progenitor_support::encode_path(&{}_.to_string()),",
                                n
                            ));
                        } else {
                            out.push_str(&format!(
                                "crate::progenitor_support::encode_path(&{}.to_string()),",
                                n
                            ));
                        }
                    }
                }
                if !query_params.is_empty() {
                    for key in query_params.values() {
                        out.push_str(&format!("{}, ", key));
                    }
                }
                out.push_str(");\n");
            }
        }
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
        let want = "let url = format!(\"/measure/{}\",
crate::progenitor_support::encode_path(&number.to_string()),);\n";
        assert_eq!(want, &out);
        Ok(())
    }
}

pub fn generate_docs(name: &str, version: &str) -> String {
    format!(
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
        name, version, name, name, version, name, name, version, name, name, name, name
    )
}
