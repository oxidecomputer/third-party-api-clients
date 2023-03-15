use crate::Client;
use crate::ClientResult;

pub struct TransactionalTemplates {
    pub client: Client,
}

impl TransactionalTemplates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TransactionalTemplates { client }
    }

    /**
     * Retrieve paged transactional templates.
     *
     * This function performs a `GET` to the `/templates` endpoint.
     *
     * **This endpoint allows you to retrieve all transactional templates.**
     *
     * **Parameters:**
     *
     * * `generations: crate::types::Generations` -- Comma-delimited list specifying which generations of templates to return. Options are `legacy`, `dynamic` or `legacy,dynamic`.
     * * `page_size: f64` -- The number of templates to be returned in each page of results.
     * * `page_token: &str` -- A token corresponding to a specific page of results, as provided by metadata.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_templates(
        &self,
        generations: crate::types::Generations,
        page_size: f64,
        page_token: &str,
    ) -> ClientResult<crate::types::GetTemplatesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !generations.to_string().is_empty() {
            query_args.push(("generations".to_string(), generations.to_string()));
        }
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/templates?{}", query_), None);
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
     * Create a transactional template.
     *
     * This function performs a `POST` to the `/templates` endpoint.
     *
     * **This endpoint allows you to create a transactional template.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_template(
        &self,
        body: &crate::types::PostTemplatesRequest,
    ) -> ClientResult<crate::types::TransactionalTemplateAllOf> {
        let url = self.client.url("/templates", None);
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
     * Retrieve a single transactional template.
     *
     * This function performs a `GET` to the `/templates/{template_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a single transactional template.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_templates_template(
        &self,
        template_id: &str,
    ) -> ClientResult<crate::types::TransactionalTemplateAllOf> {
        let url = self.client.url(
            &format!(
                "/templates/{}",
                crate::progenitor_support::encode_path(template_id),
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
     * Duplicate a transactional template.
     *
     * This function performs a `POST` to the `/templates/{template_id}` endpoint.
     *
     * **This endpoint allows you to duplicate a transactional template.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_templates_template(
        &self,
        template_id: &str,
        body: &crate::types::PostTemplatesTemplateRequest,
    ) -> ClientResult<crate::types::TransactionalTemplateAllOf> {
        let url = self.client.url(
            &format!(
                "/templates/{}",
                crate::progenitor_support::encode_path(template_id),
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
    /**
     * Delete a template.
     *
     * This function performs a `DELETE` to the `/templates/{template_id}` endpoint.
     *
     * **This endpoint allows you to delete a transactional template.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_templates_template(
        &self,
        template_id: &str,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/templates/{}",
                crate::progenitor_support::encode_path(template_id),
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
     * Edit a transactional template.
     *
     * This function performs a `PATCH` to the `/templates/{template_id}` endpoint.
     *
     * **This endpoint allows you to edit the name of a transactional template.**
     *
     * To edit the template itself, [create a new transactional template version](https://sendgrid.api-docs.io/v3.0/transactional-templates-versions/create-a-new-transactional-template-version).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_templates_template(
        &self,
        template_id: &str,
        body: &crate::types::PatchTemplatesTemplateRequest,
    ) -> ClientResult<crate::types::TransactionalTemplateAllOf> {
        let url = self.client.url(
            &format!(
                "/templates/{}",
                crate::progenitor_support::encode_path(template_id),
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
