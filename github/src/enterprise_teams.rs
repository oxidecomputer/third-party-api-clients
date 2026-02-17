use crate::Client;
use crate::ClientResult;

pub struct EnterpriseTeams {
    pub client: Client,
}

impl EnterpriseTeams {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnterpriseTeams { client }
    }

    /**
     * List enterprise teams.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/teams` endpoint.
     *
     * List all teams in the enterprise for the authenticated user
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-teams#list-enterprise-teams>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list(
        &self,
        enterprise: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::EnterpriseTeam>>> {
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
                "/enterprises/{}/teams?{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * List enterprise teams.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/teams` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * List all teams in the enterprise for the authenticated user
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-teams#list-enterprise-teams>
     */
    pub async fn list_all(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::EnterpriseTeam>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * Create an enterprise team.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/teams` endpoint.
     *
     * To create an enterprise team, the authenticated user must be an owner of the enterprise.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-teams#create-an-enterprise-team>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     */
    pub async fn create(
        &self,
        enterprise: &str,
        body: &crate::types::EnterpriseTeamsCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::EnterpriseTeam>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * Get an enterprise team.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/teams/{team_slug}` endpoint.
     *
     * Gets a team using the team's slug. To create the slug, GitHub replaces special characters in the name string, changes all words to lowercase, and replaces spaces with a `-` separator and adds the "ent:" prefix. For example, "My TEam Näme" would become `ent:my-team-name`.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-teams#get-an-enterprise-team>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `team_slug: &str` -- The slug of the team name.
     */
    pub async fn get(
        &self,
        enterprise: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<crate::types::EnterpriseTeam>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * Delete an enterprise team.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/teams/{team_slug}` endpoint.
     *
     * To delete an enterprise team, the authenticated user must be an enterprise owner.
     *
     * If you are an enterprise owner, deleting an enterprise team will delete all of its IdP mappings as well.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-teams#delete-an-enterprise-team>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `team_slug: &str` -- The slug of the team name.
     */
    pub async fn delete(
        &self,
        enterprise: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * Update an enterprise team.
     *
     * This function performs a `PATCH` to the `/enterprises/{enterprise}/teams/{team_slug}` endpoint.
     *
     * To edit a team, the authenticated user must be an enterprise owner.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-teams#update-an-enterprise-team>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `team_slug: &str` -- The slug of the team name.
     */
    pub async fn update(
        &self,
        enterprise: &str,
        team_slug: &str,
        body: &crate::types::EnterpriseTeamsUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::EnterpriseTeam>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
}
