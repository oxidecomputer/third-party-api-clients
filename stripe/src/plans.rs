use crate::Client;
use crate::ClientResult;

pub struct Plans {
    pub client: Client,
}

impl Plans {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Plans { client }
    }

    /**
     * This function performs a `GET` to the `/v1/plans` endpoint.
     *
     * <p>Returns a list of your plans.</p>
     *
     * **Parameters:**
     *
     * * `active: bool` -- Only return plans that are active or inactive (e.g., pass `false` to list all inactive plans).
     * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `product: &str` -- Only return plans for the given product.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        active: bool,
        _created: &str,
        ending_before: &str,
        limit: i64,
        product: &str,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::PlanData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/plans?{}", query_), None);
        let resp: crate::types::PlanList = self
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
     * This function performs a `GET` to the `/v1/plans` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your plans.</p>
     */
    pub async fn get_all(
        &self,
        active: bool,
        _created: &str,
        product: &str,
    ) -> ClientResult<Vec<crate::types::PlanData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/plans?{}", query_), None);
        let mut resp: crate::types::PlanList = self
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
     * This function performs a `POST` to the `/v1/plans` endpoint.
     *
     * <p>You can now model subscriptions more flexibly using the <a href="#prices">Prices API</a>. It replaces the Plans API and is backwards compatible to simplify your migration.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::PlanData> {
        let url = self.client.url("/v1/plans", None);
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
     * This function performs a `GET` to the `/v1/plans/{plan}` endpoint.
     *
     * <p>Retrieves the plan with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `plan: &str` -- The account's country.
     */
    pub async fn get(&self, plan: &str) -> ClientResult<crate::types::PlanData> {
        let url = self.client.url(
            &format!("/v1/plans/{}", crate::progenitor_support::encode_path(plan),),
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
     * This function performs a `POST` to the `/v1/plans/{plan}` endpoint.
     *
     * <p>Updates the specified plan by setting the values of the parameters passed. Any parameters not provided are left unchanged. By design, you cannot change a plan’s ID, amount, currency, or billing cycle.</p>
     *
     * **Parameters:**
     *
     * * `plan: &str` -- The account's country.
     */
    pub async fn post_plans(&self, plan: &str) -> ClientResult<crate::types::PlanData> {
        let url = self.client.url(
            &format!("/v1/plans/{}", crate::progenitor_support::encode_path(plan),),
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
     * This function performs a `DELETE` to the `/v1/plans/{plan}` endpoint.
     *
     * <p>Deleting plans means new subscribers can’t be added. Existing subscribers aren’t affected.</p>
     *
     * **Parameters:**
     *
     * * `plan: &str` -- The account's country.
     */
    pub async fn delete(&self, plan: &str) -> ClientResult<crate::types::DeletedPlan> {
        let url = self.client.url(
            &format!("/v1/plans/{}", crate::progenitor_support::encode_path(plan),),
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
