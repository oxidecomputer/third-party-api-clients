use anyhow::Result;

use crate::Client;

pub struct SearchCampaigns {
    pub client: Client,
}

impl SearchCampaigns {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SearchCampaigns { client }
    }

    /**
    * Search campaigns.
    *
    * This function performs a `GET` to the `/search-campaigns` endpoint.
    *
    * Search all campaigns for the specified query terms.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `query: &str` -- The search query used to filter results.
    */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        query: &str,
    ) -> Result<crate::types::Campaigns> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/search-campaigns?{}", query_);

        self.client.get(&url, None).await
    }
}
