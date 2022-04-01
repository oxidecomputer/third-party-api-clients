use anyhow::Result;

use crate::Client;

pub struct CustomIds {
    pub client: Client,
}

impl CustomIds {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CustomIds { client }
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
    pub async fn get_custom_provider(&self) -> Result<crate::types::GetCustomProviderResponse> {
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
    * * `authorization_bearer_111111111111: &str` -- The OAuth2 token header.
    */
    pub async fn postcustom_provider(&self) -> Result<crate::types::PostcustomProviderResponse> {
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
    pub async fn post_custom_provider_application_link(
        &self,
        body: &crate::types::GetCustomProviderResponse,
    ) -> Result<()> {
        let url = "/custom-id-provider/application-link".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Convert custom id to ramp id.
    *
    * This function performs a `GET` to the `/custom-id-provider/{entity_type}/{custom_id}/ramp-id` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `authorization: &str` -- The OAuth2 token header.
    */
    pub async fn get_entity_type_custom_ramp(
        &self,
        entity_type: &str,
        custom_id: &str,
    ) -> Result<crate::types::GetEntityTypeCustomRampResponse> {
        let url = format!(
            "/custom-id-provider/{}/{}/ramp-id",
            crate::progenitor_support::encode_path(entity_type),
            crate::progenitor_support::encode_path(custom_id),
        );

        self.client.get(&url, None).await
    }

    /**
    * Convert ramp id to custom id.
    *
    * This function performs a `GET` to the `/custom-id-provider/{entity_type}/{ramp_id}/custom-id` endpoint.
    *
    * **Parameters:**
    *
    * * `authorization: &str` -- The OAuth2 token header.
    */
    pub async fn get_entity_type_ramp_custom(
        &self,
        entity_type: &str,
        ramp_id: &str,
    ) -> Result<crate::types::GetEntityTypeRampCustomResponse> {
        let url = format!(
            "/custom-id-provider/{}/{}/custom-id",
            crate::progenitor_support::encode_path(entity_type),
            crate::progenitor_support::encode_path(ramp_id),
        );

        self.client.get(&url, None).await
    }

    /**
    * Create custom id link.
    *
    * This function performs a `POST` to the `/custom-id-provider/{entity_type}/custom-id-link` endpoint.
    *
    * Create a mapping between custom\_id and ramp\_id under the namespace specified by entity\_type.
    */
    pub async fn post_custom_provider_entity_type_link(
        &self,
        entity_type: &str,
        body: &crate::types::PostCustomProviderEntityTypeLinkRequest,
    ) -> Result<()> {
        let url = format!(
            "/custom-id-provider/{}/custom-id-link",
            crate::progenitor_support::encode_path(entity_type),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
