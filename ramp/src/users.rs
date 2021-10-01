use anyhow::Result;

use crate::Client;

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
    pub async fn get(&self, id: &str) -> Result<crate::types::User> {
        let url = format!(
            "/users/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Suspend a user.
     *
     * This function performs a `DELETE` to the `/users/{id}` endpoint.
     *
     * Suspends a user. Does not delete the user's cards. Currently this action is not reversible.
     */
    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = format!(
            "/users/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Modify Existing User.
     *
     * This function performs a `PATCH` to the `/users/{id}` endpoint.
     *
     * Modify information about a user.
     */
    pub async fn patch(&self, id: &str, body: &crate::types::PatchUsersRequest) -> Result<()> {
        let url = format!(
            "/users/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
     * * `start: uuid::Uuid` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     * * `department_id: uuid::Uuid` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `location_id: uuid::Uuid` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     */
    pub async fn get_page(
        &self,
        start: uuid::Uuid,
        page_size: f64,
        department_id: uuid::Uuid,
        location_id: uuid::Uuid,
    ) -> Result<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !department_id.to_string().is_empty() {
            query_args.push(("department_id".to_string(), department_id.to_string()));
        }
        if !location_id.to_string().is_empty() {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !start.to_string().is_empty() {
            query_args.push(("start".to_string(), start.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users?{}", query_);

        let resp: crate::types::GetUsersResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data)
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
        department_id: uuid::Uuid,
        location_id: uuid::Uuid,
    ) -> Result<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !department_id.to_string().is_empty() {
            query_args.push(("department_id".to_string(), department_id.to_string()));
        }
        if !location_id.to_string().is_empty() {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users?{}", query_);

        let mut resp: crate::types::GetUsersResponse = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut page = if let Some(p) = resp.page.next {
            p.to_string()
        } else {
            "".to_string()
        };

        // Paginate if we should.
        while !page.is_empty() {
            match self
                .client
                .get::<crate::types::GetUsersResponse>(
                    page.trim_start_matches(crate::DEFAULT_HOST),
                    None,
                )
                .await
            {
                Ok(resp) => {
                    data.append(&mut resp.data);

                    page = if let Some(p) = resp.page.next {
                        if p.to_string() != page {
                            p.to_string()
                        } else {
                            "".to_string()
                        }
                    } else {
                        "".to_string()
                    };
                }
                Err(e) => {
                    if e.to_string().contains("404 Not Found") {
                        page = "".to_string();
                    } else {
                        anyhow::bail!(e);
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
    ) -> Result<crate::types::User> {
        let url = "/users/deferred".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    ) -> Result<crate::types::GetUsersDeferredStatusResponse> {
        let url = format!(
            "/users/deferred/status/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
