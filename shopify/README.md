# `shopify`

A fully generated, opinionated API client library for Shopify.

[![docs.rs](https://docs.rs/shopify/badge.svg)](https://docs.rs/shopify)

## API Details

The REST Admin API lets you build apps and other integrations for the Shopify admin.






## Client Details

This client is generated from the [Shopify OpenAPI
specs](https://raw.githubusercontent.com/allengrant/shopify_openapi/master/shopify_openapi.json) based on API spec version `2020-10`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
shopify = "0.3.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use shopify::Client;

let shopify = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `SHOPIFY_CLIENT_ID`
- `SHOPIFY_CLIENT_SECRET`
- `SHOPIFY_REDIRECT_URI`

And then you can create a client from the environment.

```
use shopify::Client;

let shopify = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use shopify::Client;

async fn do_call() {
    let mut shopify = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = shopify.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = shopify.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = shopify.refresh_access_token().await.unwrap();
}
```
