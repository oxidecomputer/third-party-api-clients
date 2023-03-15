use crate::Client;
use crate::ClientResult;

pub struct Transfers {
    pub client: Client,
}

impl Transfers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Transfers { client }
    }

    /**
     * This function performs a `GET` to the `/v1/transfers` endpoint.
     *
     * <p>Returns a list of existing transfers sent to connected accounts. The transfers are returned in sorted order, with the most recently created transfers appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str`
     * * `destination: &str` -- Only return transfers for the destination specified by this account ID.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `transfer_group: &str` -- Only return transfers with the specified transfer group.
     */
    pub async fn get_page(
        &self,
        _created: &str,
        destination: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        transfer_group: &str,
    ) -> ClientResult<Vec<crate::types::Transfer>> {
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
        if !transfer_group.is_empty() {
            query_args.push(("transfer_group".to_string(), transfer_group.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/transfers?{}", query_), None);
        let resp: crate::types::TransferList = self
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
     * This function performs a `GET` to the `/v1/transfers` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of existing transfers sent to connected accounts. The transfers are returned in sorted order, with the most recently created transfers appearing first.</p>
     */
    pub async fn get_all(
        &self,
        _created: &str,
        destination: &str,
        transfer_group: &str,
    ) -> ClientResult<Vec<crate::types::Transfer>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !destination.is_empty() {
            query_args.push(("destination".to_string(), destination.to_string()));
        }
        if !transfer_group.is_empty() {
            query_args.push(("transfer_group".to_string(), transfer_group.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/transfers?{}", query_), None);
        let mut resp: crate::types::TransferList = self
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
     * This function performs a `POST` to the `/v1/transfers` endpoint.
     *
     * <p>To send funds from your Stripe account to a connected account, you create a new transfer object. Your <a href="#balance">Stripe balance</a> must be able to cover the transfer amount, or you’ll receive an “Insufficient Funds” error.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Transfer> {
        let url = self.client.url("/v1/transfers", None);
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
     * This function performs a `GET` to the `/v1/transfers/{id}/reversals` endpoint.
     *
     * <p>You can see a list of the reversals belonging to a specific transfer. Note that the 10 most recent reversals are always available by default on the transfer object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional reversals.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_reversals(
        &self,
        ending_before: &str,
        id: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::TransferReversal>> {
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
                "/v1/transfers/{}/reversals?{}",
                crate::progenitor_support::encode_path(id),
                query_
            ),
            None,
        );
        let resp: crate::types::Reversals = self
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
     * This function performs a `GET` to the `/v1/transfers/{id}/reversals` endpoint.
     *
     * As opposed to `get_reversals`, this function returns all the pages of the request at once.
     *
     * <p>You can see a list of the reversals belonging to a specific transfer. Note that the 10 most recent reversals are always available by default on the transfer object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional reversals.</p>
     */
    pub async fn get_all_reversals(
        &self,
        id: &str,
    ) -> ClientResult<Vec<crate::types::TransferReversal>> {
        let url = self.client.url(
            &format!(
                "/v1/transfers/{}/reversals",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        let mut resp: crate::types::Reversals = self
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
     * This function performs a `POST` to the `/v1/transfers/{id}/reversals` endpoint.
     *
     * <p>When you create a new reversal, you must specify a transfer to create it on.</p>
     *
     * <p>When reversing transfers, you can optionally reverse part of the transfer. You can do so as many times as you wish until the entire transfer has been reversed.</p>
     *
     * <p>Once entirely reversed, a transfer can’t be reversed again. This method will return an error when called on an already-reversed transfer, or when trying to reverse more money than is left on a transfer.</p>
     *
     * **Parameters:**
     *
     * * `id: &str` -- The account's country.
     */
    pub async fn post_reversal(&self, id: &str) -> ClientResult<crate::types::TransferReversal> {
        let url = self.client.url(
            &format!(
                "/v1/transfers/{}/reversals",
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
     * This function performs a `GET` to the `/v1/transfers/{transfer}` endpoint.
     *
     * <p>Retrieves the details of an existing transfer. Supply the unique transfer ID from either a transfer creation request or the transfer list, and Stripe will return the corresponding transfer information.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `transfer: &str` -- The account's country.
     */
    pub async fn get(&self, transfer: &str) -> ClientResult<crate::types::Transfer> {
        let url = self.client.url(
            &format!(
                "/v1/transfers/{}",
                crate::progenitor_support::encode_path(transfer),
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
     * This function performs a `POST` to the `/v1/transfers/{transfer}` endpoint.
     *
     * <p>Updates the specified transfer by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * <p>This request accepts only metadata as an argument.</p>
     *
     * **Parameters:**
     *
     * * `transfer: &str` -- The account's country.
     */
    pub async fn post_transfers(&self, transfer: &str) -> ClientResult<crate::types::Transfer> {
        let url = self.client.url(
            &format!(
                "/v1/transfers/{}",
                crate::progenitor_support::encode_path(transfer),
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
     * This function performs a `GET` to the `/v1/transfers/{transfer}/reversals/{id}` endpoint.
     *
     * <p>By default, you can see the 10 most recent reversals stored directly on the transfer object, but you can also retrieve details about a specific reversal stored on the transfer.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     * * `transfer: &str` -- The account's country.
     */
    pub async fn get_reversal(
        &self,
        id: &str,
        transfer: &str,
    ) -> ClientResult<crate::types::TransferReversal> {
        let url = self.client.url(
            &format!(
                "/v1/transfers/{}/reversals/{}",
                crate::progenitor_support::encode_path(transfer),
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
     * This function performs a `POST` to the `/v1/transfers/{transfer}/reversals/{id}` endpoint.
     *
     * <p>Updates the specified reversal by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * <p>This request only accepts metadata and description as arguments.</p>
     *
     * **Parameters:**
     *
     * * `id: &str` -- The account's country.
     * * `transfer: &str` -- The account's country.
     */
    pub async fn post_reversal_transfers(
        &self,
        id: &str,
        transfer: &str,
    ) -> ClientResult<crate::types::TransferReversal> {
        let url = self.client.url(
            &format!(
                "/v1/transfers/{}/reversals/{}",
                crate::progenitor_support::encode_path(transfer),
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
