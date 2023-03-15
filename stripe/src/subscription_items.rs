use crate::Client;
use crate::ClientResult;

pub struct SubscriptionItems {
    pub client: Client,
}

impl SubscriptionItems {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SubscriptionItems { client }
    }

    /**
     * This function performs a `GET` to the `/v1/subscription_items` endpoint.
     *
     * <p>Returns a list of your subscription items for a given subscription.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `subscription: &str` -- The ID of the subscription whose items will be retrieved.
     */
    pub async fn get_page(
        &self,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        subscription: &str,
    ) -> ClientResult<Vec<crate::types::SubscriptionItem>> {
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
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/subscription_items?{}", query_), None);
        let resp: crate::types::Items = self
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
     * This function performs a `GET` to the `/v1/subscription_items` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your subscription items for a given subscription.</p>
     */
    pub async fn get_all(
        &self,
        subscription: &str,
    ) -> ClientResult<Vec<crate::types::SubscriptionItem>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/subscription_items?{}", query_), None);
        let mut resp: crate::types::Items = self
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
     * This function performs a `POST` to the `/v1/subscription_items` endpoint.
     *
     * <p>Adds a new item to an existing subscription. No existing items will be changed or replaced.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::SubscriptionItem> {
        let url = self.client.url("/v1/subscription_items", None);
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
     * This function performs a `GET` to the `/v1/subscription_items/{item}` endpoint.
     *
     * <p>Retrieves the subscription item with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `item: &str` -- The account's country.
     */
    pub async fn get_item(&self, item: &str) -> ClientResult<crate::types::SubscriptionItem> {
        let url = self.client.url(
            &format!(
                "/v1/subscription_items/{}",
                crate::progenitor_support::encode_path(item),
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
     * This function performs a `POST` to the `/v1/subscription_items/{item}` endpoint.
     *
     * <p>Updates the plan or quantity of an item on a current subscription.</p>
     *
     * **Parameters:**
     *
     * * `item: &str` -- The account's country.
     */
    pub async fn post_item(&self, item: &str) -> ClientResult<crate::types::SubscriptionItem> {
        let url = self.client.url(
            &format!(
                "/v1/subscription_items/{}",
                crate::progenitor_support::encode_path(item),
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
     * This function performs a `DELETE` to the `/v1/subscription_items/{item}` endpoint.
     *
     * <p>Deletes an item from the subscription. Removing a subscription item from a subscription will not cancel the subscription.</p>
     *
     * **Parameters:**
     *
     * * `item: &str` -- The account's country.
     */
    pub async fn delete_item(
        &self,
        item: &str,
    ) -> ClientResult<crate::types::DeletedSubscriptionItem> {
        let url = self.client.url(
            &format!(
                "/v1/subscription_items/{}",
                crate::progenitor_support::encode_path(item),
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
     * This function performs a `GET` to the `/v1/subscription_items/{subscription_item}/usage_record_summaries` endpoint.
     *
     * <p>For the specified subscription item, returns a list of summary objects. Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).</p>
     *
     * <p>The list is sorted in reverse-chronological order (newest first). The first list item represents the most current usage period that hasn’t ended yet. Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `subscription_item: &str` -- The account's country.
     */
    pub async fn get_item_usage_record_summaries(
        &self,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        subscription_item: &str,
    ) -> ClientResult<Vec<crate::types::UsageRecordSummary>> {
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
                "/v1/subscription_items/{}/usage_record_summaries?{}",
                crate::progenitor_support::encode_path(subscription_item),
                query_
            ),
            None,
        );
        let resp: crate::types::GetSubscriptionItemsItemUsageRecordSummariesResponse = self
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
     * This function performs a `GET` to the `/v1/subscription_items/{subscription_item}/usage_record_summaries` endpoint.
     *
     * As opposed to `get_item_usage_record_summaries`, this function returns all the pages of the request at once.
     *
     * <p>For the specified subscription item, returns a list of summary objects. Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).</p>
     *
     * <p>The list is sorted in reverse-chronological order (newest first). The first list item represents the most current usage period that hasn’t ended yet. Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.</p>
     */
    pub async fn get_all_item_usage_record_summaries(
        &self,
        subscription_item: &str,
    ) -> ClientResult<Vec<crate::types::UsageRecordSummary>> {
        let url = self.client.url(
            &format!(
                "/v1/subscription_items/{}/usage_record_summaries",
                crate::progenitor_support::encode_path(subscription_item),
            ),
            None,
        );
        let mut resp: crate::types::GetSubscriptionItemsItemUsageRecordSummariesResponse = self
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
     * This function performs a `POST` to the `/v1/subscription_items/{subscription_item}/usage_records` endpoint.
     *
     * <p>Creates a usage record for a specified subscription item and date, and fills it with a quantity.</p>
     *
     * <p>Usage records provide <code>quantity</code> information that Stripe uses to track how much a customer is using your service. With usage information and the pricing model set up by the <a href="https://stripe.com/docs/billing/subscriptions/metered-billing">metered billing</a> plan, Stripe helps you send accurate invoices to your customers.</p>
     *
     * <p>The default calculation for usage is to add up all the <code>quantity</code> values of the usage records within a billing period. You can change this default behavior with the billing plan’s <code>aggregate_usage</code> <a href="/docs/api/plans/create#create_plan-aggregate_usage">parameter</a>. When there is more than one usage record with the same timestamp, Stripe adds the <code>quantity</code> values together. In most cases, this is the desired resolution, however, you can change this behavior with the <code>action</code> parameter.</p>
     *
     * <p>The default pricing model for metered billing is <a href="/docs/api/plans/object#plan_object-billing_scheme">per-unit pricing</a>. For finer granularity, you can configure metered billing to have a <a href="https://stripe.com/docs/billing/subscriptions/tiers">tiered pricing</a> model.</p>
     *
     * **Parameters:**
     *
     * * `subscription_item: &str` -- The account's country.
     */
    pub async fn post_item_usage_record(
        &self,
        subscription_item: &str,
    ) -> ClientResult<crate::types::UsageRecord> {
        let url = self.client.url(
            &format!(
                "/v1/subscription_items/{}/usage_records",
                crate::progenitor_support::encode_path(subscription_item),
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
