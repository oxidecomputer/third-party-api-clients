use crate::Client;
use crate::ClientResult;

pub struct Sources {
    pub client: Client,
}

impl Sources {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Sources { client }
    }

    /**
     * This function performs a `POST` to the `/v1/sources` endpoint.
     *
     * <p>Creates a new source object.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::SourceData> {
        let url = self.client.url("/v1/sources", None);
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
     * This function performs a `GET` to the `/v1/sources/{source}` endpoint.
     *
     * <p>Retrieves an existing source object. Supply the unique source ID from a source creation request and Stripe will return the corresponding up-to-date source object information.</p>
     *
     * **Parameters:**
     *
     * * `client_secret: &str` -- The client secret of the source. Required if a publishable key is used to retrieve the source.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `source: &str` -- The account's country.
     */
    pub async fn get(
        &self,
        client_secret: &str,
        source: &str,
    ) -> ClientResult<crate::types::SourceData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_secret.is_empty() {
            query_args.push(("client_secret".to_string(), client_secret.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/sources/{}?{}",
                crate::progenitor_support::encode_path(source),
                query_
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
     * This function performs a `POST` to the `/v1/sources/{source}` endpoint.
     *
     * <p>Updates the specified source by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * <p>This request accepts the <code>metadata</code> and <code>owner</code> as arguments. It is also possible to update type specific information for selected payment methods. Please refer to our <a href="/docs/sources">payment method guides</a> for more detail.</p>
     *
     * **Parameters:**
     *
     * * `source: &str` -- The account's country.
     */
    pub async fn post_sources(&self, source: &str) -> ClientResult<crate::types::SourceData> {
        let url = self.client.url(
            &format!(
                "/v1/sources/{}",
                crate::progenitor_support::encode_path(source),
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
     * This function performs a `GET` to the `/v1/sources/{source}/mandate_notifications/{mandate_notification}` endpoint.
     *
     * <p>Retrieves a new Source MandateNotification.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `mandate_notification: &str` -- The account's country.
     * * `source: &str` -- The account's country.
     */
    pub async fn get_mandate_notifications_notification(
        &self,
        mandate_notification: &str,
        source: &str,
    ) -> ClientResult<crate::types::SourceMandateNotification> {
        let url = self.client.url(
            &format!(
                "/v1/sources/{}/mandate_notifications/{}",
                crate::progenitor_support::encode_path(source),
                crate::progenitor_support::encode_path(mandate_notification),
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
     * This function performs a `GET` to the `/v1/sources/{source}/source_transactions` endpoint.
     *
     * <p>List source transactions for a given source.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `source: &str` -- The account's country.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_transactions(
        &self,
        ending_before: &str,
        limit: i64,
        source: &str,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::SourceTransaction>> {
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
                "/v1/sources/{}/source_transactions?{}",
                crate::progenitor_support::encode_path(source),
                query_
            ),
            None,
        );
        let resp: crate::types::ApmsSourcesSourceTransactionList = self
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
     * This function performs a `GET` to the `/v1/sources/{source}/source_transactions` endpoint.
     *
     * As opposed to `get_transactions`, this function returns all the pages of the request at once.
     *
     * <p>List source transactions for a given source.</p>
     */
    pub async fn get_all_transactions(
        &self,
        source: &str,
    ) -> ClientResult<Vec<crate::types::SourceTransaction>> {
        let url = self.client.url(
            &format!(
                "/v1/sources/{}/source_transactions",
                crate::progenitor_support::encode_path(source),
            ),
            None,
        );
        let mut resp: crate::types::ApmsSourcesSourceTransactionList = self
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
     * This function performs a `GET` to the `/v1/sources/{source}/source_transactions/{source_transaction}` endpoint.
     *
     * <p>Retrieve an existing source transaction object. Supply the unique source ID from a source creation request and the source transaction ID and Stripe will return the corresponding up-to-date source object information.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `source: &str` -- The account's country.
     * * `source_transaction: &str` -- The account's country.
     */
    pub async fn get_transactions_transaction(
        &self,
        source: &str,
        source_transaction: &str,
    ) -> ClientResult<crate::types::SourceTransaction> {
        let url = self.client.url(
            &format!(
                "/v1/sources/{}/source_transactions/{}",
                crate::progenitor_support::encode_path(source),
                crate::progenitor_support::encode_path(source_transaction),
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
     * This function performs a `POST` to the `/v1/sources/{source}/verify` endpoint.
     *
     * <p>Verify a given source.</p>
     *
     * **Parameters:**
     *
     * * `source: &str` -- The account's country.
     */
    pub async fn post_verify(&self, source: &str) -> ClientResult<crate::types::SourceData> {
        let url = self.client.url(
            &format!(
                "/v1/sources/{}/verify",
                crate::progenitor_support::encode_path(source),
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
