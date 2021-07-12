A fully generated, opinionated API client library for GitHub.

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of `Credentials`.

```
use github_api_client::{auth::Credentials, Client};

let github = Client::new(
  String::from("user-agent-name"),
  Credentials::Token(
    String::from("personal-access-token")
  ),
);
```

If you are a GitHub enterprise customer, you will want to create a client with the
[Client#host](struct.Client.html#method.host) method.

## Feature flags

### `httpcache`

Github supports conditional HTTP requests using etags to checksum responses
Experimental support for utilizing this to cache responses locally with the
`httpcache` feature flag.

To enable this, add the following to your `Cargo.toml` file:

```toml
[dependencies.github_api_client]
 version = "..."
 default-features = false
 features = ["httpcache"]
```

Then use the `Client::custom` constructor to provide a cache implementation.

Here is an example:

```
use github_api_client::{auth::Credentials, Client};
#[cfg(feature = "httpcache")]
use github_api_client::http_cache::HttpCache;

#[cfg(feature = "httpcache")]
let http_cache = HttpCache::in_home_dir();

#[cfg(not(feature = "httpcache"))]
let github = Client::custom(
    "https://api.github.com",
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    Credentials::Token(
      String::from("personal-access-token")
    ),
    reqwest::Client::builder().build().unwrap(),
);

#[cfg(feature = "httpcache")]
let github = Client::custom(
    "https://api.github.com",
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

use github_api_client::{Client, auth::{Credentials, InstallationTokenGenerator, JWTCredentials}};
#[cfg(feature = "httpcache")]
use github_api_client::http_cache::FileBasedCache;

let app_id_str = env::var("GH_APP_ID").unwrap();
let app_id = app_id_str.parse::<u64>().unwrap();

let app_installation_id_str = env::var("GH_INSTALLATION_ID").unwrap();
let app_installation_id = app_installation_id_str.parse::<u64>().unwrap();

let encoded_private_key = env::var("GH_PRIVATE_KEY").unwrap();
let private_key = base64::decode(encoded_private_key).unwrap();

// Decode the key.
let key = nom_pem::decode_block(&private_key).unwrap();

// Get the JWT credentials.
let jwt = JWTCredentials::new(app_id, key.data).unwrap();

#[cfg(feature = "httpcache")]
{
    // Create the HTTP cache.
    let mut dir = dirs::home_dir().expect("Expected a home dir");
    dir.push(".cache/github");
    let http_cache = Box::new(FileBasedCache::new(dir));
}

let token_generator = InstallationTokenGenerator::new(app_installation_id, jwt);

#[cfg(not(feature = "httpcache"))]
let github = Client::custom(
    "https://api.github.com",
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    Credentials::InstallationToken(token_generator),
    reqwest::Client::builder().build().unwrap(),
);

#[cfg(feature = "httpcache")]
let github = Client::custom(
    "https://api.github.com",
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
