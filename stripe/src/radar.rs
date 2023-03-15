use crate::Client;
use crate::ClientResult;

pub struct Radar {
    pub client: Client,
}

impl Radar {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Radar { client }
    }

    /**
     * This function performs a `GET` to the `/v1/radar/early_fraud_warnings` endpoint.
     *
     * <p>Returns a list of early fraud warnings.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- Only return early fraud warnings for the charge specified by this charge ID.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `payment_intent: &str` -- Only return early fraud warnings for charges that were created by the PaymentIntent specified by this PaymentIntent ID.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_early_fraud_warnings(
        &self,
        charge: &str,
        ending_before: &str,
        limit: i64,
        payment_intent: &str,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::RadarEarlyFraudWarning>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/radar/early_fraud_warnings?{}", query_), None);
        let resp: crate::types::RadarEarlyFraudWarningList = self
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
     * This function performs a `GET` to the `/v1/radar/early_fraud_warnings` endpoint.
     *
     * As opposed to `get_early_fraud_warnings`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of early fraud warnings.</p>
     */
    pub async fn get_all_early_fraud_warnings(
        &self,
        charge: &str,
        payment_intent: &str,
    ) -> ClientResult<Vec<crate::types::RadarEarlyFraudWarning>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/radar/early_fraud_warnings?{}", query_), None);
        let mut resp: crate::types::RadarEarlyFraudWarningList = self
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
     * This function performs a `GET` to the `/v1/radar/early_fraud_warnings/{early_fraud_warning}` endpoint.
     *
     * <p>Retrieves the details of an early fraud warning that has previously been created. </p>
     *
     * <p>Please refer to the <a href="#early_fraud_warning_object">early fraud warning</a> object reference for more details.</p>
     *
     * **Parameters:**
     *
     * * `early_fraud_warning: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_early_fraud_warnings_warning(
        &self,
        early_fraud_warning: &str,
    ) -> ClientResult<crate::types::RadarEarlyFraudWarning> {
        let url = self.client.url(
            &format!(
                "/v1/radar/early_fraud_warnings/{}",
                crate::progenitor_support::encode_path(early_fraud_warning),
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
     * This function performs a `GET` to the `/v1/radar/value_list_items` endpoint.
     *
     * <p>Returns a list of <code>ValueListItem</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str`
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `value: &str` -- Return items belonging to the parent list whose value matches the specified value (using an "is like" match).
     * * `value_list: &str` -- Identifier for the parent value list this item belongs to.
     */
    pub async fn get_value_list_items(
        &self,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        value: &str,
        value_list: &str,
    ) -> ClientResult<Vec<crate::types::RadarListItem>> {
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
        if !value.is_empty() {
            query_args.push(("value".to_string(), value.to_string()));
        }
        if !value_list.is_empty() {
            query_args.push(("value_list".to_string(), value_list.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/radar/value_list_items?{}", query_), None);
        let resp: crate::types::ListItems = self
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
     * This function performs a `GET` to the `/v1/radar/value_list_items` endpoint.
     *
     * As opposed to `get_value_list_items`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of <code>ValueListItem</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     */
    pub async fn get_all_value_list_all_items(
        &self,
        _created: &str,
        value: &str,
        value_list: &str,
    ) -> ClientResult<Vec<crate::types::RadarListItem>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !value.is_empty() {
            query_args.push(("value".to_string(), value.to_string()));
        }
        if !value_list.is_empty() {
            query_args.push(("value_list".to_string(), value_list.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/radar/value_list_items?{}", query_), None);
        let mut resp: crate::types::ListItems = self
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
     * This function performs a `POST` to the `/v1/radar/value_list_items` endpoint.
     *
     * <p>Creates a new <code>ValueListItem</code> object, which is added to the specified parent value list.</p>
     */
    pub async fn post_value_list_item(&self) -> ClientResult<crate::types::RadarListItem> {
        let url = self.client.url("/v1/radar/value_list_items", None);
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
     * This function performs a `GET` to the `/v1/radar/value_list_items/{item}` endpoint.
     *
     * <p>Retrieves a <code>ValueListItem</code> object.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `item: &str` -- The account's country.
     */
    pub async fn get_value_list_items_item(
        &self,
        item: &str,
    ) -> ClientResult<crate::types::RadarListItem> {
        let url = self.client.url(
            &format!(
                "/v1/radar/value_list_items/{}",
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
     * This function performs a `DELETE` to the `/v1/radar/value_list_items/{item}` endpoint.
     *
     * <p>Deletes a <code>ValueListItem</code> object, removing it from its parent value list.</p>
     *
     * **Parameters:**
     *
     * * `item: &str` -- The account's country.
     */
    pub async fn delete_value_list_items_item(
        &self,
        item: &str,
    ) -> ClientResult<crate::types::RadarListDeletedItem> {
        let url = self.client.url(
            &format!(
                "/v1/radar/value_list_items/{}",
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
     * This function performs a `GET` to the `/v1/radar/value_lists` endpoint.
     *
     * <p>Returns a list of <code>ValueList</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     *
     * **Parameters:**
     *
     * * `alias: &str` -- The alias used to reference the value list when writing rules.
     * * `contains: &str` -- A value contained within a value list - returns all value lists containing this value.
     * * `created: &str`
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_value_lists(
        &self,
        alias: &str,
        contains: &str,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::RadarList>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !alias.is_empty() {
            query_args.push(("alias".to_string(), alias.to_string()));
        }
        if !contains.is_empty() {
            query_args.push(("contains".to_string(), contains.to_string()));
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/radar/value_lists?{}", query_), None);
        let resp: crate::types::GetRadarValueListsResponse = self
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
     * This function performs a `GET` to the `/v1/radar/value_lists` endpoint.
     *
     * As opposed to `get_value_lists`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of <code>ValueList</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     */
    pub async fn get_all_value_lists(
        &self,
        alias: &str,
        contains: &str,
        _created: &str,
    ) -> ClientResult<Vec<crate::types::RadarList>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !alias.is_empty() {
            query_args.push(("alias".to_string(), alias.to_string()));
        }
        if !contains.is_empty() {
            query_args.push(("contains".to_string(), contains.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/radar/value_lists?{}", query_), None);
        let mut resp: crate::types::GetRadarValueListsResponse = self
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
     * This function performs a `POST` to the `/v1/radar/value_lists` endpoint.
     *
     * <p>Creates a new <code>ValueList</code> object, which can then be referenced in rules.</p>
     */
    pub async fn post_value_list(&self) -> ClientResult<crate::types::RadarList> {
        let url = self.client.url("/v1/radar/value_lists", None);
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
     * This function performs a `GET` to the `/v1/radar/value_lists/{value_list}` endpoint.
     *
     * <p>Retrieves a <code>ValueList</code> object.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `value_list: &str` -- The account's country.
     */
    pub async fn get_value_lists_list(
        &self,
        value_list: &str,
    ) -> ClientResult<crate::types::RadarList> {
        let url = self.client.url(
            &format!(
                "/v1/radar/value_lists/{}",
                crate::progenitor_support::encode_path(value_list),
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
     * This function performs a `POST` to the `/v1/radar/value_lists/{value_list}` endpoint.
     *
     * <p>Updates a <code>ValueList</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Note that <code>item_type</code> is immutable.</p>
     *
     * **Parameters:**
     *
     * * `value_list: &str` -- The account's country.
     */
    pub async fn post_value_lists_list(
        &self,
        value_list: &str,
    ) -> ClientResult<crate::types::RadarList> {
        let url = self.client.url(
            &format!(
                "/v1/radar/value_lists/{}",
                crate::progenitor_support::encode_path(value_list),
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
     * This function performs a `DELETE` to the `/v1/radar/value_lists/{value_list}` endpoint.
     *
     * <p>Deletes a <code>ValueList</code> object, also deleting any items contained within the value list. To be deleted, a value list must not be referenced in any rules.</p>
     *
     * **Parameters:**
     *
     * * `value_list: &str` -- The account's country.
     */
    pub async fn delete_value_lists_list(
        &self,
        value_list: &str,
    ) -> ClientResult<crate::types::RadarListDeleted> {
        let url = self.client.url(
            &format!(
                "/v1/radar/value_lists/{}",
                crate::progenitor_support::encode_path(value_list),
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
