use crate::Client;
use crate::ClientResult;

pub struct LandingPages {
    pub client: Client,
}

impl LandingPages {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        LandingPages { client }
    }

    /**
     * List landing pages.
     *
     * This function performs a `GET` to the `/landing-pages` endpoint.
     *
     * Get all landing pages.
     *
     * **Parameters:**
     *
     * * `sort_dir: crate::types::SortDir` -- Determines the order direction for sorted results.
     * * `sort_field: crate::types::GetAllLandingPagesSortField` -- Returns files sorted by the specified field.
     * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
     */
    pub async fn get_all(
        &self,
        sort_dir: crate::types::SortDir,
        sort_field: crate::types::GetAllLandingPagesSortField,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
    ) -> ClientResult<crate::types::GetAllLandingPagesResponse> {
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
        if !sort_dir.to_string().is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/landing-pages?{}", query_), None);
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
     * Add landing page.
     *
     * This function performs a `POST` to the `/landing-pages` endpoint.
     *
     * Create a new Mailchimp landing page.
     *
     * **Parameters:**
     *
     * * `use_default_list: bool` -- Will create the Landing Page using the account's Default List instead of requiring a list_id.
     */
    pub async fn post_all(
        &self,
        use_default_list: bool,
        body: &crate::types::LandingPageData,
    ) -> ClientResult<crate::types::LandingPage> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if use_default_list {
            query_args.push(("use_default_list".to_string(), use_default_list.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/landing-pages?{}", query_), None);
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
     * Get landing page info.
     *
     * This function performs a `GET` to the `/landing-pages/{page_id}` endpoint.
     *
     * Get information about a specific page.
     *
     * **Parameters:**
     *
     * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `page_id: &str` -- The unique id for the page.
     */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        page_id: &str,
    ) -> ClientResult<crate::types::LandingPage> {
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
                "/landing-pages/{}?{}",
                crate::progenitor_support::encode_path(page_id),
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
     * Delete landing page.
     *
     * This function performs a `DELETE` to the `/landing-pages/{page_id}` endpoint.
     *
     * Delete a landing page.
     *
     * **Parameters:**
     *
     * * `page_id: &str` -- The unique id for the page.
     */
    pub async fn delete(&self, page_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/landing-pages/{}",
                crate::progenitor_support::encode_path(page_id),
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
     * Update landing page.
     *
     * This function performs a `PATCH` to the `/landing-pages/{page_id}` endpoint.
     *
     * Update a landing page.
     *
     * **Parameters:**
     *
     * * `page_id: &str` -- The unique id for the page.
     */
    pub async fn patch(
        &self,
        page_id: &str,
        body: &crate::types::LandingPageDataType,
    ) -> ClientResult<crate::types::LandingPage> {
        let url = self.client.url(
            &format!(
                "/landing-pages/{}",
                crate::progenitor_support::encode_path(page_id),
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
    /**
     * Publish landing page.
     *
     * This function performs a `POST` to the `/landing-pages/{page_id}/actions/publish` endpoint.
     *
     * Publish a landing page that is in draft, unpublished, or has been previously published and edited.
     *
     * **Parameters:**
     *
     * * `page_id: &str` -- The unique id for the page.
     */
    pub async fn post_actions_publish(
        &self,
        page_id: &str,
    ) -> ClientResult<crate::types::LandingPage> {
        let url = self.client.url(
            &format!(
                "/landing-pages/{}/actions/publish",
                crate::progenitor_support::encode_path(page_id),
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
     * Unpublish landing page.
     *
     * This function performs a `POST` to the `/landing-pages/{page_id}/actions/unpublish` endpoint.
     *
     * Unpublish a landing page that is in draft or has been published.
     *
     * **Parameters:**
     *
     * * `page_id: &str` -- The unique id for the page.
     */
    pub async fn post_actions_unpublish(&self, page_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/landing-pages/{}/actions/unpublish",
                crate::progenitor_support::encode_path(page_id),
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
     * Get landing page content.
     *
     * This function performs a `GET` to the `/landing-pages/{page_id}/content` endpoint.
     *
     * Get the the HTML for your landing page.
     *
     * **Parameters:**
     *
     * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `page_id: &str` -- The unique id for the page.
     */
    pub async fn get_content(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        page_id: &str,
    ) -> ClientResult<crate::types::LandingPageContent> {
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
                "/landing-pages/{}/content?{}",
                crate::progenitor_support::encode_path(page_id),
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
}
