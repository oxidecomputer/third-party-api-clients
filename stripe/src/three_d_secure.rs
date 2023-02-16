use anyhow::Result;

use crate::Client;

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
    pub async fn post_3d_secure(&self) -> Result<crate::types::ThreeDSecure> {
        let url = "/v1/3d_secure".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None).await
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
    ) -> Result<crate::types::ThreeDSecure> {
        let url = format!(
            "/v1/3d_secure/{}",
            crate::progenitor_support::encode_path(three_d_secure),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
}
