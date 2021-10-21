# `gsuite-api`

A fully generated, opinionated API client library for Google Admin.

[![docs.rs](https://docs.rs/gsuite-api/badge.svg)](https://docs.rs/gsuite-api)

## API Details

Admin SDK lets administrators of enterprise domains to view and manage resources like user, groups etc. It also provides audit and usage reports of domain.

[API Terms of Service](https://developers.google.com/terms/)

### Contact


| name | url |
|----|----|
| Google | <https://google.com> |

### License


| name | url |
|----|----|
| Creative Commons Attribution 3.0 | <http://creativecommons.org/licenses/by/3.0/> |


## Client Details

This client is generated from the [Google Admin OpenAPI
specs](https://admin.googleapis.com/iscovery/rest?version=directory_v1) based on API spec version `directory_v1`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
gsuite-api = "0.2.3"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use gsuite_api::Client;

let google_admin = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `GOOGLE_ADMIN_CLIENT_ID`
- `GOOGLE_ADMIN_CLIENT_SECRET`
- `GOOGLE_ADMIN_REDIRECT_URI`

And then you can create a client from the environment.

```
use gsuite_api::Client;

let google_admin = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use gsuite_api::Client;

async fn do_call() {
    let mut google_admin = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = google_admin.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = google_admin.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = google_admin.refresh_access_token().await.unwrap();
}
```
