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
     * List teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams` endpoint.
     *
     * Lists all teams in an organization that are visible to the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-teams>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `team_type: crate::types::TeamType` -- Filter team results by their type. For more information, see "[What kind of team should I use?](https://docs.github.com/enterprise-cloud@latest/admin/concepts/enterprise-fundamentals/teams-in-an-enterprise#what-kind-of-team-should-i-use)".
     */
    pub async fn list(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        team_type: crate::types::TeamType,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !team_type.to_string().is_empty() {
            query_args.push(("team_type".to_string(), team_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * FROM: <https://docs.github.com/rest/teams/teams#list-teams>
     */
    pub async fn list_all(
        &self,
        org: &str,
        team_type: crate::types::TeamType,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !team_type.to_string().is_empty() {
            query_args.push(("team_type".to_string(), team_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Create a team.
     *
     * This function performs a `POST` to the `/orgs/{org}/teams` endpoint.
     *
     * To create a team, the authenticated user must be a member or owner of `{org}`. By default, organization members can create teams. Organization owners can limit team creation to organization owners. For more information, see "[Setting team creation permissions](https://docs.github.com/articles/setting-team-creation-permissions-in-your-organization)."
     *
     * When you create a new team, you automatically become a team maintainer without explicitly adding yourself to the optional array of `maintainers`. For more information, see "[About teams](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/about-teams)".
     *
     * FROM: <https://docs.github.com/rest/teams/teams#create-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create(
        &self,
        org: &str,
        body: &crate::types::TeamsCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::FullTeam>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Gets a team using the team's `slug`. To create the `slug`, GitHub replaces special characters in the `name` string, changes all words to lowercase, and replaces spaces with a `-` separator. For example, `"My TEam Näme"` would become `my-team-name`.
     *
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#get-a-team-by-name>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     */
    pub async fn get_by_name(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<crate::types::FullTeam>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#delete-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     */
    pub async fn delete_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#update-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/invitations`.
     *
     * FROM: <https://docs.github.com/rest/teams/members#list-pending-team-invitations>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/invitations`.
     *
     * FROM: <https://docs.github.com/rest/teams/members#list-pending-team-invitations>
     */
    pub async fn list_all_pending_invitations_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/invitations",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * FROM: <https://docs.github.com/rest/teams/members#list-team-members>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `role: crate::types::TeamsListMembersInOrgRole` -- Filters members returned by their role in the team.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * FROM: <https://docs.github.com/rest/teams/members#list-team-members>
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/memberships/{username}`.
     *
     * > [!NOTE]
     * > The response contains the `state` of the membership and the member's `role`.
     *
     * The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see [Create a team](https://docs.github.com/rest/teams/teams#create-a-team).
     *
     * FROM: <https://docs.github.com/rest/teams/members#get-team-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Adds an organization member to a team. An authenticated organization owner or team maintainer can add organization members to a team.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * > [!NOTE]
     * > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * An organization owner can add someone who is not part of the team's organization to a team. When an organization owner adds someone to a team who is not an organization member, this endpoint will send an invitation to the person via email. This newly-created membership will be in the "pending" state until the person accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team.
     *
     * If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
     *
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/memberships/{username}`.
     *
     * FROM: <https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * > [!NOTE]
     * > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/memberships/{username}`.
     *
     * FROM: <https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-team-repositories>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-team-repositories>
     */
    pub async fn list_all_repos_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/repos",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types/) via the `application/vnd.github.v3.repository+json` accept header.
     *
     * If a team doesn't have permission for the repository, you will receive a `404 Not Found` response status.
     *
     * If the repository is private, you must have at least `read` permission for that repository, and your token must have the `repo` or `admin:org` scope. Otherwise, you will receive a `404 Not Found` response status.
     *
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
     *
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
     *
     * For more information about the permission levels, see "[Repository permission levels for an organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)".
     *
     * FROM: <https://docs.github.com/rest/teams/teams#add-or-update-team-repository-permissions>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn add_or_update_repo_permissions_in_org(
        &self,
        org: &str,
        team_slug: &str,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposAddCollaboratorRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/repos/{}/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#remove-a-repository-from-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List child teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/teams` endpoint.
     *
     * Lists the child teams of the team specified by `{team_slug}`.
     *
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/teams`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-child-teams>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * > [!NOTE]
     * > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/teams`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-child-teams>
     */
    pub async fn list_all_child_in_org(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/teams",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the [Get a team by name](https://docs.github.com/rest/teams/teams#get-a-team-by-name) endpoint.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#get-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Delete a team](https://docs.github.com/rest/teams/teams#delete-a-team) endpoint.
     *
     * To delete a team, the authenticated user must be an organization owner or team maintainer.
     *
     * If you are an organization owner, deleting a parent team will delete all of its child teams as well.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#delete-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a team](https://docs.github.com/rest/teams/teams#update-a-team) endpoint.
     *
     * To edit a team, the authenticated user must either be an organization owner or a team maintainer.
     *
     * > [!NOTE]
     * > With nested teams, the `privacy` for parent teams cannot be `secret`.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#update-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     */
    pub async fn update_legacy(
        &self,
        team_id: i64,
        body: &crate::types::TeamsUpdateLegacyRequest,
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
     * List pending team invitations (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/invitations` endpoint.
     *
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List pending team invitations`](https://docs.github.com/rest/teams/members#list-pending-team-invitations) endpoint.
     *
     * The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
     *
     * FROM: <https://docs.github.com/rest/teams/members#list-pending-team-invitations-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List pending team invitations`](https://docs.github.com/rest/teams/members#list-pending-team-invitations) endpoint.
     *
     * The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
     *
     * FROM: <https://docs.github.com/rest/teams/members#list-pending-team-invitations-legacy>
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team members`](https://docs.github.com/rest/teams/members#list-team-members) endpoint.
     *
     * Team members will include the members of child teams.
     *
     * FROM: <https://docs.github.com/rest/teams/members#list-team-members-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `role: crate::types::TeamsListMembersInOrgRole` -- Filters members returned by their role in the team.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team members`](https://docs.github.com/rest/teams/members#list-team-members) endpoint.
     *
     * Team members will include the members of child teams.
     *
     * FROM: <https://docs.github.com/rest/teams/members#list-team-members-legacy>
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
     * The "Get team member" endpoint (described below) is closing down.
     *
     * We recommend using the [Get team membership for a user](https://docs.github.com/rest/teams/members#get-team-membership-for-a-user) endpoint instead. It allows you to get both active and pending memberships.
     *
     * To list members in a team, the team must be visible to the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/teams/members#get-team-member-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * The "Add team member" endpoint (described below) is closing down.
     *
     * We recommend using the [Add or update team membership for a user](https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user) endpoint instead. It allows you to invite new organization members to your teams.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To add someone to a team, the authenticated user must be an organization owner or a team maintainer in the team they're changing. The person being added to the team must be a member of the team's organization.
     *
     * > [!NOTE]
     * > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
     *
     * FROM: <https://docs.github.com/rest/teams/members#add-team-member-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * The "Remove team member" endpoint (described below) is closing down.
     *
     * We recommend using the [Remove team membership for a user](https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user) endpoint instead. It allows you to remove both active and pending memberships.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To remove a team member, the authenticated user must have 'admin' permissions to the team or be an owner of the org that the team is associated with. Removing a team member does not delete the user, it just removes them from the team.
     *
     * > [!NOTE]
     * > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * FROM: <https://docs.github.com/rest/teams/members#remove-team-member-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get team membership for a user](https://docs.github.com/rest/teams/members#get-team-membership-for-a-user) endpoint.
     *
     * Team members will include the members of child teams.
     *
     * To get a user's membership with a team, the team must be visible to the authenticated user.
     *
     * **Note:**
     * The response contains the `state` of the membership and the member's `role`.
     *
     * The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see [Create a team](https://docs.github.com/rest/teams/teams#create-a-team).
     *
     * FROM: <https://docs.github.com/rest/teams/members#get-team-membership-for-a-user-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team membership for a user](https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user) endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * If the user is already a member of the team's organization, this endpoint will add the user to the team. To add a membership between an organization member and a team, the authenticated user must be an organization owner or a team maintainer.
     *
     * > [!NOTE]
     * > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * If the user is unaffiliated with the team's organization, this endpoint will send an invitation to the user via email. This newly-created membership will be in the "pending" state until the user accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team. To add a membership between an unaffiliated user and a team, the authenticated user must be an organization owner.
     *
     * If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
     *
     * FROM: <https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove team membership for a user](https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user) endpoint.
     *
     * Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.
     *
     * > [!NOTE]
     * > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
     *
     * FROM: <https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [List team repositories](https://docs.github.com/rest/teams/teams#list-team-repositories) endpoint.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-team-repositories-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [List team repositories](https://docs.github.com/rest/teams/teams#list-team-repositories) endpoint.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-team-repositories-legacy>
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a repository](https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-repository) endpoint.
     *
     * > [!NOTE]
     * > Repositories inherited through a parent team will also be checked.
     *
     * You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types/) via the `Accept` header:
     *
     * FROM: <https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-repository-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new "[Add or update team repository permissions](https://docs.github.com/rest/teams/teams#add-or-update-team-repository-permissions)" endpoint.
     *
     * To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization.
     *
     * Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
     *
     * FROM: <https://docs.github.com/rest/teams/teams#add-or-update-team-repository-permissions-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove a repository from a team](https://docs.github.com/rest/teams/teams#remove-a-repository-from-a-team) endpoint.
     *
     * If the authenticated user is an organization owner or a team maintainer, they can remove any repositories from the team. To remove a repository from a team as an organization member, the authenticated user must have admin access to the repository and must be able to see the team. NOTE: This does not delete the repository, it just removes it from the team.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#remove-a-repository-from-a-team-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List child teams (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/teams` endpoint.
     *
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List child teams`](https://docs.github.com/rest/teams/teams#list-child-teams) endpoint.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-child-teams-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64` -- The unique identifier of the team.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * > [!WARNING]
     * > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List child teams`](https://docs.github.com/rest/teams/teams#list-child-teams) endpoint.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-child-teams-legacy>
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
     * List all of the teams across all of the organizations to which the authenticated
     * user belongs.
     *
     * OAuth app tokens and personal access tokens (classic) need the `user`, `repo`, or `read:org` scope to use this endpoint.
     *
     * When using a fine-grained personal access token, the resource owner of the token must be a single organization, and the response will only include the teams from that organization.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-teams-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * List all of the teams across all of the organizations to which the authenticated
     * user belongs.
     *
     * OAuth app tokens and personal access tokens (classic) need the `user`, `repo`, or `read:org` scope to use this endpoint.
     *
     * When using a fine-grained personal access token, the resource owner of the token must be a single organization, and the response will only include the teams from that organization.
     *
     * FROM: <https://docs.github.com/rest/teams/teams#list-teams-for-the-authenticated-user>
     */
    pub async fn list_all_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::FullTeam>>> {
        let url = self.client.url(&"/user/teams".to_string(), None);
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
