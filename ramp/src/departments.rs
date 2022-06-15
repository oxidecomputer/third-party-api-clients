use anyhow::Result;

use crate::Client;

pub struct Departments {
    pub client: Client,
}

impl Departments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Departments { client }
    }

    /**
     * List departments.
     *
     * This function performs a `GET` to the `/departments` endpoint.
     *
     * Retrieve all departments.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_page(
        &self,
        start: &str,
        page_size: f64,
    ) -> Result<Vec<crate::types::Department>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !start.is_empty() {
            query_args.push(("start".to_string(), start.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/departments?{}", query_);

        let resp: crate::types::GetDepartmentsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * List departments.
     *
     * This function performs a `GET` to the `/departments` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Retrieve all departments.
     */
    pub async fn get_all(&self) -> Result<Vec<crate::types::Department>> {
        let url = "/departments".to_string();
        let resp: crate::types::GetDepartmentsResponse = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut page = resp.page.next.to_string();

        // Paginate if we should.
        while !page.is_empty() {
            match self
                .client
                .get::<crate::types::GetDepartmentsResponse>(
                    page.trim_start_matches(crate::DEFAULT_HOST),
                    None,
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
                        anyhow::bail!(e);
                    }
                }
            }
        }

        // Return our response data.
        Ok(data)
    }

    /**
     * Create department.
     *
     * This function performs a `POST` to the `/departments` endpoint.
     *
     * Create a new department.
     */
    pub async fn post(
        &self,
        body: &crate::types::PostLocationRequest,
    ) -> Result<crate::types::Department> {
        let url = "/departments".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * GET a department.
     *
     * This function performs a `GET` to the `/departments/{id}` endpoint.
     *
     * Retrieve a single department.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get(&self, id: &str) -> Result<crate::types::Department> {
        let url = format!(
            "/departments/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update department.
     *
     * This function performs a `PATCH` to the `/departments/{id}` endpoint.
     *
     * Modify a department.
     */
    pub async fn patch(
        &self,
        id: &str,
        body: &crate::types::PostLocationRequest,
    ) -> Result<crate::types::Department> {
        let url = format!(
            "/departments/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
