# `shipbob`

A fully generated, opinionated API client library for ShipBob.

[![docs.rs](https://docs.rs/shipbob/badge.svg)](https://docs.rs/shipbob)

## API Details

ShipBob Developer API Documentation

# Authentication

<!-- ReDoc-Inject: <security-definitions> -->






## Client Details

This client is generated from the [ShipBob OpenAPI
specs](https://developer.shipbob.com/c196c993-6cf8-4901-84aa-b425f3448df3) based on API spec version `1.0`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
shipbob = "0.1.2"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use shipbob::Client;

let shipbob = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token"),
    String::from("shipbob_channel_id")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `SHIPBOB_CLIENT_ID`
- `SHIPBOB_CLIENT_SECRET`
- `SHIPBOB_REDIRECT_URI`

And then you can create a client from the environment.

```
use shipbob::Client;

let shipbob = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token"),
    String::from("shipbob_channel_id")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use shipbob::Client;

async fn do_call() {
    let mut shipbob = Client::new_from_env("", "", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = shipbob.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = shipbob.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = shipbob.refresh_access_token().await.unwrap();
}
```
