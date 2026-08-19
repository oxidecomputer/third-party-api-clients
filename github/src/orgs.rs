use crate::Client;
use crate::ClientResult;

pub struct Orgs {
    pub client: Client,
}

impl Orgs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Orgs { client }
    }

    /**
     * List organizations.
     *
     * This function performs a `GET` to the `/organizations` endpoint.
     *
     * Lists all organizations, in the order that they were created on GitHub.
     *
     * **Note:** Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header) to get the URL for the next page of organizations.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organizations>
     *
     * **Parameters:**
     *
     * * `since: i64` -- An organization ID. Only return organizations with an ID greater than this ID.
     * * `per_page: i64` -- Results per page (max 100).
     */
    pub async fn list(
        &self,
        since: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/organizations?{}", query_), None);
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
     * List organizations.
     *
     * This function performs a `GET` to the `/organizations` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists all organizations, in the order that they were created on GitHub.
     *
     * **Note:** Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header) to get the URL for the next page of organizations.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organizations>
     */
    pub async fn list_all(
        &self,
        since: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/organizations?{}", query_), None);
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
     * Get an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}` endpoint.
     *
     * To see many of the organization response values, you need to be an authenticated organization owner with the `admin:org` scope. When the value of `two_factor_requirement_enabled` is `true`, the organization requires all members, billing managers, and outside collaborators to enable [two-factor authentication](https://help.github.com/articles/securing-your-account-with-two-factor-authentication-2fa/).
     *
     * GitHub Apps with the `Organization plan` permission can use this endpoint to retrieve information about an organization's GitHub plan. See "[Authenticating with GitHub Apps](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/)" for details. For an example response, see 'Response with GitHub plan information' below."
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#get-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn get(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::OrganizationFull>> {
        let url = self.client.url(
            &format!("/orgs/{}", crate::progenitor_support::encode_path(org),),
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
     * Update an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}` endpoint.
     *
     * **Parameter Deprecation Notice:** GitHub will replace and discontinue `members_allowed_repository_creation_type` in favor of more granular permissions. The new input parameters are `members_can_create_public_repositories`, `members_can_create_private_repositories` for all organizations and `members_can_create_internal_repositories` for organizations associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see the [blog post](https://developer.github.com/changes/2019-12-03-internal-visibility-changes).
     *
     * Enables an authenticated organization owner with the `admin:org` scope to update the organization's profile and member privileges.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs/#update-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn update(
        &self,
        org: &str,
        body: &crate::types::OrgsUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::OrganizationFull>> {
        let url = self.client.url(
            &format!("/orgs/{}", crate::progenitor_support::encode_path(org),),
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
     * Get the audit log for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/audit-log` endpoint.
     *
     * Gets the audit log for an organization. For more information, see "[Reviewing the audit log for your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/reviewing-the-audit-log-for-your-organization)."
     *
     * To use this endpoint, you must be an organization owner, and you must use an access token with the `admin:org` scope. GitHub Apps must have the `organization_administration` read permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#get-audit-log>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `phrase: &str` -- A search phrase. For more information, see [Searching the audit log](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/reviewing-the-audit-log-for-your-organization#searching-the-audit-log).
     * * `include: crate::types::Include` -- The event types to include:
     *  
     *  - `web` - returns web (non-Git) events
     *  - `git` - returns Git events
     *  - `all` - returns both web and Git events
     *  
     *  The default is `web`.
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header). If specified, the query only searches for events after this cursor.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header). If specified, the query only searches for events before this cursor.
     * * `order: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn get_audit_log(
        &self,
        org: &str,
        phrase: &str,
        include: crate::types::Include,
        after: &str,
        before: &str,
        order: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::AuditLogEvent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !include.to_string().is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !phrase.is_empty() {
            query_args.push(("phrase".to_string(), phrase.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/audit-log?{}",
                crate::progenitor_support::encode_path(org),
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
     * Get the audit log for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/audit-log` endpoint.
     *
     * As opposed to `get_audit_log`, this function returns all the pages of the request at once.
     *
     * Gets the audit log for an organization. For more information, see "[Reviewing the audit log for your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/reviewing-the-audit-log-for-your-organization)."
     *
     * To use this endpoint, you must be an organization owner, and you must use an access token with the `admin:org` scope. GitHub Apps must have the `organization_administration` read permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#get-audit-log>
     */
    pub async fn get_all_audit_log(
        &self,
        org: &str,
        phrase: &str,
        include: crate::types::Include,
        after: &str,
        before: &str,
        order: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::AuditLogEvent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !include.to_string().is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !phrase.is_empty() {
            query_args.push(("phrase".to_string(), phrase.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/audit-log?{}",
                crate::progenitor_support::encode_path(org),
                query_
            ),
            None,
        );
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
     * List users blocked by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/blocks` endpoint.
     *
     * List the users blocked by an organization.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-users-blocked-by-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn list_blocked_users(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks",
                crate::progenitor_support::encode_path(org),
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
     * List users blocked by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/blocks` endpoint.
     *
     * As opposed to `list_blocked_users`, this function returns all the pages of the request at once.
     *
     * List the users blocked by an organization.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-users-blocked-by-an-organization>
     */
    pub async fn list_all_blocked_users(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks",
                crate::progenitor_support::encode_path(org),
            ),
            None,
        );
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
     * Check if a user is blocked by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/blocks/{username}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#check-if-a-user-is-blocked-by-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn check_blocked_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
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
     * Block a user from an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/blocks/{username}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#block-a-user-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn block_user(&self, org: &str, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Unblock a user from an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/blocks/{username}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#unblock-a-user-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn unblock_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
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
     * List SAML SSO authorizations for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/credential-authorizations` endpoint.
     *
     * Listing and deleting credential authorizations is available to organizations with GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products).
     *
     * An authenticated organization owner with the `read:org` scope can list all credential authorizations for an organization that uses SAML single sign-on (SSO). The credentials are either personal access tokens or SSH keys that organization members have authorized for the organization. For more information, see [About authentication with SAML single sign-on](https://help.github.com/en/articles/about-authentication-with-saml-single-sign-on).
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-saml-sso-authorizations-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn list_saml_sso_authorizations(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CredentialAuthorization>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/credential-authorizations",
                crate::progenitor_support::encode_path(org),
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
     * List SAML SSO authorizations for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/credential-authorizations` endpoint.
     *
     * As opposed to `list_saml_sso_authorizations`, this function returns all the pages of the request at once.
     *
     * Listing and deleting credential authorizations is available to organizations with GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products).
     *
     * An authenticated organization owner with the `read:org` scope can list all credential authorizations for an organization that uses SAML single sign-on (SSO). The credentials are either personal access tokens or SSH keys that organization members have authorized for the organization. For more information, see [About authentication with SAML single sign-on](https://help.github.com/en/articles/about-authentication-with-saml-single-sign-on).
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-saml-sso-authorizations-for-an-organization>
     */
    pub async fn list_all_saml_sso_authorizations(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CredentialAuthorization>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/credential-authorizations",
                crate::progenitor_support::encode_path(org),
            ),
            None,
        );
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
     * Remove a SAML SSO authorization for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/credential-authorizations/{credential_id}` endpoint.
     *
     * Listing and deleting credential authorizations is available to organizations with GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products).
     *
     * An authenticated organization owner with the `admin:org` scope can remove a credential authorization for an organization that uses SAML SSO. Once you remove someone's credential authorization, they will need to create a new personal access token or SSH key and authorize it for the organization they want to access.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#remove-a-saml-sso-authorization-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `credential_id: i64`
     */
    pub async fn remove_saml_sso_authorization(
        &self,
        org: &str,
        credential_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/credential-authorizations/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&credential_id.to_string()),
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
     * List failed organization invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/failed_invitations` endpoint.
     *
     * The return hash contains `failed_at` and `failed_reason` fields which represent the time at which the invitation failed and the reason for the failure.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-failed-organization-invitations>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_failed_invitations(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/failed_invitations?{}",
                crate::progenitor_support::encode_path(org),
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
     * List failed organization invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/failed_invitations` endpoint.
     *
     * As opposed to `list_failed_invitations`, this function returns all the pages of the request at once.
     *
     * The return hash contains `failed_at` and `failed_reason` fields which represent the time at which the invitation failed and the reason for the failure.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-failed-organization-invitations>
     */
    pub async fn list_all_failed_invitations(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/failed_invitations",
                crate::progenitor_support::encode_path(org),
            ),
            None,
        );
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
     * List organization webhooks.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organization-webhooks>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_webhooks(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgHook>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks?{}",
                crate::progenitor_support::encode_path(org),
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
     * List organization webhooks.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks` endpoint.
     *
     * As opposed to `list_webhooks`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organization-webhooks>
     */
    pub async fn list_all_webhooks(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgHook>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks",
                crate::progenitor_support::encode_path(org),
            ),
            None,
        );
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
     * Create an organization webhook.
     *
     * This function performs a `POST` to the `/orgs/{org}/hooks` endpoint.
     *
     * Here's how you can create a hook that posts payloads in JSON format:
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#create-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn create_webhook(
        &self,
        org: &str,
        body: &crate::types::OrgsCreateWebhookRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgHook>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks",
                crate::progenitor_support::encode_path(org),
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
     * Get an organization webhook.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}` endpoint.
     *
     * Returns a webhook configured in an organization. To get only the webhook `config` properties, see "[Get a webhook configuration for an organization](/rest/reference/orgs#get-a-webhook-configuration-for-an-organization)."
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#get-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `hook_id: i64`
     */
    pub async fn get_webhook(
        &self,
        org: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgHook>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Delete an organization webhook.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/hooks/{hook_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#delete-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `hook_id: i64`
     */
    pub async fn delete_webhook(
        &self,
        org: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Update an organization webhook.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/hooks/{hook_id}` endpoint.
     *
     * Updates a webhook configured in an organization. When you update a webhook, the `secret` will be overwritten. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use "[Update a webhook configuration for an organization](/rest/reference/orgs#update-a-webhook-configuration-for-an-organization)."
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#update-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `hook_id: i64`
     */
    pub async fn update_webhook(
        &self,
        org: &str,
        hook_id: i64,
        body: &crate::types::OrgsUpdateWebhookRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgHook>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Get a webhook configuration for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}/config` endpoint.
     *
     * Returns the webhook configuration for an organization. To get more information about the webhook, including the `active` state and `events`, use "[Get an organization webhook ](/rest/reference/orgs#get-an-organization-webhook)."
     *
     * Access tokens must have the `admin:org_hook` scope, and GitHub Apps must have the `organization_hooks:read` permission.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#get-a-webhook-configuration-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `hook_id: i64`
     */
    pub async fn get_webhook_config_for_org(
        &self,
        org: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/config",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Update a webhook configuration for an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/hooks/{hook_id}/config` endpoint.
     *
     * Updates the webhook configuration for an organization. To update more information about the webhook, including the `active` state and `events`, use "[Update an organization webhook ](/rest/reference/orgs#update-an-organization-webhook)."
     *
     * Access tokens must have the `admin:org_hook` scope, and GitHub Apps must have the `organization_hooks:write` permission.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#update-a-webhook-configuration-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `hook_id: i64`
     */
    pub async fn update_webhook_config_for_org(
        &self,
        org: &str,
        hook_id: i64,
        body: &crate::types::AppsUpdateWebhookConfigAppRequest,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/config",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * List deliveries for an organization webhook.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}/deliveries` endpoint.
     *
     * Returns a list of webhook deliveries for a webhook configured in an organization.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-deliveries-for-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `hook_id: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `cursor: &str` -- Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors.
     */
    pub async fn list_webhook_deliveries(
        &self,
        org: &str,
        hook_id: i64,
        per_page: i64,
        cursor: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::HookDeliveryItem>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/deliveries?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * List deliveries for an organization webhook.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}/deliveries` endpoint.
     *
     * As opposed to `list_webhook_deliveries`, this function returns all the pages of the request at once.
     *
     * Returns a list of webhook deliveries for a webhook configured in an organization.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-deliveries-for-an-organization-webhook>
     */
    pub async fn list_all_webhook_deliveries(
        &self,
        org: &str,
        hook_id: i64,
        cursor: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::HookDeliveryItem>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/deliveries?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
                query_
            ),
            None,
        );
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
     * Get a webhook delivery for an organization webhook.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}` endpoint.
     *
     * Returns a delivery for a webhook configured in an organization.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#get-a-webhook-delivery-for-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `hook_id: i64`
     * * `delivery_id: i64`
     */
    pub async fn get_webhook_delivery(
        &self,
        org: &str,
        hook_id: i64,
        delivery_id: i64,
    ) -> ClientResult<crate::Response<crate::types::HookDelivery>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/deliveries/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
                crate::progenitor_support::encode_path(&delivery_id.to_string()),
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
     * Redeliver a delivery for an organization webhook.
     *
     * This function performs a `POST` to the `/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}/attempts` endpoint.
     *
     * Redeliver a delivery for a webhook configured in an organization.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#redeliver-a-delivery-for-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `hook_id: i64`
     * * `delivery_id: i64`
     */
    pub async fn redeliver_webhook_delivery(
        &self,
        org: &str,
        hook_id: i64,
        delivery_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/deliveries/{}/attempts",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
                crate::progenitor_support::encode_path(&delivery_id.to_string()),
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
     * Ping an organization webhook.
     *
     * This function performs a `POST` to the `/orgs/{org}/hooks/{hook_id}/pings` endpoint.
     *
     * This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event) to be sent to the hook.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#ping-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `hook_id: i64`
     */
    pub async fn ping_webhook(&self, org: &str, hook_id: i64) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/pings",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * List app installations for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/installations` endpoint.
     *
     * Lists all GitHub Apps in an organization. The installation count includes all GitHub Apps installed on repositories in the organization. You must be an organization owner with `admin:read` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-app-installations-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_app_installations(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::AppsListInstallationsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/installations?{}",
                crate::progenitor_support::encode_path(org),
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
     * List pending organization invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/invitations` endpoint.
     *
     * The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-pending-organization-invitations>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_pending_invitations(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations?{}",
                crate::progenitor_support::encode_path(org),
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
     * List pending organization invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/invitations` endpoint.
     *
     * As opposed to `list_pending_invitations`, this function returns all the pages of the request at once.
     *
     * The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-pending-organization-invitations>
     */
    pub async fn list_all_pending_invitations(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations",
                crate::progenitor_support::encode_path(org),
            ),
            None,
        );
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
     * Create an organization invitation.
     *
     * This function performs a `POST` to the `/orgs/{org}/invitations` endpoint.
     *
     * Invite people to an organization by using their GitHub user ID or their email address. In order to create invitations in an organization, the authenticated user must be an organization owner.
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#create-an-organization-invitation>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn create_invitation(
        &self,
        org: &str,
        body: &crate::types::OrgsCreateInvitationRequest,
    ) -> ClientResult<crate::Response<crate::types::OrganizationInvitation>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations",
                crate::progenitor_support::encode_path(org),
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
     * Cancel an organization invitation.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/invitations/{invitation_id}` endpoint.
     *
     * Cancel an organization invitation. In order to cancel an organization invitation, the authenticated user must be an organization owner.
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications).
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#cancel-an-organization-invitation>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `invitation_id: i64` -- invitation_id parameter.
     */
    pub async fn cancel_invitation(
        &self,
        org: &str,
        invitation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
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
     * List organization invitation teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/invitations/{invitation_id}/teams` endpoint.
     *
     * List all teams associated with an invitation. In order to see invitations in an organization, the authenticated user must be an organization owner.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organization-invitation-teams>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `invitation_id: i64` -- invitation_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_invitation_teams(
        &self,
        org: &str,
        invitation_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations/{}/teams?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
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
     * List organization invitation teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/invitations/{invitation_id}/teams` endpoint.
     *
     * As opposed to `list_invitation_teams`, this function returns all the pages of the request at once.
     *
     * List all teams associated with an invitation. In order to see invitations in an organization, the authenticated user must be an organization owner.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organization-invitation-teams>
     */
    pub async fn list_all_invitation_teams(
        &self,
        org: &str,
        invitation_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations/{}/teams",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
            ),
            None,
        );
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
     * List organization members.
     *
     * This function performs a `GET` to the `/orgs/{org}/members` endpoint.
     *
     * List all users who are members of an organization. If the authenticated user is also a member of this organization then both concealed and public members will be returned.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organization-members>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `filter: crate::types::OrgsListMembersFilter` -- Filter members returned in the list. Can be one of:  
     *  \\* `2fa_disabled` - Members without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled. Available for organization owners.  
     *  \\* `all` - All members the authenticated user can see.
     * * `role: crate::types::OrgsListMembersRole` -- Filter members returned by their role. Can be one of:  
     *  \\* `all` - All members of the organization, regardless of role.  
     *  \\* `admin` - Organization owners.  
     *  \\* `member` - Non-owner organization members.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_members(
        &self,
        org: &str,
        filter: crate::types::OrgsListMembersFilter,
        role: crate::types::OrgsListMembersRole,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/members?{}",
                crate::progenitor_support::encode_path(org),
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
     * List organization members.
     *
     * This function performs a `GET` to the `/orgs/{org}/members` endpoint.
     *
     * As opposed to `list_members`, this function returns all the pages of the request at once.
     *
     * List all users who are members of an organization. If the authenticated user is also a member of this organization then both concealed and public members will be returned.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organization-members>
     */
    pub async fn list_all_members(
        &self,
        org: &str,
        filter: crate::types::OrgsListMembersFilter,
        role: crate::types::OrgsListMembersRole,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/members?{}",
                crate::progenitor_support::encode_path(org),
                query_
            ),
            None,
        );
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
     * Check organization membership for a user.
     *
     * This function performs a `GET` to the `/orgs/{org}/members/{username}` endpoint.
     *
     * Check if a user is, publicly or privately, a member of the organization.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#check-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn check_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/members/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
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
     * Remove an organization member.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/members/{username}` endpoint.
     *
     * Removing a user from this list will remove them from all teams and they will no longer have any access to the organization's repositories.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#remove-an-organization-member>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn remove_member(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/members/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
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
     * Get organization membership for a user.
     *
     * This function performs a `GET` to the `/orgs/{org}/memberships/{username}` endpoint.
     *
     * In order to get a user's membership with an organization, the authenticated user must be an organization member. The `state` parameter in the response can be used to identify the user's membership status.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#get-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn get_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgMembership>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/memberships/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
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
     * Set organization membership for a user.
     *
     * This function performs a `PUT` to the `/orgs/{org}/memberships/{username}` endpoint.
     *
     * Only authenticated organization owners can add a member to the organization or update the member's role.
     *
     * *   If the authenticated user is _adding_ a member to the organization, the invited user will receive an email inviting them to the organization. The user's [membership status](https://docs.github.com/rest/reference/orgs#get-organization-membership-for-a-user) will be `pending` until they accept the invitation.
     *     
     * *   Authenticated users can _update_ a user's membership by passing the `role` parameter. If the authenticated user changes a member's role to `admin`, the affected user will receive an email notifying them that they've been made an organization owner. If the authenticated user changes an owner's role to `member`, no email will be sent.
     *
     * **Rate limits**
     *
     * To prevent abuse, the authenticated user is limited to 50 organization invitations per 24 hour period. If the organization is more than one month old or on a paid plan, the limit is 500 invitations per 24 hour period.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#set-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn set_membership_for_user(
        &self,
        org: &str,
        username: &str,
        body: &crate::types::OrgsSetMembershipUserRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgMembership>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/memberships/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Remove organization membership for a user.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/memberships/{username}` endpoint.
     *
     * In order to remove a user's membership with an organization, the authenticated user must be an organization owner.
     *
     * If the specified user is an active member of the organization, this will remove them from the organization. If the specified user has been invited to the organization, this will cancel their invitation. The specified user will receive an email notification in both cases.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#remove-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn remove_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/memberships/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
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
     * List outside collaborators for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/outside_collaborators` endpoint.
     *
     * List all users who are outside collaborators of an organization.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-outside-collaborators-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `filter: crate::types::OrgsListMembersFilter` -- Filter members returned in the list. Can be one of:  
     *  \\* `2fa_disabled` - Members without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled. Available for organization owners.  
     *  \\* `all` - All members the authenticated user can see.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_outside_collaborators(
        &self,
        org: &str,
        filter: crate::types::OrgsListMembersFilter,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/outside_collaborators?{}",
                crate::progenitor_support::encode_path(org),
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
     * List outside collaborators for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/outside_collaborators` endpoint.
     *
     * As opposed to `list_outside_collaborators`, this function returns all the pages of the request at once.
     *
     * List all users who are outside collaborators of an organization.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-outside-collaborators-for-an-organization>
     */
    pub async fn list_all_outside_collaborators(
        &self,
        org: &str,
        filter: crate::types::OrgsListMembersFilter,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/outside_collaborators?{}",
                crate::progenitor_support::encode_path(org),
                query_
            ),
            None,
        );
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
     * Convert an organization member to outside collaborator.
     *
     * This function performs a `PUT` to the `/orgs/{org}/outside_collaborators/{username}` endpoint.
     *
     * When an organization member is converted to an outside collaborator, they'll only have access to the repositories that their current team membership allows. The user will no longer be a member of the organization. For more information, see "[Converting an organization member to an outside collaborator](https://help.github.com/articles/converting-an-organization-member-to-an-outside-collaborator/)".
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#convert-an-organization-member-to-outside-collaborator>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn convert_member_to_outside_collaborator(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/outside_collaborators/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Remove outside collaborator from an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/outside_collaborators/{username}` endpoint.
     *
     * Removing a user from this list will remove them from all the organization's repositories.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#remove-outside-collaborator-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn remove_outside_collaborator(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/outside_collaborators/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
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
     * List public organization members.
     *
     * This function performs a `GET` to the `/orgs/{org}/public_members` endpoint.
     *
     * Members of an organization can choose to have their membership publicized or not.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-public-organization-members>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public_members(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members?{}",
                crate::progenitor_support::encode_path(org),
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
     * List public organization members.
     *
     * This function performs a `GET` to the `/orgs/{org}/public_members` endpoint.
     *
     * As opposed to `list_public_members`, this function returns all the pages of the request at once.
     *
     * Members of an organization can choose to have their membership publicized or not.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-public-organization-members>
     */
    pub async fn list_all_public_members(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members",
                crate::progenitor_support::encode_path(org),
            ),
            None,
        );
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
     * Check public organization membership for a user.
     *
     * This function performs a `GET` to the `/orgs/{org}/public_members/{username}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#check-public-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn check_public_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
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
     * Set public organization membership for the authenticated user.
     *
     * This function performs a `PUT` to the `/orgs/{org}/public_members/{username}` endpoint.
     *
     * The user can publicize their own membership. (A user cannot publicize the membership for another user.)
     *
     * Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#set-public-organization-membership-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn set_public_membership_for_authenticated_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Remove public organization membership for the authenticated user.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/public_members/{username}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#remove-public-organization-membership-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `username: &str`
     */
    pub async fn remove_public_membership_for_authenticated_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(username),
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
     * List organization memberships for the authenticated user.
     *
     * This function performs a `GET` to the `/user/memberships/orgs` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organization-memberships-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `state: crate::types::OrgMembershipState` -- Indicates the state of the memberships to return. Can be either `active` or `pending`. If not specified, the API returns both active and pending memberships.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_memberships_for_authenticated_user(
        &self,
        state: crate::types::OrgMembershipState,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgMembership>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/memberships/orgs?{}", query_), None);
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
     * List organization memberships for the authenticated user.
     *
     * This function performs a `GET` to the `/user/memberships/orgs` endpoint.
     *
     * As opposed to `list_memberships_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organization-memberships-for-the-authenticated-user>
     */
    pub async fn list_all_memberships_for_authenticated_user(
        &self,
        state: crate::types::OrgMembershipState,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgMembership>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/memberships/orgs?{}", query_), None);
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
     * Get an organization membership for the authenticated user.
     *
     * This function performs a `GET` to the `/user/memberships/orgs/{org}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#get-an-organization-membership-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn get_membership_for_authenticated_user(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgMembership>> {
        let url = self.client.url(
            &format!(
                "/user/memberships/orgs/{}",
                crate::progenitor_support::encode_path(org),
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
     * Update an organization membership for the authenticated user.
     *
     * This function performs a `PATCH` to the `/user/memberships/orgs/{org}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#update-an-organization-membership-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn update_membership_for_authenticated_user(
        &self,
        org: &str,
        body: &crate::types::OrgsUpdateMembershipRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgMembership>> {
        let url = self.client.url(
            &format!(
                "/user/memberships/orgs/{}",
                crate::progenitor_support::encode_path(org),
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
     * List organizations for the authenticated user.
     *
     * This function performs a `GET` to the `/user/orgs` endpoint.
     *
     * List organizations for the authenticated user.
     *
     * **OAuth scope requirements**
     *
     * This only lists organizations that your authorization allows you to operate on in some way (e.g., you can list teams with `read:org` scope, you can publicize your organization membership with `user` scope, etc.). Therefore, this API requires at least `user` or `read:org` scope. OAuth requests with insufficient scope receive a `403 Forbidden` response.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organizations-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/orgs?{}", query_), None);
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
     * List organizations for the authenticated user.
     *
     * This function performs a `GET` to the `/user/orgs` endpoint.
     *
     * As opposed to `list_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * List organizations for the authenticated user.
     *
     * **OAuth scope requirements**
     *
     * This only lists organizations that your authorization allows you to operate on in some way (e.g., you can list teams with `read:org` scope, you can publicize your organization membership with `user` scope, etc.). Therefore, this API requires at least `user` or `read:org` scope. OAuth requests with insufficient scope receive a `403 Forbidden` response.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organizations-for-the-authenticated-user>
     */
    pub async fn list_all_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let url = self.client.url("/user/orgs", None);
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
     * List organizations for a user.
     *
     * This function performs a `GET` to the `/users/{username}/orgs` endpoint.
     *
     * List [public organization memberships](https://help.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.
     *
     * This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/reference/orgs#list-organizations-for-the-authenticated-user) API instead.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organizations-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/orgs?{}",
                crate::progenitor_support::encode_path(username),
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
     * List organizations for a user.
     *
     * This function performs a `GET` to the `/users/{username}/orgs` endpoint.
     *
     * As opposed to `list_for_user`, this function returns all the pages of the request at once.
     *
     * List [public organization memberships](https://help.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.
     *
     * This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/reference/orgs#list-organizations-for-the-authenticated-user) API instead.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organizations-for-a-user>
     */
    pub async fn list_all_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/orgs",
                crate::progenitor_support::encode_path(username),
            ),
            None,
        );
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
}
