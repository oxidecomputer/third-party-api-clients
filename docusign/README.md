# `docusign`

A fully generated, opinionated API client library for DocuSign.

[![docs.rs](https://docs.rs/docusign/badge.svg)](https://docs.rs/docusign)

## API Details

The DocuSign REST API provides you with a powerful, convenient, and simple Web services API for interacting with DocuSign.

[API Terms of Service](https://www.docusign.com/company/terms-and-conditions/web)

### Contact


| name | url | email |
|----|----|----|
| DocuSign Developer Center | <https://developers.docusign.com/> | devcenter@docusign.com |



## Client Details

This client is generated from the [DocuSign OpenAPI
specs](https://github.com/docusign/OpenAPI-Specifications) based on API spec version `v2.1`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
docusign = "0.4.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use docusign::Client;

let docusign = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `DOCUSIGN_CLIENT_ID`
- `DOCUSIGN_CLIENT_SECRET`
- `DOCUSIGN_REDIRECT_URI`

And then you can create a client from the environment.

```
use docusign::Client;

let docusign = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use docusign::Client;

async fn do_call() {
    let mut docusign = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = docusign.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = docusign.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = docusign.refresh_access_token().await.unwrap();
}
```
