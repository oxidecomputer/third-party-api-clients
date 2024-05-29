# `gusto-api`

A fully generated, opinionated API client library for Gusto.


[![docs.rs](https://docs.rs/gusto-api/badge.svg)](https://docs.rs/gusto-api)

## API Details

Welcome to Gusto's API documentation.

[API Terms of Service](https://gusto.com/about/terms/developer-terms-of-service)

### Contact


| name | email |
|----|----|
| Developer Relations | developer@gusto.com |



## Client Details

This client is generated from the [Gusto OpenAPI
specs](https://github.com/Gusto-API/api.gusto.dev) based on API spec version `1.0`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
gusto-api = "0.8.0-rc.1"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust
use gusto_api::Client;

let gusto = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token"),
    gusto_api::RootProductionServer
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `GUSTO_CLIENT_ID`
- `GUSTO_CLIENT_SECRET`
- `GUSTO_REDIRECT_URI`

And then you can create a client from the environment.

```rust
use gusto_api::Client;

let gusto = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token"),
    gusto_api::RootProductionServer
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```rust
use gusto_api::Client;

async fn do_call() {
    let mut gusto = Client::new_from_env("", "", gusto_api::RootProductionServer);

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = gusto.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = gusto.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = gusto.refresh_access_token().await.unwrap();
}
```
