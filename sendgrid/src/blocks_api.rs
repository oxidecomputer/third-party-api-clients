use anyhow::Result;

use crate::Client;

pub struct BlocksApi {
    pub client: Client,
}

impl BlocksApi {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        BlocksApi { client }
    }

    /**
     * Retrieve all blocks.
     *
     * This function performs a `GET` to the `/suppression/blocks` endpoint.
     *
     * **This endpoint allows you to retrieve all email addresses that are currently on your blocks list.**
     *
     * **Parameters:**
     *
     * * `start_time: i64` -- The start of the time range when a blocked email was created (inclusive). This is a unix timestamp.
     * * `end_time: i64` -- The end of the time range when a blocked email was created (inclusive). This is a unix timestamp.
     * * `limit: i64` -- Limit the number of results to be displayed per page.
     * * `offset: i64` -- The point in the list to begin displaying results.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_suppression_blocks(
        &self,
        start_time: i64,
        end_time: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<crate::types::BlocksResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if end_time > 0 {
            query_args.push(("end_time".to_string(), end_time.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if start_time > 0 {
            query_args.push(("start_time".to_string(), start_time.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/suppression/blocks?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Retrieve all blocks.
     *
     * This function performs a `GET` to the `/suppression/blocks` endpoint.
     *
     * As opposed to `get_suppression_blocks`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all email addresses that are currently on your blocks list.**
     */
    pub async fn get_all_suppression_blocks(
        &self,
        start_time: i64,
        end_time: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<crate::types::BlocksResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if end_time > 0 {
            query_args.push(("end_time".to_string(), end_time.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if start_time > 0 {
            query_args.push(("start_time".to_string(), start_time.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/suppression/blocks?{}", query_);

        self.client.get_all_pages(&url, None).await
    }

    /**
     * Delete blocks.
     *
     * This function performs a `DELETE` to the `/suppression/blocks` endpoint.
     *
     * **This endpoint allows you to delete all email addresses on your blocks list.**
     *
     * There are two options for deleting blocked emails:
     *
     * 1. You can delete all blocked emails by setting `delete_all` to `true` in the request body.
     * 2. You can delete a selection of blocked emails by specifying the email addresses in the `emails` array of the request body.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_suppression_blocks(
        &self,
        body: &crate::types::DeleteSuppressionBlocksRequest,
    ) -> Result<crate::types::Help> {
        let url = "/suppression/blocks".to_string();
        self.client
            .delete(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Retrieve a specific block.
     *
     * This function performs a `GET` to the `/suppression/blocks/{email}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific email address from your blocks list.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_suppression_blocks_email(
        &self,
        email: &str,
    ) -> Result<Vec<crate::types::BlocksResponse>> {
        let url = format!(
            "/suppression/blocks/{}",
            crate::progenitor_support::encode_path(&email.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Retrieve a specific block.
     *
     * This function performs a `GET` to the `/suppression/blocks/{email}` endpoint.
     *
     * As opposed to `get_suppression_blocks_email`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a specific email address from your blocks list.**
     */
    pub async fn get_all_suppression_blocks_email(
        &self,
        email: &str,
    ) -> Result<Vec<crate::types::BlocksResponse>> {
        let url = format!(
            "/suppression/blocks/{}",
            crate::progenitor_support::encode_path(&email.to_string()),
        );

        self.client.get_all_pages(&url, None).await
    }

    /**
     * Delete a specific block.
     *
     * This function performs a `DELETE` to the `/suppression/blocks/{email}` endpoint.
     *
     * **This endpoint allows you to delete a specific email address from your blocks list.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_suppression_blocks_email(&self, email: &str) -> Result<crate::types::Help> {
        let url = format!(
            "/suppression/blocks/{}",
            crate::progenitor_support::encode_path(&email.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
