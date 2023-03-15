use crate::Client;
use crate::ClientResult;

pub struct Balance {
    pub client: Client,
}

impl Balance {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Balance { client }
    }

    /**
     * This function performs a `GET` to the `/v1/balance` endpoint.
     *
     * <p>Retrieves the current account balance, based on the authentication that was used to make the request.
     *  For a sample request, see <a href="/docs/connect/account-balances#accounting-for-negative-balances">Accounting for negative balances</a>.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get(&self) -> ClientResult<crate::types::Balance> {
        let url = self.client.url("/v1/balance", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v1/balance/history` endpoint.
     *
     * <p>Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth). The transactions are returned in sorted order, with the most recent transactions appearing first.</p>
     *
     * <p>Note that this endpoint was previously called “Balance history” and used the path <code>/v1/balance/history</code>.</p>
     *
     * **Parameters:**
     *
     * * `created: &str`
     * * `currency: &str` -- Only return transactions in a certain currency. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `payout: &str` -- For automatic Stripe payouts only, only returns transactions that were paid out on the specified payout ID.
     * * `source: &str` -- Only returns the original transaction.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `type_: &str` -- Only returns transactions of the given type. One of: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
     */
    pub async fn get_history(
        &self,
        _created: &str,
        currency: &str,
        ending_before: &str,
        limit: i64,
        payout: &str,
        source: &str,
        starting_after: &str,
        type_: &str,
    ) -> ClientResult<Vec<crate::types::BalanceTransaction>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payout.is_empty() {
            query_args.push(("payout".to_string(), payout.to_string()));
        }
        if !source.is_empty() {
            query_args.push(("source".to_string(), source.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/balance/history?{}", query_), None);
        let resp: crate::types::BalanceTransactionsList = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v1/balance/history` endpoint.
     *
     * As opposed to `get_history`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth). The transactions are returned in sorted order, with the most recent transactions appearing first.</p>
     *
     * <p>Note that this endpoint was previously called “Balance history” and used the path <code>/v1/balance/history</code>.</p>
     */
    pub async fn get_all_history(
        &self,
        _created: &str,
        currency: &str,
        payout: &str,
        source: &str,
        type_: &str,
    ) -> ClientResult<Vec<crate::types::BalanceTransaction>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !payout.is_empty() {
            query_args.push(("payout".to_string(), payout.to_string()));
        }
        if !source.is_empty() {
            query_args.push(("source".to_string(), source.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/balance/history?{}", query_), None);
        let mut resp: crate::types::BalanceTransactionsList = self
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
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(serde_json::Value::String(s)) = o.get("id") {
                        page = s.to_string();
                    }
                }
            }

            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?startng_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                resp = self
                    .client
                    .get(
                        &format!("{}&starting_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            data.append(&mut resp.data);

            has_more = resp.has_more;
        }

        // Return our response data.
        Ok(data.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v1/balance/history/{id}` endpoint.
     *
     * <p>Retrieves the balance transaction with the given ID.</p>
     *
     * <p>Note that this endpoint previously used the path <code>/v1/balance/history/:id</code>.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     */
    pub async fn get_history_balance(
        &self,
        id: &str,
    ) -> ClientResult<crate::types::BalanceTransaction> {
        let url = self.client.url(
            &format!(
                "/v1/balance/history/{}",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
