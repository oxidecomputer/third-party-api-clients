use crate::Client;
use crate::ClientResult;

pub struct Certificates {
    pub client: Client,
}

impl Certificates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Certificates { client }
    }

    /**
     * Create an SSO Certificate.
     *
     * This function performs a `POST` to the `/sso/certificates` endpoint.
     *
     * **This endpoint allows you to create an SSO certificate.**
     */
    pub async fn post_sso(
        &self,
        body: &crate::types::PostSsoCertificatesRequest,
    ) -> ClientResult<crate::types::SsoCertificateBody> {
        let url = self.client.url("/sso/certificates", None);
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
     * Get All SSO Certificates by Integration.
     *
     * This function performs a `GET` to the `/sso/integrations/{integration_id}/certificates` endpoint.
     *
     * **This endpoint allows you to retrieve all your IdP configurations by configuration ID.**
     *
     * The `integration_id` expected by this endpoint is the `id` returned in the response by the "Get All SSO Integrations" endpoint.
     */
    pub async fn get_sso_integrations_integration(
        &self,
        integration_id: &str,
    ) -> ClientResult<Vec<crate::types::SsoCertificateBody>> {
        let url = self.client.url(
            &format!(
                "/sso/integrations/{}/certificates",
                crate::progenitor_support::encode_path(integration_id),
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
     * Get All SSO Certificates by Integration.
     *
     * This function performs a `GET` to the `/sso/integrations/{integration_id}/certificates` endpoint.
     *
     * As opposed to `get_sso_integrations_integration`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all your IdP configurations by configuration ID.**
     *
     * The `integration_id` expected by this endpoint is the `id` returned in the response by the "Get All SSO Integrations" endpoint.
     */
    pub async fn get_all_sso_integrations_integration(
        &self,
        integration_id: &str,
    ) -> ClientResult<Vec<crate::types::SsoCertificateBody>> {
        let url = self.client.url(
            &format!(
                "/sso/integrations/{}/certificates",
                crate::progenitor_support::encode_path(integration_id),
            ),
            None,
        );
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
     * Get an SSO Certificate.
     *
     * This function performs a `GET` to the `/sso/certificates/{cert_id}` endpoint.
     *
     * **This endpoint allows you to retrieve an individual SSO certificate.**
     */
    pub async fn get_sso_cert(
        &self,
        cert_id: &str,
    ) -> ClientResult<crate::types::SsoCertificateBody> {
        let url = self.client.url(
            &format!(
                "/sso/certificates/{}",
                crate::progenitor_support::encode_path(cert_id),
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
     * Delete an SSO Certificate.
     *
     * This function performs a `DELETE` to the `/sso/certificates/{cert_id}` endpoint.
     *
     * **This endpoint allows you to delete an SSO certificate.**
     *
     * You can retrieve a certificate's ID from the response provided by the "Get All SSO Integrations" endpoint.
     */
    pub async fn delete_sso_cert(
        &self,
        cert_id: &str,
    ) -> ClientResult<crate::types::SsoCertificateBody> {
        let url = self.client.url(
            &format!(
                "/sso/certificates/{}",
                crate::progenitor_support::encode_path(cert_id),
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
     * Update SSO Certificate.
     *
     * This function performs a `PATCH` to the `/sso/certificates/{cert_id}` endpoint.
     *
     * **This endpoint allows you to update an existing certificate by ID.**
     *
     * You can retrieve a certificate's ID from the response provided by the "Get All SSO Integrations" endpoint.
     */
    pub async fn patch_sso_cert(
        &self,
        cert_id: &str,
        body: &crate::types::PatchSsoCertificatesCertRequest,
    ) -> ClientResult<Vec<crate::types::SsoErrorResponse>> {
        let url = self.client.url(
            &format!(
                "/sso/certificates/{}",
                crate::progenitor_support::encode_path(cert_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
