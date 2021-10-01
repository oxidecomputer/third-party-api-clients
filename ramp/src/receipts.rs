use anyhow::Result;

use crate::Client;

pub struct Receipts {
    pub client: Client,
}

impl Receipts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Receipts { client }
    }

    /**
     * List receipts.
     *
     * This function performs a `GET` to the `/receipts` endpoint.
     *
     * Returns description of all receipts of a business.
     *
     * **Parameters:**
     *
     * * `from_date: chrono::DateTime<chrono::Utc>` -- Filter for receipts related to transactions which occurred after the specified date.
     * * `to_date: chrono::DateTime<chrono::Utc>` -- Filter for receipts related to transactions which occurred before the specified date.
     * * `created_after: chrono::DateTime<chrono::Utc>` -- Filter for receipts that were created after the specified date.
     * * `created_before: chrono::DateTime<chrono::Utc>` -- Filter for receipts that were created before the specified date.
     * * `start: uuid::Uuid` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_page(
        &self,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        created_after: Option<chrono::DateTime<chrono::Utc>>,
        created_before: Option<chrono::DateTime<chrono::Utc>>,
        start: uuid::Uuid,
        page_size: f64,
    ) -> Result<Vec<crate::types::Receipt>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = created_after {
            query_args.push(("created_after".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = created_before {
            query_args.push(("created_before".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = from_date {
            query_args.push(("from_date".to_string(), date.to_rfc3339()));
        }
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if start.to_string() != uuid::Uuid::nil().to_string() {
            query_args.push(("start".to_string(), start.to_string()));
        }
        if let Some(date) = to_date {
            query_args.push(("to_date".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/receipts?{}", query_);

        let resp: crate::types::GetReceiptsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * List receipts.
     *
     * This function performs a `GET` to the `/receipts` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Returns description of all receipts of a business.
     */
    pub async fn get_all(
        &self,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        created_after: Option<chrono::DateTime<chrono::Utc>>,
        created_before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<crate::types::Receipt>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = created_after {
            query_args.push(("created_after".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = created_before {
            query_args.push(("created_before".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = from_date {
            query_args.push(("from_date".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = to_date {
            query_args.push(("to_date".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/receipts?{}", query_);

        let resp: crate::types::GetReceiptsResponse = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut page = resp.page.next.to_string();

        // Paginate if we should.
        while !page.is_empty() {
            match self
                .client
                .get::<crate::types::GetReceiptsResponse>(
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
     * Get details for one receipt.
     *
     * This function performs a `GET` to the `/receipts/{id}` endpoint.
     *
     *
     */
    pub async fn get(&self, id: &str) -> Result<crate::types::Receipt> {
        let url = format!(
            "/receipts/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
