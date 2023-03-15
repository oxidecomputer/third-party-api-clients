use crate::Client;
use crate::ClientResult;

pub struct Topups {
    pub client: Client,
}

impl Topups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Topups { client }
    }

    /**
     * This function performs a `GET` to the `/v1/topups` endpoint.
     *
     * <p>Returns a list of top-ups.</p>
     *
     * **Parameters:**
     *
     * * `amount: &str` -- A positive integer representing how much to transfer.
     * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::GetTopupsStatus` -- Only return top-ups that have the given status. One of `canceled`, `failed`, `pending` or `succeeded`.
     */
    pub async fn get_page(
        &self,
        _amount: &str,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        status: crate::types::GetTopupsStatus,
    ) -> ClientResult<Vec<crate::types::Topup>> {
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
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/topups?{}", query_), None);
        let resp: crate::types::TopupList = self
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
     * This function performs a `GET` to the `/v1/topups` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of top-ups.</p>
     */
    pub async fn get_all(
        &self,
        _amount: &str,
        _created: &str,
        status: crate::types::GetTopupsStatus,
    ) -> ClientResult<Vec<crate::types::Topup>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/topups?{}", query_), None);
        let mut resp: crate::types::TopupList = self
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
     * This function performs a `POST` to the `/v1/topups` endpoint.
     *
     * <p>Top up the balance of an account</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Topup> {
        let url = self.client.url("/v1/topups", None);
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
     * This function performs a `GET` to the `/v1/topups/{topup}` endpoint.
     *
     * <p>Retrieves the details of a top-up that has previously been created. Supply the unique top-up ID that was returned from your previous request, and Stripe will return the corresponding top-up information.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `topup: &str` -- The account's country.
     */
    pub async fn get(&self, topup: &str) -> ClientResult<crate::types::Topup> {
        let url = self.client.url(
            &format!(
                "/v1/topups/{}",
                crate::progenitor_support::encode_path(topup),
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
     * This function performs a `POST` to the `/v1/topups/{topup}` endpoint.
     *
     * <p>Updates the metadata of a top-up. Other top-up details are not editable by design.</p>
     *
     * **Parameters:**
     *
     * * `topup: &str` -- The account's country.
     */
    pub async fn post_topups(&self, topup: &str) -> ClientResult<crate::types::Topup> {
        let url = self.client.url(
            &format!(
                "/v1/topups/{}",
                crate::progenitor_support::encode_path(topup),
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
     * This function performs a `POST` to the `/v1/topups/{topup}/cancel` endpoint.
     *
     * <p>Cancels a top-up. Only pending top-ups can be canceled.</p>
     *
     * **Parameters:**
     *
     * * `topup: &str` -- The account's country.
     */
    pub async fn post_cancel(&self, topup: &str) -> ClientResult<crate::types::Topup> {
        let url = self.client.url(
            &format!(
                "/v1/topups/{}/cancel",
                crate::progenitor_support::encode_path(topup),
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
