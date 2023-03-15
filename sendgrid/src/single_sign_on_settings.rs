use crate::Client;
use crate::ClientResult;

pub struct SingleSignOnSettings {
    pub client: Client,
}

impl SingleSignOnSettings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SingleSignOnSettings { client }
    }

    /**
     * Get All SSO Integrations.
     *
     * This function performs a `GET` to the `/sso/integrations` endpoint.
     *
     * **This endpoint allows you to retrieve all SSO integrations tied to your Twilio SendGrid account.**
     *
     * The IDs returned by this endpoint can be used by the APIs additional endpoints to modify your SSO integrations.
     *
     * **Parameters:**
     *
     * * `si: bool` -- If this parameter is set to `true`, the response will include the `completed_integration` field.
     */
    pub async fn get_sso_integrations(
        &self,
        si: bool,
    ) -> ClientResult<Vec<crate::types::SsoIntegrationAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if si {
            query_args.push(("si".to_string(), si.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/sso/integrations?{}", query_), None);
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
     * Get All SSO Integrations.
     *
     * This function performs a `GET` to the `/sso/integrations` endpoint.
     *
     * As opposed to `get_sso_integrations`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all SSO integrations tied to your Twilio SendGrid account.**
     *
     * The IDs returned by this endpoint can be used by the APIs additional endpoints to modify your SSO integrations.
     */
    pub async fn get_all_sso_integrations(
        &self,
        si: bool,
    ) -> ClientResult<Vec<crate::types::SsoIntegrationAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if si {
            query_args.push(("si".to_string(), si.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/sso/integrations?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create an SSO Integration.
     *
     * This function performs a `POST` to the `/sso/integrations` endpoint.
     *
     * **This endpoint allows you to create an SSO integration.**
     */
    pub async fn post_sso_integration(
        &self,
        body: &crate::types::CreateIntegrationRequest,
    ) -> ClientResult<crate::types::SsoIntegrationAllOf> {
        let url = self.client.url("/sso/integrations", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get an SSO Integration.
     *
     * This function performs a `GET` to the `/sso/integrations/{id}` endpoint.
     *
     * **This endpoint allows you to retrieve an SSO integration by ID.**
     *
     * You can retrieve the IDs for your configurations from the response provided by the "Get All SSO Integrations" endpoint.
     *
     * **Parameters:**
     *
     * * `si: bool` -- If this parameter is set to `true`, the response will include the `completed_integration` field.
     */
    pub async fn get_sso_integration(
        &self,
        id: &str,
        si: bool,
    ) -> ClientResult<crate::types::SsoIntegrationAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if si {
            query_args.push(("si".to_string(), si.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/sso/integrations/{}?{}",
                crate::progenitor_support::encode_path(id),
                query_
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
     * Delete an SSO Integration.
     *
     * This function performs a `DELETE` to the `/sso/integrations/{id}` endpoint.
     *
     * **This endpoint allows you to delete an IdP configuration by ID.**
     *
     * You can retrieve the IDs for your configurations from the response provided by the "Get All SSO Integrations" endpoint.
     */
    pub async fn delete_sso_integrations(&self, id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/sso/integrations/{}",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update an SSO Integration.
     *
     * This function performs a `PATCH` to the `/sso/integrations/{id}` endpoint.
     *
     * **This endpoint allows you to modify an exisiting SSO integration.**
     *
     * You can retrieve the IDs for your configurations from the response provided by the "Get All SSO Integrations" endpoint.
     *
     * **Parameters:**
     *
     * * `si: bool` -- If this parameter is set to `true`, the response will include the `completed_integration` field.
     */
    pub async fn patch_sso_integrations(
        &self,
        id: &str,
        si: bool,
        body: &crate::types::CreateIntegrationRequest,
    ) -> ClientResult<crate::types::SsoIntegrationAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if si {
            query_args.push(("si".to_string(), si.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/sso/integrations/{}?{}",
                crate::progenitor_support::encode_path(id),
                query_
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
}
