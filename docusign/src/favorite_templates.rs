use anyhow::Result;

use crate::Client;

pub struct FavoriteTemplates {
    pub client: Client,
}

impl FavoriteTemplates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        FavoriteTemplates { client }
    }

    /**
    * Retrieves the list of favorited templates for this caller.
    *
    * This function performs a `GET` to the `/v2.1/accounts/{accountId}/favorite_templates` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn get(&self, account_id: &str) -> Result<crate::types::FavoriteTemplatesInfo> {
        let url = format!(
            "/v2.1/accounts/{}/favorite_templates",
            crate::progenitor_support::encode_path(account_id),
        );

        self.client.get(&url, None).await
    }

    /**
    * Sets a template as a favorite.
    .
    *
    * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/favorite_templates` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn put_template(
        &self,
        account_id: &str,
        body: &crate::types::FavoriteTemplatesInfo,
    ) -> Result<crate::types::FavoriteTemplatesInfo> {
        let url = format!(
            "/v2.1/accounts/{}/favorite_templates",
            crate::progenitor_support::encode_path(account_id),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Unfavorites a template.
    *
    * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/favorite_templates` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn un_template(
        &self,
        account_id: &str,
        body: &crate::types::FavoriteTemplatesInfo,
    ) -> Result<crate::types::FavoriteTemplatesInfo> {
        let url = format!(
            "/v2.1/accounts/{}/favorite_templates",
            crate::progenitor_support::encode_path(account_id),
        );

        self.client
            .delete(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
