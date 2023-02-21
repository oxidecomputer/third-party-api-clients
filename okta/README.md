# `okta`

A fully generated, opinionated API client library for Okta.

[![docs.rs](https://docs.rs/okta/badge.svg)](https://docs.rs/okta)

## API Details

Allows customers to easily access the Okta API

[API Terms of Service](http://developer.okta.com/terms/)

### Contact


| name | url | email |
|----|----|----|
| Okta Developer Team | <http://developer.okta.com/> | devex-public@okta.com |

### License


| name | url |
|----|----|
| Apache-2.0 | <http://www.apache.org/licenses/LICENSE-2.0.html> |


## Client Details

This client is generated from the [Okta OpenAPI
specs](https://github.com/okta/okta-management-openapi-spec) based on API spec version `2.5.0`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
okta = "0.4.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use okta::Client;

let okta = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `OKTA_API_KEY`

And then you can create a client from the environment.

```
use okta::Client;

let okta = Client::new_from_env();
```
