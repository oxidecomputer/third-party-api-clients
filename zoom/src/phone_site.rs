use anyhow::Result;

use crate::Client;

pub struct PhoneSite {
    client: Client,
}

impl PhoneSite {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PhoneSite { client }
    }

    /**
     * List phone sites.
     *
     * This function performs a `GET` to the `/phone/sites` endpoint.
     *
     * Sites allow you to organize Zoom Phone users in your organization. Use this API to list all the [sites](https://support.zoom.us/hc/en-us/articles/360020809672) that have been created for an account.<br>
     * **Prerequisites:**<br>
     * * Multiple Sites must be [enabled](https://support.zoom.us/hc/en-us/articles/360020809672-Managing-Multiple-Sites#h_05c88e35-1593-491f-b1a8-b7139a75dc15).
     * * Pro or a higher account with Zoom Phone enabled.
     *
     * **Scope:** `phone:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list(
        &self,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::ListPhoneSitesResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(format!("next_page_token={}", next_page_token));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/phone/sites?{}", query);

        self.client.get(&url, None).await
    }

    /**
     * Create a phone site.
     *
     * This function performs a `POST` to the `/phone/sites` endpoint.
     *
     * Sites allow you to organize Zoom Phone users in your organization. Use this API to create a [Site](https://support.zoom.us/hc/en-us/articles/360020809672).<br>
     * **Prerequisites:**<br>
     * * Multiple Sites must be [enabled](https://support.zoom.us/hc/en-us/articles/360020809672-Managing-Multiple-Sites#h_05c88e35-1593-491f-b1a8-b7139a75dc15).
     * * Pro or a higher account with Zoom Phone enabled.
     * **Scope:** `phone:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     */
    pub async fn create(
        &self,
        body: &crate::types::CreatePhoneSiteRequest,
    ) -> Result<crate::types::CreatePhoneSiteResponse> {
        let url = "/phone/sites".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get phone site details.
     *
     * This function performs a `GET` to the `/phone/sites/{siteId}` endpoint.
     *
     * Sites allow you to organize Zoom Phone users in your organization. Use this API to get information about a specific [site](https://support.zoom.us/hc/en-us/articles/360020809672).
     *
     *
     * **Prerequisites:** <br>
     * * Account must have a Pro or a higher plan with Zoom Phone license.
     * * Multiple Sites must be [enabled](https://support.zoom.us/hc/en-us/articles/360020809672-Managing-Multiple-Sites#h_05c88e35-1593-491f-b1a8-b7139a75dc15).<br>
     * **Scope:** `phone:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `site_id: &str` -- Unique Identifier of the Site.
     */
    pub async fn get_a_site(&self, site_id: &str) -> Result<crate::types::GetSiteResponse> {
        let url = format!(
            "/phone/sites/{}",
            crate::progenitor_support::encode_path(&site_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a phone site.
     *
     * This function performs a `DELETE` to the `/phone/sites/{siteId}` endpoint.
     *
     * Sites allow you to organize Zoom Phone users in your organization. Use this API to delete a specific [site](https://support.zoom.us/hc/en-us/articles/360020809672) in a Zoom account. To delete a site, in the query parameter, you must provide the Site ID of another site where the assets of current site (users, numbers and phones) can be transferred to.  You cannot use this API to delete the main site.
     *
     * **Prerequisites:** <br>
     * * Account must have a Pro or a higher plan with Zoom Phone license.
     * * [Multiple Sites](https://support.zoom.us/hc/en-us/articles/360020809672-Managing-Multiple-Sites) must be enabled.<br>
     * **Scope:** `phone:write:admin`
     * <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `site_id: &str` -- Unique Identifier of the Site.
     * * `transfer_site_id: &str` -- The Site ID of another site where the assets of the current site (users, numbers and phones) can be transferred to.
     */
    pub async fn delete(&self, site_id: &str, transfer_site_id: &str) -> Result<()> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !transfer_site_id.is_empty() {
            query_args.push(format!("transfer_site_id={}", transfer_site_id));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/phone/sites/{}?{}",
            crate::progenitor_support::encode_path(&site_id.to_string()),
            query
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update phone site details.
     *
     * This function performs a `PATCH` to the `/phone/sites/{siteId}` endpoint.
     *
     * Sites allow you to organize Zoom Phone users in your organization. Use this API to update information about a specific [site](https://support.zoom.us/hc/en-us/articles/360020809672).
     *
     *
     * **Prerequisites:** <br>
     * * Account must have a Pro or a higher plan with Zoom Phone license.
     * * **Scope:** `phone:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `site_id: &str` -- Unique Identifier of the Site.
     */
    pub async fn update_site_details(
        &self,
        site_id: &str,
        body: &crate::types::UpdateSiteDetailsRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/sites/{}",
            crate::progenitor_support::encode_path(&site_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
