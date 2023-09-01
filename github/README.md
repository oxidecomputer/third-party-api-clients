# `octorust`

A fully generated, opinionated API client library for GitHub.


[![docs.rs](https://docs.rs/octorust/badge.svg)](https://docs.rs/octorust)

## API Details

GitHub's v3 REST API.

[API Terms of Service](https://docs.github.com/articles/github-terms-of-service)

### Contact


| name | url |
|----|----|
| Support | <https://support.github.com/contact?tags=rest-api> |

### License


| name | url |
|----|----|
| MIT | <https://spdx.org/licenses/MIT> |


## Client Details

This client is generated from the [GitHub OpenAPI
specs](https://github.com/github/rest-api-description) based on API spec version `1.1.4`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
octorust = "0.7.1"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of `auth::Credentials`.

```rust
use octorust::{auth::Credentials, Client};

let github = Client::new(
  String::from("user-agent-name"),
  Credentials::Token(
    String::from("personal-access-token")
  ),
);
```

If you are a GitHub enterprise customer, you will want to create a client with the
[Client#host_override](https://docs.rs/octorust/0.7.1/octorust/struct.Client.html#method.host_override) method.

## Feature flags

### httpcache

Github supports conditional HTTP requests using etags to checksum responses
Experimental support for utilizing this to cache responses locally with the
`httpcache` feature flag.

To enable this, add the following to your `Cargo.toml` file:

```toml
[dependencies]
octorust = { version = "0.7.1", features = ["httpcache"] }
```

Then use the `Client::custom` constructor to provide a cache implementation.

Here is an example:

```rust
use octorust::{auth::Credentials, Client};
#[cfg(feature = "httpcache")]
use octorust::http_cache::HttpCache;

#[cfg(feature = "httpcache")]
let http_cache = HttpCache::in_home_dir();

#[cfg(not(feature = "httpcache"))]
let github = Client::custom(
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    Credentials::Token(
      String::from("personal-access-token")
    ),
    reqwest::Client::builder().build().unwrap(),
);

#[cfg(feature = "httpcache")]
let github = Client::custom(
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    Credentials::Token(
      String::from("personal-access-token")
    ),
    reqwest::Client::builder().build().unwrap(),
    http_cache
);
```
## Authenticating GitHub apps

You can also authenticate via a GitHub app.

Here is an example:

```rust
use std::env;

use octorust::{Client, auth::{Credentials, InstallationTokenGenerator, JWTCredentials}};
#[cfg(feature = "httpcache")]
use octorust::http_cache::FileBasedCache;
use base64::{Engine, engine::general_purpose::STANDARD};

let app_id_str = env::var("GH_APP_ID").unwrap();
let app_id = app_id_str.parse::<u64>().unwrap();

let app_installation_id_str = env::var("GH_INSTALLATION_ID").unwrap();
let app_installation_id = app_installation_id_str.parse::<u64>().unwrap();

let encoded_private_key = env::var("GH_PRIVATE_KEY").unwrap();
let private_key = STANDARD.decode(encoded_private_key).unwrap();

// Decode the key.
let key = nom_pem::decode_block(&private_key).unwrap();

// Get the JWT credentials.
let jwt = JWTCredentials::new(app_id, key.data).unwrap();

// Create the HTTP cache.
#[cfg(feature = "httpcache")]
let mut dir = dirs::home_dir().expect("Expected a home dir");
#[cfg(feature = "httpcache")]
dir.push(".cache/github");
#[cfg(feature = "httpcache")]
let http_cache = Box::new(FileBasedCache::new(dir));

let token_generator = InstallationTokenGenerator::new(app_installation_id, jwt);

#[cfg(not(feature = "httpcache"))]
let github = Client::custom(
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    Credentials::InstallationToken(token_generator),
    reqwest::Client::builder().build().unwrap(),
);

#[cfg(feature = "httpcache")]
let github = Client::custom(
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    Credentials::InstallationToken(token_generator),
    reqwest::Client::builder().build().unwrap(),
    http_cache,
);
```

## Acknowledgements

Shout out to [hubcaps](https://github.com/softprops/hubcaps) for paving the
way here. This extends that effort in a generated way so the library is
always up to the date with the OpenAPI spec and no longer requires manual
contributions to add new endpoints.
