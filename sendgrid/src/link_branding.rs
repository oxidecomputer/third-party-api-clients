use crate::Client;
use crate::ClientResult;

pub struct LinkBranding {
    pub client: Client,
}

impl LinkBranding {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        LinkBranding { client }
    }

    /**
     * Retrieve all branded links.
     *
     * This function performs a `GET` to the `/whitelabel/links` endpoint.
     *
     * **This endpoint allows you to retrieve all branded links**.
     *
     * You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.
     *
     * **Parameters:**
     *
     * * `limit: i64` -- Limits the number of results returned per page.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_whitelabel_links(
        &self,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::LinkBranding200Response>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/links?{}", query_), None);
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
     * Retrieve all branded links.
     *
     * This function performs a `GET` to the `/whitelabel/links` endpoint.
     *
     * As opposed to `get_whitelabel_links`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all branded links**.
     *
     * You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.
     */
    pub async fn get_all_whitelabel_links(
        &self,
    ) -> ClientResult<Vec<crate::types::LinkBranding200Response>> {
        let url = self.client.url("/whitelabel/links", None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create a branded link.
     *
     * This function performs a `POST` to the `/whitelabel/links` endpoint.
     *
     * **This endpoint allows you to create a new branded link.**
     *
     * To create the link branding, supply the root domain and, optionally, the subdomain — these go into separate fields in your request body. The root domain should match your FROM email address. If you provide a  subdomain, it must be different from the subdomain you used for authenticating your domain.
     *
     * You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_whitelabel_link(
        &self,
        body: &crate::types::PostWhitelabelLinksRequest,
    ) -> ClientResult<crate::types::LinkBranding200Response> {
        let url = self.client.url("/whitelabel/links", None);
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
     * Validate a branded link.
     *
     * This function performs a `POST` to the `/whitelabel/links/{id}/validate` endpoint.
     *
     * **This endpoint allows you to validate a branded link.**
     *
     * You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_whitelabel_links_validate(
        &self,
        id: i64,
    ) -> ClientResult<crate::types::PostWhitelabelLinksValidateResponse> {
        let url = self.client.url(
            &format!(
                "/whitelabel/links/{}/validate",
                crate::progenitor_support::encode_path(&id.to_string()),
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
    /**
     * Associate a branded link with a subuser.
     *
     * This function performs a `POST` to the `/whitelabel/links/{link_id}/subuser` endpoint.
     *
     * **This endpoint allows you to associate a branded link with a subuser account.**
     *
     * Link branding can be associated with subusers from the parent account. This functionality allows subusers to send mail using their parent's link branding. To associate link branding, the parent account must first create a branded link and validate it. The parent may then associate that branded link with a subuser via the API or the [Subuser Management page of the Twilio SendGrid App](https://app.sendgrid.com/settings/subusers).
     */
    pub async fn post_whitelabel_links_link_subuser(
        &self,
        link_id: i64,
        body: &crate::types::PostWhitelabelLinksLinkSubuserRequest,
    ) -> ClientResult<crate::types::LinkBranding200Response> {
        let url = self.client.url(
            &format!(
                "/whitelabel/links/{}/subuser",
                crate::progenitor_support::encode_path(&link_id.to_string()),
            ),
            None,
        );
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
     * Retrieve a branded link.
     *
     * This function performs a `GET` to the `/whitelabel/links/{id}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific branded link by providing its ID.**
     *
     * You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_whitelabel_links_link_branding(
        &self,
        id: i64,
    ) -> ClientResult<crate::types::LinkBranding200Response> {
        let url = self.client.url(
            &format!(
                "/whitelabel/links/{}",
                crate::progenitor_support::encode_path(&id.to_string()),
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
     * Delete a branded link.
     *
     * This function performs a `DELETE` to the `/whitelabel/links/{id}` endpoint.
     *
     * **This endpoint allows you to delete a branded link.**
     *
     * Your request will receive a response with a 204 status code if the deletion was successful. The call does not return the link's details, so if you wish to record these make sure you call the  "Retrieve a branded link" endpoint *before* you request its deletion.
     *
     * You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_whitelabel_links(&self, id: i64) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/whitelabel/links/{}",
                crate::progenitor_support::encode_path(&id.to_string()),
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
     * Update a branded link.
     *
     * This function performs a `PATCH` to the `/whitelabel/links/{id}` endpoint.
     *
     * **This endpoint allows you to update a specific branded link. You can use this endpoint to change a branded link's default status.**
     *
     * You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_whitelabel_links(
        &self,
        id: i64,
        body: &crate::types::PatchWhitelabelLinksRequest,
    ) -> ClientResult<crate::types::LinkBranding200Response> {
        let url = self.client.url(
            &format!(
                "/whitelabel/links/{}",
                crate::progenitor_support::encode_path(&id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieve the default branded link.
     *
     * This function performs a `GET` to the `/whitelabel/links/default` endpoint.
     *
     * **This endpoint allows you to retrieve the default branded link.**
     *
     * The default branded link is the actual URL to be used when sending messages. If you have more than one branded link, the default is determined by the following order:
     *
     * * The validated branded link marked as `default` (set when you call the "Create a branded link" endpoint or by calling the "Update a branded link" endpoint on an existing link)
     * * Legacy branded links (migrated from the whitelabel wizard)
     * * Default SendGrid-branded links (i.e., `100.ct.sendgrid.net`)
     *
     * You can submit this request as one of your subusers if you include their ID in the `on-behalf-of` header in the request.
     *
     * **Parameters:**
     *
     * * `domain: &str` -- The domain to match against when finding the default branded link.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_whitelabel_links_default(
        &self,
        domain: &str,
    ) -> ClientResult<crate::types::LinkBranding200Response> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/links/default?{}", query_), None);
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
     * Retrieve a subuser's branded link.
     *
     * This function performs a `GET` to the `/whitelabel/links/subuser` endpoint.
     *
     * **This endpoint allows you to retrieve the branded link associated with a subuser.**
     *
     * Link branding can be associated with subusers from the parent account. This functionality allows subusers to send mail using their parent's link branding. To associate link branding, the parent account must first create a branded link and then validate it. The parent may then associate that branded link with a subuser via the API or the [Subuser Management page of the Twilio SendGrid App](https://app.sendgrid.com/settings/subusers).
     *
     * **Parameters:**
     *
     * * `username: &str` -- The username of the subuser to retrieve associated branded links for.
     */
    pub async fn get_whitelabel_links_subuser(
        &self,
        username: &str,
    ) -> ClientResult<crate::types::LinkBranding200Response> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !username.is_empty() {
            query_args.push(("username".to_string(), username.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/links/subuser?{}", query_), None);
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
     * Disassociate a branded link from a subuser.
     *
     * This function performs a `DELETE` to the `/whitelabel/links/subuser` endpoint.
     *
     * **This endpoint allows you to take a branded link away from a subuser.**
     *
     * Link branding can be associated with subusers from the parent account. This functionality allows subusers to send mail using their parent's link branding. To associate link branding, the parent account must first create a branded link and validate it. The parent may then associate that branded link with a subuser via the API or the [Subuser Management page of the Twilio SendGrid App](https://app.sendgrid.com/settings/subusers).
     *
     * Your request will receive a response with a 204 status code if the disassociation was successful.
     *
     * **Parameters:**
     *
     * * `username: &str` -- The username of the subuser account that you want to disassociate a branded link from.
     */
    pub async fn delete_whitelabel_links_subuser(
        &self,
        username: &str,
    ) -> ClientResult<crate::types::Help> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !username.is_empty() {
            query_args.push(("username".to_string(), username.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/links/subuser?{}", query_), None);
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
}
