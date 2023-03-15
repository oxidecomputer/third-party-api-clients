use crate::Client;
use crate::ClientResult;

pub struct Invoices {
    pub client: Client,
}

impl Invoices {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Invoices { client }
    }

    /**
     * This function performs a `GET` to the `/v1/invoices` endpoint.
     *
     * <p>You can list all invoices, or list the invoices for a specific customer. The invoices are returned sorted by creation date, with the most recently created invoices appearing first.</p>
     *
     * **Parameters:**
     *
     * * `collection_method: crate::types::CollectionMethod` -- Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer. When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
     * * `created: &str`
     * * `customer: &str` -- Only return invoices for the customer specified by this customer ID.
     * * `due_date: &str`
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::GetInvoicesStatus` -- The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`. [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
     * * `subscription: &str` -- Only return invoices for the subscription specified by this subscription ID.
     */
    pub async fn get_page(
        &self,
        collection_method: crate::types::CollectionMethod,
        _created: &str,
        customer: &str,
        _due_date: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        status: crate::types::GetInvoicesStatus,
        subscription: &str,
    ) -> ClientResult<Vec<crate::types::Invoice>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_method.to_string().is_empty() {
            query_args.push((
                "collection_method".to_string(),
                collection_method.to_string(),
            ));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
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
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/invoices?{}", query_), None);
        let resp: crate::types::InvoicesList = self
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
     * This function performs a `GET` to the `/v1/invoices` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>You can list all invoices, or list the invoices for a specific customer. The invoices are returned sorted by creation date, with the most recently created invoices appearing first.</p>
     */
    pub async fn get_all(
        &self,
        collection_method: crate::types::CollectionMethod,
        _created: &str,
        customer: &str,
        _due_date: &str,
        status: crate::types::GetInvoicesStatus,
        subscription: &str,
    ) -> ClientResult<Vec<crate::types::Invoice>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_method.to_string().is_empty() {
            query_args.push((
                "collection_method".to_string(),
                collection_method.to_string(),
            ));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/invoices?{}", query_), None);
        let mut resp: crate::types::InvoicesList = self
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
     * This function performs a `POST` to the `/v1/invoices` endpoint.
     *
     * <p>This endpoint creates a draft invoice for a given customer. The draft invoice created pulls in all pending invoice items on that customer, including prorations. The invoice remains a draft until you <a href="#finalize_invoice">finalize</a> the invoice, which allows you to <a href="#pay_invoice">pay</a> or <a href="#send_invoice">send</a> the invoice to your customers.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Invoice> {
        let url = self.client.url("/v1/invoices", None);
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
     * This function performs a `GET` to the `/v1/invoices/search` endpoint.
     *
     * <p>Search for invoices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
     * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
     * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
     * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
     * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for invoices](https://stripe.com/docs/search#query-fields-for-invoices).
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
            .url(&format!("/v1/invoices/search?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/invoices/search` endpoint.
     *
     * As opposed to `get_search`, this function returns all the pages of the request at once.
     *
     * <p>Search for invoices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
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
            .url(&format!("/v1/invoices/search?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/invoices/upcoming` endpoint.
     *
     * <p>At any time, you can preview the upcoming invoice for a customer. This will show you all the charges that are pending, including subscription renewal charges, invoice item charges, etc. It will also show you any discounts that are applicable to the invoice.</p>
     *
     * <p>Note that when you are viewing an upcoming invoice, you are simply viewing a preview – the invoice has not yet been created. As such, the upcoming invoice will not show up in invoice listing calls, and you cannot use the API to pay or edit the invoice. If you want to change the amount that your customer will be billed, you can add, remove, or update pending invoice items, or update the customer’s discount.</p>
     *
     * <p>You can preview the effects of updating a subscription, including a preview of what proration will take place. To ensure that the actual proration is calculated exactly the same as the previewed proration, you should pass a <code>proration_date</code> parameter when doing the actual subscription update. The value passed in should be the same as the <code>subscription_proration_date</code> returned on the upcoming invoice resource. The recommended way to get only the prorations being previewed is to consider only proration line items where <code>period[start]</code> is equal to the <code>subscription_proration_date</code> on the upcoming invoice resource.</p>
     *
     * **Parameters:**
     *
     * * `automatic_tax: &str` -- Settings for automatic tax lookup for this invoice preview.
     * * `coupon: &str` -- The code of the coupon to apply. If `subscription` or `subscription_items` is provided, the invoice returned will preview updating or creating a subscription with that coupon. Otherwise, it will preview applying that coupon to the customer for the next upcoming invoice from among the customer's subscriptions. The invoice can be previewed without a coupon by passing this value as an empty string.
     * * `customer: &str` -- The identifier of the customer whose upcoming invoice you'd like to retrieve.
     * * `customer_details: &str` -- Details about the customer you want to invoice or overrides for an existing customer.
     * * `discounts: &str` -- The coupons to redeem into discounts for the invoice preview. If not specified, inherits the discount from the customer or subscription. This only works for coupons directly applied to the invoice. To apply a coupon to a subscription, you must use the `coupon` parameter instead. Pass an empty string to avoid inheriting any discounts. To preview the upcoming invoice for a subscription that hasn't been created, use `coupon` instead.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `invoice_items: &[String]` -- List of invoice items to add or update in the upcoming invoice preview.
     * * `schedule: &str` -- The identifier of the unstarted schedule whose upcoming invoice you'd like to retrieve. Cannot be used with subscription or subscription fields.
     * * `subscription: &str` -- The identifier of the subscription for which you'd like to retrieve the upcoming invoice. If not provided, but a `subscription_items` is provided, you will preview creating a subscription with those items. If neither `subscription` nor `subscription_items` is provided, you will retrieve the next upcoming invoice from among the customer's subscriptions.
     * * `subscription_billing_cycle_anchor: &str` -- For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle). This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. For existing subscriptions, the value can only be set to `now` or `unchanged`.
     * * `subscription_cancel_at: &str` -- Timestamp indicating when the subscription should be scheduled to cancel. Will prorate if within the current period and prorations have been enabled using `proration_behavior`.
     * * `subscription_cancel_at_period_end: bool` -- Boolean indicating whether this subscription should cancel at the end of the current period.
     * * `subscription_cancel_now: bool` -- This simulates the subscription being canceled or expired immediately.
     * * `subscription_default_tax_rates: &str` -- If provided, the invoice returned will preview updating or creating a subscription with these default tax rates. The default tax rates will apply to any line item that does not have `tax_rates` set.
     * * `subscription_items: &[String]` -- A list of up to 20 subscription items, each with an attached price.
     * * `subscription_proration_behavior: crate::types::ProrationBehavior` -- Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes. Valid values are `create_prorations`, `none`, or `always_invoice`.
     *   
     *   Passing `create_prorations` will cause proration invoice items to be created when applicable. These proration items will only be invoiced immediately under [certain conditions](https://stripe.com/docs/subscriptions/upgrading-downgrading#immediate-payment). In order to always invoice immediately for prorations, pass `always_invoice`.
     *   
     *   Prorations can be disabled by passing `none`.
     * * `subscription_proration_date: i64` -- If previewing an update to a subscription, and doing proration, `subscription_proration_date` forces the proration to be calculated as though the update was done at the specified time. The time given must be within the current subscription period, and cannot be before the subscription was on its current plan. If set, `subscription`, and one of `subscription_items`, or `subscription_trial_end` are required. Also, `subscription_proration_behavior` cannot be set to 'none'.
     * * `subscription_start_date: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
     * * `subscription_trial_end: &str` -- If provided, the invoice returned will preview updating or creating a subscription with that trial end. If set, one of `subscription_items` or `subscription` is required.
     * * `subscription_trial_from_plan: bool` -- Indicates if a plan's `trial_period_days` should be applied to the subscription. Setting `subscription_trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `subscription_trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
     */
    pub async fn get_upcoming(
        &self,
        _automatic_tax: &str,
        coupon: &str,
        customer: &str,
        _customer_details: &str,
        _discounts: &str,
        _invoice_items: &[String],
        schedule: &str,
        subscription: &str,
        _subscription_billing_cycle_anchor: &str,
        _subscription_cancel_at: &str,
        subscription_cancel_at_period_end: bool,
        subscription_cancel_now: bool,
        _subscription_default_tax_rates: &str,
        _subscription_items: &[String],
        subscription_proration_behavior: crate::types::ProrationBehavior,
        subscription_proration_date: i64,
        subscription_start_date: i64,
        _subscription_trial_end: &str,
        subscription_trial_from_plan: bool,
    ) -> ClientResult<crate::types::Invoice> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !coupon.is_empty() {
            query_args.push(("coupon".to_string(), coupon.to_string()));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !schedule.is_empty() {
            query_args.push(("schedule".to_string(), schedule.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        if subscription_cancel_at_period_end {
            query_args.push((
                "subscription_cancel_at_period_end".to_string(),
                subscription_cancel_at_period_end.to_string(),
            ));
        }
        if subscription_cancel_now {
            query_args.push((
                "subscription_cancel_now".to_string(),
                subscription_cancel_now.to_string(),
            ));
        }
        if !subscription_proration_behavior.to_string().is_empty() {
            query_args.push((
                "subscription_proration_behavior".to_string(),
                subscription_proration_behavior.to_string(),
            ));
        }
        if subscription_proration_date > 0 {
            query_args.push((
                "subscription_proration_date".to_string(),
                subscription_proration_date.to_string(),
            ));
        }
        if subscription_start_date > 0 {
            query_args.push((
                "subscription_start_date".to_string(),
                subscription_start_date.to_string(),
            ));
        }
        if subscription_trial_from_plan {
            query_args.push((
                "subscription_trial_from_plan".to_string(),
                subscription_trial_from_plan.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/invoices/upcoming?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/invoices/upcoming/lines` endpoint.
     *
     * <p>When retrieving an upcoming invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     *
     * **Parameters:**
     *
     * * `automatic_tax: &str` -- Settings for automatic tax lookup for this invoice preview.
     * * `coupon: &str` -- The code of the coupon to apply. If `subscription` or `subscription_items` is provided, the invoice returned will preview updating or creating a subscription with that coupon. Otherwise, it will preview applying that coupon to the customer for the next upcoming invoice from among the customer's subscriptions. The invoice can be previewed without a coupon by passing this value as an empty string.
     * * `customer: &str` -- The identifier of the customer whose upcoming invoice you'd like to retrieve.
     * * `customer_details: &str` -- Details about the customer you want to invoice or overrides for an existing customer.
     * * `discounts: &str` -- The coupons to redeem into discounts for the invoice preview. If not specified, inherits the discount from the customer or subscription. This only works for coupons directly applied to the invoice. To apply a coupon to a subscription, you must use the `coupon` parameter instead. Pass an empty string to avoid inheriting any discounts. To preview the upcoming invoice for a subscription that hasn't been created, use `coupon` instead.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `invoice_items: &[String]` -- List of invoice items to add or update in the upcoming invoice preview.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `schedule: &str` -- The identifier of the unstarted schedule whose upcoming invoice you'd like to retrieve. Cannot be used with subscription or subscription fields.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `subscription: &str` -- The identifier of the subscription for which you'd like to retrieve the upcoming invoice. If not provided, but a `subscription_items` is provided, you will preview creating a subscription with those items. If neither `subscription` nor `subscription_items` is provided, you will retrieve the next upcoming invoice from among the customer's subscriptions.
     * * `subscription_billing_cycle_anchor: &str` -- For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle). This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. For existing subscriptions, the value can only be set to `now` or `unchanged`.
     * * `subscription_cancel_at: &str` -- Timestamp indicating when the subscription should be scheduled to cancel. Will prorate if within the current period and prorations have been enabled using `proration_behavior`.
     * * `subscription_cancel_at_period_end: bool` -- Boolean indicating whether this subscription should cancel at the end of the current period.
     * * `subscription_cancel_now: bool` -- This simulates the subscription being canceled or expired immediately.
     * * `subscription_default_tax_rates: &str` -- If provided, the invoice returned will preview updating or creating a subscription with these default tax rates. The default tax rates will apply to any line item that does not have `tax_rates` set.
     * * `subscription_items: &[String]` -- A list of up to 20 subscription items, each with an attached price.
     * * `subscription_proration_behavior: crate::types::ProrationBehavior` -- Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes. Valid values are `create_prorations`, `none`, or `always_invoice`.
     *   
     *   Passing `create_prorations` will cause proration invoice items to be created when applicable. These proration items will only be invoiced immediately under [certain conditions](https://stripe.com/docs/subscriptions/upgrading-downgrading#immediate-payment). In order to always invoice immediately for prorations, pass `always_invoice`.
     *   
     *   Prorations can be disabled by passing `none`.
     * * `subscription_proration_date: i64` -- If previewing an update to a subscription, and doing proration, `subscription_proration_date` forces the proration to be calculated as though the update was done at the specified time. The time given must be within the current subscription period, and cannot be before the subscription was on its current plan. If set, `subscription`, and one of `subscription_items`, or `subscription_trial_end` are required. Also, `subscription_proration_behavior` cannot be set to 'none'.
     * * `subscription_start_date: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
     * * `subscription_trial_end: &str` -- If provided, the invoice returned will preview updating or creating a subscription with that trial end. If set, one of `subscription_items` or `subscription` is required.
     * * `subscription_trial_from_plan: bool` -- Indicates if a plan's `trial_period_days` should be applied to the subscription. Setting `subscription_trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `subscription_trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
     */
    pub async fn get_upcoming_lines(
        &self,
        _automatic_tax: &str,
        coupon: &str,
        customer: &str,
        _customer_details: &str,
        _discounts: &str,
        ending_before: &str,
        _invoice_items: &[String],
        limit: i64,
        schedule: &str,
        starting_after: &str,
        subscription: &str,
        _subscription_billing_cycle_anchor: &str,
        _subscription_cancel_at: &str,
        subscription_cancel_at_period_end: bool,
        subscription_cancel_now: bool,
        _subscription_default_tax_rates: &str,
        _subscription_items: &[String],
        subscription_proration_behavior: crate::types::ProrationBehavior,
        subscription_proration_date: i64,
        subscription_start_date: i64,
        _subscription_trial_end: &str,
        subscription_trial_from_plan: bool,
    ) -> ClientResult<Vec<crate::types::LineItem>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !coupon.is_empty() {
            query_args.push(("coupon".to_string(), coupon.to_string()));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !schedule.is_empty() {
            query_args.push(("schedule".to_string(), schedule.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        if subscription_cancel_at_period_end {
            query_args.push((
                "subscription_cancel_at_period_end".to_string(),
                subscription_cancel_at_period_end.to_string(),
            ));
        }
        if subscription_cancel_now {
            query_args.push((
                "subscription_cancel_now".to_string(),
                subscription_cancel_now.to_string(),
            ));
        }
        if !subscription_proration_behavior.to_string().is_empty() {
            query_args.push((
                "subscription_proration_behavior".to_string(),
                subscription_proration_behavior.to_string(),
            ));
        }
        if subscription_proration_date > 0 {
            query_args.push((
                "subscription_proration_date".to_string(),
                subscription_proration_date.to_string(),
            ));
        }
        if subscription_start_date > 0 {
            query_args.push((
                "subscription_start_date".to_string(),
                subscription_start_date.to_string(),
            ));
        }
        if subscription_trial_from_plan {
            query_args.push((
                "subscription_trial_from_plan".to_string(),
                subscription_trial_from_plan.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/invoices/upcoming/lines?{}", query_), None);
        let resp: crate::types::InvoiceLinesList = self
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
     * This function performs a `GET` to the `/v1/invoices/upcoming/lines` endpoint.
     *
     * As opposed to `get_upcoming_lines`, this function returns all the pages of the request at once.
     *
     * <p>When retrieving an upcoming invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     */
    pub async fn get_all_upcoming_lines(
        &self,
        _automatic_tax: &str,
        coupon: &str,
        customer: &str,
        _customer_details: &str,
        _discounts: &str,
        _invoice_items: &[String],
        schedule: &str,
        subscription: &str,
        _subscription_billing_cycle_anchor: &str,
        _subscription_cancel_at: &str,
        subscription_cancel_at_period_end: bool,
        subscription_cancel_now: bool,
        _subscription_default_tax_rates: &str,
        _subscription_items: &[String],
        subscription_proration_behavior: crate::types::ProrationBehavior,
        subscription_proration_date: i64,
        subscription_start_date: i64,
        _subscription_trial_end: &str,
        subscription_trial_from_plan: bool,
    ) -> ClientResult<Vec<crate::types::LineItem>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !coupon.is_empty() {
            query_args.push(("coupon".to_string(), coupon.to_string()));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !schedule.is_empty() {
            query_args.push(("schedule".to_string(), schedule.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        if subscription_cancel_at_period_end {
            query_args.push((
                "subscription_cancel_at_period_end".to_string(),
                subscription_cancel_at_period_end.to_string(),
            ));
        }
        if subscription_cancel_now {
            query_args.push((
                "subscription_cancel_now".to_string(),
                subscription_cancel_now.to_string(),
            ));
        }
        if !subscription_proration_behavior.to_string().is_empty() {
            query_args.push((
                "subscription_proration_behavior".to_string(),
                subscription_proration_behavior.to_string(),
            ));
        }
        if subscription_proration_date > 0 {
            query_args.push((
                "subscription_proration_date".to_string(),
                subscription_proration_date.to_string(),
            ));
        }
        if subscription_start_date > 0 {
            query_args.push((
                "subscription_start_date".to_string(),
                subscription_start_date.to_string(),
            ));
        }
        if subscription_trial_from_plan {
            query_args.push((
                "subscription_trial_from_plan".to_string(),
                subscription_trial_from_plan.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/invoices/upcoming/lines?{}", query_), None);
        let mut resp: crate::types::InvoiceLinesList = self
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
     * This function performs a `GET` to the `/v1/invoices/{invoice}` endpoint.
     *
     * <p>Retrieves the invoice with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `invoice: &str` -- The account's country.
     */
    pub async fn get(&self, invoice: &str) -> ClientResult<crate::types::Invoice> {
        let url = self.client.url(
            &format!(
                "/v1/invoices/{}",
                crate::progenitor_support::encode_path(invoice),
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
     * This function performs a `POST` to the `/v1/invoices/{invoice}` endpoint.
     *
     * <p>Draft invoices are fully editable. Once an invoice is <a href="/docs/billing/invoices/workflow#finalized">finalized</a>,
     * monetary values, as well as <code>collection_method</code>, become uneditable.</p>
     *
     * <p>If you would like to stop the Stripe Billing engine from automatically finalizing, reattempting payments on,
     * sending reminders for, or <a href="/docs/billing/invoices/reconciliation">automatically reconciling</a> invoices, pass
     * <code>auto_advance=false</code>.</p>
     *
     * **Parameters:**
     *
     * * `invoice: &str` -- The account's country.
     */
    pub async fn post_invoices(&self, invoice: &str) -> ClientResult<crate::types::Invoice> {
        let url = self.client.url(
            &format!(
                "/v1/invoices/{}",
                crate::progenitor_support::encode_path(invoice),
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
     * This function performs a `DELETE` to the `/v1/invoices/{invoice}` endpoint.
     *
     * <p>Permanently deletes a one-off invoice draft. This cannot be undone. Attempts to delete invoices that are no longer in a draft state will fail; once an invoice has been finalized or if an invoice is for a subscription, it must be <a href="#void_invoice">voided</a>.</p>
     *
     * **Parameters:**
     *
     * * `invoice: &str` -- The account's country.
     */
    pub async fn delete(&self, invoice: &str) -> ClientResult<crate::types::DeletedInvoice> {
        let url = self.client.url(
            &format!(
                "/v1/invoices/{}",
                crate::progenitor_support::encode_path(invoice),
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
     * This function performs a `POST` to the `/v1/invoices/{invoice}/finalize` endpoint.
     *
     * <p>Stripe automatically finalizes drafts before sending and attempting payment on invoices. However, if you’d like to finalize a draft invoice manually, you can do so using this method.</p>
     *
     * **Parameters:**
     *
     * * `invoice: &str` -- The account's country.
     */
    pub async fn post_finalize(&self, invoice: &str) -> ClientResult<crate::types::Invoice> {
        let url = self.client.url(
            &format!(
                "/v1/invoices/{}/finalize",
                crate::progenitor_support::encode_path(invoice),
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
     * This function performs a `GET` to the `/v1/invoices/{invoice}/lines` endpoint.
     *
     * <p>When retrieving an invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `invoice: &str` -- The account's country.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_lines(
        &self,
        ending_before: &str,
        invoice: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::LineItem>> {
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
                "/v1/invoices/{}/lines?{}",
                crate::progenitor_support::encode_path(invoice),
                query_
            ),
            None,
        );
        let resp: crate::types::InvoiceLinesList = self
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
     * This function performs a `GET` to the `/v1/invoices/{invoice}/lines` endpoint.
     *
     * As opposed to `get_lines`, this function returns all the pages of the request at once.
     *
     * <p>When retrieving an invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     */
    pub async fn get_all_lines(&self, invoice: &str) -> ClientResult<Vec<crate::types::LineItem>> {
        let url = self.client.url(
            &format!(
                "/v1/invoices/{}/lines",
                crate::progenitor_support::encode_path(invoice),
            ),
            None,
        );
        let mut resp: crate::types::InvoiceLinesList = self
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
     * This function performs a `POST` to the `/v1/invoices/{invoice}/mark_uncollectible` endpoint.
     *
     * <p>Marking an invoice as uncollectible is useful for keeping track of bad debts that can be written off for accounting purposes.</p>
     *
     * **Parameters:**
     *
     * * `invoice: &str` -- The account's country.
     */
    pub async fn post_mark_uncollectible(
        &self,
        invoice: &str,
    ) -> ClientResult<crate::types::Invoice> {
        let url = self.client.url(
            &format!(
                "/v1/invoices/{}/mark_uncollectible",
                crate::progenitor_support::encode_path(invoice),
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
     * This function performs a `POST` to the `/v1/invoices/{invoice}/pay` endpoint.
     *
     * <p>Stripe automatically creates and then attempts to collect payment on invoices for customers on subscriptions according to your <a href="https://dashboard.stripe.com/account/billing/automatic">subscriptions settings</a>. However, if you’d like to attempt payment on an invoice out of the normal collection schedule or for some other reason, you can do so.</p>
     *
     * **Parameters:**
     *
     * * `invoice: &str` -- The account's country.
     */
    pub async fn post_pay(&self, invoice: &str) -> ClientResult<crate::types::Invoice> {
        let url = self.client.url(
            &format!(
                "/v1/invoices/{}/pay",
                crate::progenitor_support::encode_path(invoice),
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
     * This function performs a `POST` to the `/v1/invoices/{invoice}/send` endpoint.
     *
     * <p>Stripe will automatically send invoices to customers according to your <a href="https://dashboard.stripe.com/account/billing/automatic">subscriptions settings</a>. However, if you’d like to manually send an invoice to your customer out of the normal schedule, you can do so. When sending invoices that have already been paid, there will be no reference to the payment in the email.</p>
     *
     * <p>Requests made in test-mode result in no emails being sent, despite sending an <code>invoice.sent</code> event.</p>
     *
     * **Parameters:**
     *
     * * `invoice: &str` -- The account's country.
     */
    pub async fn post_send(&self, invoice: &str) -> ClientResult<crate::types::Invoice> {
        let url = self.client.url(
            &format!(
                "/v1/invoices/{}/send",
                crate::progenitor_support::encode_path(invoice),
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
     * This function performs a `POST` to the `/v1/invoices/{invoice}/void` endpoint.
     *
     * <p>Mark a finalized invoice as void. This cannot be undone. Voiding an invoice is similar to <a href="#delete_invoice">deletion</a>, however it only applies to finalized invoices and maintains a papertrail where the invoice can still be found.</p>
     *
     * **Parameters:**
     *
     * * `invoice: &str` -- The account's country.
     */
    pub async fn post_void(&self, invoice: &str) -> ClientResult<crate::types::Invoice> {
        let url = self.client.url(
            &format!(
                "/v1/invoices/{}/void",
                crate::progenitor_support::encode_path(invoice),
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
