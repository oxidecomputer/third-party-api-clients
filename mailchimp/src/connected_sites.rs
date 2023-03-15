use crate::Client;
use crate::ClientResult;

pub struct ConnectedSites {
    pub client: Client,
}

impl ConnectedSites {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ConnectedSites { client }
    }

    /**
     * List connected sites.
     *
     * This function performs a `GET` to the `/connected-sites` endpoint.
     *
     * Get all connected sites in an account.
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
    ) -> ClientResult<crate::types::ConnectedSites> {
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
        let url = self
            .client
            .url(&format!("/connected-sites?{}", query_), None);
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
     * Add connected site.
     *
     * This function performs a `POST` to the `/connected-sites` endpoint.
     *
     * Create a new Mailchimp connected site.
     */
    pub async fn post(
        &self,
        body: &crate::types::ConnectedSite,
    ) -> ClientResult<crate::types::Sites> {
        let url = self.client.url("/connected-sites", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get connected site.
     *
     * This function performs a `GET` to the `/connected-sites/{connected_site_id}` endpoint.
     *
     * Get information about a specific connected site.
     *
     * **Parameters:**
     *
     * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `connected_site_id: &str` -- The unique identifier for the site.
     */
    pub async fn get_connected_sites(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        connected_site_id: &str,
    ) -> ClientResult<crate::types::Sites> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/connected-sites/{}?{}",
                crate::progenitor_support::encode_path(connected_site_id),
                query_
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
     * Delete connected site.
     *
     * This function performs a `DELETE` to the `/connected-sites/{connected_site_id}` endpoint.
     *
     * Remove a connected site from your Mailchimp account.
     *
     * **Parameters:**
     *
     * * `connected_site_id: &str` -- The unique identifier for the site.
     */
    pub async fn delete(&self, connected_site_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/connected-sites/{}",
                crate::progenitor_support::encode_path(connected_site_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Verify connected site script.
     *
     * This function performs a `POST` to the `/connected-sites/{connected_site_id}/actions/verify-script-installation` endpoint.
     *
     * Verify that the connected sites script has been installed, either via the script URL or fragment.
     *
     * **Parameters:**
     *
     * * `connected_site_id: &str` -- The unique identifier for the site.
     */
    pub async fn post_actions_verify_script_installation(
        &self,
        connected_site_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/connected-sites/{}/actions/verify-script-installation",
                crate::progenitor_support::encode_path(connected_site_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
