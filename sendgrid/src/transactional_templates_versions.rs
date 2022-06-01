use anyhow::Result;

use crate::Client;

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
    ) -> Result<crate::types::TransactionalTemplateVersionOutputAllOf> {
        let url = format!(
            "/templates/{}/versions",
            crate::progenitor_support::encode_path(template_id),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    ) -> Result<crate::types::TransactionalTemplateVersionOutputAllOf> {
        let url = format!(
            "/templates/{}/versions/{}/activate",
            crate::progenitor_support::encode_path(template_id),
            crate::progenitor_support::encode_path(version_id),
        );

        self.client.post(&url, None).await
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
    ) -> Result<crate::types::TransactionalTemplateVersionOutputAllOf> {
        let url = format!(
            "/templates/{}/versions/{}",
            crate::progenitor_support::encode_path(template_id),
            crate::progenitor_support::encode_path(version_id),
        );

        self.client.get(&url, None).await
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
    ) -> Result<()> {
        let url = format!(
            "/templates/{}/versions/{}",
            crate::progenitor_support::encode_path(template_id),
            crate::progenitor_support::encode_path(version_id),
        );

        self.client.delete(&url, None).await
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
    ) -> Result<crate::types::TransactionalTemplateVersionOutputAllOf> {
        let url = format!(
            "/templates/{}/versions/{}",
            crate::progenitor_support::encode_path(template_id),
            crate::progenitor_support::encode_path(version_id),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
