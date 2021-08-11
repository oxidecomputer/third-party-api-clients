use anyhow::Result;

use crate::Client;

pub struct Receipt {
    client: Client,
}

impl Receipt {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Receipt { client }
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
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_receipts(
        &self,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        created_after: Option<chrono::DateTime<chrono::Utc>>,
        created_before: Option<chrono::DateTime<chrono::Utc>>,
        start: &str,
        page_size: f64,
    ) -> Result<crate::types::GetReceiptsResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if let Some(date) = created_after {
            query_args.push(format!("created_after={}", &date.to_rfc3339()));
        }
        if let Some(date) = created_before {
            query_args.push(format!("created_before={}", &date.to_rfc3339()));
        }
        if let Some(date) = from_date {
            query_args.push(format!("from_date={}", &date.to_rfc3339()));
        }
        query_args.push(format!("page_size={}", page_size));
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        if let Some(date) = to_date {
            query_args.push(format!("to_date={}", &date.to_rfc3339()));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/receipts?{}", query);

        self.client.get(&url, None).await
    }

    /**
     * Get details for one receipt.
     *
     * This function performs a `GET` to the `/receipts/<id>` endpoint.
     *
     *
     */
    pub async fn get_receipts_receipt_id(&self) -> Result<crate::types::Receipt> {
        let url = "/receipts/<id>".to_string();
        self.client.get(&url, None).await
    }
}
