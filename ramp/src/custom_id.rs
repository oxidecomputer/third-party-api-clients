use anyhow::Result;

use crate::Client;

pub struct CustomId {
    client: Client,
}

impl CustomId {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CustomId { client }
    }

    /**
     * GET the Custom ID provider linked to the current OAuth token.
     *
     * This function performs a `GET` to the `/custom-id-provider` endpoint.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_custom_id_custom_id_provider(
        &self,
    ) -> Result<crate::types::GetCustomIdProviderResponse> {
        let url = "/custom-id-provider".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Create a Custom ID provider.
     *
     * This function performs a `POST` to the `/custom-id-provider` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `authorization_bearer_111111111111: &str` -- OAuth Access token.
     */
    pub async fn postcustom_id_provider(
        &self,
    ) -> Result<crate::types::PostcustomIdProviderResponse> {
        let url = "/custom-id-provider".to_string();
        self.client.post(&url, None).await
    }

    /**
     * .
     *
     * This function performs a `POST` to the `/custom-id-provider/application-link` endpoint.
     *
     * Register an access token with a custom ID provider
     */
    pub async fn post_custom_id_provider_application_link(
        &self,
        body: &crate::types::GetCustomIdProviderResponse,
    ) -> Result<()> {
        let url = "/custom-id-provider/application-link".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Convert custom id to ramp id.
     *
     * This function performs a `GET` to the `/custom-id-provider/<entity-type>/<custom-id>/ramp-id` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_entity_type_custom_id_ramp_id(
        &self,
    ) -> Result<crate::types::GetEntityTypeCustomIdRampResponse> {
        let url = "/custom-id-provider/<entity-type>/<custom-id>/ramp-id".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Convert ramp id to custom id.
     *
     * This function performs a `GET` to the `/custom-id-provider/<entity-type>/<ramp-id>/custom-id` endpoint.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_entity_type_ramp_id_custom_id(
        &self,
    ) -> Result<crate::types::GetEntityTypeRampIdCustomResponse> {
        let url = "/custom-id-provider/<entity-type>/<ramp-id>/custom-id".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Create custom id link.
     *
     * This function performs a `POST` to the `/custom-id-provider/<entity-type>/custom-id-link` endpoint.
     *
     * Create a mapping between custom\_id and ramp\_id under the namespace specified by entity\_type.
     */
    pub async fn post_custom_id_provider_entity_type_custom_id_link(
        &self,
        body: &crate::types::PostCustomIdProviderEntityTypeLinkRequest,
    ) -> Result<()> {
        let url = "/custom-id-provider/<entity-type>/custom-id-link".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
