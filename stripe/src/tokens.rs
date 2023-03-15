use crate::Client;
use crate::ClientResult;

pub struct Tokens {
    pub client: Client,
}

impl Tokens {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Tokens { client }
    }

    /**
     * This function performs a `POST` to the `/v1/tokens` endpoint.
     *
     * <p>Creates a single-use token that represents a bank account’s details.
     * This token can be used with any API method in place of a bank account dictionary. This token can be used only once, by attaching it to a <a href="#accounts">Custom account</a>.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Token> {
        let url = self.client.url("/v1/tokens", None);
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
    /**
     * This function performs a `GET` to the `/v1/tokens/{token}` endpoint.
     *
     * <p>Retrieves the token with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `token: &str` -- The account's country.
     */
    pub async fn get(&self, token: &str) -> ClientResult<crate::types::Token> {
        let url = self.client.url(
            &format!(
                "/v1/tokens/{}",
                crate::progenitor_support::encode_path(token),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
