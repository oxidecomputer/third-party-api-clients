use crate::Client;
use crate::ClientResult;

pub struct Invoiceitems {
    pub client: Client,
}

impl Invoiceitems {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Invoiceitems { client }
    }

    /**
     * This function performs a `GET` to the `/v1/invoiceitems` endpoint.
     *
     * <p>Returns a list of your invoice items. Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str`
     * * `customer: &str` -- The identifier of the customer whose invoice items to return. If none is provided, all invoice items will be returned.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `invoice: &str` -- Only return invoice items belonging to this invoice. If none is provided, all invoice items will be returned. If specifying an invoice, no customer identifier is needed.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `pending: bool` -- Set to `true` to only show pending invoice items, which are not yet attached to any invoices. Set to `false` to only show invoice items already attached to invoices. If unspecified, no filter is applied.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        _created: &str,
        customer: &str,
        ending_before: &str,
        invoice: &str,
        limit: i64,
        pending: bool,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::InvoiceItem>> {
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
        if pending {
            query_args.push(("pending".to_string(), pending.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/invoiceitems?{}", query_), None);
        let resp: crate::types::GetInvoiceitemsResponse = self
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
     * This function performs a `GET` to the `/v1/invoiceitems` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your invoice items. Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.</p>
     */
    pub async fn get_all(
        &self,
        _created: &str,
        customer: &str,
        invoice: &str,
        pending: bool,
    ) -> ClientResult<Vec<crate::types::InvoiceItem>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        if pending {
            query_args.push(("pending".to_string(), pending.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/invoiceitems?{}", query_), None);
        let mut resp: crate::types::GetInvoiceitemsResponse = self
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
     * This function performs a `POST` to the `/v1/invoiceitems` endpoint.
     *
     * <p>Creates an item to be added to a draft invoice (up to 250 items per invoice). If no invoice is specified, the item will be on the next invoice created for the customer specified.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::InvoiceItem> {
        let url = self.client.url("/v1/invoiceitems", None);
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
     * This function performs a `GET` to the `/v1/invoiceitems/{invoiceitem}` endpoint.
     *
     * <p>Retrieves the invoice item with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `invoiceitem: &str` -- The account's country.
     */
    pub async fn get(&self, invoiceitem: &str) -> ClientResult<crate::types::InvoiceItem> {
        let url = self.client.url(
            &format!(
                "/v1/invoiceitems/{}",
                crate::progenitor_support::encode_path(invoiceitem),
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
     * This function performs a `POST` to the `/v1/invoiceitems/{invoiceitem}` endpoint.
     *
     * <p>Updates the amount or description of an invoice item on an upcoming invoice. Updating an invoice item is only possible before the invoice it’s attached to is closed.</p>
     *
     * **Parameters:**
     *
     * * `invoiceitem: &str` -- The account's country.
     */
    pub async fn post_invoiceitems(
        &self,
        invoiceitem: &str,
    ) -> ClientResult<crate::types::InvoiceItem> {
        let url = self.client.url(
            &format!(
                "/v1/invoiceitems/{}",
                crate::progenitor_support::encode_path(invoiceitem),
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
     * This function performs a `DELETE` to the `/v1/invoiceitems/{invoiceitem}` endpoint.
     *
     * <p>Deletes an invoice item, removing it from an invoice. Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.</p>
     *
     * **Parameters:**
     *
     * * `invoiceitem: &str` -- The account's country.
     */
    pub async fn delete(
        &self,
        invoiceitem: &str,
    ) -> ClientResult<crate::types::DeletedInvoiceItem> {
        let url = self.client.url(
            &format!(
                "/v1/invoiceitems/{}",
                crate::progenitor_support::encode_path(invoiceitem),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
