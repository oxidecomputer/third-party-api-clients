use anyhow::Result;

use crate::Client;

pub struct Templates {
    pub client: Client,
}

impl Templates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Templates { client }
    }

    /**
    * List templates.
    *
    * This function performs a `GET` to the `/templates` endpoint.
    *
    * Get a list of an account's available templates.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `created_by: &str` -- The Mailchimp account user who created the template.
    * * `since_date_created: &str` -- Restrict the response to templates created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `before_date_created: &str` -- Restrict the response to templates created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `type_: &str` -- Limit results based on template type.
    * * `category: &str` -- Limit results based on category.
    * * `folder_id: &str` -- The name of the folder.
    * * `sort_field: crate::types::GetTemplatesSortField` -- Returns user templates sorted by the specified field.
    * * `sort_dir: crate::types::SortDir` -- Determines the order direction for sorted results.
    */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        created_by: &str,
        since_date_created: &str,
        before_date_created: &str,
        type_: &str,
        category: &str,
        folder_id: &str,
        sort_field: crate::types::GetTemplatesSortField,
        sort_dir: crate::types::SortDir,
    ) -> Result<crate::types::TemplatesData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !before_date_created.is_empty() {
            query_args.push((
                "before_date_created".to_string(),
                before_date_created.to_string(),
            ));
        }
        if !category.is_empty() {
            query_args.push(("category".to_string(), category.to_string()));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !created_by.is_empty() {
            query_args.push(("created_by".to_string(), created_by.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if !folder_id.is_empty() {
            query_args.push(("folder_id".to_string(), folder_id.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !since_date_created.is_empty() {
            query_args.push((
                "since_date_created".to_string(),
                since_date_created.to_string(),
            ));
        }
        if !sort_dir.to_string().is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/templates?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Add template.
    *
    * This function performs a `POST` to the `/templates` endpoint.
    *
    * Create a new template for the account. Only Classic templates are supported.
    */
    pub async fn post(
        &self,
        body: &crate::types::TemplateInstance,
    ) -> Result<crate::types::Templates> {
        let url = "/templates".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get template info.
    *
    * This function performs a `GET` to the `/templates/{template_id}` endpoint.
    *
    * Get information about a specific template.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `template_id: &str` -- The unique id for the template.
    */
    pub async fn get_templates(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        template_id: &str,
    ) -> Result<crate::types::Templates> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/templates/{}?{}",
            crate::progenitor_support::encode_path(&template_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete template.
    *
    * This function performs a `DELETE` to the `/templates/{template_id}` endpoint.
    *
    * Delete a specific template.
    *
    * **Parameters:**
    *
    * * `template_id: &str` -- The unique id for the template.
    */
    pub async fn delete(&self, template_id: &str) -> Result<()> {
        let url = format!(
            "/templates/{}",
            crate::progenitor_support::encode_path(&template_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update template.
    *
    * This function performs a `PATCH` to the `/templates/{template_id}` endpoint.
    *
    * Update the name, HTML, or `folder_id` of an existing template.
    *
    * **Parameters:**
    *
    * * `template_id: &str` -- The unique id for the template.
    */
    pub async fn patch(
        &self,
        template_id: &str,
        body: &crate::types::TemplateInstance,
    ) -> Result<crate::types::Templates> {
        let url = format!(
            "/templates/{}",
            crate::progenitor_support::encode_path(&template_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * View default content.
    *
    * This function performs a `GET` to the `/templates/{template_id}/default-content` endpoint.
    *
    * Get the sections that you can edit in a template, including each section's default content.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `template_id: &str` -- The unique id for the template.
    */
    pub async fn get_default_content(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        template_id: &str,
    ) -> Result<crate::types::TemplateDefaultContent> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/templates/{}/default-content?{}",
            crate::progenitor_support::encode_path(&template_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
