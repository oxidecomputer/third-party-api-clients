# `stripe-api`

A fully generated, opinionated API client library for Stripe.

[![docs.rs](https://docs.rs/stripe-api/badge.svg)](https://docs.rs/stripe-api)

## API Details

The Stripe REST API. Please see https://stripe.com/docs/api for more details.

[API Terms of Service](https://stripe.com/us/terms/)

### Contact


| name | url | email |
|----|----|----|
| Stripe Dev Platform Team | <https://stripe.com> | dev-platform@stripe.com |



## Client Details

This client is generated from the [Stripe OpenAPI
specs](https://raw.githubusercontent.com/stripe/openapi/master/openapi/spec3.json) based on API spec version `2020-08-27`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
stripe-api = "0.1.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use stripe_api::Client;

let stripe = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `STRIPE_CLIENT_ID`
- `STRIPE_CLIENT_SECRET`
- `STRIPE_REDIRECT_URI`

And then you can create a client from the environment.

```
use stripe_api::Client;

let stripe = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use stripe_api::Client;

async fn do_call() {
    let mut stripe = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = stripe.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = stripe.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = stripe.refresh_access_token().await.unwrap();
}
```
