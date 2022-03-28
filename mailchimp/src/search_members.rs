use anyhow::Result;

use crate::Client;

pub struct SearchMembers {
    pub client: Client,
}

impl SearchMembers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SearchMembers { client }
    }

    /**
    * Search members.
    *
    * This function performs a `GET` to the `/search-members` endpoint.
    *
    * Search for list members. This search can be restricted to a specific list, or can be used to search across all lists in an account.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `query: &str` -- The search query used to filter results. Query should be a valid email, or a string representing a contact's first or last name.
    * * `list_id: &str` -- The unique id for the list.
    */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        query: &str,
        list_id: &str,
    ) -> Result<crate::types::MembersData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if !list_id.is_empty() {
            query_args.push(("list_id".to_string(), list_id.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/search-members?{}", query_);

        self.client.get(&url, None).await
    }
}
