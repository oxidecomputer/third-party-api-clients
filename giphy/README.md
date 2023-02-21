# `giphy-api`

A fully generated, opinionated API client library for Giphy.

[![docs.rs](https://docs.rs/giphy-api/badge.svg)](https://docs.rs/giphy-api)

## API Details

Giphy API

[API Terms of Service](https://developers.giphy.com/)

### Contact


| email |
|----|
| support@giphy.com |



## Client Details

This client is generated from the [Giphy OpenAPI
specs](https://github.com/APIs-guru/openapi-directory/tree/main/APIs/giphy.com) based on API spec version `1.0`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
giphy-api = "0.4.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use giphy_api::Client;

let giphy = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `GIPHY_API_KEY`

And then you can create a client from the environment.

```
use giphy_api::Client;

let giphy = Client::new_from_env();
```
