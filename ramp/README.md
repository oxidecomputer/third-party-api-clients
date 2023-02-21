# `ramp-api`

A fully generated, opinionated API client library for Ramp.

[![docs.rs](https://docs.rs/ramp-api/badge.svg)](https://docs.rs/ramp-api)

## API Details

Ramp's developer API makes it easier for companies to programmatically set up and manage their card program.

It provides the ability to automate card issuing and the ability to integrate features like reporting with other software platforms.

[API Terms of Service](https://ramp.com/developer-agreement)




## Client Details

This client is generated from the [Ramp OpenAPI
specs](https://github.com/sumatokado/ramp-developer) based on API spec version `1.0`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
ramp-api = "0.4.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use ramp_api::Client;

let ramp = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `RAMP_CLIENT_ID`
- `RAMP_CLIENT_SECRET`
- `RAMP_REDIRECT_URI`

And then you can create a client from the environment.

```
use ramp_api::Client;

let ramp = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use ramp_api::Client;

async fn do_call() {
    let mut ramp = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = ramp.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = ramp.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = ramp.refresh_access_token().await.unwrap();
}
```
