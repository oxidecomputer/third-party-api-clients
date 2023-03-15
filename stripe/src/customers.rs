use crate::Client;
use crate::ClientResult;

pub struct Customers {
    pub client: Client,
}

impl Customers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Customers { client }
    }

    /**
     * This function performs a `GET` to the `/v1/customers` endpoint.
     *
     * <p>Returns a list of your customers. The customers are returned sorted by creation date, with the most recent customers appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str`
     * * `email: &str` -- A case-sensitive filter on the list based on the customer's `email` field. The value must be a string.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `test_clock: &str` -- Provides a list of customers that are associated with the specified test clock. The response will not include customers with test clocks if this parameter is not set.
     */
    pub async fn get_page(
        &self,
        _created: &str,
        email: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        test_clock: &str,
    ) -> ClientResult<Vec<crate::types::Customer>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
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
        if !test_clock.is_empty() {
            query_args.push(("test_clock".to_string(), test_clock.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/customers?{}", query_), None);
        let resp: crate::types::GetCustomersResponse = self
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
     * This function performs a `GET` to the `/v1/customers` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your customers. The customers are returned sorted by creation date, with the most recent customers appearing first.</p>
     */
    pub async fn get_all(
        &self,
        _created: &str,
        email: &str,
        test_clock: &str,
    ) -> ClientResult<Vec<crate::types::Customer>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !test_clock.is_empty() {
            query_args.push(("test_clock".to_string(), test_clock.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/customers?{}", query_), None);
        let mut resp: crate::types::GetCustomersResponse = self
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
     * This function performs a `POST` to the `/v1/customers` endpoint.
     *
     * <p>Creates a new customer object.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Customer> {
        let url = self.client.url("/v1/customers", None);
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
     * This function performs a `GET` to the `/v1/customers/search` endpoint.
     *
     * <p>Search for customers you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
     * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
     * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
     * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
     * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for customers](https://stripe.com/docs/search#query-fields-for-customers).
     */
    pub async fn get_search(
        &self,
        limit: i64,
        page: &str,
        query: &str,
    ) -> ClientResult<Vec<crate::types::Charge>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/customers/search?{}", query_), None);
        let resp: crate::types::SearchResult = self
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
     * This function performs a `GET` to the `/v1/customers/search` endpoint.
     *
     * As opposed to `get_search`, this function returns all the pages of the request at once.
     *
     * <p>Search for customers you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
     * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
     * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
     * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
     */
    pub async fn get_all_search(&self, query: &str) -> ClientResult<Vec<crate::types::Charge>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/customers/search?{}", query_), None);
        let mut resp: crate::types::SearchResult = self
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
     * This function performs a `GET` to the `/v1/customers/{customer}` endpoint.
     *
     * <p>Retrieves a Customer object.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get(
        &self,
        customer: &str,
    ) -> ClientResult<crate::types::GetCustomersCustomerResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `POST` to the `/v1/customers/{customer}` endpoint.
     *
     * <p>Updates the specified customer by setting the values of the parameters passed. Any parameters not provided will be left unchanged. For example, if you pass the <strong>source</strong> parameter, that becomes the customer’s active source (e.g., a card) to be used for all charges in the future. When you update a customer to a new valid card source by passing the <strong>source</strong> parameter: for each of the customer’s current subscriptions, if the subscription bills automatically and is in the <code>past_due</code> state, then the latest open invoice for the subscription with automatic collection enabled will be retried. This retry will not count as an automatic retry, and will not affect the next regularly scheduled payment for the invoice. Changing the <strong>default_source</strong> for a customer will not trigger this behavior.</p>
     *
     * <p>This request accepts mostly the same arguments as the customer creation call.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     */
    pub async fn post_customers(&self, customer: &str) -> ClientResult<crate::types::Customer> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `DELETE` to the `/v1/customers/{customer}` endpoint.
     *
     * <p>Permanently deletes a customer. It cannot be undone. Also immediately cancels any active subscriptions on the customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     */
    pub async fn delete(&self, customer: &str) -> ClientResult<crate::types::DeletedCustomer> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}",
                crate::progenitor_support::encode_path(customer),
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
    /**
     * This function performs a `GET` to the `/v1/customers/{customer}/balance_transactions` endpoint.
     *
     * <p>Returns a list of transactions that updated the customer’s <a href="/docs/billing/customer/balance">balances</a>.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_balance_transactions(
        &self,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::CustomerBalanceTransaction>> {
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
                "/v1/customers/{}/balance_transactions?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::CustomerBalanceTransactionList = self
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
     * This function performs a `GET` to the `/v1/customers/{customer}/balance_transactions` endpoint.
     *
     * As opposed to `get_balance_transactions`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of transactions that updated the customer’s <a href="/docs/billing/customer/balance">balances</a>.</p>
     */
    pub async fn get_all_balance_transactions(
        &self,
        customer: &str,
    ) -> ClientResult<Vec<crate::types::CustomerBalanceTransaction>> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/balance_transactions",
                crate::progenitor_support::encode_path(customer),
            ),
            None,
        );
        let mut resp: crate::types::CustomerBalanceTransactionList = self
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
     * This function performs a `POST` to the `/v1/customers/{customer}/balance_transactions` endpoint.
     *
     * <p>Creates an immutable transaction that updates the customer’s credit <a href="/docs/billing/customer/balance">balance</a>.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     */
    pub async fn post_balance_transaction(
        &self,
        customer: &str,
    ) -> ClientResult<crate::types::CustomerBalanceTransaction> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/balance_transactions",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/v1/customers/{customer}/balance_transactions/{transaction}` endpoint.
     *
     * <p>Retrieves a specific customer balance transaction that updated the customer’s <a href="/docs/billing/customer/balance">balances</a>.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `transaction: &str` -- The account's country.
     */
    pub async fn get_balance_transactions_transaction(
        &self,
        customer: &str,
        transaction: &str,
    ) -> ClientResult<crate::types::CustomerBalanceTransaction> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/balance_transactions/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(transaction),
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
     * This function performs a `POST` to the `/v1/customers/{customer}/balance_transactions/{transaction}` endpoint.
     *
     * <p>Most credit balance transaction fields are immutable, but you may update its <code>description</code> and <code>metadata</code>.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `transaction: &str` -- The account's country.
     */
    pub async fn post_balance_transactions_transaction(
        &self,
        customer: &str,
        transaction: &str,
    ) -> ClientResult<crate::types::CustomerBalanceTransaction> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/balance_transactions/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(transaction),
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
     * This function performs a `GET` to the `/v1/customers/{customer}/bank_accounts` endpoint.
     *
     * <p>You can see a list of the bank accounts belonging to a Customer. Note that the 10 most recent sources are always available by default on the Customer. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional bank accounts.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_bank_accounts(
        &self,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::BankAccount>> {
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
                "/v1/customers/{}/bank_accounts?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::BankAccountList = self
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
     * This function performs a `GET` to the `/v1/customers/{customer}/bank_accounts` endpoint.
     *
     * As opposed to `get_bank_accounts`, this function returns all the pages of the request at once.
     *
     * <p>You can see a list of the bank accounts belonging to a Customer. Note that the 10 most recent sources are always available by default on the Customer. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional bank accounts.</p>
     */
    pub async fn get_all_bank_accounts(
        &self,
        customer: &str,
    ) -> ClientResult<Vec<crate::types::BankAccount>> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/bank_accounts",
                crate::progenitor_support::encode_path(customer),
            ),
            None,
        );
        let mut resp: crate::types::BankAccountList = self
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
     * This function performs a `POST` to the `/v1/customers/{customer}/bank_accounts` endpoint.
     *
     * <p>When you create a new credit card, you must specify a customer or recipient on which to create it.</p>
     *
     * <p>If the card’s owner has no default card, then the new card will become the default.
     * However, if the owner already has a default, then it will not change.
     * To change the default, you should <a href="/docs/api#update_customer">update the customer</a> to have a new <code>default_source</code>.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     */
    pub async fn post_bank_account(
        &self,
        customer: &str,
    ) -> ClientResult<crate::types::PaymentSourceAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/bank_accounts",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/v1/customers/{customer}/bank_accounts/{id}` endpoint.
     *
     * <p>By default, you can see the 10 most recent sources stored on a Customer directly on the object, but you can also retrieve details about a specific bank account stored on the Stripe account.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     */
    pub async fn get_bank_account(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::BankAccount> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/bank_accounts/{}",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `POST` to the `/v1/customers/{customer}/bank_accounts/{id}` endpoint.
     *
     * <p>Update a specified source for a given customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn post_bank_account_customers(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::SourceAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/bank_accounts/{}",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `DELETE` to the `/v1/customers/{customer}/bank_accounts/{id}` endpoint.
     *
     * <p>Delete a specified source for a given customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn delete_bank_accounts(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::DeleteCustomersCustomerCardsResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/bank_accounts/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(id),
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
    /**
     * This function performs a `POST` to the `/v1/customers/{customer}/bank_accounts/{id}/verify` endpoint.
     *
     * <p>Verify a specified bank account for a given customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn post_bank_accounts_verify(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::BankAccount> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/bank_accounts/{}/verify",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/v1/customers/{customer}/cards` endpoint.
     *
     * <p>You can see a list of the cards belonging to a customer.
     * Note that the 10 most recent sources are always available on the <code>Customer</code> object.
     * If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional cards.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_cards(
        &self,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::Card>> {
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
                "/v1/customers/{}/cards?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::Cards = self
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
     * This function performs a `GET` to the `/v1/customers/{customer}/cards` endpoint.
     *
     * As opposed to `get_cards`, this function returns all the pages of the request at once.
     *
     * <p>You can see a list of the cards belonging to a customer.
     * Note that the 10 most recent sources are always available on the <code>Customer</code> object.
     * If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional cards.</p>
     */
    pub async fn get_all_cards(&self, customer: &str) -> ClientResult<Vec<crate::types::Card>> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/cards",
                crate::progenitor_support::encode_path(customer),
            ),
            None,
        );
        let mut resp: crate::types::Cards = self
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
     * This function performs a `POST` to the `/v1/customers/{customer}/cards` endpoint.
     *
     * <p>When you create a new credit card, you must specify a customer or recipient on which to create it.</p>
     *
     * <p>If the card’s owner has no default card, then the new card will become the default.
     * However, if the owner already has a default, then it will not change.
     * To change the default, you should <a href="/docs/api#update_customer">update the customer</a> to have a new <code>default_source</code>.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     */
    pub async fn post_card(
        &self,
        customer: &str,
    ) -> ClientResult<crate::types::PaymentSourceAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/cards",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/v1/customers/{customer}/cards/{id}` endpoint.
     *
     * <p>You can always see the 10 most recent cards directly on a customer; this method lets you retrieve details about a specific card stored on the customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     */
    pub async fn get_card(&self, customer: &str, id: &str) -> ClientResult<crate::types::Card> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/cards/{}",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `POST` to the `/v1/customers/{customer}/cards/{id}` endpoint.
     *
     * <p>Update a specified source for a given customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn post_card_customers(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::SourceAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/cards/{}",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `DELETE` to the `/v1/customers/{customer}/cards/{id}` endpoint.
     *
     * <p>Delete a specified source for a given customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn delete_cards(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::DeleteCustomersCustomerCardsResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/cards/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(id),
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
    /**
     * This function performs a `GET` to the `/v1/customers/{customer}/discount` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_discount(&self, customer: &str) -> ClientResult<crate::types::DiscountData> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/discount",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `DELETE` to the `/v1/customers/{customer}/discount` endpoint.
     *
     * <p>Removes the currently applied discount on a customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     */
    pub async fn delete_discount(
        &self,
        customer: &str,
    ) -> ClientResult<crate::types::DeletedDiscount> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/discount",
                crate::progenitor_support::encode_path(customer),
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
    /**
     * This function performs a `GET` to the `/v1/customers/{customer}/payment_methods` endpoint.
     *
     * <p>Returns a list of PaymentMethods for a given Customer</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `type_: crate::types::GetCustomersCustomerPaymentMethodsType` -- A required filter on the list, based on the object `type` field.
     */
    pub async fn get_payment_methods(
        &self,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        type_: crate::types::GetCustomersCustomerPaymentMethodsType,
    ) -> ClientResult<Vec<crate::types::PaymentMethod>> {
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
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/payment_methods?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::PaymentFlowsMethodList = self
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
     * This function performs a `GET` to the `/v1/customers/{customer}/payment_methods` endpoint.
     *
     * As opposed to `get_payment_methods`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of PaymentMethods for a given Customer</p>
     */
    pub async fn get_all_payment_methods(
        &self,
        customer: &str,
        type_: crate::types::GetCustomersCustomerPaymentMethodsType,
    ) -> ClientResult<Vec<crate::types::PaymentMethod>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/payment_methods?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let mut resp: crate::types::PaymentFlowsMethodList = self
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
     * This function performs a `GET` to the `/v1/customers/{customer}/sources` endpoint.
     *
     * <p>List sources for a specified customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `object: &str` -- Filter sources according to a particular object type.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_sources(
        &self,
        customer: &str,
        ending_before: &str,
        limit: i64,
        object: &str,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::CustomerSourcesDataAnyOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !object.is_empty() {
            query_args.push(("object".to_string(), object.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/sources?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::Sources = self
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
     * This function performs a `GET` to the `/v1/customers/{customer}/sources` endpoint.
     *
     * As opposed to `get_sources`, this function returns all the pages of the request at once.
     *
     * <p>List sources for a specified customer.</p>
     */
    pub async fn get_all_sources(
        &self,
        customer: &str,
        object: &str,
    ) -> ClientResult<Vec<crate::types::CustomerSourcesDataAnyOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !object.is_empty() {
            query_args.push(("object".to_string(), object.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/sources?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let mut resp: crate::types::Sources = self
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
     * This function performs a `POST` to the `/v1/customers/{customer}/sources` endpoint.
     *
     * <p>When you create a new credit card, you must specify a customer or recipient on which to create it.</p>
     *
     * <p>If the card’s owner has no default card, then the new card will become the default.
     * However, if the owner already has a default, then it will not change.
     * To change the default, you should <a href="/docs/api#update_customer">update the customer</a> to have a new <code>default_source</code>.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     */
    pub async fn post_source(
        &self,
        customer: &str,
    ) -> ClientResult<crate::types::PaymentSourceAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/sources",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/v1/customers/{customer}/sources/{id}` endpoint.
     *
     * <p>Retrieve a specified source for a given customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     */
    pub async fn get_source(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::PaymentSourceAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/sources/{}",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `POST` to the `/v1/customers/{customer}/sources/{id}` endpoint.
     *
     * <p>Update a specified source for a given customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn post_source_customers(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::SourceAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/sources/{}",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `DELETE` to the `/v1/customers/{customer}/sources/{id}` endpoint.
     *
     * <p>Delete a specified source for a given customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn delete_sources(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::DeleteCustomersCustomerCardsResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/sources/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(id),
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
    /**
     * This function performs a `POST` to the `/v1/customers/{customer}/sources/{id}/verify` endpoint.
     *
     * <p>Verify a specified bank account for a given customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn post_sources_verify(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::BankAccount> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/sources/{}/verify",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/v1/customers/{customer}/subscriptions` endpoint.
     *
     * <p>You can see a list of the customer’s active subscriptions. Note that the 10 most recent active subscriptions are always available by default on the customer object. If you need more than those 10, you can use the limit and starting_after parameters to page through additional subscriptions.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_subscriptions(
        &self,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::Subscription>> {
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
                "/v1/customers/{}/subscriptions?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::Subscriptions = self
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
     * This function performs a `GET` to the `/v1/customers/{customer}/subscriptions` endpoint.
     *
     * As opposed to `get_subscriptions`, this function returns all the pages of the request at once.
     *
     * <p>You can see a list of the customer’s active subscriptions. Note that the 10 most recent active subscriptions are always available by default on the customer object. If you need more than those 10, you can use the limit and starting_after parameters to page through additional subscriptions.</p>
     */
    pub async fn get_all_subscriptions(
        &self,
        customer: &str,
    ) -> ClientResult<Vec<crate::types::Subscription>> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/subscriptions",
                crate::progenitor_support::encode_path(customer),
            ),
            None,
        );
        let mut resp: crate::types::Subscriptions = self
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
     * This function performs a `POST` to the `/v1/customers/{customer}/subscriptions` endpoint.
     *
     * <p>Creates a new subscription on an existing customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     */
    pub async fn post_subscription(
        &self,
        customer: &str,
    ) -> ClientResult<crate::types::Subscription> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/subscriptions",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}` endpoint.
     *
     * <p>Retrieves the subscription with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `subscription_exposed_id: &str` -- The account's country.
     */
    pub async fn get_subscriptions_subscription_exposed(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> ClientResult<crate::types::Subscription> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/subscriptions/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(subscription_exposed_id),
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
     * This function performs a `POST` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}` endpoint.
     *
     * <p>Updates an existing subscription on a customer to match the specified parameters. When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes. To preview how the proration will be calculated, use the <a href="#upcoming_invoice">upcoming invoice</a> endpoint.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `subscription_exposed_id: &str` -- The account's country.
     */
    pub async fn post_subscriptions_subscription_exposed(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> ClientResult<crate::types::Subscription> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/subscriptions/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(subscription_exposed_id),
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
     * This function performs a `DELETE` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}` endpoint.
     *
     * <p>Cancels a customer’s subscription. If you set the <code>at_period_end</code> parameter to <code>true</code>, the subscription will remain active until the end of the period, at which point it will be canceled and not renewed. Otherwise, with the default <code>false</code> value, the subscription is terminated immediately. In either case, the customer will not be charged again for the subscription.</p>
     *
     * <p>Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually <a href="#delete_invoiceitem">deleted</a>. If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period. But if the subscription is set to cancel immediately, pending prorations will be removed.</p>
     *
     * <p>By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer. This is intended to prevent unexpected payment attempts after the customer has canceled a subscription. However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed. Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `subscription_exposed_id: &str` -- The account's country.
     */
    pub async fn delete_subscriptions_subscription_exposed(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> ClientResult<crate::types::Subscription> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/subscriptions/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(subscription_exposed_id),
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
    /**
     * This function performs a `GET` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}/discount` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `subscription_exposed_id: &str` -- The account's country.
     */
    pub async fn get_subscriptions_subscription_exposed_discount(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> ClientResult<crate::types::DiscountData> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/subscriptions/{}/discount",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(subscription_exposed_id),
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
     * This function performs a `DELETE` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}/discount` endpoint.
     *
     * <p>Removes the currently applied discount on a customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `subscription_exposed_id: &str` -- The account's country.
     */
    pub async fn delete_subscriptions_subscription_exposed_discount(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> ClientResult<crate::types::DeletedDiscount> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/subscriptions/{}/discount",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(subscription_exposed_id),
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
    /**
     * This function performs a `GET` to the `/v1/customers/{customer}/tax_ids` endpoint.
     *
     * <p>Returns a list of tax IDs for a customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_tax_ids(
        &self,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::TaxId>> {
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
                "/v1/customers/{}/tax_ids?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::TaxIds = self
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
     * This function performs a `GET` to the `/v1/customers/{customer}/tax_ids` endpoint.
     *
     * As opposed to `get_tax_ids`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of tax IDs for a customer.</p>
     */
    pub async fn get_all_tax_ids(&self, customer: &str) -> ClientResult<Vec<crate::types::TaxId>> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/tax_ids",
                crate::progenitor_support::encode_path(customer),
            ),
            None,
        );
        let mut resp: crate::types::TaxIds = self
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
     * This function performs a `POST` to the `/v1/customers/{customer}/tax_ids` endpoint.
     *
     * <p>Creates a new <code>TaxID</code> object for a customer.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     */
    pub async fn post_tax_id(&self, customer: &str) -> ClientResult<crate::types::TaxId> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/tax_ids",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/v1/customers/{customer}/tax_ids/{id}` endpoint.
     *
     * <p>Retrieves the <code>TaxID</code> object with the given identifier.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     */
    pub async fn get_tax_id(&self, customer: &str, id: &str) -> ClientResult<crate::types::TaxId> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/tax_ids/{}",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `DELETE` to the `/v1/customers/{customer}/tax_ids/{id}` endpoint.
     *
     * <p>Deletes an existing <code>TaxID</code> object.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn delete_tax_ids(
        &self,
        customer: &str,
        id: &str,
    ) -> ClientResult<crate::types::DeletedTaxId> {
        let url = self.client.url(
            &format!(
                "/v1/customers/{}/tax_ids/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(id),
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
