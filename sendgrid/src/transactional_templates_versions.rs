use crate::Client;
use crate::ClientResult;

pub struct TransactionalTemplatesVersions {
    pub client: Client,
}

impl TransactionalTemplatesVersions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TransactionalTemplatesVersions { client }
    }

    /**
     * Create a new transactional template version.
     *
     * This function performs a `POST` to the `/templates/{template_id}/versions` endpoint.
     *
     * **This endpoint allows you to create a new version of a template.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_templates_template_version(
        &self,
        template_id: &str,
        body: &crate::types::TransactionalTemplateVersionCreate,
    ) -> ClientResult<crate::types::TransactionalTemplateVersionOutputAllOf> {
        let url = self.client.url(
            &format!(
                "/templates/{}/versions",
                crate::progenitor_support::encode_path(template_id),
            ),
            None,
        );
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
     * Activate a transactional template version.
     *
     * This function performs a `POST` to the `/templates/{template_id}/versions/{version_id}/activate` endpoint.
     *
     * **This endpoint allows you to activate a version of one of your templates.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_templates_template_versions_version_activate(
        &self,
        template_id: &str,
        version_id: &str,
    ) -> ClientResult<crate::types::TransactionalTemplateVersionOutputAllOf> {
        let url = self.client.url(
            &format!(
                "/templates/{}/versions/{}/activate",
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(version_id),
            ),
            None,
        );
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
     * Retrieve a specific transactional template version.
     *
     * This function performs a `GET` to the `/templates/{template_id}/versions/{version_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific version of a template.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_templates_template_versions_version(
        &self,
        template_id: &str,
        version_id: &str,
    ) -> ClientResult<crate::types::TransactionalTemplateVersionOutputAllOf> {
        let url = self.client.url(
            &format!(
                "/templates/{}/versions/{}",
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(version_id),
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
     * Delete a transactional template version.
     *
     * This function performs a `DELETE` to the `/templates/{template_id}/versions/{version_id}` endpoint.
     *
     * **This endpoint allows you to delete a transactional template version.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_templates_template_versions_version(
        &self,
        template_id: &str,
        version_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/templates/{}/versions/{}",
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(version_id),
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
     * Edit a transactional template version.
     *
     * This function performs a `PATCH` to the `/templates/{template_id}/versions/{version_id}` endpoint.
     *
     * **This endpoint allows you to edit the content of your template version.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_templates_template_versions_version(
        &self,
        template_id: &str,
        version_id: &str,
        body: &crate::types::TransactionalTemplateVersionCreate,
    ) -> ClientResult<crate::types::TransactionalTemplateVersionOutputAllOf> {
        let url = self.client.url(
            &format!(
                "/templates/{}/versions/{}",
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(version_id),
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
