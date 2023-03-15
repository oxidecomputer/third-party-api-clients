use crate::Client;
use crate::ClientResult;

pub struct AccountCustomFields {
    pub client: Client,
}

impl AccountCustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountCustomFields { client }
    }

    /**
     * Gets a list of custom fields.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/custom_fields` endpoint.
     *
     * This method returns a list of the envelope and document custom fields associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(&self, account_id: &str) -> ClientResult<crate::types::AccountCustomFields> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/custom_fields",
                crate::progenitor_support::encode_path(account_id),
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
     * Creates an account custom field.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/custom_fields` endpoint.
     *
     * This method creates a custom field and makes it available for all new envelopes associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `apply_to_templates: &str` -- (Optional) When set to **true**, the new custom field is applied to all of the templates on the account.
     */
    pub async fn post(
        &self,
        account_id: &str,
        apply_to_templates: &str,
        body: &crate::types::CustomField,
    ) -> ClientResult<crate::types::AccountCustomFields> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !apply_to_templates.is_empty() {
            query_args.push((
                "apply_to_templates".to_string(),
                apply_to_templates.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/custom_fields?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
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
     * Updates an account custom field.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/custom_fields/{customFieldId}` endpoint.
     *
     * This method updates an existing account custom field.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `custom_field_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `apply_to_templates: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        custom_field_id: &str,
        apply_to_templates: &str,
        body: &crate::types::CustomField,
    ) -> ClientResult<crate::types::AccountCustomFields> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !apply_to_templates.is_empty() {
            query_args.push((
                "apply_to_templates".to_string(),
                apply_to_templates.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/custom_fields/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(custom_field_id),
                query_
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
     * Deletes an account custom field.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/custom_fields/{customFieldId}` endpoint.
     *
     * This method deletes an existing account custom field.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `custom_field_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `apply_to_templates: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete(
        &self,
        account_id: &str,
        custom_field_id: &str,
        apply_to_templates: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !apply_to_templates.is_empty() {
            query_args.push((
                "apply_to_templates".to_string(),
                apply_to_templates.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/custom_fields/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(custom_field_id),
                query_
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
