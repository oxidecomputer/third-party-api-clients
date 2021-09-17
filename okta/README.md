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
okta = "0.2.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use okta::Client;

let okta = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `OKTA_CLIENT_ID`
- `OKTA_CLIENT_SECRET`
- `OKTA_REDIRECT_URI`

And then you can create a client from the environment.

```
use okta::Client;

let okta = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use okta::Client;

async fn do_call() {
    let mut okta = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = okta.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = okta.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = okta.refresh_access_token().await.unwrap();
}
```
