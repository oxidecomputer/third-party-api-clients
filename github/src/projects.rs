use crate::Client;
use crate::ClientResult;

pub struct Projects {
    pub client: Client,
}

impl Projects {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Projects { client }
    }

    /**
     * List organization projects.
     *
     * This function performs a `GET` to the `/orgs/{org}/projects` endpoint.
     *
     * Lists the projects in an organization. Returns a `404 Not Found` status if projects are disabled in the organization. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-organization-projects>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_org(
        &self,
        org: &str,
        state: crate::types::IssuesListState,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Project>>> {
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
        let url = self.client.url(
            &format!(
                "/orgs/{}/projects?{}",
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
     * List organization projects.
     *
     * This function performs a `GET` to the `/orgs/{org}/projects` endpoint.
     *
     * As opposed to `list_for_org`, this function returns all the pages of the request at once.
     *
     * Lists the projects in an organization. Returns a `404 Not Found` status if projects are disabled in the organization. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-organization-projects>
     */
    pub async fn list_all_for_org(
        &self,
        org: &str,
        state: crate::types::IssuesListState,
    ) -> ClientResult<crate::Response<Vec<crate::types::Project>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projects?{}",
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
     * Create an organization project.
     *
     * This function performs a `POST` to the `/orgs/{org}/projects` endpoint.
     *
     * Creates an organization project board. Returns a `404 Not Found` status if projects are disabled in the organization. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#create-an-organization-project>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn create_for_org(
        &self,
        org: &str,
        body: &crate::types::ProjectsCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::Project>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/projects",
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
     * Get a project card.
     *
     * This function performs a `GET` to the `/projects/columns/cards/{card_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#get-a-project-card>
     *
     * **Parameters:**
     *
     * * `card_id: i64` -- card_id parameter.
     */
    pub async fn get_card(
        &self,
        card_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ProjectCard>> {
        let url = self.client.url(
            &format!(
                "/projects/columns/cards/{}",
                crate::progenitor_support::encode_path(&card_id.to_string()),
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
     * Delete a project card.
     *
     * This function performs a `DELETE` to the `/projects/columns/cards/{card_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#delete-a-project-card>
     *
     * **Parameters:**
     *
     * * `card_id: i64` -- card_id parameter.
     */
    pub async fn delete_card(&self, card_id: i64) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/projects/columns/cards/{}",
                crate::progenitor_support::encode_path(&card_id.to_string()),
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
     * Update an existing project card.
     *
     * This function performs a `PATCH` to the `/projects/columns/cards/{card_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#update-a-project-card>
     *
     * **Parameters:**
     *
     * * `card_id: i64` -- card_id parameter.
     */
    pub async fn update_card(
        &self,
        card_id: i64,
        body: &crate::types::ProjectsUpdateCardRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectCard>> {
        let url = self.client.url(
            &format!(
                "/projects/columns/cards/{}",
                crate::progenitor_support::encode_path(&card_id.to_string()),
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
     * Move a project card.
     *
     * This function performs a `POST` to the `/projects/columns/cards/{card_id}/moves` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#move-a-project-card>
     *
     * **Parameters:**
     *
     * * `card_id: i64` -- card_id parameter.
     */
    pub async fn move_card(
        &self,
        card_id: i64,
        body: &crate::types::ProjectsMoveCardRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/projects/columns/cards/{}/moves",
                crate::progenitor_support::encode_path(&card_id.to_string()),
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
     * Get a project column.
     *
     * This function performs a `GET` to the `/projects/columns/{column_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#get-a-project-column>
     *
     * **Parameters:**
     *
     * * `column_id: i64` -- column_id parameter.
     */
    pub async fn get_column(
        &self,
        column_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ProjectColumn>> {
        let url = self.client.url(
            &format!(
                "/projects/columns/{}",
                crate::progenitor_support::encode_path(&column_id.to_string()),
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
     * Delete a project column.
     *
     * This function performs a `DELETE` to the `/projects/columns/{column_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#delete-a-project-column>
     *
     * **Parameters:**
     *
     * * `column_id: i64` -- column_id parameter.
     */
    pub async fn delete_column(&self, column_id: i64) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/projects/columns/{}",
                crate::progenitor_support::encode_path(&column_id.to_string()),
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
     * Update an existing project column.
     *
     * This function performs a `PATCH` to the `/projects/columns/{column_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#update-a-project-column>
     *
     * **Parameters:**
     *
     * * `column_id: i64` -- column_id parameter.
     */
    pub async fn update_column(
        &self,
        column_id: i64,
        body: &crate::types::ProjectsUpdateColumnRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectColumn>> {
        let url = self.client.url(
            &format!(
                "/projects/columns/{}",
                crate::progenitor_support::encode_path(&column_id.to_string()),
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
     * List project cards.
     *
     * This function performs a `GET` to the `/projects/columns/{column_id}/cards` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-project-cards>
     *
     * **Parameters:**
     *
     * * `column_id: i64` -- column_id parameter.
     * * `archived_state: crate::types::ArchivedState` -- Filters the project cards that are returned by the card's state. Can be one of `all`,`archived`, or `not_archived`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_cards(
        &self,
        column_id: i64,
        archived_state: crate::types::ArchivedState,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectCard>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !archived_state.to_string().is_empty() {
            query_args.push(("archived_state".to_string(), archived_state.to_string()));
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
                "/projects/columns/{}/cards?{}",
                crate::progenitor_support::encode_path(&column_id.to_string()),
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
     * List project cards.
     *
     * This function performs a `GET` to the `/projects/columns/{column_id}/cards` endpoint.
     *
     * As opposed to `list_cards`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-project-cards>
     */
    pub async fn list_all_cards(
        &self,
        column_id: i64,
        archived_state: crate::types::ArchivedState,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectCard>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !archived_state.to_string().is_empty() {
            query_args.push(("archived_state".to_string(), archived_state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/projects/columns/{}/cards?{}",
                crate::progenitor_support::encode_path(&column_id.to_string()),
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
     * Create a project card.
     *
     * This function performs a `POST` to the `/projects/columns/{column_id}/cards` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#create-a-project-card>
     *
     * **Parameters:**
     *
     * * `column_id: i64` -- column_id parameter.
     */
    pub async fn create_card(
        &self,
        column_id: i64,
        body: &crate::types::ProjectsCreateCardRequestOneOf,
    ) -> ClientResult<crate::Response<crate::types::ProjectCard>> {
        let url = self.client.url(
            &format!(
                "/projects/columns/{}/cards",
                crate::progenitor_support::encode_path(&column_id.to_string()),
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
     * Move a project column.
     *
     * This function performs a `POST` to the `/projects/columns/{column_id}/moves` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#move-a-project-column>
     *
     * **Parameters:**
     *
     * * `column_id: i64` -- column_id parameter.
     */
    pub async fn move_column(
        &self,
        column_id: i64,
        body: &crate::types::ProjectsMoveColumnRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/projects/columns/{}/moves",
                crate::progenitor_support::encode_path(&column_id.to_string()),
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
     * Get a project.
     *
     * This function performs a `GET` to the `/projects/{project_id}` endpoint.
     *
     * Gets a project by its `id`. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#get-a-project>
     *
     * **Parameters:**
     *
     * * `project_id: i64`
     */
    pub async fn get(
        &self,
        project_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Project>> {
        let url = self.client.url(
            &format!(
                "/projects/{}",
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
     * Delete a project.
     *
     * This function performs a `DELETE` to the `/projects/{project_id}` endpoint.
     *
     * Deletes a project board. Returns a `404 Not Found` status if projects are disabled.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#delete-a-project>
     *
     * **Parameters:**
     *
     * * `project_id: i64`
     */
    pub async fn delete(&self, project_id: i64) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/projects/{}",
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
     * Update a project.
     *
     * This function performs a `PATCH` to the `/projects/{project_id}` endpoint.
     *
     * Updates a project board's information. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#update-a-project>
     *
     * **Parameters:**
     *
     * * `project_id: i64`
     */
    pub async fn update(
        &self,
        project_id: i64,
        body: &crate::types::ProjectsUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::Project>> {
        let url = self.client.url(
            &format!(
                "/projects/{}",
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * List project collaborators.
     *
     * This function performs a `GET` to the `/projects/{project_id}/collaborators` endpoint.
     *
     * Lists the collaborators for an organization project. For a project, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners. You must be an organization owner or a project `admin` to list collaborators.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-project-collaborators>
     *
     * **Parameters:**
     *
     * * `project_id: i64`
     * * `affiliation: crate::types::Affiliation` -- Filters the collaborators by their affiliation. Can be one of:  
     *  \\* `outside`: Outside collaborators of a project that are not a member of the project's organization.  
     *  \\* `direct`: Collaborators with permissions to a project, regardless of organization membership status.  
     *  \\* `all`: All collaborators the authenticated user can see.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_collaborators(
        &self,
        project_id: i64,
        affiliation: crate::types::Affiliation,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !affiliation.to_string().is_empty() {
            query_args.push(("affiliation".to_string(), affiliation.to_string()));
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
                "/projects/{}/collaborators?{}",
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * List project collaborators.
     *
     * This function performs a `GET` to the `/projects/{project_id}/collaborators` endpoint.
     *
     * As opposed to `list_collaborators`, this function returns all the pages of the request at once.
     *
     * Lists the collaborators for an organization project. For a project, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners. You must be an organization owner or a project `admin` to list collaborators.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-project-collaborators>
     */
    pub async fn list_all_collaborators(
        &self,
        project_id: i64,
        affiliation: crate::types::Affiliation,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !affiliation.to_string().is_empty() {
            query_args.push(("affiliation".to_string(), affiliation.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/projects/{}/collaborators?{}",
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * Add project collaborator.
     *
     * This function performs a `PUT` to the `/projects/{project_id}/collaborators/{username}` endpoint.
     *
     * Adds a collaborator to an organization project and sets their permission level. You must be an organization owner or a project `admin` to add a collaborator.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#add-project-collaborator>
     *
     * **Parameters:**
     *
     * * `project_id: i64`
     * * `username: &str`
     */
    pub async fn add_collaborator(
        &self,
        project_id: i64,
        username: &str,
        body: &crate::types::ProjectsAddCollaboratorRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/projects/{}/collaborators/{}",
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * Remove user as a collaborator.
     *
     * This function performs a `DELETE` to the `/projects/{project_id}/collaborators/{username}` endpoint.
     *
     * Removes a collaborator from an organization project. You must be an organization owner or a project `admin` to remove a collaborator.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#remove-project-collaborator>
     *
     * **Parameters:**
     *
     * * `project_id: i64`
     * * `username: &str`
     */
    pub async fn remove_collaborator(
        &self,
        project_id: i64,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/projects/{}/collaborators/{}",
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * Get project permission for a user.
     *
     * This function performs a `GET` to the `/projects/{project_id}/collaborators/{username}/permission` endpoint.
     *
     * Returns the collaborator's permission level for an organization project. Possible values for the `permission` key: `admin`, `write`, `read`, `none`. You must be an organization owner or a project `admin` to review a user's permission level.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#get-project-permission-for-a-user>
     *
     * **Parameters:**
     *
     * * `project_id: i64`
     * * `username: &str`
     */
    pub async fn get_permission_for_user(
        &self,
        project_id: i64,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::RepositoryCollaboratorPermission>> {
        let url = self.client.url(
            &format!(
                "/projects/{}/collaborators/{}/permission",
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * List project columns.
     *
     * This function performs a `GET` to the `/projects/{project_id}/columns` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-project-columns>
     *
     * **Parameters:**
     *
     * * `project_id: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_columns(
        &self,
        project_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectColumn>>> {
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
                "/projects/{}/columns?{}",
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * List project columns.
     *
     * This function performs a `GET` to the `/projects/{project_id}/columns` endpoint.
     *
     * As opposed to `list_columns`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-project-columns>
     */
    pub async fn list_all_columns(
        &self,
        project_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectColumn>>> {
        let url = self.client.url(
            &format!(
                "/projects/{}/columns",
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * Create a project column.
     *
     * This function performs a `POST` to the `/projects/{project_id}/columns` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#create-a-project-column>
     *
     * **Parameters:**
     *
     * * `project_id: i64`
     */
    pub async fn create_column(
        &self,
        project_id: i64,
        body: &crate::types::ProjectsUpdateColumnRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectColumn>> {
        let url = self.client.url(
            &format!(
                "/projects/{}/columns",
                crate::progenitor_support::encode_path(&project_id.to_string()),
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
     * List repository projects.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/projects` endpoint.
     *
     * Lists the projects in a repository. Returns a `404 Not Found` status if projects are disabled in the repository. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-repository-projects>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::IssuesListState,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Project>>> {
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
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/projects?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List repository projects.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/projects` endpoint.
     *
     * As opposed to `list_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists the projects in a repository. Returns a `404 Not Found` status if projects are disabled in the repository. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-repository-projects>
     */
    pub async fn list_all_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::IssuesListState,
    ) -> ClientResult<crate::Response<Vec<crate::types::Project>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/projects?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Create a repository project.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/projects` endpoint.
     *
     * Creates a repository project board. Returns a `404 Not Found` status if projects are disabled in the repository. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
     *
     * FROM: <https://docs.github.com/rest/reference/projects#create-a-repository-project>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_for_repo(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ProjectsCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::Project>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/projects",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Create a user project.
     *
     * This function performs a `POST` to the `/user/projects` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#create-a-user-project>
     */
    pub async fn create_for_authenticated_user(
        &self,
        body: &crate::types::ProjectsCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::Project>> {
        let url = self.client.url("/user/projects", None);
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
     * List user projects.
     *
     * This function performs a `GET` to the `/users/{username}/projects` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-user-projects>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        state: crate::types::IssuesListState,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Project>>> {
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
        let url = self.client.url(
            &format!(
                "/users/{}/projects?{}",
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
     * List user projects.
     *
     * This function performs a `GET` to the `/users/{username}/projects` endpoint.
     *
     * As opposed to `list_for_user`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-user-projects>
     */
    pub async fn list_all_for_user(
        &self,
        username: &str,
        state: crate::types::IssuesListState,
    ) -> ClientResult<crate::Response<Vec<crate::types::Project>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projects?{}",
                crate::progenitor_support::encode_path(username),
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
}
