use crate::Client;
use crate::ClientResult;

pub struct DomainAuthentication {
    pub client: Client,
}

impl DomainAuthentication {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        DomainAuthentication { client }
    }

    /**
     * List all authenticated domains.
     *
     * This function performs a `GET` to the `/whitelabel/domains` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all domains you have authenticated.**
     *
     * **Parameters:**
     *
     * * `limit: i64` -- Number of domains to return.
     * * `offset: i64` -- Paging offset.
     * * `exclude_subusers: bool` -- Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     * * `username: &str` -- The license key provided with your New Relic account.
     * * `domain: &str` -- The license key provided with your New Relic account.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_whitelabel_domains(
        &self,
        limit: i64,
        offset: i64,
        exclude_subusers: bool,
        username: &str,
        domain: &str,
    ) -> ClientResult<Vec<crate::types::DomainAuthentication200ResponseAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
        }
        if exclude_subusers {
            query_args.push(("exclude_subusers".to_string(), exclude_subusers.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !username.is_empty() {
            query_args.push(("username".to_string(), username.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/domains?{}", query_), None);
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
     * List all authenticated domains.
     *
     * This function performs a `GET` to the `/whitelabel/domains` endpoint.
     *
     * As opposed to `get_whitelabel_domains`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a list of all domains you have authenticated.**
     */
    pub async fn get_all_whitelabel_domains(
        &self,
        offset: i64,
        exclude_subusers: bool,
        username: &str,
        domain: &str,
    ) -> ClientResult<Vec<crate::types::DomainAuthentication200ResponseAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
        }
        if exclude_subusers {
            query_args.push(("exclude_subusers".to_string(), exclude_subusers.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !username.is_empty() {
            query_args.push(("username".to_string(), username.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/domains?{}", query_), None);
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
     * Authenticate a domain.
     *
     * This function performs a `POST` to the `/whitelabel/domains` endpoint.
     *
     * **This endpoint allows you to authenticate a domain.**
     *
     * If you are authenticating a domain for a subuser, you have two options:
     * 1. Use the "username" parameter. This allows you to authenticate a domain on behalf of your subuser. This means the subuser is able to see and modify the authenticated domain.
     * 2. Use the Association workflow (see Associate Domain section). This allows you to authenticate a domain created by the parent to a subuser. This means the subuser will default to the assigned domain, but will not be able to see or modify that authenticated domain. However, if the subuser authenticates their own domain it will overwrite the assigned domain.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_whitelabel_domain(
        &self,
        body: &crate::types::PostWhitelabelDomainsRequest,
    ) -> ClientResult<crate::types::AuthenticationDomain> {
        let url = self.client.url("/whitelabel/domains", None);
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
     * Retrieve an authenticated domain.
     *
     * This function performs a `GET` to the `/whitelabel/domains/{domain_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific authenticated domain.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_whitelabel_domains_domain(
        &self,
        domain_id: &str,
    ) -> ClientResult<crate::types::AuthenticationDomain> {
        let url = self.client.url(
            &format!(
                "/whitelabel/domains/{}",
                crate::progenitor_support::encode_path(domain_id),
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
     * Delete an authenticated domain.
     *
     * This function performs a `DELETE` to the `/whitelabel/domains/{domain_id}` endpoint.
     *
     * **This endpoint allows you to delete an authenticated domain.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_whitelabel_domains_domain(
        &self,
        domain_id: &str,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/whitelabel/domains/{}",
                crate::progenitor_support::encode_path(domain_id),
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
     * Update an authenticated domain.
     *
     * This function performs a `PATCH` to the `/whitelabel/domains/{domain_id}` endpoint.
     *
     * **This endpoint allows you to update the settings for an authenticated domain.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_whitelabel_domains_domain(
        &self,
        domain_id: &str,
        body: &crate::types::PatchWhitelabelDomainsDomainRequest,
    ) -> ClientResult<Vec<crate::types::DomainAuthentication200ResponseAllOf>> {
        let url = self.client.url(
            &format!(
                "/whitelabel/domains/{}",
                crate::progenitor_support::encode_path(domain_id),
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
     * Get the default authentication.
     *
     * This function performs a `GET` to the `/whitelabel/domains/default` endpoint.
     *
     * **This endpoint allows you to retrieve the default authentication for a domain.**
     *
     * When creating or updating a domain authentication, you can set the domain as a default. The default domain will be used to send all mail. If you have multiple authenticated domains, the authenticated domain matching the domain of the From address will be used, and the default will be overridden.
     *
     * This endpoint will return a default domain and its details only if a default is set. You are not required to set a default. If you do not set a default domain, this endpoint will return general information about your domain authentication status.
     *
     * **Parameters:**
     *
     * * `domain: &str` -- The license key provided with your New Relic account.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_whitelabel_domains_default(
        &self,
        domain: &str,
    ) -> ClientResult<Vec<crate::types::DomainAuthentication200ResponseAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/domains/default?{}", query_), None);
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
     * Get the default authentication.
     *
     * This function performs a `GET` to the `/whitelabel/domains/default` endpoint.
     *
     * As opposed to `get_whitelabel_domains_default`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve the default authentication for a domain.**
     *
     * When creating or updating a domain authentication, you can set the domain as a default. The default domain will be used to send all mail. If you have multiple authenticated domains, the authenticated domain matching the domain of the From address will be used, and the default will be overridden.
     *
     * This endpoint will return a default domain and its details only if a default is set. You are not required to set a default. If you do not set a default domain, this endpoint will return general information about your domain authentication status.
     */
    pub async fn get_all_whitelabel_domains_default(
        &self,
        domain: &str,
    ) -> ClientResult<Vec<crate::types::DomainAuthentication200ResponseAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/domains/default?{}", query_), None);
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
     * Add an IP to an authenticated domain.
     *
     * This function performs a `POST` to the `/whitelabel/domains/{id}/ips` endpoint.
     *
     * **This endpoint allows you to add an IP address to an authenticated domain.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_whitelabel_domains_ip(
        &self,
        id: i64,
        body: &crate::types::Ips,
    ) -> ClientResult<crate::types::DomainAuthentication> {
        let url = self.client.url(
            &format!(
                "/whitelabel/domains/{}/ips",
                crate::progenitor_support::encode_path(&id.to_string()),
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
     * Remove an IP from an authenticated domain.
     *
     * This function performs a `DELETE` to the `/whitelabel/domains/{id}/ips/{ip}` endpoint.
     *
     * **This endpoint allows you to remove an IP address from that domain's authentication.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_whitelabel_domains_ips_ip(
        &self,
        id: i64,
        ip: &str,
    ) -> ClientResult<crate::types::DomainAuthentication> {
        let url = self.client.url(
            &format!(
                "/whitelabel/domains/{}/ips/{}",
                crate::progenitor_support::encode_path(&id.to_string()),
                crate::progenitor_support::encode_path(ip),
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
     * Validate a domain authentication.
     *
     * This function performs a `POST` to the `/whitelabel/domains/{id}/validate` endpoint.
     *
     * **This endpoint allows you to validate an authenticated domain. If it fails, it will return an error message describing why the domain could not be validated.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_whitelabel_domains_validate(
        &self,
        id: i64,
    ) -> ClientResult<crate::types::PostWhitelabelDomainsValidateResponse> {
        let url = self.client.url(
            &format!(
                "/whitelabel/domains/{}/validate",
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
     * List the authenticated domain associated with the given user.
     *
     * This function performs a `GET` to the `/whitelabel/domains/subuser` endpoint.
     *
     * **This endpoint allows you to retrieve all of the authenticated domains that have been assigned to a specific subuser.**
     *
     * Authenticated domains can be associated with (i.e. assigned to) subusers from a parent account. This functionality allows subusers to send mail using their parent's domain authentication. To associate a authenticated domain with a subuser, the parent account must first authenticate and validate the domain. The the parent may then associate the authenticated domain via the subuser management tools.
     *
     * **Parameters:**
     *
     * * `username: &str` -- Username for the subuser to find associated authenticated domain.
     */
    pub async fn get_whitelabel_domains_subuser(
        &self,
        username: &str,
    ) -> ClientResult<crate::types::DomainAuthentication> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !username.is_empty() {
            query_args.push(("username".to_string(), username.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/domains/subuser?{}", query_), None);
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
     * Disassociate an authenticated domain from a given user.
     *
     * This function performs a `DELETE` to the `/whitelabel/domains/subuser` endpoint.
     *
     * **This endpoint allows you to disassociate a specific authenticated domain from a subuser.**
     *
     * Authenticated domains can be associated with (i.e. assigned to) subusers from a parent account. This functionality allows subusers to send mail using their parent's domain authentication. To associate a authenticated domain with a subuser, the parent account must first authenticate and validate the domain. The the parent may then associate the authenticated domain via the subuser management tools.
     *
     * **Parameters:**
     *
     * * `username: &str` -- Username for the subuser to find associated authenticated domain.
     */
    pub async fn delete_whitelabel_domains_subuser(
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
            .url(&format!("/whitelabel/domains/subuser?{}", query_), None);
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
     * Associate a authenticated domain with a given user.
     *
     * This function performs a `POST` to the `/whitelabel/domains/{domain_id}/subuser` endpoint.
     *
     * **This endpoint allows you to associate a specific authenticated domain with a subuser.**
     *
     * Authenticated domains can be associated with (i.e. assigned to) subusers from a parent account. This functionality allows subusers to send mail using their parent's domain authentication. To associate a authenticated domain with a subuser, the parent account must first authenticate and validate the domain. The the parent may then associate the authenticated domain via the subuser management tools.
     */
    pub async fn post_whitelabel_domains_domain_subuser(
        &self,
        domain_id: i64,
        body: &crate::types::PutUserUsernameResponse,
    ) -> ClientResult<crate::types::DomainAuthentication> {
        let url = self.client.url(
            &format!(
                "/whitelabel/domains/{}/subuser",
                crate::progenitor_support::encode_path(&domain_id.to_string()),
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
}
