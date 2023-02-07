use anyhow::Result;

use crate::Client;

pub struct CampaignFolders {
    pub client: Client,
}

impl CampaignFolders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CampaignFolders { client }
    }

    /**
    * List campaign folders.
    *
    * This function performs a `GET` to the `/campaign-folders` endpoint.
    *
    * Get all folders used to organize campaigns.
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
    ) -> Result<crate::types::CampaignFolders> {
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
        let url = format!("/campaign-folders?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Add campaign folder.
    *
    * This function performs a `POST` to the `/campaign-folders` endpoint.
    *
    * Create a new campaign folder.
    */
    pub async fn post(
        &self,
        body: &crate::types::GalleryFolder,
    ) -> Result<crate::types::CampaignFolder> {
        let url = "/campaign-folders".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get campaign folder.
    *
    * This function performs a `GET` to the `/campaign-folders/{folder_id}` endpoint.
    *
    * Get information about a specific folder used to organize campaigns.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `folder_id: &str` -- The unique id for the campaign folder.
    */
    pub async fn get_campaign_folders(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        folder_id: &str,
    ) -> Result<crate::types::CampaignFolder> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/campaign-folders/{}?{}",
            crate::progenitor_support::encode_path(&folder_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete campaign folder.
    *
    * This function performs a `DELETE` to the `/campaign-folders/{folder_id}` endpoint.
    *
    * Delete a specific campaign folder, and mark all the campaigns in the folder as 'unfiled'.
    *
    * **Parameters:**
    *
    * * `folder_id: &str` -- The unique id for the campaign folder.
    */
    pub async fn delete(&self, folder_id: &str) -> Result<()> {
        let url = format!(
            "/campaign-folders/{}",
            crate::progenitor_support::encode_path(&folder_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update campaign folder.
    *
    * This function performs a `PATCH` to the `/campaign-folders/{folder_id}` endpoint.
    *
    * Update a specific folder used to organize campaigns.
    *
    * **Parameters:**
    *
    * * `folder_id: &str` -- The unique id for the campaign folder.
    */
    pub async fn patch(
        &self,
        folder_id: &str,
        body: &crate::types::GalleryFolder,
    ) -> Result<crate::types::CampaignFolder> {
        let url = format!(
            "/campaign-folders/{}",
            crate::progenitor_support::encode_path(&folder_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
