use crate::Client;
use crate::ClientResult;

pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
     * Get User Info by User ID.
     *
     * This function performs a `GET` to the `/users/{id}` endpoint.
     *
     * Retrieve the information of the user with the matching user ID.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get(&self, id: &str) -> ClientResult<crate::types::User> {
        let url = self.client.url(
            &format!("/users/{}", crate::progenitor_support::encode_path(id),),
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
     * Suspend a user.
     *
     * This function performs a `DELETE` to the `/users/{id}` endpoint.
     *
     * Suspends a user. Does not delete the user's cards. Currently this action is not reversible.
     */
    pub async fn delete(&self, id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!("/users/{}", crate::progenitor_support::encode_path(id),),
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
     * Modify Existing User.
     *
     * This function performs a `PATCH` to the `/users/{id}` endpoint.
     *
     * Modify information about a user.
     */
    pub async fn patch(
        &self,
        id: &str,
        body: &crate::types::PatchUsersRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!("/users/{}", crate::progenitor_support::encode_path(id),),
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
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * Retrieve all users of the business.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     * * `department_id: &str` -- The OAuth2 token header.
     * * `location_id: &str` -- The OAuth2 token header.
     */
    pub async fn get_page(
        &self,
        start: &str,
        page_size: f64,
        department_id: &str,
        location_id: &str,
    ) -> ClientResult<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !department_id.is_empty() {
            query_args.push(("department_id".to_string(), department_id.to_string()));
        }
        if !location_id.is_empty() {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !start.is_empty() {
            query_args.push(("start".to_string(), start.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/users?{}", query_), None);
        let resp: crate::types::GetUsersResponse = self
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
        Ok(resp.data.to_vec())
    }
    /**
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Retrieve all users of the business.
     */
    pub async fn get_all(
        &self,
        department_id: &str,
        location_id: &str,
    ) -> ClientResult<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !department_id.is_empty() {
            query_args.push(("department_id".to_string(), department_id.to_string()));
        }
        if !location_id.is_empty() {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/users?{}", query_), None);
        let resp: crate::types::GetUsersResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut data = resp.data;
        let mut page = resp.page.next.to_string();

        // Paginate if we should.
        while !page.is_empty() {
            match self
                .client
                .get::<crate::types::GetUsersResponse>(
                    page.trim_start_matches(&self.client.host),
                    crate::Message {
                        body: None,
                        content_type: None,
                    },
                )
                .await
            {
                Ok(mut resp) => {
                    data.append(&mut resp.data);

                    page = if resp.page.next != page {
                        resp.page.next.to_string()
                    } else {
                        "".to_string()
                    };
                }
                Err(e) => {
                    if e.to_string().contains("404 Not Found") {
                        page = "".to_string();
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        // Return our response data.
        Ok(data)
    }
    /**
     * Invite a new user.
     *
     * This function performs a `POST` to the `/users/deferred` endpoint.
     *
     * Creates an invite for the user to accept. Also optionally sets department, location, and manager.
     */
    pub async fn post_deferred(
        &self,
        body: &crate::types::PostUsersDeferredRequest,
    ) -> ClientResult<crate::types::User> {
        let url = self.client.url("/users/deferred", None);
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
     * Get status of a deferred user task.
     *
     * This function performs a `GET` to the `/users/deferred/status/{id}` endpoint.
     *
     * Gets status of a deferred task for users
     */
    pub async fn get_deferred_status(
        &self,
        id: &str,
    ) -> ClientResult<crate::types::GetUsersDeferredStatusResponse> {
        let url = self.client.url(
            &format!(
                "/users/deferred/status/{}",
                crate::progenitor_support::encode_path(id),
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
}
