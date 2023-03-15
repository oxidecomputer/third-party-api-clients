use crate::Client;
use crate::ClientResult;

pub struct Groups {
    pub client: Client,
}

impl Groups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Groups { client }
    }

    /**
     * List Groups.
     *
     * This function performs a `GET` to the `/api/v1/groups` endpoint.
     *
     * Enumerates groups in your organization with pagination. A subset of groups can be returned that match a supported filter expression or query.
     *
     * **Parameters:**
     *
     * * `q: &str` -- Searches the name property of groups for matching value.
     * * `search: &str` -- Filter expression for groups.
     * * `after: &str` -- Specifies the pagination cursor for the next page of groups.
     * * `limit: i64` -- Specifies the number of group results in a page.
     * * `expand: &str` -- If specified, it causes additional metadata to be included in the response.
     */
    pub async fn list(
        &self,
        q: &str,
        search: &str,
        after: &str,
        limit: i64,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::Group>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/groups?{}", query_), None);
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
     * List Groups.
     *
     * This function performs a `GET` to the `/api/v1/groups` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Enumerates groups in your organization with pagination. A subset of groups can be returned that match a supported filter expression or query.
     */
    pub async fn list_all(
        &self,
        q: &str,
        search: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::Group>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/groups?{}", query_), None);
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
     * Add Group.
     *
     * This function performs a `POST` to the `/api/v1/groups` endpoint.
     *
     * Adds a new group with `OKTA_GROUP` type to your organization.
     */
    pub async fn create(&self, body: &crate::types::Group) -> ClientResult<crate::types::Group> {
        let url = self.client.url("/api/v1/groups", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List Group Rules.
     *
     * This function performs a `GET` to the `/api/v1/groups/rules` endpoint.
     *
     * Lists all group rules for your organization.
     *
     * **Parameters:**
     *
     * * `limit: i64` -- Specifies the number of rule results in a page.
     * * `after: &str` -- Specifies the pagination cursor for the next page of rules.
     * * `search: &str` -- Specifies the keyword to search fules for.
     * * `expand: &str` -- If specified as `groupIdToGroupNameMap`, then show group names.
     */
    pub async fn list_rules(
        &self,
        limit: i64,
        after: &str,
        search: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::GroupRule>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/groups/rules?{}", query_), None);
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
     * List Group Rules.
     *
     * This function performs a `GET` to the `/api/v1/groups/rules` endpoint.
     *
     * As opposed to `list_rules`, this function returns all the pages of the request at once.
     *
     * Lists all group rules for your organization.
     */
    pub async fn list_all_rules(
        &self,
        search: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::GroupRule>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/groups/rules?{}", query_), None);
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
     * Create Group Rule.
     *
     * This function performs a `POST` to the `/api/v1/groups/rules` endpoint.
     *
     * Creates a group rule to dynamically add users to the specified group if they match the condition
     */
    pub async fn create_rule(
        &self,
        body: &crate::types::GroupRule,
    ) -> ClientResult<crate::types::GroupRule> {
        let url = self.client.url("/api/v1/groups/rules", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get Group Rule.
     *
     * This function performs a `GET` to the `/api/v1/groups/rules/{ruleId}` endpoint.
     *
     * Fetches a specific group rule by id from your organization
     *
     * **Parameters:**
     *
     * * `rule_id: &str`
     * * `expand: &str`
     */
    pub async fn get_rule(
        &self,
        rule_id: &str,
        expand: &str,
    ) -> ClientResult<crate::types::GroupRule> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/groups/rules/{}?{}",
                crate::progenitor_support::encode_path(rule_id),
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
     * This function performs a `PUT` to the `/api/v1/groups/rules/{ruleId}` endpoint.
     *
     * Updates a group rule. Only `INACTIVE` rules can be updated.
     *
     * **Parameters:**
     *
     * * `rule_id: &str`
     */
    pub async fn update_rule(
        &self,
        rule_id: &str,
        body: &crate::types::GroupRule,
    ) -> ClientResult<crate::types::GroupRule> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/rules/{}",
                crate::progenitor_support::encode_path(rule_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a group Rule.
     *
     * This function performs a `DELETE` to the `/api/v1/groups/rules/{ruleId}` endpoint.
     *
     * Removes a specific group rule by id from your organization
     *
     * **Parameters:**
     *
     * * `rule_id: &str`
     * * `remove_users: bool` -- Indicates whether to keep or remove users from groups assigned by this rule.
     */
    pub async fn delete_rule(&self, rule_id: &str, remove_users: bool) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if remove_users {
            query_args.push(("removeUsers".to_string(), remove_users.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/groups/rules/{}?{}",
                crate::progenitor_support::encode_path(rule_id),
                query_
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
     * Activate a group Rule.
     *
     * This function performs a `POST` to the `/api/v1/groups/rules/{ruleId}/lifecycle/activate` endpoint.
     *
     * Activates a specific group rule by id from your organization
     *
     * **Parameters:**
     *
     * * `rule_id: &str`
     */
    pub async fn activate_rule(&self, rule_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/rules/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(rule_id),
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
     * Deactivate a group Rule.
     *
     * This function performs a `POST` to the `/api/v1/groups/rules/{ruleId}/lifecycle/deactivate` endpoint.
     *
     * Deactivates a specific group rule by id from your organization
     *
     * **Parameters:**
     *
     * * `rule_id: &str`
     */
    pub async fn deactivate_rule(&self, rule_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/rules/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(rule_id),
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
     * List Group Rules.
     *
     * This function performs a `GET` to the `/api/v1/groups/{groupId}` endpoint.
     *
     * Lists all group rules for your organization.
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     */
    pub async fn get(&self, group_id: &str) -> ClientResult<crate::types::Group> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}",
                crate::progenitor_support::encode_path(group_id),
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
     * Update Group.
     *
     * This function performs a `PUT` to the `/api/v1/groups/{groupId}` endpoint.
     *
     * Updates the profile for a group with `OKTA_GROUP` type from your organization.
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     */
    pub async fn update(
        &self,
        group_id: &str,
        body: &crate::types::Group,
    ) -> ClientResult<crate::types::Group> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}",
                crate::progenitor_support::encode_path(group_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Remove Group.
     *
     * This function performs a `DELETE` to the `/api/v1/groups/{groupId}` endpoint.
     *
     * Removes a group with `OKTA_GROUP` type from your organization.
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     */
    pub async fn delete(&self, group_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}",
                crate::progenitor_support::encode_path(group_id),
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
     * List Assigned Applications.
     *
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/apps` endpoint.
     *
     * Enumerates all applications that are assigned to a group.
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `after: &str` -- Specifies the pagination cursor for the next page of apps.
     * * `limit: i64` -- Specifies the number of app results for a page.
     */
    pub async fn list_assigned_applications_fors(
        &self,
        group_id: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::Application>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/apps?{}",
                crate::progenitor_support::encode_path(group_id),
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
     * List Assigned Applications.
     *
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/apps` endpoint.
     *
     * As opposed to `list_assigned_applications_for`, this function returns all the pages of the request at once.
     *
     * Enumerates all applications that are assigned to a group.
     */
    pub async fn list_all_assigned_applications_fors(
        &self,
        group_id: &str,
    ) -> ClientResult<Vec<crate::types::Application>> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/apps",
                crate::progenitor_support::encode_path(group_id),
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
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/roles` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `expand: &str`
     */
    pub async fn list_assigned_roles(
        &self,
        group_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::Role>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles?{}",
                crate::progenitor_support::encode_path(group_id),
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
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/roles` endpoint.
     *
     * As opposed to `list_assigned_roles`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_assigned_roles(
        &self,
        group_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::Role>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles?{}",
                crate::progenitor_support::encode_path(group_id),
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
     * This function performs a `POST` to the `/api/v1/groups/{groupId}/roles` endpoint.
     *
     * Assigns a Role to a Group
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `disable_notifications: &str`
     */
    pub async fn assign_role(
        &self,
        group_id: &str,
        disable_notifications: &str,
        body: &crate::types::AssignRoleRequest,
    ) -> ClientResult<crate::types::Role> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !disable_notifications.is_empty() {
            query_args.push((
                "disableNotifications".to_string(),
                disable_notifications.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles?{}",
                crate::progenitor_support::encode_path(group_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/roles/{roleId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     */
    pub async fn get_role(
        &self,
        group_id: &str,
        role_id: &str,
    ) -> ClientResult<crate::types::Role> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `DELETE` to the `/api/v1/groups/{groupId}/roles/{roleId}` endpoint.
     *
     * Unassigns a Role from a Group
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     */
    pub async fn remove_role_from(&self, group_id: &str, role_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/catalog/apps` endpoint.
     *
     * Lists all App targets for an `APP_ADMIN` Role assigned to a Group. This methods return list may include full Applications or Instances. The response for an instance will have an `ID` value, while Application will not have an ID.
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_application_targets_for_administrator_roles(
        &self,
        group_id: &str,
        role_id: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::CatalogApplication>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/catalog/apps?{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/catalog/apps` endpoint.
     *
     * As opposed to `list_application_targets_for_administrator_role`, this function returns all the pages of the request at once.
     *
     * Lists all App targets for an `APP_ADMIN` Role assigned to a Group. This methods return list may include full Applications or Instances. The response for an instance will have an `ID` value, while Application will not have an ID.
     */
    pub async fn list_all_application_targets_for_administrator_roles(
        &self,
        group_id: &str,
        role_id: &str,
    ) -> ClientResult<Vec<crate::types::CatalogApplication>> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/catalog/apps",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `PUT` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/catalog/apps/{appName}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     * * `app_name: &str`
     */
    pub async fn add_application_target_admin_role_given(
        &self,
        group_id: &str,
        role_id: &str,
        app_name: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/catalog/apps/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(app_name),
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
     * This function performs a `DELETE` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/catalog/apps/{appName}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     * * `app_name: &str`
     */
    pub async fn remove_application_target_from_administrator_role_given(
        &self,
        group_id: &str,
        role_id: &str,
        app_name: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/catalog/apps/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(app_name),
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
     * Add App Instance Target to App Administrator Role given to a Group.
     *
     * This function performs a `PUT` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/catalog/apps/{appName}/{applicationId}` endpoint.
     *
     * Add App Instance Target to App Administrator Role given to a Group
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     * * `app_name: &str`
     * * `application_id: &str`
     */
    pub async fn add_application_instance_target_app_admin_role_given(
        &self,
        group_id: &str,
        role_id: &str,
        app_name: &str,
        application_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/catalog/apps/{}/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(app_name),
                crate::progenitor_support::encode_path(application_id),
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
     * Remove App Instance Target to App Administrator Role given to a Group.
     *
     * This function performs a `DELETE` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/catalog/apps/{appName}/{applicationId}` endpoint.
     *
     * Remove App Instance Target to App Administrator Role given to a Group
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     * * `app_name: &str`
     * * `application_id: &str`
     */
    pub async fn remove_application_target_from_administrator_role_given_groups(
        &self,
        group_id: &str,
        role_id: &str,
        app_name: &str,
        application_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/catalog/apps/{}/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(app_name),
                crate::progenitor_support::encode_path(application_id),
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
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/groups` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_targets_for_roles(
        &self,
        group_id: &str,
        role_id: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::Group>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/groups?{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/groups` endpoint.
     *
     * As opposed to `list_targets_for_role`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_targets_for_roles(
        &self,
        group_id: &str,
        role_id: &str,
    ) -> ClientResult<Vec<crate::types::Group>> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/groups",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `PUT` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/groups/{targetGroupId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     * * `target_group_id: &str`
     */
    pub async fn add_target_administrator_role_for(
        &self,
        group_id: &str,
        role_id: &str,
        target_group_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/groups/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(target_group_id),
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
     * This function performs a `DELETE` to the `/api/v1/groups/{groupId}/roles/{roleId}/targets/groups/{targetGroupId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `role_id: &str`
     * * `target_group_id: &str`
     */
    pub async fn remove_target_from_administrator_role_given(
        &self,
        group_id: &str,
        role_id: &str,
        target_group_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/roles/{}/targets/groups/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(target_group_id),
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
     * List Group Members.
     *
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/users` endpoint.
     *
     * Enumerates all users that are a member of a group.
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `after: &str` -- Specifies the pagination cursor for the next page of users.
     * * `limit: i64` -- Specifies the number of user results in a page.
     */
    pub async fn list_users(
        &self,
        group_id: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/users?{}",
                crate::progenitor_support::encode_path(group_id),
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
     * List Group Members.
     *
     * This function performs a `GET` to the `/api/v1/groups/{groupId}/users` endpoint.
     *
     * As opposed to `list_users`, this function returns all the pages of the request at once.
     *
     * Enumerates all users that are a member of a group.
     */
    pub async fn list_all_users(&self, group_id: &str) -> ClientResult<Vec<crate::types::User>> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/users",
                crate::progenitor_support::encode_path(group_id),
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
     * Add User to Group.
     *
     * This function performs a `PUT` to the `/api/v1/groups/{groupId}/users/{userId}` endpoint.
     *
     * Adds a user to a group with 'OKTA_GROUP' type.
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `user_id: &str`
     */
    pub async fn add_user(&self, group_id: &str, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/users/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Remove User from Group.
     *
     * This function performs a `DELETE` to the `/api/v1/groups/{groupId}/users/{userId}` endpoint.
     *
     * Removes a user from a group with 'OKTA_GROUP' type.
     *
     * **Parameters:**
     *
     * * `group_id: &str`
     * * `user_id: &str`
     */
    pub async fn remove_user_from(&self, group_id: &str, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/groups/{}/users/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(user_id),
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
