use crate::Client;
use crate::ClientResult;

pub struct Issuing {
    pub client: Client,
}

impl Issuing {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Issuing { client }
    }

    /**
     * This function performs a `GET` to the `/v1/issuing/authorizations` endpoint.
     *
     * <p>Returns a list of Issuing <code>Authorization</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     *
     * **Parameters:**
     *
     * * `card: &str` -- Only return authorizations that belong to the given card.
     * * `cardholder: &str` -- Only return authorizations that belong to the given cardholder.
     * * `created: &str` -- Only return authorizations that were created during the given date interval.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::IssuingAuthorizationStatus` -- Only return authorizations with the given status. One of `pending`, `closed`, or `reversed`.
     */
    pub async fn get_authorizations(
        &self,
        card: &str,
        cardholder: &str,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        status: crate::types::IssuingAuthorizationStatus,
    ) -> ClientResult<Vec<crate::types::IssuingAuthorization>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !card.is_empty() {
            query_args.push(("card".to_string(), card.to_string()));
        }
        if !cardholder.is_empty() {
            query_args.push(("cardholder".to_string(), cardholder.to_string()));
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/authorizations?{}", query_), None);
        let resp: crate::types::GetIssuingAuthorizationsResponse = self
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
     * This function performs a `GET` to the `/v1/issuing/authorizations` endpoint.
     *
     * As opposed to `get_authorizations`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of Issuing <code>Authorization</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     */
    pub async fn get_all_authorizations(
        &self,
        card: &str,
        cardholder: &str,
        _created: &str,
        status: crate::types::IssuingAuthorizationStatus,
    ) -> ClientResult<Vec<crate::types::IssuingAuthorization>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !card.is_empty() {
            query_args.push(("card".to_string(), card.to_string()));
        }
        if !cardholder.is_empty() {
            query_args.push(("cardholder".to_string(), cardholder.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/authorizations?{}", query_), None);
        let mut resp: crate::types::GetIssuingAuthorizationsResponse = self
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
     * This function performs a `GET` to the `/v1/issuing/authorizations/{authorization}` endpoint.
     *
     * <p>Retrieves an Issuing <code>Authorization</code> object.</p>
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_authorizations_authorization(
        &self,
        authorization: &str,
    ) -> ClientResult<crate::types::IssuingAuthorization> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/authorizations/{}",
                crate::progenitor_support::encode_path(authorization),
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
     * This function performs a `POST` to the `/v1/issuing/authorizations/{authorization}` endpoint.
     *
     * <p>Updates the specified Issuing <code>Authorization</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The account's country.
     */
    pub async fn post_authorizations_authorization(
        &self,
        authorization: &str,
    ) -> ClientResult<crate::types::IssuingAuthorization> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/authorizations/{}",
                crate::progenitor_support::encode_path(authorization),
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
     * This function performs a `POST` to the `/v1/issuing/authorizations/{authorization}/approve` endpoint.
     *
     * <p>Approves a pending Issuing <code>Authorization</code> object. This request should be made within the timeout window of the <a href="/docs/issuing/controls/real-time-authorizations">real-time authorization</a> flow.</p>
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The account's country.
     */
    pub async fn post_authorizations_authorization_approve(
        &self,
        authorization: &str,
    ) -> ClientResult<crate::types::IssuingAuthorization> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/authorizations/{}/approve",
                crate::progenitor_support::encode_path(authorization),
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
     * This function performs a `POST` to the `/v1/issuing/authorizations/{authorization}/decline` endpoint.
     *
     * <p>Declines a pending Issuing <code>Authorization</code> object. This request should be made within the timeout window of the <a href="/docs/issuing/controls/real-time-authorizations">real time authorization</a> flow.</p>
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The account's country.
     */
    pub async fn post_authorizations_authorization_decline(
        &self,
        authorization: &str,
    ) -> ClientResult<crate::types::IssuingAuthorization> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/authorizations/{}/decline",
                crate::progenitor_support::encode_path(authorization),
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
     * This function performs a `GET` to the `/v1/issuing/cardholders` endpoint.
     *
     * <p>Returns a list of Issuing <code>Cardholder</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str` -- Only return cardholders that were created during the given date interval.
     * * `email: &str` -- Only return cardholders that have the given email address.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `phone_number: &str` -- Only return cardholders that have the given phone number.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::IssuingCardholderStatus` -- Only return cardholders that have the given status. One of `active`, `inactive`, or `blocked`.
     * * `type_: crate::types::AccountHolderType` -- Type of entity that holds the account. This can be either `individual` or `company`.
     */
    pub async fn get_cardholders(
        &self,
        _created: &str,
        email: &str,
        ending_before: &str,
        limit: i64,
        phone_number: &str,
        starting_after: &str,
        status: crate::types::IssuingCardholderStatus,
        type_: crate::types::AccountHolderType,
    ) -> ClientResult<Vec<crate::types::IssuingCardholder>> {
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
        if !phone_number.is_empty() {
            query_args.push(("phone_number".to_string(), phone_number.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/cardholders?{}", query_), None);
        let resp: crate::types::GetIssuingCardholdersResponse = self
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
     * This function performs a `GET` to the `/v1/issuing/cardholders` endpoint.
     *
     * As opposed to `get_cardholders`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of Issuing <code>Cardholder</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     */
    pub async fn get_all_cardholders(
        &self,
        _created: &str,
        email: &str,
        phone_number: &str,
        status: crate::types::IssuingCardholderStatus,
        type_: crate::types::AccountHolderType,
    ) -> ClientResult<Vec<crate::types::IssuingCardholder>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !phone_number.is_empty() {
            query_args.push(("phone_number".to_string(), phone_number.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/cardholders?{}", query_), None);
        let mut resp: crate::types::GetIssuingCardholdersResponse = self
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
     * This function performs a `POST` to the `/v1/issuing/cardholders` endpoint.
     *
     * <p>Creates a new Issuing <code>Cardholder</code> object that can be issued cards.</p>
     */
    pub async fn post_cardholder(&self) -> ClientResult<crate::types::IssuingCardholder> {
        let url = self.client.url("/v1/issuing/cardholders", None);
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
     * This function performs a `GET` to the `/v1/issuing/cardholders/{cardholder}` endpoint.
     *
     * <p>Retrieves an Issuing <code>Cardholder</code> object.</p>
     *
     * **Parameters:**
     *
     * * `cardholder: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_cardholders_cardholder(
        &self,
        cardholder: &str,
    ) -> ClientResult<crate::types::IssuingCardholder> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/cardholders/{}",
                crate::progenitor_support::encode_path(cardholder),
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
     * This function performs a `POST` to the `/v1/issuing/cardholders/{cardholder}` endpoint.
     *
     * <p>Updates the specified Issuing <code>Cardholder</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `cardholder: &str` -- The account's country.
     */
    pub async fn post_cardholders_cardholder(
        &self,
        cardholder: &str,
    ) -> ClientResult<crate::types::IssuingCardholder> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/cardholders/{}",
                crate::progenitor_support::encode_path(cardholder),
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
     * This function performs a `GET` to the `/v1/issuing/cards` endpoint.
     *
     * <p>Returns a list of Issuing <code>Card</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     *
     * **Parameters:**
     *
     * * `cardholder: &str` -- Only return cards belonging to the Cardholder with the provided ID.
     * * `created: &str` -- Only return cards that were issued during the given date interval.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `exp_month: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
     * * `exp_year: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `last_4: &str` -- Only return cards that have the given last four digits.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::IssuingCardStatus` -- Only return cards that have the given status. One of `active`, `inactive`, or `canceled`.
     * * `type_: crate::types::IssuingCardType` -- Only return cards that have the given type. One of `virtual` or `physical`.
     */
    pub async fn get_cards(
        &self,
        cardholder: &str,
        _created: &str,
        ending_before: &str,
        exp_month: i64,
        exp_year: i64,
        last_4: &str,
        limit: i64,
        starting_after: &str,
        status: crate::types::IssuingCardStatus,
        type_: crate::types::IssuingCardType,
    ) -> ClientResult<Vec<crate::types::IssuingCard>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cardholder.is_empty() {
            query_args.push(("cardholder".to_string(), cardholder.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if exp_month > 0 {
            query_args.push(("exp_month".to_string(), exp_month.to_string()));
        }
        if exp_year > 0 {
            query_args.push(("exp_year".to_string(), exp_year.to_string()));
        }
        if !last_4.is_empty() {
            query_args.push(("last4".to_string(), last_4.to_string()));
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
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/cards?{}", query_), None);
        let resp: crate::types::GetIssuingCardsResponse = self
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
     * This function performs a `GET` to the `/v1/issuing/cards` endpoint.
     *
     * As opposed to `get_cards`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of Issuing <code>Card</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     */
    pub async fn get_all_cards(
        &self,
        cardholder: &str,
        _created: &str,
        exp_month: i64,
        exp_year: i64,
        last_4: &str,
        status: crate::types::IssuingCardStatus,
        type_: crate::types::IssuingCardType,
    ) -> ClientResult<Vec<crate::types::IssuingCard>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cardholder.is_empty() {
            query_args.push(("cardholder".to_string(), cardholder.to_string()));
        }
        if exp_month > 0 {
            query_args.push(("exp_month".to_string(), exp_month.to_string()));
        }
        if exp_year > 0 {
            query_args.push(("exp_year".to_string(), exp_year.to_string()));
        }
        if !last_4.is_empty() {
            query_args.push(("last4".to_string(), last_4.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/cards?{}", query_), None);
        let mut resp: crate::types::GetIssuingCardsResponse = self
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
     * This function performs a `POST` to the `/v1/issuing/cards` endpoint.
     *
     * <p>Creates an Issuing <code>Card</code> object.</p>
     */
    pub async fn post_card(&self) -> ClientResult<crate::types::IssuingCard> {
        let url = self.client.url("/v1/issuing/cards", None);
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
     * This function performs a `GET` to the `/v1/issuing/cards/{card}` endpoint.
     *
     * <p>Retrieves an Issuing <code>Card</code> object.</p>
     *
     * **Parameters:**
     *
     * * `card: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_cards_card(&self, card: &str) -> ClientResult<crate::types::IssuingCard> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/cards/{}",
                crate::progenitor_support::encode_path(card),
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
     * This function performs a `POST` to the `/v1/issuing/cards/{card}` endpoint.
     *
     * <p>Updates the specified Issuing <code>Card</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `card: &str` -- The account's country.
     */
    pub async fn post_cards_card(&self, card: &str) -> ClientResult<crate::types::IssuingCard> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/cards/{}",
                crate::progenitor_support::encode_path(card),
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
     * This function performs a `GET` to the `/v1/issuing/disputes` endpoint.
     *
     * <p>Returns a list of Issuing <code>Dispute</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str` -- Select Issuing disputes that were created during the given date interval.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::IssuingDisputeStatus` -- Select Issuing disputes with the given status.
     * * `transaction: &str` -- Select the Issuing dispute for the given transaction.
     */
    pub async fn get_disputes(
        &self,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        status: crate::types::IssuingDisputeStatus,
        transaction: &str,
    ) -> ClientResult<Vec<crate::types::IssuingDispute>> {
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
        if !transaction.is_empty() {
            query_args.push(("transaction".to_string(), transaction.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/disputes?{}", query_), None);
        let resp: crate::types::IssuingDisputeList = self
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
     * This function performs a `GET` to the `/v1/issuing/disputes` endpoint.
     *
     * As opposed to `get_disputes`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of Issuing <code>Dispute</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     */
    pub async fn get_all_disputes(
        &self,
        _created: &str,
        status: crate::types::IssuingDisputeStatus,
        transaction: &str,
    ) -> ClientResult<Vec<crate::types::IssuingDispute>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !transaction.is_empty() {
            query_args.push(("transaction".to_string(), transaction.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/disputes?{}", query_), None);
        let mut resp: crate::types::IssuingDisputeList = self
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
     * This function performs a `POST` to the `/v1/issuing/disputes` endpoint.
     *
     * <p>Creates an Issuing <code>Dispute</code> object. Individual pieces of evidence within the <code>evidence</code> object are optional at this point. Stripe only validates that required evidence is present during submission. Refer to <a href="/docs/issuing/purchases/disputes#dispute-reasons-and-evidence">Dispute reasons and evidence</a> for more details about evidence requirements.</p>
     */
    pub async fn post_dispute(&self) -> ClientResult<crate::types::IssuingDispute> {
        let url = self.client.url("/v1/issuing/disputes", None);
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
     * This function performs a `GET` to the `/v1/issuing/disputes/{dispute}` endpoint.
     *
     * <p>Retrieves an Issuing <code>Dispute</code> object.</p>
     *
     * **Parameters:**
     *
     * * `dispute: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_disputes_dispute(
        &self,
        dispute: &str,
    ) -> ClientResult<crate::types::IssuingDispute> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/disputes/{}",
                crate::progenitor_support::encode_path(dispute),
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
     * This function performs a `POST` to the `/v1/issuing/disputes/{dispute}` endpoint.
     *
     * <p>Updates the specified Issuing <code>Dispute</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Properties on the <code>evidence</code> object can be unset by passing in an empty string.</p>
     *
     * **Parameters:**
     *
     * * `dispute: &str` -- The account's country.
     */
    pub async fn post_disputes_dispute(
        &self,
        dispute: &str,
    ) -> ClientResult<crate::types::IssuingDispute> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/disputes/{}",
                crate::progenitor_support::encode_path(dispute),
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
     * This function performs a `POST` to the `/v1/issuing/disputes/{dispute}/submit` endpoint.
     *
     * <p>Submits an Issuing <code>Dispute</code> to the card network. Stripe validates that all evidence fields required for the dispute’s reason are present. For more details, see <a href="/docs/issuing/purchases/disputes#dispute-reasons-and-evidence">Dispute reasons and evidence</a>.</p>
     *
     * **Parameters:**
     *
     * * `dispute: &str` -- The account's country.
     */
    pub async fn post_disputes_dispute_submit(
        &self,
        dispute: &str,
    ) -> ClientResult<crate::types::IssuingDispute> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/disputes/{}/submit",
                crate::progenitor_support::encode_path(dispute),
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
     * This function performs a `GET` to the `/v1/issuing/settlements` endpoint.
     *
     * <p>Returns a list of Issuing <code>Settlement</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str` -- Only return issuing settlements that were created during the given date interval.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_settlements(
        &self,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::IssuingSettlement>> {
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
        let url = self
            .client
            .url(&format!("/v1/issuing/settlements?{}", query_), None);
        let resp: crate::types::GetIssuingSettlementsResponse = self
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
     * This function performs a `GET` to the `/v1/issuing/settlements` endpoint.
     *
     * As opposed to `get_settlements`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of Issuing <code>Settlement</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     */
    pub async fn get_all_settlements(
        &self,
        _created: &str,
    ) -> ClientResult<Vec<crate::types::IssuingSettlement>> {
        let url = self.client.url("/v1/issuing/settlements", None);
        let mut resp: crate::types::GetIssuingSettlementsResponse = self
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
     * This function performs a `GET` to the `/v1/issuing/settlements/{settlement}` endpoint.
     *
     * <p>Retrieves an Issuing <code>Settlement</code> object.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `settlement: &str` -- The account's country.
     */
    pub async fn get_settlements_settlement(
        &self,
        settlement: &str,
    ) -> ClientResult<crate::types::IssuingSettlement> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/settlements/{}",
                crate::progenitor_support::encode_path(settlement),
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
     * This function performs a `POST` to the `/v1/issuing/settlements/{settlement}` endpoint.
     *
     * <p>Updates the specified Issuing <code>Settlement</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `settlement: &str` -- The account's country.
     */
    pub async fn post_settlements_settlement(
        &self,
        settlement: &str,
    ) -> ClientResult<crate::types::IssuingSettlement> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/settlements/{}",
                crate::progenitor_support::encode_path(settlement),
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
     * This function performs a `GET` to the `/v1/issuing/transactions` endpoint.
     *
     * <p>Returns a list of Issuing <code>Transaction</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     *
     * **Parameters:**
     *
     * * `card: &str` -- Only return transactions that belong to the given card.
     * * `cardholder: &str` -- Only return transactions that belong to the given cardholder.
     * * `created: &str` -- Only return transactions that were created during the given date interval.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `type_: crate::types::IssuingTransactionType` -- Only return transactions that have the given type. One of `capture` or `refund`.
     */
    pub async fn get_transactions(
        &self,
        card: &str,
        cardholder: &str,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        type_: crate::types::IssuingTransactionType,
    ) -> ClientResult<Vec<crate::types::IssuingTransaction>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !card.is_empty() {
            query_args.push(("card".to_string(), card.to_string()));
        }
        if !cardholder.is_empty() {
            query_args.push(("cardholder".to_string(), cardholder.to_string()));
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
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/transactions?{}", query_), None);
        let resp: crate::types::GetIssuingTransactionsResponse = self
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
     * This function performs a `GET` to the `/v1/issuing/transactions` endpoint.
     *
     * As opposed to `get_transactions`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of Issuing <code>Transaction</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
     */
    pub async fn get_all_transactions(
        &self,
        card: &str,
        cardholder: &str,
        _created: &str,
        type_: crate::types::IssuingTransactionType,
    ) -> ClientResult<Vec<crate::types::IssuingTransaction>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !card.is_empty() {
            query_args.push(("card".to_string(), card.to_string()));
        }
        if !cardholder.is_empty() {
            query_args.push(("cardholder".to_string(), cardholder.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuing/transactions?{}", query_), None);
        let mut resp: crate::types::GetIssuingTransactionsResponse = self
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
     * This function performs a `GET` to the `/v1/issuing/transactions/{transaction}` endpoint.
     *
     * <p>Retrieves an Issuing <code>Transaction</code> object.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `transaction: &str` -- The account's country.
     */
    pub async fn get_transactions_transaction(
        &self,
        transaction: &str,
    ) -> ClientResult<crate::types::IssuingTransaction> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/transactions/{}",
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
     * This function performs a `POST` to the `/v1/issuing/transactions/{transaction}` endpoint.
     *
     * <p>Updates the specified Issuing <code>Transaction</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `transaction: &str` -- The account's country.
     */
    pub async fn post_transactions_transaction(
        &self,
        transaction: &str,
    ) -> ClientResult<crate::types::IssuingTransaction> {
        let url = self.client.url(
            &format!(
                "/v1/issuing/transactions/{}",
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
}
