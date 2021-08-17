# `sendgrid-api`

A fully generated, opinionated API client library for SendGrid.

## API Details

The Beta endpoints for the new Email Activity APIs - functionality is subject to change without notice. You may not have access to this Beta endpoint.

Email Activity offers filtering and search by event type for two days worth of data. There is an optional add-on to store 60 days worth of data. This add-on also gives you access to the ability to download a CSV of the 60 days worth of email event data. The Beta endpoints for the new Email Activity APIs - functionality is subject to change without notice. You may not have access to this Beta endpoint.

Email Activity offers filtering and search by event type for two days worth of data. There is an optional add-on to store 60 days worth of data. This add-on also gives you access to the ability to download a CSV of the 60 days worth of email event data.






## Client Details

This client is generated from the [SendGrid OpenAPI
specs](https://raw.githubusercontent.com/sendgrid/sendgrid-oai/main/oai.json) based on API spec version ``. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
sendgrid_api = "0.2.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use sendgrid_api::Client;

let sendgrid = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `SENDGRID_CLIENT_ID`
- `SENDGRID_CLIENT_SECRET`
- `SENDGRID_REDIRECT_URI`

And then you can create a client from the environment.

```
use sendgrid_api::Client;

let sendgrid = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use sendgrid_api::Client;

async fn do_call() {
    let mut sendgrid = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = sendgrid.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = sendgrid.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = sendgrid.refresh_access_token().await.unwrap();
}
```
