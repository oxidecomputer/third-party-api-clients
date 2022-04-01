use anyhow::Result;

use crate::Client;

pub struct FacebookAds {
    pub client: Client,
}

impl FacebookAds {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        FacebookAds { client }
    }

    /**
    * List facebook ads.
    *
    * This function performs a `GET` to the `/facebook-ads` endpoint.
    *
    * Get list of Facebook ads.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `sort_field: crate::types::GetAllFacebookAdsSortField` -- Returns files sorted by the specified field.
    * * `sort_dir: crate::types::SortDir` -- Determines the order direction for sorted results.
    */
    pub async fn get_all(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        sort_field: crate::types::GetAllFacebookAdsSortField,
        sort_dir: crate::types::SortDir,
    ) -> Result<crate::types::GetAllFacebookAdsResponse> {
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
        if !sort_dir.to_string().is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/facebook-ads?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Get facebook ad info.
    *
    * This function performs a `GET` to the `/facebook-ads/{outreach_id}` endpoint.
    *
    * Get details of a Facebook ad.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `outreach_id: &str` -- The name of the folder.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get(
        &self,
        fields: &[String],
        outreach_id: &str,
        exclude_fields: &[String],
    ) -> Result<crate::types::FacebookAdsAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/facebook-ads/{}?{}",
            crate::progenitor_support::encode_path(&outreach_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
