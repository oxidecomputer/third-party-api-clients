use anyhow::Result;

use crate::Client;

pub struct Reimbursements {
    client: Client,
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
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_reimbursement(
        &self,
        start: &str,
        page_size: f64,
    ) -> Result<Option<crate::types::GetResponse>> {
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
        let url = format!("/reimbursements?{}", query);

        let resp: crate::types::GetReimbursementsResponse =
            self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * Get details for one reimbursement.
     *
     * This function performs a `GET` to the `/reimbursements/<id>` endpoint.
     */
    pub async fn get(&self) -> Result<crate::types::GetResponse> {
        let url = "/reimbursements/<id>".to_string();
        self.client.get(&url, None).await
    }
}
