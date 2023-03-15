use crate::Client;
use crate::ClientResult;

pub struct SubscriptionSchedules {
    pub client: Client,
}

impl SubscriptionSchedules {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SubscriptionSchedules { client }
    }

    /**
     * This function performs a `GET` to the `/v1/subscription_schedules` endpoint.
     *
     * <p>Retrieves the list of your subscription schedules.</p>
     *
     * **Parameters:**
     *
     * * `canceled_at: &str` -- Only return subscription schedules that were created canceled the given date interval.
     * * `completed_at: &str` -- Only return subscription schedules that completed during the given date interval.
     * * `created: &str` -- Only return subscription schedules that were created during the given date interval.
     * * `customer: &str` -- Only return subscription schedules for the given customer.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `released_at: &str` -- Only return subscription schedules that were released during the given date interval.
     * * `scheduled: bool` -- Only return subscription schedules that have not started yet.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        _canceled_at: &str,
        _completed_at: &str,
        _created: &str,
        customer: &str,
        ending_before: &str,
        limit: i64,
        _released_at: &str,
        scheduled: bool,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::SubscriptionSchedule>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if scheduled {
            query_args.push(("scheduled".to_string(), scheduled.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/subscription_schedules?{}", query_), None);
        let resp: crate::types::GetSubscriptionSchedulesResponse = self
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
     * This function performs a `GET` to the `/v1/subscription_schedules` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Retrieves the list of your subscription schedules.</p>
     */
    pub async fn get_all(
        &self,
        _canceled_at: &str,
        _completed_at: &str,
        _created: &str,
        customer: &str,
        _released_at: &str,
        scheduled: bool,
    ) -> ClientResult<Vec<crate::types::SubscriptionSchedule>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if scheduled {
            query_args.push(("scheduled".to_string(), scheduled.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/subscription_schedules?{}", query_), None);
        let mut resp: crate::types::GetSubscriptionSchedulesResponse = self
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
     * This function performs a `POST` to the `/v1/subscription_schedules` endpoint.
     *
     * <p>Creates a new subscription schedule object. Each customer can have up to 500 active or scheduled subscriptions.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::SubscriptionSchedule> {
        let url = self.client.url("/v1/subscription_schedules", None);
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
     * This function performs a `GET` to the `/v1/subscription_schedules/{schedule}` endpoint.
     *
     * <p>Retrieves the details of an existing subscription schedule. You only need to supply the unique subscription schedule identifier that was returned upon subscription schedule creation.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `schedule: &str` -- The account's country.
     */
    pub async fn get_schedule(
        &self,
        schedule: &str,
    ) -> ClientResult<crate::types::SubscriptionSchedule> {
        let url = self.client.url(
            &format!(
                "/v1/subscription_schedules/{}",
                crate::progenitor_support::encode_path(schedule),
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
     * This function performs a `POST` to the `/v1/subscription_schedules/{schedule}` endpoint.
     *
     * <p>Updates an existing subscription schedule.</p>
     *
     * **Parameters:**
     *
     * * `schedule: &str` -- The account's country.
     */
    pub async fn post_schedule(
        &self,
        schedule: &str,
    ) -> ClientResult<crate::types::SubscriptionSchedule> {
        let url = self.client.url(
            &format!(
                "/v1/subscription_schedules/{}",
                crate::progenitor_support::encode_path(schedule),
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
     * This function performs a `POST` to the `/v1/subscription_schedules/{schedule}/cancel` endpoint.
     *
     * <p>Cancels a subscription schedule and its associated subscription immediately (if the subscription schedule has an active subscription). A subscription schedule can only be canceled if its status is <code>not_started</code> or <code>active</code>.</p>
     *
     * **Parameters:**
     *
     * * `schedule: &str` -- The account's country.
     */
    pub async fn post_schedule_cancel(
        &self,
        schedule: &str,
    ) -> ClientResult<crate::types::SubscriptionSchedule> {
        let url = self.client.url(
            &format!(
                "/v1/subscription_schedules/{}/cancel",
                crate::progenitor_support::encode_path(schedule),
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
     * This function performs a `POST` to the `/v1/subscription_schedules/{schedule}/release` endpoint.
     *
     * <p>Releases the subscription schedule immediately, which will stop scheduling of its phases, but leave any existing subscription in place. A schedule can only be released if its status is <code>not_started</code> or <code>active</code>. If the subscription schedule is currently associated with a subscription, releasing it will remove its <code>subscription</code> property and set the subscription’s ID to the <code>released_subscription</code> property.</p>
     *
     * **Parameters:**
     *
     * * `schedule: &str` -- The account's country.
     */
    pub async fn post_schedule_release(
        &self,
        schedule: &str,
    ) -> ClientResult<crate::types::SubscriptionSchedule> {
        let url = self.client.url(
            &format!(
                "/v1/subscription_schedules/{}/release",
                crate::progenitor_support::encode_path(schedule),
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
