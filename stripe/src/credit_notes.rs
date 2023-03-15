use crate::Client;
use crate::ClientResult;

pub struct CreditNotes {
    pub client: Client,
}

impl CreditNotes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CreditNotes { client }
    }

    /**
     * This function performs a `GET` to the `/v1/credit_notes` endpoint.
     *
     * <p>Returns a list of credit notes.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Only return credit notes for the customer specified by this customer ID.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `invoice: &str` -- Only return credit notes for the invoice specified by this invoice ID.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        customer: &str,
        ending_before: &str,
        invoice: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::CreditNote>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/credit_notes?{}", query_), None);
        let resp: crate::types::CreditNotesList = self
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
     * This function performs a `GET` to the `/v1/credit_notes` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of credit notes.</p>
     */
    pub async fn get_all(
        &self,
        customer: &str,
        invoice: &str,
    ) -> ClientResult<Vec<crate::types::CreditNote>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/credit_notes?{}", query_), None);
        let mut resp: crate::types::CreditNotesList = self
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
     * This function performs a `POST` to the `/v1/credit_notes` endpoint.
     *
     * <p>Issue a credit note to adjust the amount of a finalized invoice. For a <code>status=open</code> invoice, a credit note reduces
     * its <code>amount_due</code>. For a <code>status=paid</code> invoice, a credit note does not affect its <code>amount_due</code>. Instead, it can result
     * in any combination of the following:</p>
     *
     * <ul>
     * <li>Refund: create a new refund (using <code>refund_amount</code>) or link an existing refund (using <code>refund</code>).</li>
     * <li>Customer balance credit: credit the customer’s balance (using <code>credit_amount</code>) which will be automatically applied to their next invoice when it’s finalized.</li>
     * <li>Outside of Stripe credit: record the amount that is or will be credited outside of Stripe (using <code>out_of_band_amount</code>).</li>
     * </ul>
     *
     * <p>For post-payment credit notes the sum of the refund, credit and outside of Stripe amounts must equal the credit note total.</p>
     *
     * <p>You may issue multiple credit notes for an invoice. Each credit note will increment the invoice’s <code>pre_payment_credit_notes_amount</code>
     * or <code>post_payment_credit_notes_amount</code> depending on its <code>status</code> at the time of credit note creation.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::CreditNote> {
        let url = self.client.url("/v1/credit_notes", None);
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
     * This function performs a `GET` to the `/v1/credit_notes/preview` endpoint.
     *
     * <p>Get a preview of a credit note without creating it.</p>
     *
     * **Parameters:**
     *
     * * `amount: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
     * * `credit_amount: i64` -- The integer amount in %s representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `invoice: &str` -- The account's country.
     * * `lines: &[String]` -- Line items that make up the credit note.
     * * `memo: &str` -- The credit note's memo appears on the credit note PDF.
     * * `metadata: &str` -- Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format. Individual keys can be unset by posting an empty value to them. All keys can be unset by posting an empty value to `metadata`.
     * * `out_of_band_amount: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
     * * `reason: crate::types::Reason` -- Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
     * * `refund: &str` -- ID of an existing refund to link this credit note to.
     * * `refund_amount: i64` -- The integer amount in %s representing the amount to refund. If set, a refund will be created for the charge associated with the invoice.
     */
    pub async fn get_preview(
        &self,
        amount: i64,
        credit_amount: i64,
        invoice: &str,
        _lines: &[String],
        memo: &str,
        _metadata: &str,
        out_of_band_amount: i64,
        reason: crate::types::Reason,
        refund: &str,
        refund_amount: i64,
    ) -> ClientResult<crate::types::CreditNote> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if amount > 0 {
            query_args.push(("amount".to_string(), amount.to_string()));
        }
        if credit_amount > 0 {
            query_args.push(("credit_amount".to_string(), credit_amount.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        if !memo.is_empty() {
            query_args.push(("memo".to_string(), memo.to_string()));
        }
        if out_of_band_amount > 0 {
            query_args.push((
                "out_of_band_amount".to_string(),
                out_of_band_amount.to_string(),
            ));
        }
        if !reason.to_string().is_empty() {
            query_args.push(("reason".to_string(), reason.to_string()));
        }
        if !refund.is_empty() {
            query_args.push(("refund".to_string(), refund.to_string()));
        }
        if refund_amount > 0 {
            query_args.push(("refund_amount".to_string(), refund_amount.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/credit_notes/preview?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/credit_notes/preview/lines` endpoint.
     *
     * <p>When retrieving a credit note preview, you’ll get a <strong>lines</strong> property containing the first handful of those items. This URL you can retrieve the full (paginated) list of line items.</p>
     *
     * **Parameters:**
     *
     * * `amount: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
     * * `credit_amount: i64` -- The integer amount in %s representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `invoice: &str` -- The account's country.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `lines: &[String]` -- Line items that make up the credit note.
     * * `memo: &str` -- The credit note's memo appears on the credit note PDF.
     * * `metadata: &str` -- Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format. Individual keys can be unset by posting an empty value to them. All keys can be unset by posting an empty value to `metadata`.
     * * `out_of_band_amount: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
     * * `reason: crate::types::Reason` -- Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
     * * `refund: &str` -- ID of an existing refund to link this credit note to.
     * * `refund_amount: i64` -- The integer amount in %s representing the amount to refund. If set, a refund will be created for the charge associated with the invoice.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_preview_lines(
        &self,
        amount: i64,
        credit_amount: i64,
        ending_before: &str,
        invoice: &str,
        limit: i64,
        _lines: &[String],
        memo: &str,
        _metadata: &str,
        out_of_band_amount: i64,
        reason: crate::types::Reason,
        refund: &str,
        refund_amount: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::CreditNoteLineItem>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if amount > 0 {
            query_args.push(("amount".to_string(), amount.to_string()));
        }
        if credit_amount > 0 {
            query_args.push(("credit_amount".to_string(), credit_amount.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !memo.is_empty() {
            query_args.push(("memo".to_string(), memo.to_string()));
        }
        if out_of_band_amount > 0 {
            query_args.push((
                "out_of_band_amount".to_string(),
                out_of_band_amount.to_string(),
            ));
        }
        if !reason.to_string().is_empty() {
            query_args.push(("reason".to_string(), reason.to_string()));
        }
        if !refund.is_empty() {
            query_args.push(("refund".to_string(), refund.to_string()));
        }
        if refund_amount > 0 {
            query_args.push(("refund_amount".to_string(), refund_amount.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/credit_notes/preview/lines?{}", query_), None);
        let resp: crate::types::Lines = self
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
     * This function performs a `GET` to the `/v1/credit_notes/preview/lines` endpoint.
     *
     * As opposed to `get_preview_lines`, this function returns all the pages of the request at once.
     *
     * <p>When retrieving a credit note preview, you’ll get a <strong>lines</strong> property containing the first handful of those items. This URL you can retrieve the full (paginated) list of line items.</p>
     */
    pub async fn get_all_preview_lines(
        &self,
        amount: i64,
        credit_amount: i64,
        invoice: &str,
        _lines: &[String],
        memo: &str,
        _metadata: &str,
        out_of_band_amount: i64,
        reason: crate::types::Reason,
        refund: &str,
        refund_amount: i64,
    ) -> ClientResult<Vec<crate::types::CreditNoteLineItem>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if amount > 0 {
            query_args.push(("amount".to_string(), amount.to_string()));
        }
        if credit_amount > 0 {
            query_args.push(("credit_amount".to_string(), credit_amount.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        if !memo.is_empty() {
            query_args.push(("memo".to_string(), memo.to_string()));
        }
        if out_of_band_amount > 0 {
            query_args.push((
                "out_of_band_amount".to_string(),
                out_of_band_amount.to_string(),
            ));
        }
        if !reason.to_string().is_empty() {
            query_args.push(("reason".to_string(), reason.to_string()));
        }
        if !refund.is_empty() {
            query_args.push(("refund".to_string(), refund.to_string()));
        }
        if refund_amount > 0 {
            query_args.push(("refund_amount".to_string(), refund_amount.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/credit_notes/preview/lines?{}", query_), None);
        let mut resp: crate::types::Lines = self
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
     * This function performs a `GET` to the `/v1/credit_notes/{credit_note}/lines` endpoint.
     *
     * <p>When retrieving a credit note, you’ll get a <strong>lines</strong> property containing the the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     *
     * **Parameters:**
     *
     * * `credit_note: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_note_lines(
        &self,
        credit_note: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::CreditNoteLineItem>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/credit_notes/{}/lines?{}",
                crate::progenitor_support::encode_path(credit_note),
                query_
            ),
            None,
        );
        let resp: crate::types::Lines = self
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
     * This function performs a `GET` to the `/v1/credit_notes/{credit_note}/lines` endpoint.
     *
     * As opposed to `get_note_lines`, this function returns all the pages of the request at once.
     *
     * <p>When retrieving a credit note, you’ll get a <strong>lines</strong> property containing the the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     */
    pub async fn get_all_note_lines(
        &self,
        credit_note: &str,
    ) -> ClientResult<Vec<crate::types::CreditNoteLineItem>> {
        let url = self.client.url(
            &format!(
                "/v1/credit_notes/{}/lines",
                crate::progenitor_support::encode_path(credit_note),
            ),
            None,
        );
        let mut resp: crate::types::Lines = self
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
     * This function performs a `GET` to the `/v1/credit_notes/{id}` endpoint.
     *
     * <p>Retrieves the credit note object with the given identifier.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     */
    pub async fn get(&self, id: &str) -> ClientResult<crate::types::CreditNote> {
        let url = self.client.url(
            &format!(
                "/v1/credit_notes/{}",
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
    /**
     * This function performs a `POST` to the `/v1/credit_notes/{id}` endpoint.
     *
     * <p>Updates an existing credit note.</p>
     *
     * **Parameters:**
     *
     * * `id: &str` -- The account's country.
     */
    pub async fn post_credit_notes(&self, id: &str) -> ClientResult<crate::types::CreditNote> {
        let url = self.client.url(
            &format!(
                "/v1/credit_notes/{}",
                crate::progenitor_support::encode_path(id),
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
     * This function performs a `POST` to the `/v1/credit_notes/{id}/void` endpoint.
     *
     * <p>Marks a credit note as void. Learn more about <a href="/docs/billing/invoices/credit-notes#voiding">voiding credit notes</a>.</p>
     *
     * **Parameters:**
     *
     * * `id: &str` -- The account's country.
     */
    pub async fn post_void(&self, id: &str) -> ClientResult<crate::types::CreditNote> {
        let url = self.client.url(
            &format!(
                "/v1/credit_notes/{}/void",
                crate::progenitor_support::encode_path(id),
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
