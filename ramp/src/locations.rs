use anyhow::Result;

use crate::Client;

pub struct Locations {
    pub client: Client,
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
    pub async fn get_page(
        &self,
        start: &str,
        page_size: f64,
    ) -> Result<Vec<crate::types::Location>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !start.is_empty() {
            query_args.push(("start".to_string(), start.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/locations?{}", query_);

        let resp: crate::types::GetLocationResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * List locations.
     *
     * This function performs a `GET` to the `/locations` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Retrieves all locations for your business.
     */
    pub async fn get_all(&self) -> Result<Vec<crate::types::Location>> {
        let url = "/locations".to_string();
        let resp: crate::types::GetLocationResponse = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut page = resp.page.next.to_string();

        // Paginate if we should.
        while !page.is_empty() {
            match self
                .client
                .get::<crate::types::GetLocationResponse>(
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
    pub async fn post(
        &self,
        body: &crate::types::PostLocationRequest,
    ) -> Result<crate::types::Location> {
        let url = "/locations".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn get(&self, id: &str) -> Result<crate::types::Location> {
        let url = format!("/locations/{}", crate::progenitor_support::encode_path(id),);

        self.client.get(&url, None).await
    }

    /**
     * Update location.
     *
     * This function performs a `PATCH` to the `/locations/{id}` endpoint.
     *
     * Modifies a specific location.
     */
    pub async fn patch(
        &self,
        id: &str,
        body: &crate::types::PostLocationRequest,
    ) -> Result<crate::types::Location> {
        let url = format!("/locations/{}", crate::progenitor_support::encode_path(id),);

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
