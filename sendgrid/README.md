# `sendgrid-api`

A fully generated, opinionated API client library for SendGrid.

[![docs.rs](https://docs.rs/sendgrid-api/badge.svg)](https://docs.rs/sendgrid-api)

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
sendgrid-api = "0.4.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use sendgrid_api::Client;

let sendgrid = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `SENDGRID_API_KEY`

And then you can create a client from the environment.

```
use sendgrid_api::Client;

let sendgrid = Client::new_from_env();
```
