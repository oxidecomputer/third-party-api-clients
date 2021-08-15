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
    pub async fn get_page(
        &self,
        start: &str,
        page_size: f64,
    ) -> Result<Vec<crate::types::Reimbursement>> {
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
        let url = format!("/reimbursements?{}", query_);

        let resp: crate::types::GetReimbursementsResponse =
            self.client.get(&url, None).await.unwrap();

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
        let mut resp: crate::types::GetReimbursementsResponse =
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
