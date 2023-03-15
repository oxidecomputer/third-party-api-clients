use crate::Client;
use crate::ClientResult;

pub struct GroupBrands {
    pub client: Client,
}

impl GroupBrands {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        GroupBrands { client }
    }

    /**
     * Gets the brand information for a group.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/groups/{groupId}/brands` endpoint.
     *
     * This method returns information about the brands associated with a group.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `group_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brands_get_group(
        &self,
        account_id: &str,
        group_id: &str,
    ) -> ClientResult<crate::types::GroupBrands> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/groups/{}/brands",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(group_id),
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
     * Adds an existing brand to a group.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/groups/{groupId}/brands` endpoint.
     *
     * This method adds one or more existing brands to a group based on the `groupId`.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `group_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brands_put_group(
        &self,
        account_id: &str,
        group_id: &str,
        body: &crate::types::BrandsRequest,
    ) -> ClientResult<crate::types::GroupBrands> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/groups/{}/brands",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(group_id),
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
     * Deletes brand information from a group.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/groups/{groupId}/brands` endpoint.
     *
     * This method deletes one or more brands from a group.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `group_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brands_delete_group(
        &self,
        account_id: &str,
        group_id: &str,
        body: &crate::types::BrandsRequest,
    ) -> ClientResult<crate::types::GroupBrands> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/groups/{}/brands",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(group_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
}
