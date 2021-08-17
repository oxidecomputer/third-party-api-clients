use anyhow::Result;

use crate::Client;

pub struct Notary {
    pub client: Client,
}

impl Notary {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Notary { client }
    }

    /**
     * Gets settings for a  notary user.
     *
     * This function performs a `GET` to the `/v2.1/current_user/notary` endpoint.
     *
     * Gets settings for a notary user.
     * The current user must be a notary.
     *
     * **Parameters:**
     *
     * * `include_jurisdictions: &str` -- If **true**, the response will include a `jurisdiction` property that contains an array of all supported jurisdictions for the current user.
     */
    pub async fn get(&self, include_jurisdictions: &str) -> Result<crate::types::NotaryResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_jurisdictions.is_empty() {
            query_args.push((
                "include_jurisdictions".to_string(),
                include_jurisdictions.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v2.1/current_user/notary?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Updates notary information for the current user.
     *
     * This function performs a `PUT` to the `/v2.1/current_user/notary` endpoint.
     *
     * Updates notary information for the current user.
     */
    pub async fn put(&self, body: &crate::types::NotaryData) -> Result<crate::types::NotaryData> {
        let url = "/v2.1/current_user/notary".to_string();
        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Registers the current user as a notary.
     *
     * This function performs a `POST` to the `/v2.1/current_user/notary` endpoint.
     *
     * Registers the current user as a notary.
     */
    pub async fn post(&self, body: &crate::types::NotaryData) -> Result<crate::types::NotaryData> {
        let url = "/v2.1/current_user/notary".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
