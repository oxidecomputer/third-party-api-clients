use crate::Client;
use crate::ClientResult;

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
    pub async fn get_custom_provider(
        &self,
    ) -> ClientResult<crate::types::GetCustomProviderResponse> {
        let url = self.client.url("/custom-id-provider", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
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
    pub async fn postcustom_provider(
        &self,
    ) -> ClientResult<crate::types::PostcustomProviderResponse> {
        let url = self.client.url("/custom-id-provider", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
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
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/custom-id-provider/application-link", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
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
    ) -> ClientResult<crate::types::GetEntityTypeCustomRampResponse> {
        let url = self.client.url(
            &format!(
                "/custom-id-provider/{}/{}/ramp-id",
                crate::progenitor_support::encode_path(entity_type),
                crate::progenitor_support::encode_path(custom_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
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
    ) -> ClientResult<crate::types::GetEntityTypeRampCustomResponse> {
        let url = self.client.url(
            &format!(
                "/custom-id-provider/{}/{}/custom-id",
                crate::progenitor_support::encode_path(entity_type),
                crate::progenitor_support::encode_path(ramp_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
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
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/custom-id-provider/{}/custom-id-link",
                crate::progenitor_support::encode_path(entity_type),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
