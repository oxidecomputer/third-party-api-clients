use crate::Client;
use crate::ClientResult;

pub struct Teams {
    pub client: Client,
}

impl Teams {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Teams { client }
    }

    /**
     * List IdP groups for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/team-sync/groups` endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * List IdP groups available in an organization. You can limit your page results using the `per_page` parameter. GitHub generates a url-encoded `page` token using a cursor value for where the next page begins. For more information on cursor pagination, see "[Offset and Cursor Pagination explained](https://dev.to/jackmarchant/offset-and-cursor-pagination-explained-b89)."
     *
     * The `per_page` parameter provides pagination for a list of IdP groups the authenticated user can access in an organization. For example, if the user `octocat` wants to see two groups per page in `octo-org` via cURL, it would look like this:
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-idp-groups-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: &str` -- Page token.
     */
    pub async fn list_idp_groups_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: &str,
    ) -> ClientResult<crate::Response<crate::types::GroupMapping>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/team-sync/groups?{}",
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
     * List teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams` endpoint.
     *
     * Lists all teams in an organization that are visible to the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-teams>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list(
        &self,
        org: &str,
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
                "/orgs/{}/teams?{}",
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
     * List teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists all teams in an organization that are visible to the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-teams>
     */
    pub async fn list_all(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams",
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
     * Create a team.
     *
     * This function performs a `POST` to the `/orgs/{org}/teams` endpoint.
     *
     * To create a team, the authenticated user must be a member or owner of `{org}`. By default, organization members can create teams. Organization owners can limit team creation to organization owners. For more information, see "[Setting team creation permissions](https://help.github.com/en/articles/setting-team-creation-permissions-in-your-organization)."
     *
     * When you create a new team, you automatically become a team maintainer without explicitly adding yourself to the optional array of `maintainers`. For more information, see "[About teams](https://help.github.com/en/github/setting-up-and-managing-organizations-and-teams/about-teams)".
     *
     * FROM: <https://docs.github.com/rest/reference/teams#create-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn create(
        &self,
        org: &str,
        body: &crate::types::TeamsCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::FullTeam>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams",
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
     * Get a team by name.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}` endpoint.
     *
     * Gets a team using the team's `slug`. GitHub generates the `slug` from the team `name`.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#get-a-team-by-name>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     */
    pub async fn get_by_name(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<crate::types::FullTeam>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Delete a team.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/teams/{team_slug}` endpoint.
     *
     * To delete a team, the authenticated user must be an organization owner or team maintainer.
     *
     * If you are an organization owner, deleting a parent team will delete all of its child teams as well.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#delete-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     */
    pub async fn delete_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Update a team.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/teams/{team_slug}` endpoint.
     *
     * To edit a team, the authenticated user must either be an organization owner or a team maintainer.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#update-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     */
    pub async fn update_in_org(
        &self,
        org: &str,
        team_slug: &str,
        body: &crate::types::TeamsUpdateInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::FullTeam>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List discussions.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions` endpoint.
     *
     * List all discussions on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-discussions>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     * * `pinned: &str` -- Pinned discussions only filter.
     */
    pub async fn list_discussions_in_org(
        &self,
        org: &str,
        team_slug: &str,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
        pinned: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDiscussion>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !pinned.is_empty() {
            query_args.push(("pinned".to_string(), pinned.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List discussions.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions` endpoint.
     *
     * As opposed to `list_discussions_in_org`, this function returns all the pages of the request at once.
     *
     * List all discussions on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-discussions>
     */
    pub async fn list_all_discussions_in_org(
        &self,
        org: &str,
        team_slug: &str,
        direction: crate::types::Order,
        pinned: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDiscussion>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !pinned.is_empty() {
            query_args.push(("pinned".to_string(), pinned.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Create a discussion.
     *
     * This function performs a `POST` to the `/orgs/{org}/teams/{team_slug}/discussions` endpoint.
     *
     * Creates a new discussion post on a team's page. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/{org_id}/team/{team_id}/discussions`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#create-a-discussion>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     */
    pub async fn create_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        body: &crate::types::TeamsCreateDiscussionInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussion>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Get a discussion.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}` endpoint.
     *
     * Get a specific discussion on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#get-a-discussion>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     */
    pub async fn get_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussion>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Delete a discussion.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}` endpoint.
     *
     * Delete a discussion from a team's page. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#delete-a-discussion>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     */
    pub async fn delete_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Update a discussion.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}` endpoint.
     *
     * Edits the title and body text of a discussion post. Only the parameters you provide are updated. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#update-a-discussion>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     */
    pub async fn update_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        body: &crate::types::TeamsUpdateDiscussionInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussion>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * List discussion comments.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments` endpoint.
     *
     * List all comments on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-discussion-comments>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_discussion_comments_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDiscussionComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
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
                "/orgs/{}/teams/{}/discussions/{}/comments?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * List discussion comments.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments` endpoint.
     *
     * As opposed to `list_discussion_comments_in_org`, this function returns all the pages of the request at once.
     *
     * List all comments on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-discussion-comments>
     */
    pub async fn list_all_discussion_comments_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDiscussionComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/comments?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Create a discussion comment.
     *
     * This function performs a `POST` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments` endpoint.
     *
     * Creates a new comment on a team discussion. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#create-a-discussion-comment>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     */
    pub async fn create_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussionComment>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/comments",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Get a discussion comment.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}` endpoint.
     *
     * Get a specific comment on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#get-a-discussion-comment>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     * * `comment_number: i64`
     */
    pub async fn get_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussionComment>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/comments/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * Delete a discussion comment.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}` endpoint.
     *
     * Deletes a comment on a team discussion. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#delete-a-discussion-comment>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     * * `comment_number: i64`
     */
    pub async fn delete_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/comments/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * Update a discussion comment.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}` endpoint.
     *
     * Edits the body text of a discussion comment. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#update-a-discussion-comment>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     * * `comment_number: i64`
     */
    pub async fn update_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussionComment>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/comments/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * List pending team invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/invitations` endpoint.
     *
     * The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/invitations`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-pending-team-invitations>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_pending_invitations_in_org(
        &self,
        org: &str,
        team_slug: &str,
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
                "/orgs/{}/teams/{}/invitations?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List pending team invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/invitations` endpoint.
     *
     * As opposed to `list_pending_invitations_in_org`, this function returns all the pages of the request at once.
     *
     * The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/invitations`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-pending-team-invitations>
     */
    pub async fn list_all_pending_invitations_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/invitations",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List team members.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/members` endpoint.
     *
     * Team members will include the members of child teams.
     *
     * To list members in a team, the team must be visible to the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-team-members>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `role: crate::types::TeamsListMembersInOrgRole` -- Filters members returned by their role in the team. Can be one of:  
     *  \\* `member` - normal members of the team.  
     *  \\* `maintainer` - team maintainers.  
     *  \\* `all` - all members of the team.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_members_in_org(
        &self,
        org: &str,
        team_slug: &str,
        role: crate::types::TeamsListMembersInOrgRole,
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
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/members?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List team members.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/members` endpoint.
     *
     * As opposed to `list_members_in_org`, this function returns all the pages of the request at once.
     *
     * Team members will include the members of child teams.
     *
     * To list members in a team, the team must be visible to the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-team-members>
     */
    pub async fn list_all_members_in_org(
        &self,
        org: &str,
        team_slug: &str,
        role: crate::types::TeamsListMembersInOrgRole,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/members?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Get team membership for a user.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/memberships/{username}` endpoint.
     *
     * Team members will include the members of child teams.
     *
     * To get a user's membership with a team, the team must be visible to the authenticated user.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/memberships/{username}`.
     *
     * **Note:**
     * The response contains the `state` of the membership and the member's `role`.
     *
     * The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see see [Create a team](https://docs.github.com/rest/reference/teams#create-a-team).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `username: &str`
     */
    pub async fn get_membership_for_user_in_org(
        &self,
        org: &str,
        team_slug: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::TeamMembership>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/memberships/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Add or update team membership for a user.
     *
     * This function performs a `PUT` to the `/orgs/{org}/teams/{team_slug}/memberships/{username}` endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Adds an organization member to a team. An authenticated organization owner or team maintainer can add organization members to a team.
     *
     * **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * An organization owner can add someone who is not part of the team's organization to a team. When an organization owner adds someone to a team who is not an organization member, this endpoint will send an invitation to the person via email. This newly-created membership will be in the "pending" state until the person accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team.
     *
     * If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/memberships/{username}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `username: &str`
     */
    pub async fn add_or_update_membership_for_user_in_org(
        &self,
        org: &str,
        team_slug: &str,
        username: &str,
        body: &crate::types::TeamsAddUpdateMembershipUserInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamMembership>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/memberships/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Remove team membership for a user.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/teams/{team_slug}/memberships/{username}` endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.
     *
     * **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/memberships/{username}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#remove-team-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `username: &str`
     */
    pub async fn remove_membership_for_user_in_org(
        &self,
        org: &str,
        team_slug: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/memberships/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List team projects.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/projects` endpoint.
     *
     * Lists the organization projects for a team.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-team-projects>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_projects_in_org(
        &self,
        org: &str,
        team_slug: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamProject>>> {
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
                "/orgs/{}/teams/{}/projects?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List team projects.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/projects` endpoint.
     *
     * As opposed to `list_projects_in_org`, this function returns all the pages of the request at once.
     *
     * Lists the organization projects for a team.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-team-projects>
     */
    pub async fn list_all_projects_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamProject>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/projects",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Check team permissions for a project.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/projects/{project_id}` endpoint.
     *
     * Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#check-team-permissions-for-a-project>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `project_id: i64`
     */
    pub async fn check_permissions_for_project_in_org(
        &self,
        org: &str,
        team_slug: &str,
        project_id: i64,
    ) -> ClientResult<crate::Response<crate::types::TeamProject>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/projects/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * Add or update team project permissions.
     *
     * This function performs a `PUT` to the `/orgs/{org}/teams/{team_slug}/projects/{project_id}` endpoint.
     *
     * Adds an organization project to a team. To add a project to a team or update the team's permission on a project, the authenticated user must have `admin` permissions for the project. The project and team must be part of the same organization.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#add-or-update-team-project-permissions>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `project_id: i64`
     */
    pub async fn add_or_update_project_permissions_in_org(
        &self,
        org: &str,
        team_slug: &str,
        project_id: i64,
        body: &crate::types::ProjectsAddCollaboratorRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/projects/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * Remove a project from a team.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/teams/{team_slug}/projects/{project_id}` endpoint.
     *
     * Removes an organization project from a team. An organization owner or a team maintainer can remove any project from the team. To remove a project from a team as an organization member, the authenticated user must have `read` access to both the team and project, or `admin` access to the team or project. This endpoint removes the project from the team, but does not delete the project.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#remove-a-project-from-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `project_id: i64`
     */
    pub async fn remove_project_in_org(
        &self,
        org: &str,
        team_slug: &str,
        project_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/projects/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * List team repositories.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/repos` endpoint.
     *
     * Lists a team's repositories visible to the authenticated user.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-team-repositories>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repos_in_org(
        &self,
        org: &str,
        team_slug: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
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
                "/orgs/{}/teams/{}/repos?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List team repositories.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/repos` endpoint.
     *
     * As opposed to `list_repos_in_org`, this function returns all the pages of the request at once.
     *
     * Lists a team's repositories visible to the authenticated user.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-team-repositories>
     */
    pub async fn list_all_repos_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/repos",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Check team permissions for a repository.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}` endpoint.
     *
     * Checks whether a team has `admin`, `push`, `maintain`, `triage`, or `pull` permission for a repository. Repositories inherited through a parent team will also be checked.
     *
     * You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `application/vnd.github.v3.repository+json` accept header.
     *
     * If a team doesn't have permission for the repository, you will receive a `404 Not Found` response status.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#check-team-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn check_permissions_for_repo_in_org(
        &self,
        org: &str,
        team_slug: &str,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::TeamRepository>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/repos/{}/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Add or update team repository permissions.
     *
     * This function performs a `PUT` to the `/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}` endpoint.
     *
     * To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
     *
     * For more information about the permission levels, see "[Repository permission levels for an organization](https://help.github.com/en/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)".
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#add-or-update-team-repository-permissions>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn add_or_update_repo_permissions_in_org(
        &self,
        org: &str,
        team_slug: &str,
        owner: &str,
        repo: &str,
        body: &crate::types::TeamsAddUpdateRepoPermissionsInOrgRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/repos/{}/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Remove a repository from a team.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}` endpoint.
     *
     * If the authenticated user is an organization owner or a team maintainer, they can remove any repositories from the team. To remove a repository from a team as an organization member, the authenticated user must have admin access to the repository and must be able to see the team. This does not delete the repository, it just removes it from the team.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#remove-a-repository-from-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn remove_repo_in_org(
        &self,
        org: &str,
        team_slug: &str,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/repos/{}/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List IdP groups for a team.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/team-sync/group-mappings` endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * List IdP groups connected to a team on GitHub.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/team-sync/group-mappings`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-idp-groups-for-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     */
    pub async fn list_idp_groups_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<crate::types::GroupMapping>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/team-sync/group-mappings",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Create or update IdP group connections.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/teams/{team_slug}/team-sync/group-mappings` endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Creates, updates, or removes a connection between a team and an IdP group. When adding groups to a team, you must include all new and existing groups to avoid replacing existing groups with the new ones. Specifying an empty `groups` array will remove all connections for a team.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/team-sync/group-mappings`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#create-or-update-idp-group-connections>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     */
    pub async fn create_or_update_idp_group_connections_in_org(
        &self,
        org: &str,
        team_slug: &str,
        body: &crate::types::TeamsCreateUpdateIdpGroupConnectionsInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::GroupMapping>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/team-sync/group-mappings",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List child teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/teams` endpoint.
     *
     * Lists the child teams of the team specified by `{team_slug}`.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/teams`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-child-teams>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_child_in_org(
        &self,
        org: &str,
        team_slug: &str,
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
                "/orgs/{}/teams/{}/teams?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * List child teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/teams` endpoint.
     *
     * As opposed to `list_child_in_org`, this function returns all the pages of the request at once.
     *
     * Lists the child teams of the team specified by `{team_slug}`.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/teams`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-child-teams>
     */
    pub async fn list_all_child_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/teams",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
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
     * Get a team (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the [Get a team by name](https://docs.github.com/rest/reference/teams#get-a-team-by-name) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#get-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     */
    pub async fn get_legacy(
        &self,
        team_id: i64,
    ) -> ClientResult<crate::Response<crate::types::FullTeam>> {
        let url = self.client.url(
            &format!(
                "/teams/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Delete a team (Legacy).
     *
     * This function performs a `DELETE` to the `/teams/{team_id}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Delete a team](https://docs.github.com/rest/reference/teams#delete-a-team) endpoint.
     *
     * To delete a team, the authenticated user must be an organization owner or team maintainer.
     *
     * If you are an organization owner, deleting a parent team will delete all of its child teams as well.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#delete-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     */
    pub async fn delete_legacy(&self, team_id: i64) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Update a team (Legacy).
     *
     * This function performs a `PATCH` to the `/teams/{team_id}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a team](https://docs.github.com/rest/reference/teams#update-a-team) endpoint.
     *
     * To edit a team, the authenticated user must either be an organization owner or a team maintainer.
     *
     * **Note:** With nested teams, the `privacy` for parent teams cannot be `secret`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#update-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     */
    pub async fn update_legacy(
        &self,
        team_id: i64,
        body: &crate::types::TeamsUpdateInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::FullTeam>> {
        let url = self.client.url(
            &format!(
                "/teams/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List discussions (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List discussions`](https://docs.github.com/rest/reference/teams#list-discussions) endpoint.
     *
     * List all discussions on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-discussions-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_discussions_legacy(
        &self,
        team_id: i64,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDiscussion>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
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
                "/teams/{}/discussions?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List discussions (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions` endpoint.
     *
     * As opposed to `list_discussions_legacy`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List discussions`](https://docs.github.com/rest/reference/teams#list-discussions) endpoint.
     *
     * List all discussions on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-discussions-legacy>
     */
    pub async fn list_all_discussions_legacy(
        &self,
        team_id: i64,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDiscussion>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Create a discussion (Legacy).
     *
     * This function performs a `POST` to the `/teams/{team_id}/discussions` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create a discussion`](https://docs.github.com/rest/reference/teams#create-a-discussion) endpoint.
     *
     * Creates a new discussion post on a team's page. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#create-a-discussion-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     */
    pub async fn create_discussion_legacy(
        &self,
        team_id: i64,
        body: &crate::types::TeamsCreateDiscussionInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussion>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Get a discussion (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions/{discussion_number}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion](https://docs.github.com/rest/reference/teams#get-a-discussion) endpoint.
     *
     * Get a specific discussion on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#get-a-discussion-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     */
    pub async fn get_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussion>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Delete a discussion (Legacy).
     *
     * This function performs a `DELETE` to the `/teams/{team_id}/discussions/{discussion_number}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Delete a discussion`](https://docs.github.com/rest/reference/teams#delete-a-discussion) endpoint.
     *
     * Delete a discussion from a team's page. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#delete-a-discussion-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     */
    pub async fn delete_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Update a discussion (Legacy).
     *
     * This function performs a `PATCH` to the `/teams/{team_id}/discussions/{discussion_number}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion](https://docs.github.com/rest/reference/teams#update-a-discussion) endpoint.
     *
     * Edits the title and body text of a discussion post. Only the parameters you provide are updated. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#update-a-discussion-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     */
    pub async fn update_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        body: &crate::types::TeamsUpdateDiscussionInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussion>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * List discussion comments (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions/{discussion_number}/comments` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [List discussion comments](https://docs.github.com/rest/reference/teams#list-discussion-comments) endpoint.
     *
     * List all comments on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-discussion-comments-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_discussion_comments_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDiscussionComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
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
                "/teams/{}/discussions/{}/comments?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * List discussion comments (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions/{discussion_number}/comments` endpoint.
     *
     * As opposed to `list_discussion_comments_legacy`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [List discussion comments](https://docs.github.com/rest/reference/teams#list-discussion-comments) endpoint.
     *
     * List all comments on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-discussion-comments-legacy>
     */
    pub async fn list_all_discussion_comments_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDiscussionComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/comments?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Create a discussion comment (Legacy).
     *
     * This function performs a `POST` to the `/teams/{team_id}/discussions/{discussion_number}/comments` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Create a discussion comment](https://docs.github.com/rest/reference/teams#create-a-discussion-comment) endpoint.
     *
     * Creates a new comment on a team discussion. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#create-a-discussion-comment-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     */
    pub async fn create_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussionComment>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/comments",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Get a discussion comment (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion comment](https://docs.github.com/rest/reference/teams#get-a-discussion-comment) endpoint.
     *
     * Get a specific comment on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#get-a-discussion-comment-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     * * `comment_number: i64`
     */
    pub async fn get_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussionComment>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/comments/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * Delete a discussion comment (Legacy).
     *
     * This function performs a `DELETE` to the `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Delete a discussion comment](https://docs.github.com/rest/reference/teams#delete-a-discussion-comment) endpoint.
     *
     * Deletes a comment on a team discussion. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#delete-a-discussion-comment-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     * * `comment_number: i64`
     */
    pub async fn delete_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/comments/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * Update a discussion comment (Legacy).
     *
     * This function performs a `PATCH` to the `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion comment](https://docs.github.com/rest/reference/teams#update-a-discussion-comment) endpoint.
     *
     * Edits the body text of a discussion comment. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#update-a-discussion-comment-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     * * `comment_number: i64`
     */
    pub async fn update_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamDiscussionComment>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/comments/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * List pending team invitations (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/invitations` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List pending team invitations`](https://docs.github.com/rest/reference/teams#list-pending-team-invitations) endpoint.
     *
     * The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-pending-team-invitations-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_pending_invitations_legacy(
        &self,
        team_id: i64,
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
                "/teams/{}/invitations?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List pending team invitations (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/invitations` endpoint.
     *
     * As opposed to `list_pending_invitations_legacy`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List pending team invitations`](https://docs.github.com/rest/reference/teams#list-pending-team-invitations) endpoint.
     *
     * The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-pending-team-invitations-legacy>
     */
    pub async fn list_all_pending_invitations_legacy(
        &self,
        team_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/invitations",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List team members (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/members` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team members`](https://docs.github.com/rest/reference/teams#list-team-members) endpoint.
     *
     * Team members will include the members of child teams.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-team-members-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `role: crate::types::TeamsListMembersInOrgRole` -- Filters members returned by their role in the team. Can be one of:  
     *  \\* `member` - normal members of the team.  
     *  \\* `maintainer` - team maintainers.  
     *  \\* `all` - all members of the team.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_members_legacy(
        &self,
        team_id: i64,
        role: crate::types::TeamsListMembersInOrgRole,
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
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teams/{}/members?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List team members (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/members` endpoint.
     *
     * As opposed to `list_members_legacy`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team members`](https://docs.github.com/rest/reference/teams#list-team-members) endpoint.
     *
     * Team members will include the members of child teams.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-team-members-legacy>
     */
    pub async fn list_all_members_legacy(
        &self,
        team_id: i64,
        role: crate::types::TeamsListMembersInOrgRole,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teams/{}/members?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Get team member (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/members/{username}` endpoint.
     *
     * The "Get team member" endpoint (described below) is deprecated.
     *
     * We recommend using the [Get team membership for a user](https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user) endpoint instead. It allows you to get both active and pending memberships.
     *
     * To list members in a team, the team must be visible to the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#get-team-member-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `username: &str`
     */
    pub async fn get_member_legacy(
        &self,
        team_id: i64,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/members/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Add team member (Legacy).
     *
     * This function performs a `PUT` to the `/teams/{team_id}/members/{username}` endpoint.
     *
     * The "Add team member" endpoint (described below) is deprecated.
     *
     * We recommend using the [Add or update team membership for a user](https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user) endpoint instead. It allows you to invite new organization members to your teams.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To add someone to a team, the authenticated user must be an organization owner or a team maintainer in the team they're changing. The person being added to the team must be a member of the team's organization.
     *
     * **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * FROM: <https://docs.github.com/rest/reference/teams#add-team-member-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `username: &str`
     */
    pub async fn add_member_legacy(
        &self,
        team_id: i64,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/members/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Remove team member (Legacy).
     *
     * This function performs a `DELETE` to the `/teams/{team_id}/members/{username}` endpoint.
     *
     * The "Remove team member" endpoint (described below) is deprecated.
     *
     * We recommend using the [Remove team membership for a user](https://docs.github.com/rest/reference/teams#remove-team-membership-for-a-user) endpoint instead. It allows you to remove both active and pending memberships.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To remove a team member, the authenticated user must have 'admin' permissions to the team or be an owner of the org that the team is associated with. Removing a team member does not delete the user, it just removes them from the team.
     *
     * **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * FROM: <https://docs.github.com/rest/reference/teams#remove-team-member-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `username: &str`
     */
    pub async fn remove_member_legacy(
        &self,
        team_id: i64,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/members/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Get team membership for a user (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/memberships/{username}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get team membership for a user](https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user) endpoint.
     *
     * Team members will include the members of child teams.
     *
     * To get a user's membership with a team, the team must be visible to the authenticated user.
     *
     * **Note:**
     * The response contains the `state` of the membership and the member's `role`.
     *
     * The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see [Create a team](https://docs.github.com/rest/reference/teams#create-a-team).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `username: &str`
     */
    pub async fn get_membership_for_user_legacy(
        &self,
        team_id: i64,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::TeamMembership>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/memberships/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Add or update team membership for a user (Legacy).
     *
     * This function performs a `PUT` to the `/teams/{team_id}/memberships/{username}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team membership for a user](https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user) endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * If the user is already a member of the team's organization, this endpoint will add the user to the team. To add a membership between an organization member and a team, the authenticated user must be an organization owner or a team maintainer.
     *
     * **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * If the user is unaffiliated with the team's organization, this endpoint will send an invitation to the user via email. This newly-created membership will be in the "pending" state until the user accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team. To add a membership between an unaffiliated user and a team, the authenticated user must be an organization owner.
     *
     * If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `username: &str`
     */
    pub async fn add_or_update_membership_for_user_legacy(
        &self,
        team_id: i64,
        username: &str,
        body: &crate::types::TeamsAddUpdateMembershipUserInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::TeamMembership>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/memberships/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Remove team membership for a user (Legacy).
     *
     * This function performs a `DELETE` to the `/teams/{team_id}/memberships/{username}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove team membership for a user](https://docs.github.com/rest/reference/teams#remove-team-membership-for-a-user) endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.
     *
     * **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * FROM: <https://docs.github.com/rest/reference/teams#remove-team-membership-for-a-user-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `username: &str`
     */
    pub async fn remove_membership_for_user_legacy(
        &self,
        team_id: i64,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/memberships/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List team projects (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/projects` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team projects`](https://docs.github.com/rest/reference/teams#list-team-projects) endpoint.
     *
     * Lists the organization projects for a team.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#list-team-projects-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_projects_legacy(
        &self,
        team_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamProject>>> {
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
                "/teams/{}/projects?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List team projects (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/projects` endpoint.
     *
     * As opposed to `list_projects_legacy`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team projects`](https://docs.github.com/rest/reference/teams#list-team-projects) endpoint.
     *
     * Lists the organization projects for a team.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#list-team-projects-legacy>
     */
    pub async fn list_all_projects_legacy(
        &self,
        team_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamProject>>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/projects",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Check team permissions for a project (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/projects/{project_id}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a project](https://docs.github.com/rest/reference/teams#check-team-permissions-for-a-project) endpoint.
     *
     * Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#check-team-permissions-for-a-project-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `project_id: i64`
     */
    pub async fn check_permissions_for_project_legacy(
        &self,
        team_id: i64,
        project_id: i64,
    ) -> ClientResult<crate::Response<crate::types::TeamProject>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/projects/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * Add or update team project permissions (Legacy).
     *
     * This function performs a `PUT` to the `/teams/{team_id}/projects/{project_id}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team project permissions](https://docs.github.com/rest/reference/teams#add-or-update-team-project-permissions) endpoint.
     *
     * Adds an organization project to a team. To add a project to a team or update the team's permission on a project, the authenticated user must have `admin` permissions for the project. The project and team must be part of the same organization.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#add-or-update-team-project-permissions-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `project_id: i64`
     */
    pub async fn add_or_update_project_permissions_legacy(
        &self,
        team_id: i64,
        project_id: i64,
        body: &crate::types::TeamsAddUpdateProjectPermissionsLegacyRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/projects/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * Remove a project from a team (Legacy).
     *
     * This function performs a `DELETE` to the `/teams/{team_id}/projects/{project_id}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove a project from a team](https://docs.github.com/rest/reference/teams#remove-a-project-from-a-team) endpoint.
     *
     * Removes an organization project from a team. An organization owner or a team maintainer can remove any project from the team. To remove a project from a team as an organization member, the authenticated user must have `read` access to both the team and project, or `admin` access to the team or project. **Note:** This endpoint removes the project from the team, but does not delete it.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#remove-a-project-from-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `project_id: i64`
     */
    pub async fn remove_project_legacy(
        &self,
        team_id: i64,
        project_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/projects/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * List team repositories (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/repos` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [List team repositories](https://docs.github.com/rest/reference/teams#list-team-repositories) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#list-team-repositories-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repos_legacy(
        &self,
        team_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
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
                "/teams/{}/repos?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List team repositories (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/repos` endpoint.
     *
     * As opposed to `list_repos_legacy`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [List team repositories](https://docs.github.com/rest/reference/teams#list-team-repositories) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#list-team-repositories-legacy>
     */
    pub async fn list_all_repos_legacy(
        &self,
        team_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/repos",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Check team permissions for a repository (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/repos/{owner}/{repo}` endpoint.
     *
     * **Note**: Repositories inherited through a parent team will also be checked.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a repository](https://docs.github.com/rest/reference/teams#check-team-permissions-for-a-repository) endpoint.
     *
     * You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#check-team-permissions-for-a-repository-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn check_permissions_for_repo_legacy(
        &self,
        team_id: i64,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::TeamRepository>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/repos/{}/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Add or update team repository permissions (Legacy).
     *
     * This function performs a `PUT` to the `/teams/{team_id}/repos/{owner}/{repo}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new "[Add or update team repository permissions](https://docs.github.com/rest/reference/teams#add-or-update-team-repository-permissions)" endpoint.
     *
     * To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization.
     *
     * Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#add-or-update-team-repository-permissions-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn add_or_update_repo_permissions_legacy(
        &self,
        team_id: i64,
        owner: &str,
        repo: &str,
        body: &crate::types::TeamsAddUpdateRepoPermissionsLegacyRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/repos/{}/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Remove a repository from a team (Legacy).
     *
     * This function performs a `DELETE` to the `/teams/{team_id}/repos/{owner}/{repo}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove a repository from a team](https://docs.github.com/rest/reference/teams#remove-a-repository-from-a-team) endpoint.
     *
     * If the authenticated user is an organization owner or a team maintainer, they can remove any repositories from the team. To remove a repository from a team as an organization member, the authenticated user must have admin access to the repository and must be able to see the team. NOTE: This does not delete the repository, it just removes it from the team.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#remove-a-repository-from-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn remove_repo_legacy(
        &self,
        team_id: i64,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/repos/{}/{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List IdP groups for a team (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/team-sync/group-mappings` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List IdP groups for a team`](https://docs.github.com/rest/reference/teams#list-idp-groups-for-a-team) endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * List IdP groups connected to a team on GitHub.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-idp-groups-for-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     */
    pub async fn list_idp_groups_for_legacy(
        &self,
        team_id: i64,
    ) -> ClientResult<crate::Response<crate::types::GroupMapping>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/team-sync/group-mappings",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * Create or update IdP group connections (Legacy).
     *
     * This function performs a `PATCH` to the `/teams/{team_id}/team-sync/group-mappings` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create or update IdP group connections`](https://docs.github.com/rest/reference/teams#create-or-update-idp-group-connections) endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Creates, updates, or removes a connection between a team and an IdP group. When adding groups to a team, you must include all new and existing groups to avoid replacing existing groups with the new ones. Specifying an empty `groups` array will remove all connections for a team.
     *
     * FROM: <https://docs.github.com/rest/reference/teams#create-or-update-idp-group-connections-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     */
    pub async fn create_or_update_idp_group_connections_legacy(
        &self,
        team_id: i64,
        body: &crate::types::TeamsCreateUpdateIdpGroupConnectionsLegacyRequest,
    ) -> ClientResult<crate::Response<crate::types::GroupMapping>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/team-sync/group-mappings",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List child teams (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/teams` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List child teams`](https://docs.github.com/rest/reference/teams#list-child-teams) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#list-child-teams-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_child_legacy(
        &self,
        team_id: i64,
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
                "/teams/{}/teams?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List child teams (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/teams` endpoint.
     *
     * As opposed to `list_child_legacy`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List child teams`](https://docs.github.com/rest/reference/teams#list-child-teams) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/teams/#list-child-teams-legacy>
     */
    pub async fn list_all_child_legacy(
        &self,
        team_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/teams",
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
     * List teams for the authenticated user.
     *
     * This function performs a `GET` to the `/user/teams` endpoint.
     *
     * List all of the teams across all of the organizations to which the authenticated user belongs. This method requires `user`, `repo`, or `read:org` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/) when authenticating via [OAuth](https://docs.github.com/apps/building-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-teams-for-the-authenticated-user>
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
    ) -> ClientResult<crate::Response<Vec<crate::types::FullTeam>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/teams?{}", query_), None);
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
     * List teams for the authenticated user.
     *
     * This function performs a `GET` to the `/user/teams` endpoint.
     *
     * As opposed to `list_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * List all of the teams across all of the organizations to which the authenticated user belongs. This method requires `user`, `repo`, or `read:org` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/) when authenticating via [OAuth](https://docs.github.com/apps/building-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-teams-for-the-authenticated-user>
     */
    pub async fn list_all_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::FullTeam>>> {
        let url = self.client.url("/user/teams", None);
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
