use anyhow::Result;

use crate::Client;

pub struct Locations {
    client: Client,
}

impl Locations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Locations { client }
    }

    /**
     * List locations.
     *
     * This function performs a `GET` to the `/locations` endpoint.
     *
     * Retrieves all locations for your business.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_locations(
        &self,
        start: &str,
        page_size: f64,
    ) -> Result<Vec<crate::types::Location>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("page_size={}", page_size));
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/locations?{}", query_);

        let resp: crate::types::GetLocationResponse = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * List locations.
     *
     * This function performs a `GET` to the `/locations` endpoint.
     *
     * As opposed to `get_location`, this function returns all the pages of the request at once.
     *
     * Retrieves all locations for your business.
     */
    pub async fn get_all_locations(&self) -> Result<Vec<crate::types::Location>> {
        let url = "/locations".to_string();
        let mut resp: crate::types::GetLocationResponse =
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
     * Create new location.
     *
     * This function performs a `POST` to the `/locations` endpoint.
     *
     * Creates a new location for the business.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn post_location(
        &self,
        body: &crate::types::PostLocationRequest,
    ) -> Result<crate::types::Location> {
        let url = "/locations".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * GET a location.
     *
     * This function performs a `GET` to the `/locations/{id}` endpoint.
     *
     * Retrieve a specific location.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_location(&self, id: &str) -> Result<crate::types::Location> {
        let url = format!(
            "/locations/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update location.
     *
     * This function performs a `PATCH` to the `/locations/{id}` endpoint.
     *
     * Modifies a specific location.
     */
    pub async fn patch_location(
        &self,
        id: &str,
        body: &crate::types::PostLocationRequest,
    ) -> Result<crate::types::Location> {
        let url = format!(
            "/locations/{}",
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
