use crate::Client;
use crate::ClientResult;

pub struct EnterpriseTeamMemberships {
    pub client: Client,
}

impl EnterpriseTeamMemberships {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnterpriseTeamMemberships { client }
    }

    /**
     * List members in an enterprise team.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/teams/{enterprise-team}/memberships` endpoint.
     *
     * Lists all team members in an enterprise team.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-members#list-members-in-an-enterprise-team>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list(
        &self,
        enterprise: &str,
        enterprise_team: &str,
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
                "/enterprises/{}/teams/{}/memberships?{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
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
     * List members in an enterprise team.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/teams/{enterprise-team}/memberships` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists all team members in an enterprise team.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-members#list-members-in-an-enterprise-team>
     */
    pub async fn list_all(
        &self,
        enterprise: &str,
        enterprise_team: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/memberships",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
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
     * Bulk add team members.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/teams/{enterprise-team}/memberships/add` endpoint.
     *
     * Add multiple team members to an enterprise team.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-members#bulk-add-team-members>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     */
    pub async fn bulk_add(
        &self,
        enterprise: &str,
        enterprise_team: &str,
        body: &crate::types::EnterpriseTeamMembershipsBulkAddRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/memberships/add",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
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
     * Bulk remove team members.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/teams/{enterprise-team}/memberships/remove` endpoint.
     *
     * Remove multiple team members from an enterprise team.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-members#bulk-remove-team-members>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     */
    pub async fn bulk_remove(
        &self,
        enterprise: &str,
        enterprise_team: &str,
        body: &crate::types::EnterpriseTeamMembershipsBulkAddRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/memberships/remove",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
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
     * Get enterprise team membership.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/teams/{enterprise-team}/memberships/{username}` endpoint.
     *
     * Returns whether the user is a member of the enterprise team.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-members#get-enterprise-team-membership>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get(
        &self,
        enterprise: &str,
        enterprise_team: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::SimpleUser>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/memberships/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
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
     * Add team member.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/teams/{enterprise-team}/memberships/{username}` endpoint.
     *
     * Add a team member to an enterprise team.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-members#add-team-member>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn add(
        &self,
        enterprise: &str,
        enterprise_team: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::SimpleUser>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/memberships/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
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
     * Remove team membership.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/teams/{enterprise-team}/memberships/{username}` endpoint.
     *
     * Remove membership of a specific user from a particular team in an enterprise.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-members#remove-team-membership>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn remove(
        &self,
        enterprise: &str,
        enterprise_team: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/memberships/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
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
}
