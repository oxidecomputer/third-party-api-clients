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
     * List projects for organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2` endpoint.
     *
     * List all projects owned by a specific organization accessible by the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/projects/projects#list-projects-for-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `q: &str` -- Limit results to projects of the specified type.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_org(
        &self,
        org: &str,
        q: &str,
        before: &str,
        after: &str,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsV2>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2?{}",
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
     * List projects for organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2` endpoint.
     *
     * As opposed to `list_for_org`, this function returns all the pages of the request at once.
     *
     * List all projects owned by a specific organization accessible by the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/projects/projects#list-projects-for-organization>
     */
    pub async fn list_all_for_org(
        &self,
        org: &str,
        q: &str,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsV2>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2?{}",
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
     * Get project for organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2/{project_number}` endpoint.
     *
     * Get a specific organization-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/projects#get-project-for-organization>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_for_org(
        &self,
        project_number: i64,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ProjectsV2>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Create draft item for organization owned project.
     *
     * This function performs a `POST` to the `/orgs/{org}/projectsV2/{project_number}/drafts` endpoint.
     *
     * Create draft issue item for the specified organization owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/drafts#create-draft-item-for-organization-owned-project>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `project_number: i64` -- The project's number.
     */
    pub async fn create_draft_item_for_org(
        &self,
        org: &str,
        project_number: i64,
        body: &crate::types::ProjectsCreateDraftItemRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectsItem>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/drafts",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * List project fields for organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2/{project_number}/fields` endpoint.
     *
     * List all fields for a specific organization-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/fields#list-project-fields-for-organization>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_fields_for_org(
        &self,
        project_number: i64,
        org: &str,
        per_page: i64,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsField>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/fields?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * List project fields for organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2/{project_number}/fields` endpoint.
     *
     * As opposed to `list_fields_for_org`, this function returns all the pages of the request at once.
     *
     * List all fields for a specific organization-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/fields#list-project-fields-for-organization>
     */
    pub async fn list_all_fields_for_org(
        &self,
        project_number: i64,
        org: &str,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsField>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/fields?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Add a field to an organization-owned project.
     *
     * This function performs a `POST` to the `/orgs/{org}/projectsV2/{project_number}/fields` endpoint.
     *
     * Add a field to an organization-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/fields#add-a-field-to-an-organization-owned-project>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn add_field_for_org(
        &self,
        project_number: i64,
        org: &str,
        body: &crate::types::ProjectsAddFieldOrgRequestOneOf,
    ) -> ClientResult<crate::Response<crate::types::ProjectsField>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/fields",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Get project field for organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2/{project_number}/fields/{field_id}` endpoint.
     *
     * Get a specific field for an organization-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/fields#get-project-field-for-organization>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `field_id: i64` -- The unique identifier of the field.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_field_for_org(
        &self,
        project_number: i64,
        field_id: i64,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ProjectsField>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/fields/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&field_id.to_string()),
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
     * List items for an organization owned project.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2/{project_number}/items` endpoint.
     *
     * List all items for a specific organization-owned project accessible by the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/projects/items#list-items-for-an-organization-owned-project>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `q: &str` -- Search query to filter items, see [Filtering projects](https://docs.github.com/issues/planning-and-tracking-with-projects/customizing-views-in-your-project/filtering-projects) for more information.
     * * `fields: &str` -- Limit results to specific fields, by their IDs. If not specified, the title field will be returned.
     *   
     *   Example: `fields[]=123&fields[]=456&fields[]=789` or `fields=123,456,789`.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_items_for_org(
        &self,
        project_number: i64,
        org: &str,
        q: &str,
        fields: &str,
        before: &str,
        after: &str,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsItemWithContent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/items?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * List items for an organization owned project.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2/{project_number}/items` endpoint.
     *
     * As opposed to `list_items_for_org`, this function returns all the pages of the request at once.
     *
     * List all items for a specific organization-owned project accessible by the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/projects/items#list-items-for-an-organization-owned-project>
     */
    pub async fn list_all_items_for_org(
        &self,
        project_number: i64,
        org: &str,
        q: &str,
        fields: &str,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsItemWithContent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/items?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Add item to organization owned project.
     *
     * This function performs a `POST` to the `/orgs/{org}/projectsV2/{project_number}/items` endpoint.
     *
     * Add an issue or pull request item to the specified organization owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/items#add-item-to-organization-owned-project>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `project_number: i64` -- The project's number.
     */
    pub async fn add_item_for_org(
        &self,
        org: &str,
        project_number: i64,
        body: &crate::types::ProjectsAddItemOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectsItem>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/items",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Get an item for an organization owned project.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2/{project_number}/items/{item_id}` endpoint.
     *
     * Get a specific item from an organization-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/items#get-an-item-for-an-organization-owned-project>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `item_id: i64` -- The unique identifier of the project item.
     * * `fields: &str` -- Limit results to specific fields, by their IDs. If not specified, the title field will be returned.
     *   
     *   Example: fields[]=123&fields[]=456&fields[]=789 or fields=123,456,789.
     */
    pub async fn get_org_item(
        &self,
        project_number: i64,
        org: &str,
        item_id: i64,
        fields: &str,
    ) -> ClientResult<crate::Response<crate::types::ProjectsItemWithContent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/items/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&item_id.to_string()),
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
     * Delete project item for organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/projectsV2/{project_number}/items/{item_id}` endpoint.
     *
     * Delete a specific item from an organization-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/items#delete-project-item-for-organization>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `item_id: i64` -- The unique identifier of the project item.
     */
    pub async fn delete_item_for_org(
        &self,
        project_number: i64,
        org: &str,
        item_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/items/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&item_id.to_string()),
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
     * Update project item for organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/projectsV2/{project_number}/items/{item_id}` endpoint.
     *
     * Update a specific item in an organization-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/items#update-project-item-for-organization>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `item_id: i64` -- The unique identifier of the project item.
     */
    pub async fn update_item_for_org(
        &self,
        project_number: i64,
        org: &str,
        item_id: i64,
        body: &crate::types::ProjectsUpdateItemOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectsItemWithContent>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/items/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&item_id.to_string()),
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
     * Create a view for an organization-owned project.
     *
     * This function performs a `POST` to the `/orgs/{org}/projectsV2/{project_number}/views` endpoint.
     *
     * Create a new view in an organization-owned project. Views allow you to customize how items in a project are displayed and filtered.
     *
     * FROM: <https://docs.github.com/rest/projects/views#create-a-view-for-an-organization-owned-project>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `project_number: i64` -- The project's number.
     */
    pub async fn create_view_for_org(
        &self,
        org: &str,
        project_number: i64,
        body: &crate::types::ProjectsCreateViewOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectsView>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/views",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * List items for an organization project view.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2/{project_number}/views/{view_number}/items` endpoint.
     *
     * List items in an organization project with the saved view's filter applied.
     *
     * FROM: <https://docs.github.com/rest/projects/items#list-items-for-an-organization-project-view>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `view_number: i64` -- The number that identifies the project view.
     * * `fields: &str` -- Limit results to specific fields, by their IDs. If not specified, the
     *   title field will be returned.
     *   
     *   Example: `fields[]=123&fields[]=456&fields[]=789` or `fields=123,456,789`.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_view_items_for_org(
        &self,
        project_number: i64,
        org: &str,
        view_number: i64,
        fields: &str,
        before: &str,
        after: &str,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsItemWithContent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/views/{}/items?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&view_number.to_string()),
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
     * List items for an organization project view.
     *
     * This function performs a `GET` to the `/orgs/{org}/projectsV2/{project_number}/views/{view_number}/items` endpoint.
     *
     * As opposed to `list_view_items_for_org`, this function returns all the pages of the request at once.
     *
     * List items in an organization project with the saved view's filter applied.
     *
     * FROM: <https://docs.github.com/rest/projects/items#list-items-for-an-organization-project-view>
     */
    pub async fn list_all_view_items_for_org(
        &self,
        project_number: i64,
        org: &str,
        view_number: i64,
        fields: &str,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsItemWithContent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/projectsV2/{}/views/{}/items?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&view_number.to_string()),
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
     * Create draft item for user owned project.
     *
     * This function performs a `POST` to the `/user/{user_id}/projectsV2/{project_number}/drafts` endpoint.
     *
     * Create draft issue item for the specified user owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/drafts#create-draft-item-for-user-owned-project>
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The unique identifier of the user.
     * * `project_number: i64` -- The project's number.
     */
    pub async fn create_draft_item_for_authenticated_user(
        &self,
        user_id: &str,
        project_number: i64,
        body: &crate::types::ProjectsCreateDraftItemRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectsItem>> {
        let url = self.client.url(
            &format!(
                "/user/{}/projectsV2/{}/drafts",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Create a view for a user-owned project.
     *
     * This function performs a `POST` to the `/users/{user_id}/projectsV2/{project_number}/views` endpoint.
     *
     * Create a new view in a user-owned project. Views allow you to customize how items in a project are displayed and filtered.
     *
     * FROM: <https://docs.github.com/rest/projects/views#create-a-view-for-a-user-owned-project>
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The unique identifier of the user.
     * * `project_number: i64` -- The project's number.
     */
    pub async fn create_view_for_user(
        &self,
        user_id: &str,
        project_number: i64,
        body: &crate::types::ProjectsCreateViewOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectsView>> {
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/views",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * List projects for user.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2` endpoint.
     *
     * List all projects owned by a specific user accessible by the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/projects/projects#list-projects-for-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `q: &str` -- Limit results to projects of the specified type.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        q: &str,
        before: &str,
        after: &str,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsV2>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List projects for user.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2` endpoint.
     *
     * As opposed to `list_for_user`, this function returns all the pages of the request at once.
     *
     * List all projects owned by a specific user accessible by the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/projects/projects#list-projects-for-user>
     */
    pub async fn list_all_for_user(
        &self,
        username: &str,
        q: &str,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsV2>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Get project for user.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2/{project_number}` endpoint.
     *
     * Get a specific user-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/projects#get-project-for-user>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_for_user(
        &self,
        project_number: i64,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::ProjectsV2>> {
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * List project fields for user.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2/{project_number}/fields` endpoint.
     *
     * List all fields for a specific user-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/fields#list-project-fields-for-user>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_fields_for_user(
        &self,
        project_number: i64,
        username: &str,
        per_page: i64,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsField>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/fields?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * List project fields for user.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2/{project_number}/fields` endpoint.
     *
     * As opposed to `list_fields_for_user`, this function returns all the pages of the request at once.
     *
     * List all fields for a specific user-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/fields#list-project-fields-for-user>
     */
    pub async fn list_all_fields_for_user(
        &self,
        project_number: i64,
        username: &str,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsField>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/fields?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Add field to user owned project.
     *
     * This function performs a `POST` to the `/users/{username}/projectsV2/{project_number}/fields` endpoint.
     *
     * Add a field to a specified user owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/fields#add-field-to-user-owned-project>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `project_number: i64` -- The project's number.
     */
    pub async fn add_field_for_user(
        &self,
        username: &str,
        project_number: i64,
        body: &crate::types::ProjectsAddFieldUserRequestOneOf,
    ) -> ClientResult<crate::Response<crate::types::ProjectsField>> {
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/fields",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Get project field for user.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2/{project_number}/fields/{field_id}` endpoint.
     *
     * Get a specific field for a user-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/fields#get-project-field-for-user>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `field_id: i64` -- The unique identifier of the field.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_field_for_user(
        &self,
        project_number: i64,
        field_id: i64,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::ProjectsField>> {
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/fields/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&field_id.to_string()),
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
     * List items for a user owned project.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2/{project_number}/items` endpoint.
     *
     * List all items for a specific user-owned project accessible by the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/projects/items#list-items-for-a-user-owned-project>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `q: &str` -- Search query to filter items, see [Filtering projects](https://docs.github.com/issues/planning-and-tracking-with-projects/customizing-views-in-your-project/filtering-projects) for more information.
     * * `fields: &str` -- Limit results to specific fields, by their IDs. If not specified, the title field will be returned.
     *   
     *   Example: `fields[]=123&fields[]=456&fields[]=789` or `fields=123,456,789`.
     */
    pub async fn list_items_for_user(
        &self,
        project_number: i64,
        username: &str,
        before: &str,
        after: &str,
        per_page: i64,
        q: &str,
        fields: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsItemWithContent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/items?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * List items for a user owned project.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2/{project_number}/items` endpoint.
     *
     * As opposed to `list_items_for_user`, this function returns all the pages of the request at once.
     *
     * List all items for a specific user-owned project accessible by the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/projects/items#list-items-for-a-user-owned-project>
     */
    pub async fn list_all_items_for_user(
        &self,
        project_number: i64,
        username: &str,
        before: &str,
        after: &str,
        q: &str,
        fields: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsItemWithContent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/items?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Add item to user owned project.
     *
     * This function performs a `POST` to the `/users/{username}/projectsV2/{project_number}/items` endpoint.
     *
     * Add an issue or pull request item to the specified user owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/items#add-item-to-user-owned-project>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `project_number: i64` -- The project's number.
     */
    pub async fn add_item_for_user(
        &self,
        username: &str,
        project_number: i64,
        body: &crate::types::ProjectsAddItemOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectsItem>> {
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/items",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
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
     * Get an item for a user owned project.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2/{project_number}/items/{item_id}` endpoint.
     *
     * Get a specific item from a user-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/items#get-an-item-for-a-user-owned-project>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `item_id: i64` -- The unique identifier of the project item.
     * * `fields: &str` -- Limit results to specific fields, by their IDs. If not specified, the title field will be returned.
     *   
     *   Example: fields[]=123&fields[]=456&fields[]=789 or fields=123,456,789.
     */
    pub async fn get_user_item(
        &self,
        project_number: i64,
        username: &str,
        item_id: i64,
        fields: &str,
    ) -> ClientResult<crate::Response<crate::types::ProjectsItemWithContent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/items/{}?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&item_id.to_string()),
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
     * Delete project item for user.
     *
     * This function performs a `DELETE` to the `/users/{username}/projectsV2/{project_number}/items/{item_id}` endpoint.
     *
     * Delete a specific item from a user-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/items#delete-project-item-for-user>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `item_id: i64` -- The unique identifier of the project item.
     */
    pub async fn delete_item_for_user(
        &self,
        project_number: i64,
        username: &str,
        item_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/items/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&item_id.to_string()),
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
     * Update project item for user.
     *
     * This function performs a `PATCH` to the `/users/{username}/projectsV2/{project_number}/items/{item_id}` endpoint.
     *
     * Update a specific item in a user-owned project.
     *
     * FROM: <https://docs.github.com/rest/projects/items#update-project-item-for-user>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `item_id: i64` -- The unique identifier of the project item.
     */
    pub async fn update_item_for_user(
        &self,
        project_number: i64,
        username: &str,
        item_id: i64,
        body: &crate::types::ProjectsUpdateItemOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ProjectsItemWithContent>> {
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/items/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&item_id.to_string()),
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
     * List items for a user project view.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2/{project_number}/views/{view_number}/items` endpoint.
     *
     * List items in a user project with the saved view's filter applied.
     *
     * FROM: <https://docs.github.com/rest/projects/items#list-items-for-a-user-project-view>
     *
     * **Parameters:**
     *
     * * `project_number: i64` -- The project's number.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `view_number: i64` -- The number that identifies the project view.
     * * `fields: &str` -- Limit results to specific fields, by their IDs. If not specified, the
     *   title field will be returned.
     *   
     *   Example: `fields[]=123&fields[]=456&fields[]=789` or `fields=123,456,789`.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_view_items_for_user(
        &self,
        project_number: i64,
        username: &str,
        view_number: i64,
        fields: &str,
        before: &str,
        after: &str,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsItemWithContent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/views/{}/items?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&view_number.to_string()),
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
     * List items for a user project view.
     *
     * This function performs a `GET` to the `/users/{username}/projectsV2/{project_number}/views/{view_number}/items` endpoint.
     *
     * As opposed to `list_view_items_for_user`, this function returns all the pages of the request at once.
     *
     * List items in a user project with the saved view's filter applied.
     *
     * FROM: <https://docs.github.com/rest/projects/items#list-items-for-a-user-project-view>
     */
    pub async fn list_all_view_items_for_user(
        &self,
        project_number: i64,
        username: &str,
        view_number: i64,
        fields: &str,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ProjectsItemWithContent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/projectsV2/{}/views/{}/items?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&project_number.to_string()),
                crate::progenitor_support::encode_path(&view_number.to_string()),
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
