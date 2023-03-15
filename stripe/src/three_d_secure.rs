use crate::Client;
use crate::ClientResult;

pub struct ThreeDSecure {
    pub client: Client,
}

impl ThreeDSecure {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ThreeDSecure { client }
    }

    /**
     * This function performs a `POST` to the `/v1/3d_secure` endpoint.
     *
     * <p>Initiate 3D Secure authentication.</p>
     */
    pub async fn post_3d_secure(&self) -> ClientResult<crate::types::ThreeDSecure> {
        let url = self.client.url("/v1/3d_secure", None);
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
     * This function performs a `GET` to the `/v1/3d_secure/{three_d_secure}` endpoint.
     *
     * <p>Retrieves a 3D Secure object.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `three_d_secure: &str` -- The account's country.
     */
    pub async fn get_3d_secure_three_d(
        &self,
        three_d_secure: &str,
    ) -> ClientResult<crate::types::ThreeDSecure> {
        let url = self.client.url(
            &format!(
                "/v1/3d_secure/{}",
                crate::progenitor_support::encode_path(three_d_secure),
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
