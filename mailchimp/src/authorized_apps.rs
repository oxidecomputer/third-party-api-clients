use anyhow::Result;

use crate::Client;

pub struct AuthorizedApps {
    pub client: Client,
}

impl AuthorizedApps {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AuthorizedApps { client }
    }

    /**
    * List authorized apps.
    *
    * This function performs a `GET` to the `/authorized-apps` endpoint.
    *
    * Get a list of an account's registered, connected applications.
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
    ) -> Result<crate::types::GetAuthorizedAppsResponse> {
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
        let url = format!("/authorized-apps?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Get authorized app info.
    *
    * This function performs a `GET` to the `/authorized-apps/{app_id}` endpoint.
    *
    * Get information about a specific authorized application.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `app_id: &str` -- The unique id for the connected authorized application.
    */
    pub async fn get_authorized_apps(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        app_id: &str,
    ) -> Result<crate::types::Apps> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/authorized-apps/{}?{}",
            crate::progenitor_support::encode_path(&app_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
