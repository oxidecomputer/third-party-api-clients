use crate::Client;
use crate::ClientResult;

pub struct Payouts {
    pub client: Client,
}

impl Payouts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Payouts { client }
    }

    /**
     * This function performs a `GET` to the `/v1/payouts` endpoint.
     *
     * <p>Returns a list of existing payouts sent to third-party bank accounts or that Stripe has sent you. The payouts are returned in sorted order, with the most recently created payouts appearing first.</p>
     *
     * **Parameters:**
     *
     * * `arrival_date: &str`
     * * `created: &str`
     * * `destination: &str` -- The ID of an external account - only return payouts sent to this external account.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: &str` -- Only return payouts that have the given status: `pending`, `paid`, `failed`, or `canceled`.
     */
    pub async fn get_page(
        &self,
        _arrival_date: &str,
        _created: &str,
        destination: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        status: &str,
    ) -> ClientResult<Vec<crate::types::Payout>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !destination.is_empty() {
            query_args.push(("destination".to_string(), destination.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/payouts?{}", query_), None);
        let resp: crate::types::PayoutList = self
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
     * This function performs a `GET` to the `/v1/payouts` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of existing payouts sent to third-party bank accounts or that Stripe has sent you. The payouts are returned in sorted order, with the most recently created payouts appearing first.</p>
     */
    pub async fn get_all(
        &self,
        _arrival_date: &str,
        _created: &str,
        destination: &str,
        status: &str,
    ) -> ClientResult<Vec<crate::types::Payout>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !destination.is_empty() {
            query_args.push(("destination".to_string(), destination.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/payouts?{}", query_), None);
        let mut resp: crate::types::PayoutList = self
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
     * This function performs a `POST` to the `/v1/payouts` endpoint.
     *
     * <p>To send funds to your own bank account, you create a new payout object. Your <a href="#balance">Stripe balance</a> must be able to cover the payout amount, or you’ll receive an “Insufficient Funds” error.</p>
     *
     * <p>If your API key is in test mode, money won’t actually be sent, though everything else will occur as if in live mode.</p>
     *
     * <p>If you are creating a manual payout on a Stripe account that uses multiple payment source types, you’ll need to specify the source type balance that the payout should draw from. The <a href="#balance_object">balance object</a> details available and pending amounts by source type.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Payout> {
        let url = self.client.url("/v1/payouts", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v1/payouts/{payout}` endpoint.
     *
     * <p>Retrieves the details of an existing payout. Supply the unique payout ID from either a payout creation request or the payout list, and Stripe will return the corresponding payout information.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `payout: &str` -- The account's country.
     */
    pub async fn get(&self, payout: &str) -> ClientResult<crate::types::Payout> {
        let url = self.client.url(
            &format!(
                "/v1/payouts/{}",
                crate::progenitor_support::encode_path(payout),
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
    /**
     * This function performs a `POST` to the `/v1/payouts/{payout}` endpoint.
     *
     * <p>Updates the specified payout by setting the values of the parameters passed. Any parameters not provided will be left unchanged. This request accepts only the metadata as arguments.</p>
     *
     * **Parameters:**
     *
     * * `payout: &str` -- The account's country.
     */
    pub async fn post_payouts(&self, payout: &str) -> ClientResult<crate::types::Payout> {
        let url = self.client.url(
            &format!(
                "/v1/payouts/{}",
                crate::progenitor_support::encode_path(payout),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v1/payouts/{payout}/cancel` endpoint.
     *
     * <p>A previously created payout can be canceled if it has not yet been paid out. Funds will be refunded to your available balance. You may not cancel automatic Stripe payouts.</p>
     *
     * **Parameters:**
     *
     * * `payout: &str` -- The account's country.
     */
    pub async fn post_cancel(&self, payout: &str) -> ClientResult<crate::types::Payout> {
        let url = self.client.url(
            &format!(
                "/v1/payouts/{}/cancel",
                crate::progenitor_support::encode_path(payout),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v1/payouts/{payout}/reverse` endpoint.
     *
     * <p>Reverses a payout by debiting the destination bank account. Only payouts for connected accounts to US bank accounts may be reversed at this time. If the payout is in the <code>pending</code> status, <code>/v1/payouts/:id/cancel</code> should be used instead.</p>
     *
     * <p>By requesting a reversal via <code>/v1/payouts/:id/reverse</code>, you confirm that the authorized signatory of the selected bank account has authorized the debit on the bank account and that no other authorization is required.</p>
     *
     * **Parameters:**
     *
     * * `payout: &str` -- The account's country.
     */
    pub async fn post_reverse(&self, payout: &str) -> ClientResult<crate::types::Payout> {
        let url = self.client.url(
            &format!(
                "/v1/payouts/{}/reverse",
                crate::progenitor_support::encode_path(payout),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
