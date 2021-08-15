# `tripactions`

A fully generated, opinionated API client library for TripActions.

## API Details

<p>Welcome to the TripActions Booking Data API reference documentation.</p> <p>You can use this API to retrieve your booking data from TripActions.</p> <p>To access the API, your company admin needs to supply you with a client id and a secret key. They can generate this in the TripActions Admin Dashboard, in the Integrations section of the Settings page.<p> </br>
<p>To generate a token make a request to the token API with your credentials:</p> <code> curl --request POST --url 'https://api.tripactions.com/ta-auth/oauth/token' \</br> --header 'content-type:application/x-www-form-urlencoded' \</br> --data grant_type=client_credentials \</br> --data client_id=YOURCLIENTIDHERE \</br> --data client_secret=YOURSECRETHERE </code>
<p>Once the token was generated you can call the Booking Data API:</p> <code> curl --request GET --url https://api.tripactions.com/v1/bookings\?createdFrom\=1609459200\&createdTo\=1623229202\&page\=0\&size\=10 --header 'Authorization:Bearer YOURTOKENHERE' </code>






## Client Details

This client is generated from the [TripActions OpenAPI
specs](https://app.tripactions.com/api/public/documentation/swagger-ui/index.html?configUrl=/api/public/documentation/api-docs/swagger-config) based on API spec version `1`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
tripactions = "0.2.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use tripactions::Client;

let tripactions = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `TRIPACTIONS_CLIENT_ID`
- `TRIPACTIONS_CLIENT_SECRET`
- `TRIPACTIONS_REDIRECT_URI`

And then you can create a client from the environment.

```
use tripactions::Client;

let tripactions = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for `token` and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use tripactions::Client;

async fn do_call() {
    let mut tripactions = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = tripactions.user_consent_url(&["some-scope".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let state = "state-from-redirect-url";
    let mut access_token = tripactions.get_access_token(code, state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = tripactions.refresh_access_token().await.unwrap();
}
```
