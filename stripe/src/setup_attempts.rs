use crate::Client;
use crate::ClientResult;

pub struct SetupAttempts {
    pub client: Client,
}

impl SetupAttempts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SetupAttempts { client }
    }

    /**
     * This function performs a `GET` to the `/v1/setup_attempts` endpoint.
     *
     * <p>Returns a list of SetupAttempts associated with a provided SetupIntent.</p>
     *
     * **Parameters:**
     *
     * * `created: &str` -- A filter on the list, based on the object `created` field. The value
     *   can be a string with an integer Unix timestamp, or it can be a
     *   dictionary with a number of different query options.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `setup_intent: &str` -- Only return SetupAttempts created by the SetupIntent specified by
     *   this ID.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        _created: &str,
        ending_before: &str,
        limit: i64,
        setup_intent: &str,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::SetupAttempt>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !setup_intent.is_empty() {
            query_args.push(("setup_intent".to_string(), setup_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/setup_attempts?{}", query_), None);
        let resp: crate::types::PaymentFlowsSetupIntentAttemptList = self
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
     * This function performs a `GET` to the `/v1/setup_attempts` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of SetupAttempts associated with a provided SetupIntent.</p>
     */
    pub async fn get_all(
        &self,
        _created: &str,
        setup_intent: &str,
    ) -> ClientResult<Vec<crate::types::SetupAttempt>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !setup_intent.is_empty() {
            query_args.push(("setup_intent".to_string(), setup_intent.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/setup_attempts?{}", query_), None);
        let mut resp: crate::types::PaymentFlowsSetupIntentAttemptList = self
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
}
