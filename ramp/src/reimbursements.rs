use anyhow::Result;

use crate::Client;

pub struct Reimbursements {
    pub client: Client,
}

impl Reimbursements {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reimbursements { client }
    }

    /**
     * List Reimbursements.
     *
     * This function performs a `GET` to the `/reimbursements` endpoint.
     *
     * **Parameters:**
     *
     * * `start: uuid::Uuid` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_page(
        &self,
        start: uuid::Uuid,
        page_size: f64,
    ) -> Result<Vec<crate::types::Reimbursement>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !start.to_string().is_empty() {
            query_args.push(("start".to_string(), start.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/reimbursements?{}", query_);

        let resp: crate::types::GetReimbursementsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * List Reimbursements.
     *
     * This function performs a `GET` to the `/reimbursements` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(&self) -> Result<Vec<crate::types::Reimbursement>> {
        let url = "/reimbursements".to_string();
        let mut resp: crate::types::GetReimbursementsResponse = self.client.get(&url, None).await?;

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
                .get::<crate::types::GetReimbursementsResponse>(
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
                        bail!(e);
                    }
                }
            }
        }

        // Return our response data.
        Ok(data)
    }

    /**
     * Get details for one reimbursement.
     *
     * This function performs a `GET` to the `/reimbursements/{id}` endpoint.
     */
    pub async fn get(&self, id: &str) -> Result<crate::types::Reimbursement> {
        let url = format!(
            "/reimbursements/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
