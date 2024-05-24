use crate::Client;
use crate::ClientResult;

pub struct TemplateFolders {
    pub client: Client,
}

impl TemplateFolders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateFolders { client }
    }

    /**
     * List template folders.
     *
     * This function performs a `GET` to the `/template-folders` endpoint.
     *
     * Get all folders used to organize templates.
     *
     * **Parameters:**
     *
     * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
     * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
     */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
    ) -> ClientResult<crate::Response<crate::types::TemplateFolders>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/template-folders?{}", query_), None);
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
     * Add template folder.
     *
     * This function performs a `POST` to the `/template-folders` endpoint.
     *
     * Create a new template folder.
     */
    pub async fn post(
        &self,
        body: &crate::types::GalleryFolder,
    ) -> ClientResult<crate::Response<crate::types::Folders>> {
        let url = self.client.url("/template-folders", None);
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
     * Get template folder.
     *
     * This function performs a `GET` to the `/template-folders/{folder_id}` endpoint.
     *
     * Get information about a specific folder used to organize templates.
     *
     * **Parameters:**
     *
     * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `folder_id: &str` -- The unique id for the template folder.
     */
    pub async fn get_template_folders(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        folder_id: &str,
    ) -> ClientResult<crate::Response<crate::types::Folders>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/template-folders/{}?{}",
                crate::progenitor_support::encode_path(folder_id),
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
     * Delete template folder.
     *
     * This function performs a `DELETE` to the `/template-folders/{folder_id}` endpoint.
     *
     * Delete a specific template folder, and mark all the templates in the folder as 'unfiled'.
     *
     * **Parameters:**
     *
     * * `folder_id: &str` -- The unique id for the template folder.
     */
    pub async fn delete(&self, folder_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/template-folders/{}",
                crate::progenitor_support::encode_path(folder_id),
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
     * Update template folder.
     *
     * This function performs a `PATCH` to the `/template-folders/{folder_id}` endpoint.
     *
     * Update a specific folder used to organize templates.
     *
     * **Parameters:**
     *
     * * `folder_id: &str` -- The unique id for the template folder.
     */
    pub async fn patch(
        &self,
        folder_id: &str,
        body: &crate::types::GalleryFolder,
    ) -> ClientResult<crate::Response<crate::types::Folders>> {
        let url = self.client.url(
            &format!(
                "/template-folders/{}",
                crate::progenitor_support::encode_path(folder_id),
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
