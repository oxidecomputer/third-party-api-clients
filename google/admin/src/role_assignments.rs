use crate::Client;
use crate::ClientResult;

pub struct RoleAssignments {
    pub client: Client,
}

impl RoleAssignments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        RoleAssignments { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/roleassignments` endpoint.
     *
     * Retrieves a paginated list of all roleAssignments.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `max_results: i64` -- Maximum number of results to return.
     * * `page_token: &str` -- Token to specify the next page in the list.
     * * `role_id: &str` -- Immutable ID of a role. If included in the request, returns only role assignments containing this role ID.
     * * `user_key: &str` -- The user's primary email address, alias email address, or unique user ID. If included in the request, returns role assignments only for this user.
     */
    pub async fn list(
        &self,
        customer: &str,
        max_results: i64,
        page_token: &str,
        role_id: &str,
        user_key: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RoleAssignment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !role_id.is_empty() {
            query_args.push(("roleId".to_string(), role_id.to_string()));
        }
        if !user_key.is_empty() {
            query_args.push(("userKey".to_string(), user_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/roleassignments?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::Response<crate::types::RoleAssignments> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.items.to_vec(),
        ))
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/roleassignments` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Retrieves a paginated list of all roleAssignments.
     */
    pub async fn list_all(
        &self,
        customer: &str,
        role_id: &str,
        user_key: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RoleAssignment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !role_id.is_empty() {
            query_args.push(("roleId".to_string(), role_id.to_string()));
        }
        if !user_key.is_empty() {
            query_args.push(("userKey".to_string(), user_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/roleassignments?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let crate::Response::<crate::types::RoleAssignments> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut items = body.items;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                crate::Response::<crate::types::RoleAssignments> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::RoleAssignments> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            items.append(&mut body.items);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, items))
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/roleassignments` endpoint.
     *
     * Creates a role assignment.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn insert(
        &self,
        customer: &str,
        body: &crate::types::RoleAssignment,
    ) -> ClientResult<crate::Response<crate::types::RoleAssignment>> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/roleassignments",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/roleassignments/{roleAssignmentId}` endpoint.
     *
     * Retrieves a role assignment.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `role_assignment_id: &str` -- Immutable ID of the role assignment.
     */
    pub async fn get(
        &self,
        customer: &str,
        role_assignment_id: &str,
    ) -> ClientResult<crate::Response<crate::types::RoleAssignment>> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/roleassignments/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(role_assignment_id),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/roleassignments/{roleAssignmentId}` endpoint.
     *
     * Deletes a role assignment.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `role_assignment_id: &str` -- Immutable ID of the role assignment.
     */
    pub async fn delete(
        &self,
        customer: &str,
        role_assignment_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/roleassignments/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(role_assignment_id),
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
