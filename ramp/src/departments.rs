use anyhow::Result;

use crate::Client;

pub struct Departments {
    client: Client,
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
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("page_size={}", page_size));
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/departments?{}", query);

        let resp: crate::types::GetDepartmentsResponse = self.client.get(&url, None).await.unwrap();

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
        let mut resp: crate::types::GetDepartmentsResponse =
            self.client.get(&url, None).await.unwrap();

        let mut data = resp.data;
        let mut page = resp.page.next;

        // Paginate if we should.
        while !page.is_empty() {
            resp = self
                .client
                .get(page.trim_start_matches(crate::DEFAULT_HOST), None)
                .await
                .unwrap();

            data.append(&mut resp.data);

            if !resp.page.next.is_empty() && resp.page.next != page {
                page = resp.page.next.to_string();
            } else {
                page = "".to_string();
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
    pub async fn post_department(
        &self,
        body: &crate::types::PostLocationRequest,
    ) -> Result<crate::types::Department> {
        let url = "/departments".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
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
    pub async fn get_department(&self, id: &str) -> Result<crate::types::Department> {
        let url = format!(
            "/departments/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
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
    pub async fn patch_department(
        &self,
        id: &str,
        body: &crate::types::PatchDepartmentRequest,
    ) -> Result<crate::types::Department> {
        let url = format!(
            "/departments/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
