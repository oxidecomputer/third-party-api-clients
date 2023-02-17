# `shipbob`

A fully generated, opinionated API client library for ShipBob.

[![docs.rs](https://docs.rs/shipbob/badge.svg)](https://docs.rs/shipbob)

## API Details

ShipBob Developer API Documentation

# Authentication

<!-- ReDoc-Inject: <security-definitions> -->






## Client Details

This client is generated from the [ShipBob OpenAPI
specs](https://developer.shipbob.com/c196c993-6cf8-4901-84aa-b425f3448df3) based on API spec version `1.0`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
shipbob = "0.3.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use shipbob::Client;

let shipbob = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `SHIPBOB_API_KEY`

And then you can create a client from the environment.

```
use shipbob::Client;

let shipbob = Client::new_from_env();
```
