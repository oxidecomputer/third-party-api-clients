use anyhow::Result;

use crate::Client;

pub struct Mandates {
    pub client: Client,
}

impl Mandates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Mandates { client }
    }

    /**
    * This function performs a `GET` to the `/v1/mandates/{mandate}` endpoint.
    *
    * <p>Retrieves a Mandate object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `mandate: &str` -- The account's country.
    */
    pub async fn get(&self, expand: &[String], mandate: &str) -> Result<crate::types::Mandate> {
        let url = format!(
            "/v1/mandates/{}",
            crate::progenitor_support::encode_path(&mandate.to_string()),
        );

        self.client.get(&url, None).await
    }
}
