use anyhow::Result;

use crate::Client;

pub struct CustomTabs {
    client: Client,
}

impl CustomTabs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CustomTabs { client }
    }

    /**
     * Gets a list of all account tabs.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/tab_definitions` endpoint.
     *
     * Retrieves a list of all tabs associated with the account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `custom_tab_only: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn tabs_get_tab_definition(
        &self,
        account_id: &str,
        custom_tab_only: &str,
    ) -> Result<crate::types::TabMetadataList> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !custom_tab_only.is_empty() {
            query_args.push(format!("custom_tab_only={}", custom_tab_only));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/tab_definitions?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Creates a custom tab.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/tab_definitions` endpoint.
     *
     * Creates a tab with pre-defined properties, such as a text tab with a certain font type and validation pattern. Users can access the custom tabs when sending documents through the DocuSign web application.
     *
     * Custom tabs can be created for approve, checkbox, company, date, date signed, decline, email, email address, envelope ID, first name, formula, full name, initial here, last name, list, note, number, radio, sign here, signer attachment, SSN, text, title, and zip tabs.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn tabs_post_tab_definitions(
        &self,
        account_id: &str,
        body: &crate::types::TabMetadata,
    ) -> Result<crate::types::TabMetadata> {
        let url = format!(
            "/v2.1/accounts/{}/tab_definitions",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Gets custom tab information.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/tab_definitions/{customTabId}` endpoint.
     *
     * Retrieves information about the requested custom tab on the specified account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `custom_tab_id: &str` -- The DocuSign-generated custom tab id for the custom tab to be applied. This can only be used when adding new tabs for a recipient. When used, the new tab inherits all the custom tab properties.
     */
    pub async fn tab_get_custom(
        &self,
        account_id: &str,
        custom_tab_id: &str,
    ) -> Result<crate::types::TabMetadata> {
        let url = format!(
            "/v2.1/accounts/{}/tab_definitions/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&custom_tab_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Updates custom tab information.

    .
    *
    * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/tab_definitions/{customTabId}` endpoint.
    *
    * Updates the information in a custom tab for the specified account.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `custom_tab_id: &str` -- The DocuSign-generated custom tab id for the custom tab to be applied. This can only be used when adding new tabs for a recipient. When used, the new tab inherits all the custom tab properties.
    */
    pub async fn tab_put_custom(
        &self,
        account_id: &str,
        custom_tab_id: &str,
        body: &crate::types::TabMetadata,
    ) -> Result<crate::types::TabMetadata> {
        let url = format!(
            "/v2.1/accounts/{}/tab_definitions/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&custom_tab_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Deletes custom tab information.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/tab_definitions/{customTabId}` endpoint.
     *
     * Deletes the custom from the specified account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `custom_tab_id: &str` -- The DocuSign-generated custom tab id for the custom tab to be applied. This can only be used when adding new tabs for a recipient. When used, the new tab inherits all the custom tab properties.
     */
    pub async fn tab_delete_custom(&self, account_id: &str, custom_tab_id: &str) -> Result<()> {
        let url = format!(
            "/v2.1/accounts/{}/tab_definitions/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&custom_tab_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
