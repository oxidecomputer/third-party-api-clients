use anyhow::Result;

use crate::Client;

pub struct Transactions {
    client: Client,
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
     * * `department_id: &str`
     * * `location_id: &str`
     * * `from_date: chrono::DateTime<chrono::Utc>`
     * * `to_date: chrono::DateTime<chrono::Utc>`
     * * `merchant_id: &str`
     * * `sk_category_id: &str`
     * * `order_by_date_desc: bool`
     * * `order_by_date_asc: bool`
     * * `order_by_amount_desc: bool`
     * * `order_by_amount_asc: bool`
     * * `state: &str`
     * * `min_amount: f64`
     * * `max_amount: f64`
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     * * `requires_memo: bool` -- Filters for transactions which require a memo, but do not have one. This can only be set to true.
     */
    pub async fn get_transactions(
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
    ) -> Result<Vec<crate::types::Data>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !department_id.is_empty() {
            query_args.push(format!("department_id={}", department_id));
        }
        if let Some(date) = from_date {
            query_args.push(format!("from_date={}", &date.to_rfc3339()));
        }
        if !location_id.is_empty() {
            query_args.push(format!("location_id={}", location_id));
        }
        query_args.push(format!("max_amount={}", max_amount));
        if !merchant_id.is_empty() {
            query_args.push(format!("merchant_id={}", merchant_id));
        }
        query_args.push(format!("min_amount={}", min_amount));
        if order_by_amount_asc {
            query_args.push(format!("order_by_amount_asc={}", order_by_amount_asc));
        }
        if order_by_amount_desc {
            query_args.push(format!("order_by_amount_desc={}", order_by_amount_desc));
        }
        if order_by_date_asc {
            query_args.push(format!("order_by_date_asc={}", order_by_date_asc));
        }
        if order_by_date_desc {
            query_args.push(format!("order_by_date_desc={}", order_by_date_desc));
        }
        query_args.push(format!("page_size={}", page_size));
        if requires_memo {
            query_args.push(format!("requires_memo={}", requires_memo));
        }
        if !sk_category_id.is_empty() {
            query_args.push(format!("sk_category_id={}", sk_category_id));
        }
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        if !state.is_empty() {
            query_args.push(format!("state={}", state));
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
        let url = format!("/transactions?{}", query);

        let resp: crate::types::GetTransactionResponse = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * GET a transaction.
     *
     * This function performs a `GET` to the `/transactions/<id>` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_resources_transaction(&self) -> Result<crate::types::Data> {
        let url = "/transactions/<id>".to_string();
        self.client.get(&url, None).await
    }
}
