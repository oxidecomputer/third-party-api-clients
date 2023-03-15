use crate::Client;
use crate::ClientResult;

pub struct Templates {
    pub client: Client,
}

impl Templates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Templates { client }
    }

    /**
     * List SMS Templates.
     *
     * This function performs a `GET` to the `/api/v1/templates/sms` endpoint.
     *
     * Enumerates custom SMS templates in your organization. A subset of templates can be returned that match a template type.
     *
     * **Parameters:**
     *
     * * `template_type: &str`
     */
    pub async fn list_sms(
        &self,
        template_type: &str,
    ) -> ClientResult<Vec<crate::types::SmsTemplate>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !template_type.is_empty() {
            query_args.push(("templateType".to_string(), template_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/templates/sms?{}", query_), None);
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
     * List SMS Templates.
     *
     * This function performs a `GET` to the `/api/v1/templates/sms` endpoint.
     *
     * As opposed to `list_sms`, this function returns all the pages of the request at once.
     *
     * Enumerates custom SMS templates in your organization. A subset of templates can be returned that match a template type.
     */
    pub async fn list_all_sms(
        &self,
        template_type: &str,
    ) -> ClientResult<Vec<crate::types::SmsTemplate>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !template_type.is_empty() {
            query_args.push(("templateType".to_string(), template_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/templates/sms?{}", query_), None);
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
     * Add SMS Template.
     *
     * This function performs a `POST` to the `/api/v1/templates/sms` endpoint.
     *
     * Adds a new custom SMS template to your organization.
     */
    pub async fn create_sms(
        &self,
        body: &crate::types::SmsTemplate,
    ) -> ClientResult<crate::types::SmsTemplate> {
        let url = self.client.url("/api/v1/templates/sms", None);
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
     * Get SMS Template.
     *
     * This function performs a `GET` to the `/api/v1/templates/sms/{templateId}` endpoint.
     *
     * Fetches a specific template by `id`
     *
     * **Parameters:**
     *
     * * `template_id: &str`
     */
    pub async fn get_sm(&self, template_id: &str) -> ClientResult<crate::types::SmsTemplate> {
        let url = self.client.url(
            &format!(
                "/api/v1/templates/sms/{}",
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
     * Update SMS Template.
     *
     * This function performs a `PUT` to the `/api/v1/templates/sms/{templateId}` endpoint.
     *
     * Updates the SMS template.
     *
     * **Parameters:**
     *
     * * `template_id: &str`
     */
    pub async fn update_sms(
        &self,
        template_id: &str,
        body: &crate::types::SmsTemplate,
    ) -> ClientResult<crate::types::SmsTemplate> {
        let url = self.client.url(
            &format!(
                "/api/v1/templates/sms/{}",
                crate::progenitor_support::encode_path(template_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Partial SMS Template Update.
     *
     * This function performs a `POST` to the `/api/v1/templates/sms/{templateId}` endpoint.
     *
     * Updates only some of the SMS template properties:
     *
     * **Parameters:**
     *
     * * `template_id: &str`
     */
    pub async fn partial_update_sms(
        &self,
        template_id: &str,
        body: &crate::types::SmsTemplate,
    ) -> ClientResult<crate::types::SmsTemplate> {
        let url = self.client.url(
            &format!(
                "/api/v1/templates/sms/{}",
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
     * Remove SMS Template.
     *
     * This function performs a `DELETE` to the `/api/v1/templates/sms/{templateId}` endpoint.
     *
     * Removes an SMS template.
     *
     * **Parameters:**
     *
     * * `template_id: &str`
     */
    pub async fn delete_sms(&self, template_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/templates/sms/{}",
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
}
