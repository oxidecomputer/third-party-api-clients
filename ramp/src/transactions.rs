use crate::Client;
use crate::ClientResult;

pub struct Transactions {
    pub client: Client,
}

impl Transactions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Transactions { client }
    }

    /**
     * List transactions.
     *
     * This function performs a `GET` to the `/transactions` endpoint.
     *
     * Retrieves all transactions for the business. This endpoint supports filtering and ordering. NOTE: only one ordering param is supported.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     * * `department_id: &str` -- The OAuth2 token header.
     * * `location_id: &str` -- The OAuth2 token header.
     * * `from_date: chrono::DateTime<chrono::Utc>`
     * * `to_date: chrono::DateTime<chrono::Utc>`
     * * `merchant_id: &str` -- The OAuth2 token header.
     * * `sk_category_id: &str` -- The OAuth2 token header.
     * * `order_by_date_desc: bool`
     * * `order_by_date_asc: bool`
     * * `order_by_amount_desc: bool`
     * * `order_by_amount_asc: bool`
     * * `state: &str` -- The OAuth2 token header.
     * * `min_amount: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     * * `max_amount: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     * * `requires_memo: bool` -- Filters for transactions which require a memo, but do not have one. This can only be set to true.
     */
    pub async fn get_page(
        &self,
        department_id: &str,
        location_id: &str,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        merchant_id: &str,
        sk_category_id: &str,
        order_by_date_desc: bool,
        order_by_date_asc: bool,
        order_by_amount_desc: bool,
        order_by_amount_asc: bool,
        state: &str,
        min_amount: f64,
        max_amount: f64,
        start: &str,
        page_size: f64,
        requires_memo: bool,
    ) -> ClientResult<Vec<crate::types::Data>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !department_id.is_empty() {
            query_args.push(("department_id".to_string(), department_id.to_string()));
        }
        if let Some(date) = from_date {
            query_args.push(("from_date".to_string(), date.to_rfc3339()));
        }
        if !location_id.is_empty() {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !max_amount.to_string().is_empty() {
            query_args.push(("max_amount".to_string(), max_amount.to_string()));
        }
        if !merchant_id.is_empty() {
            query_args.push(("merchant_id".to_string(), merchant_id.to_string()));
        }
        if !min_amount.to_string().is_empty() {
            query_args.push(("min_amount".to_string(), min_amount.to_string()));
        }
        if order_by_amount_asc {
            query_args.push((
                "order_by_amount_asc".to_string(),
                order_by_amount_asc.to_string(),
            ));
        }
        if order_by_amount_desc {
            query_args.push((
                "order_by_amount_desc".to_string(),
                order_by_amount_desc.to_string(),
            ));
        }
        if order_by_date_asc {
            query_args.push((
                "order_by_date_asc".to_string(),
                order_by_date_asc.to_string(),
            ));
        }
        if order_by_date_desc {
            query_args.push((
                "order_by_date_desc".to_string(),
                order_by_date_desc.to_string(),
            ));
        }
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if requires_memo {
            query_args.push(("requires_memo".to_string(), requires_memo.to_string()));
        }
        if !sk_category_id.is_empty() {
            query_args.push(("sk_category_id".to_string(), sk_category_id.to_string()));
        }
        if !start.is_empty() {
            query_args.push(("start".to_string(), start.to_string()));
        }
        if !state.is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if let Some(date) = to_date {
            query_args.push(("to_date".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/transactions?{}", query_), None);
        let resp: crate::types::GetTransactionResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }
    /**
     * List transactions.
     *
     * This function performs a `GET` to the `/transactions` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Retrieves all transactions for the business. This endpoint supports filtering and ordering. NOTE: only one ordering param is supported.
     */
    pub async fn get_all(
        &self,
        department_id: &str,
        location_id: &str,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        merchant_id: &str,
        sk_category_id: &str,
        order_by_date_desc: bool,
        order_by_date_asc: bool,
        order_by_amount_desc: bool,
        order_by_amount_asc: bool,
        state: &str,
        min_amount: f64,
        max_amount: f64,
        requires_memo: bool,
    ) -> ClientResult<Vec<crate::types::Data>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !department_id.is_empty() {
            query_args.push(("department_id".to_string(), department_id.to_string()));
        }
        if let Some(date) = from_date {
            query_args.push(("from_date".to_string(), date.to_rfc3339()));
        }
        if !location_id.is_empty() {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !max_amount.to_string().is_empty() {
            query_args.push(("max_amount".to_string(), max_amount.to_string()));
        }
        if !merchant_id.is_empty() {
            query_args.push(("merchant_id".to_string(), merchant_id.to_string()));
        }
        if !min_amount.to_string().is_empty() {
            query_args.push(("min_amount".to_string(), min_amount.to_string()));
        }
        if order_by_amount_asc {
            query_args.push((
                "order_by_amount_asc".to_string(),
                order_by_amount_asc.to_string(),
            ));
        }
        if order_by_amount_desc {
            query_args.push((
                "order_by_amount_desc".to_string(),
                order_by_amount_desc.to_string(),
            ));
        }
        if order_by_date_asc {
            query_args.push((
                "order_by_date_asc".to_string(),
                order_by_date_asc.to_string(),
            ));
        }
        if order_by_date_desc {
            query_args.push((
                "order_by_date_desc".to_string(),
                order_by_date_desc.to_string(),
            ));
        }
        if requires_memo {
            query_args.push(("requires_memo".to_string(), requires_memo.to_string()));
        }
        if !sk_category_id.is_empty() {
            query_args.push(("sk_category_id".to_string(), sk_category_id.to_string()));
        }
        if !state.is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if let Some(date) = to_date {
            query_args.push(("to_date".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/transactions?{}", query_), None);
        let resp: crate::types::GetTransactionResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut data = resp.data;
        let mut page = resp.page.next.to_string();

        // Paginate if we should.
        while !page.is_empty() {
            match self
                .client
                .get::<crate::types::GetTransactionResponse>(
                    page.trim_start_matches(&self.client.host),
                    crate::Message {
                        body: None,
                        content_type: None,
                    },
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
                        return Err(e);
                    }
                }
            }
        }

        // Return our response data.
        Ok(data)
    }
    /**
     * GET a transaction.
     *
     * This function performs a `GET` to the `/transactions/{id}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_resource(&self, id: &str) -> ClientResult<crate::types::Data> {
        let url = self.client.url(
            &format!(
                "/transactions/{}",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
