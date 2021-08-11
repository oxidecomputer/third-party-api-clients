use anyhow::Result;

use crate::Client;

pub struct Reimbursement {
    client: Client,
}

impl Reimbursement {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Reimbursement {
            client,
        }
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
pub async fn get_reimbursements(
&self,
start: &str, page_size: f64,
) -> Result<crate::types::GetReimbursementsResponse> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
query_args.push(format!("page_size={}", page_size));
if !start.is_empty() { query_args.push(format!("start={}", start)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/reimbursements?{}",
query);

self.client.get(&url, None).await
}

/**
* Get details for one reimbursement.
*
* This function performs a `GET` to the `/reimbursements/<id>` endpoint.
*/
pub async fn get_reimbursement(
&self,
) -> Result<crate::types::GetReimbursementsResponseData> {
let url =
"/reimbursements/<id>".to_string();
self.client.get(&url, None).await
}


}