# `dolladollabills`

A fully generated, opinionated API client library for Stripe.

[![docs.rs](https://docs.rs/dolladollabills/badge.svg)](https://docs.rs/dolladollabills)

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
dolladollabills = "0.4.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use dolladollabills::Client;

let stripe = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `STRIPE_API_KEY`

And then you can create a client from the environment.

```
use dolladollabills::Client;

let stripe = Client::new_from_env();
```
