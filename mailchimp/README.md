# `mailchimp-api`

A fully generated, opinionated API client library for MailChimp.

[![docs.rs](https://docs.rs/mailchimp-api/badge.svg)](https://docs.rs/mailchimp-api)

## API Details





### Contact


| name | email |
|----|----|
| Mailchimp API Support | apihelp@mailchimp.com |



## Client Details

This client is generated from the [MailChimp OpenAPI
specs](https://api.mailchimp.com/schema/3.0/Swagger.json?expand) based on API spec version `3.0.55`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
mailchimp-api = "0.4.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use mailchimp_api::Client;

let mailchimp = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `MAILCHIMP_CLIENT_ID`
- `MAILCHIMP_CLIENT_SECRET`
- `MAILCHIMP_REDIRECT_URI`

And then you can create a client from the environment.

```
use mailchimp_api::Client;

let mailchimp = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use mailchimp_api::Client;

async fn do_call() {
    let mut mailchimp = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = mailchimp.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = mailchimp.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = mailchimp.refresh_access_token().await.unwrap();
}
```
