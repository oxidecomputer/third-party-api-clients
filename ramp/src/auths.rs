use crate::Client;
use crate::ClientResult;

pub struct Auths {
    pub client: Client,
}

impl Auths {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Auths { client }
    }

    /**
     * Get OAuth2 token.
     *
     * This function performs a `POST` to the `/token` endpoint.
     *
     * Returns an access token for accessing endpoints. There are three methods to get an access token:
     * * "authorization\_code": Authorization Code Grant method, used for retrieving an access token for the first time
     * * "refresh\_token": Refresh Token method, used for retrieving subsequent access tokens using the refresh token provided from authorization code grant
     * * "client\_credentials": Client Credentials method, allows direct retrieval of access tokens with only client id and secret.
     *
     * There are two main flows: Authorization Code Grant + Refresh Token; or Client Credentials.
     *
     * For Authorization Code Grant + Refresh Token, the flow would be as follows:
     * * Follow authorization process to get an authorization code
     * * Use authorization code to retrieve an access token and refresh token from this endpoint
     * * Use refresh token to retrieve new access tokens from this endpoint (without having to go through authorization process again)
     *
     * For Client Credentials:
     * * Call token endpoint with client credentials to retrieve access token
     *
     * The request body is different for the methods:
     * * grant\_type = "authorization\_code"
     *   * code
     *   * redirect\_uri
     * * grant\_type = "refresh\_token"
     *   * refresh\_token
     * * grant\_type = "client\_credentials"
     *   * no additional data
     *
     * Some important notes:
     * * Unlike other endpoints, the data format must be "application/x-www-form-urlencoded", according to [RFC specifications](https://datatracker.ietf.org/doc/html/rfc6749#appendix-B)
     * * To use a particular code grant, it must be included in the "Grant Types" section of the App Settings modal in app.ramp.com
     * * Only the authorization code grant returns a refresh token
     *   * Additionally, the "Refresh Token" grant type must be selected in App Settings for the refresh token to be returned
     * * The token endpoint used to be "/public/v1/customer/token" - this endpoint is now deprecated and should not be used
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- Basic \<base64-encoded client_id:client_secret\>.
     */
    pub async fn post_token(&self) -> ClientResult<crate::types::OAuth2Token> {
        let url = self.client.url("/token", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
