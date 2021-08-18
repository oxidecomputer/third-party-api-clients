# `zoom-api`

A fully generated, opinionated API client library for Zoom.

[![docs.rs](https://docs.rs/zoom-api/badge.svg)](https://docs.rs/zoom-api)

## API Details

The Zoom API allows developers to access information from Zoom. You can use this API to build private services or public applications on the [Zoom App Marketplace](http://marketplace.zoom.us). To learn how to get your credentials and create private/public applications, read our [Authorization Guide](https://marketplace.zoom.us/docs/guides/authorization/credentials).
All endpoints are available via `https` and are located at `api.zoom.us/v2/`.

For instance you can list all users on an account via `https://api.zoom.us/v2/users/`.

[API Terms of Service](https://zoom.us/docs/en-us/zoom_api_license_and_tou.html)

### Contact


| name | url | email |
|----|----|----|
| Zoom Developers | <https://developer.zoom.us/> | developersupport@zoom.us |

### License


| name | url |
|----|----|
| MIT for OAS 2.0 | <https://opensource.org/licenses/MIT> |


## Client Details

This client is generated from the [Zoom OpenAPI
specs](https://marketplace.zoom.us/docs/api-reference/zoom-api/Zoom%20API.oas2.json) based on API spec version `2.0.0`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
zoom_api = "0.2.4"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use zoom_api::Client;

let zoom = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `ZOOM_CLIENT_ID`
- `ZOOM_CLIENT_SECRET`
- `ZOOM_REDIRECT_URI`

And then you can create a client from the environment.

```
use zoom_api::Client;

let zoom = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use zoom_api::Client;

async fn do_call() {
    let mut zoom = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = zoom.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = zoom.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = zoom.refresh_access_token().await.unwrap();
}
```
