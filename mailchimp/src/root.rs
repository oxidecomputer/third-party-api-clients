use anyhow::Result;

use crate::Client;

pub struct Root {
    pub client: Client,
}

impl Root {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Root { client }
    }

    /**
    * List api root resources.
    *
    * This function performs a `GET` to the `/` endpoint.
    *
    * Get links to all other resources available in the API.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
    ) -> Result<crate::types::ApiRoot> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("?{}", query_);

        self.client.get(&url, None).await
    }
}
