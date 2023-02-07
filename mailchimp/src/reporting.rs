use anyhow::Result;

use crate::Client;

pub struct Reporting {
    pub client: Client,
}

impl Reporting {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reporting { client }
    }

    /**
    * List facebook ads reports.
    *
    * This function performs a `GET` to the `/reporting/facebook-ads` endpoint.
    *
    * Get reports of Facebook ads.
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
    pub async fn get_facebook_ads(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        sort_field: crate::types::GetAllFacebookAdsSortField,
        sort_dir: crate::types::SortDir,
    ) -> Result<crate::types::GetReportingFacebookAdsResponse> {
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
        let url = format!("/reporting/facebook-ads?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Get facebook ad report.
    *
    * This function performs a `GET` to the `/reporting/facebook-ads/{outreach_id}` endpoint.
    *
    * Get report of a Facebook ad.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `outreach_id: &str` -- The name of the folder.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get_facebook_ad(
        &self,
        fields: &[String],
        outreach_id: &str,
        exclude_fields: &[String],
    ) -> Result<crate::types::GetReportingFacebookAdsResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reporting/facebook-ads/{}?{}",
            crate::progenitor_support::encode_path(&outreach_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List facebook ecommerce report.
    *
    * This function performs a `GET` to the `/reporting/facebook-ads/{outreach_id}/ecommerce-product-activity` endpoint.
    *
    * Get breakdown of product activity for an outreach.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `outreach_id: &str` -- The name of the folder.
    * * `sort_field: crate::types::GetReportsEcommerceProductActivitySortField` -- Returns files sorted by the specified field.
    */
    pub async fn get_facebook_ads_ecommerce_product_activity(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        outreach_id: &str,
        sort_field: crate::types::GetReportsEcommerceProductActivitySortField,
    ) -> Result<crate::types::GetReportsEcommerceProductActivityResponse> {
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
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reporting/facebook-ads/{}/ecommerce-product-activity?{}",
            crate::progenitor_support::encode_path(&outreach_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get landing page report.
    *
    * This function performs a `GET` to the `/reporting/landing-pages/{outreach_id}` endpoint.
    *
    * Get report of a landing page.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `outreach_id: &str` -- The name of the folder.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get_landing_page(
        &self,
        fields: &[String],
        outreach_id: &str,
        exclude_fields: &[String],
    ) -> Result<crate::types::LandingPages> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reporting/landing-pages/{}?{}",
            crate::progenitor_support::encode_path(&outreach_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List landing pages reports.
    *
    * This function performs a `GET` to the `/reporting/landing-pages` endpoint.
    *
    * Get reports of landing pages.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    */
    pub async fn get_landing_pages(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
    ) -> Result<crate::types::GetReportingLandingPagesResponse> {
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
        let url = format!("/reporting/landing-pages?{}", query_);

        self.client.get(&url, None).await
    }
}
