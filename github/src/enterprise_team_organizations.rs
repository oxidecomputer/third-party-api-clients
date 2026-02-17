use crate::Client;
use crate::ClientResult;

pub struct EnterpriseTeamOrganizations {
    pub client: Client,
}

impl EnterpriseTeamOrganizations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnterpriseTeamOrganizations { client }
    }

    /**
     * Get organization assignments.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/teams/{enterprise-team}/organizations` endpoint.
     *
     * Get all organizations assigned to an enterprise team
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-organizations#get-organization-assignments>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_assignments(
        &self,
        enterprise: &str,
        enterprise_team: &str,
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
                "/enterprises/{}/teams/{}/organizations?{}",
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
     * Get organization assignments.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/teams/{enterprise-team}/organizations` endpoint.
     *
     * As opposed to `get_assignments`, this function returns all the pages of the request at once.
     *
     * Get all organizations assigned to an enterprise team
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-organizations#get-organization-assignments>
     */
    pub async fn get_all_assignments(
        &self,
        enterprise: &str,
        enterprise_team: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/organizations",
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
     * Add organization assignments.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/teams/{enterprise-team}/organizations/add` endpoint.
     *
     * Assign an enterprise team to multiple organizations.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-organizations#add-organization-assignments>
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
        body: &crate::types::EnterpriseTeamOrganizationsBulkAddRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/organizations/add",
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
     * Remove organization assignments.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/teams/{enterprise-team}/organizations/remove` endpoint.
     *
     * Unassign an enterprise team from multiple organizations.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-organizations#remove-organization-assignments>
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
        body: &crate::types::EnterpriseTeamOrganizationsBulkAddRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/organizations/remove",
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
     * Get organization assignment.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/teams/{enterprise-team}/organizations/{org}` endpoint.
     *
     * Check if an enterprise team is assigned to an organization
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-organizations#get-organization-assignment>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_assignment(
        &self,
        enterprise: &str,
        enterprise_team: &str,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::OrganizationSimple>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/organizations/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Add an organization assignment.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/teams/{enterprise-team}/organizations/{org}` endpoint.
     *
     * Assign an enterprise team to an organization.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-organizations#add-an-organization-assignment>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn add(
        &self,
        enterprise: &str,
        enterprise_team: &str,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::OrganizationSimple>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/organizations/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Delete an organization assignment.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/teams/{enterprise-team}/organizations/{org}` endpoint.
     *
     * Unassign an enterprise team from an organization.
     *
     * FROM: <https://docs.github.com/rest/enterprise-teams/enterprise-team-organizations#delete-an-organization-assignment>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `enterprise_team: &str` -- The slug version of the enterprise team name. You can also substitute this value with the enterprise team id.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn delete(
        &self,
        enterprise: &str,
        enterprise_team: &str,
        org: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/teams/{}/organizations/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&enterprise_team.to_string()),
                crate::progenitor_support::encode_path(&org.to_string()),
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
