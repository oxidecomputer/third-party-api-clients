# `google-cloud-resource-manager`

A fully generated, opinionated API client library for Google Cloud Resource Manager.

[![docs.rs](https://docs.rs/google-cloud-resource-manager/badge.svg)](https://docs.rs/google-cloud-resource-manager)

## API Details

Creates, reads, and updates metadata for Google Cloud Platform resource containers.

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

This client is generated from the [Google Cloud Resource Manager OpenAPI
specs](https://cloudresourcemanager.googleapis.com/iscovery/rest?version=v2) based on API spec version `v2`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
google-cloud-resource-manager = "0.2.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use google_cloud_resource_manager::Client;

let google cloud resource manager = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `GOOGLE CLOUD RESOURCE MANAGER_CLIENT_ID`
- `GOOGLE CLOUD RESOURCE MANAGER_CLIENT_SECRET`
- `GOOGLE CLOUD RESOURCE MANAGER_REDIRECT_URI`

And then you can create a client from the environment.

```
use google_cloud_resource_manager::Client;

let google cloud resource manager = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use google_cloud_resource_manager::Client;

async fn do_call() {
    let mut google cloud resource manager = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = google cloud resource manager.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = google cloud resource manager.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = google cloud resource manager.refresh_access_token().await.unwrap();
}
```
