use crate::Client;
use crate::ClientResult;

pub struct Terminal {
    pub client: Client,
}

impl Terminal {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Terminal { client }
    }

    /**
     * This function performs a `POST` to the `/v1/terminal/connection_tokens` endpoint.
     *
     * <p>To connect to a reader the Stripe Terminal SDK needs to retrieve a short-lived connection token from Stripe, proxied through your server. On your backend, add an endpoint that creates and returns a connection token.</p>
     */
    pub async fn post_connection_token(
        &self,
    ) -> ClientResult<crate::types::TerminalConnectionToken> {
        let url = self.client.url("/v1/terminal/connection_tokens", None);
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
     * This function performs a `GET` to the `/v1/terminal/locations` endpoint.
     *
     * <p>Returns a list of <code>Location</code> objects.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_locations(
        &self,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::TerminalLocation>> {
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
            .url(&format!("/v1/terminal/locations?{}", query_), None);
        let resp: crate::types::TerminalLocationList = self
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
     * This function performs a `GET` to the `/v1/terminal/locations` endpoint.
     *
     * As opposed to `get_locations`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of <code>Location</code> objects.</p>
     */
    pub async fn get_all_locations(&self) -> ClientResult<Vec<crate::types::TerminalLocation>> {
        let url = self.client.url("/v1/terminal/locations", None);
        let mut resp: crate::types::TerminalLocationList = self
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
     * This function performs a `POST` to the `/v1/terminal/locations` endpoint.
     *
     * <p>Creates a new <code>Location</code> object.
     * For further details, including which address fields are required in each country, see the <a href="/docs/terminal/fleet/locations">Manage locations</a> guide.</p>
     */
    pub async fn post_location(&self) -> ClientResult<crate::types::TerminalLocation> {
        let url = self.client.url("/v1/terminal/locations", None);
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
     * This function performs a `GET` to the `/v1/terminal/locations/{location}` endpoint.
     *
     * <p>Retrieves a <code>Location</code> object.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `location: &str` -- The account's country.
     */
    pub async fn get_locations_location(
        &self,
        location: &str,
    ) -> ClientResult<crate::types::GetTerminalLocationResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/locations/{}",
                crate::progenitor_support::encode_path(location),
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
     * This function performs a `POST` to the `/v1/terminal/locations/{location}` endpoint.
     *
     * <p>Updates a <code>Location</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `location: &str` -- The account's country.
     */
    pub async fn post_locations_location(
        &self,
        location: &str,
    ) -> ClientResult<crate::types::GetTerminalLocationResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/locations/{}",
                crate::progenitor_support::encode_path(location),
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
     * This function performs a `DELETE` to the `/v1/terminal/locations/{location}` endpoint.
     *
     * <p>Deletes a <code>Location</code> object.</p>
     *
     * **Parameters:**
     *
     * * `location: &str` -- The account's country.
     */
    pub async fn delete_locations_location(
        &self,
        location: &str,
    ) -> ClientResult<crate::types::DeletedTerminalLocation> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/locations/{}",
                crate::progenitor_support::encode_path(location),
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
     * This function performs a `GET` to the `/v1/terminal/readers` endpoint.
     *
     * <p>Returns a list of <code>Reader</code> objects.</p>
     *
     * **Parameters:**
     *
     * * `device_type: crate::types::DeviceType` -- Type of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, or `verifone_P400`.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `location: &str` -- A location ID to filter the response list to only readers at the specific location.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::CustomerAcceptanceType` -- The type of customer acceptance information included with the Mandate. One of `online` or `offline`.
     */
    pub async fn get_readers(
        &self,
        device_type: crate::types::DeviceType,
        ending_before: &str,
        limit: i64,
        location: &str,
        starting_after: &str,
        status: crate::types::CustomerAcceptanceType,
    ) -> ClientResult<Vec<crate::types::TerminalReader>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !device_type.to_string().is_empty() {
            query_args.push(("device_type".to_string(), device_type.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !location.is_empty() {
            query_args.push(("location".to_string(), location.to_string()));
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
            .url(&format!("/v1/terminal/readers?{}", query_), None);
        let resp: crate::types::TerminalReaderRetrieve = self
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
     * This function performs a `GET` to the `/v1/terminal/readers` endpoint.
     *
     * As opposed to `get_readers`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of <code>Reader</code> objects.</p>
     */
    pub async fn get_all_readers(
        &self,
        device_type: crate::types::DeviceType,
        location: &str,
        status: crate::types::CustomerAcceptanceType,
    ) -> ClientResult<Vec<crate::types::TerminalReader>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !device_type.to_string().is_empty() {
            query_args.push(("device_type".to_string(), device_type.to_string()));
        }
        if !location.is_empty() {
            query_args.push(("location".to_string(), location.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/terminal/readers?{}", query_), None);
        let mut resp: crate::types::TerminalReaderRetrieve = self
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
     * This function performs a `POST` to the `/v1/terminal/readers` endpoint.
     *
     * <p>Creates a new <code>Reader</code> object.</p>
     */
    pub async fn post_reader(&self) -> ClientResult<crate::types::TerminalReader> {
        let url = self.client.url("/v1/terminal/readers", None);
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
     * This function performs a `GET` to the `/v1/terminal/readers/{reader}` endpoint.
     *
     * <p>Retrieves a <code>Reader</code> object.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `reader: &str` -- The account's country.
     */
    pub async fn get_readers_reader(
        &self,
        reader: &str,
    ) -> ClientResult<crate::types::GetTerminalReadersReaderResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/readers/{}",
                crate::progenitor_support::encode_path(reader),
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
     * This function performs a `POST` to the `/v1/terminal/readers/{reader}` endpoint.
     *
     * <p>Updates a <code>Reader</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `reader: &str` -- The account's country.
     */
    pub async fn post_readers_reader(
        &self,
        reader: &str,
    ) -> ClientResult<crate::types::GetTerminalReadersReaderResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/readers/{}",
                crate::progenitor_support::encode_path(reader),
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
     * This function performs a `DELETE` to the `/v1/terminal/readers/{reader}` endpoint.
     *
     * <p>Deletes a <code>Reader</code> object.</p>
     *
     * **Parameters:**
     *
     * * `reader: &str` -- The account's country.
     */
    pub async fn delete_readers_reader(
        &self,
        reader: &str,
    ) -> ClientResult<crate::types::DeletedTerminalReader> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/readers/{}",
                crate::progenitor_support::encode_path(reader),
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
     * This function performs a `POST` to the `/v1/terminal/readers/{reader}/cancel_action` endpoint.
     *
     * <p>Cancels the current reader action.</p>
     *
     * **Parameters:**
     *
     * * `reader: &str` -- The account's country.
     */
    pub async fn post_readers_reader_cancel_action(
        &self,
        reader: &str,
    ) -> ClientResult<crate::types::TerminalReader> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/readers/{}/cancel_action",
                crate::progenitor_support::encode_path(reader),
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
     * This function performs a `POST` to the `/v1/terminal/readers/{reader}/process_payment_intent` endpoint.
     *
     * <p>Initiates a payment flow on a Reader.</p>
     *
     * **Parameters:**
     *
     * * `reader: &str` -- The account's country.
     */
    pub async fn post_readers_reader_process_payment_intent(
        &self,
        reader: &str,
    ) -> ClientResult<crate::types::TerminalReader> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/readers/{}/process_payment_intent",
                crate::progenitor_support::encode_path(reader),
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
     * This function performs a `POST` to the `/v1/terminal/readers/{reader}/process_setup_intent` endpoint.
     *
     * <p>Initiates a setup intent flow on a Reader.</p>
     *
     * **Parameters:**
     *
     * * `reader: &str` -- The account's country.
     */
    pub async fn post_readers_reader_process_setup_intent(
        &self,
        reader: &str,
    ) -> ClientResult<crate::types::TerminalReader> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/readers/{}/process_setup_intent",
                crate::progenitor_support::encode_path(reader),
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
     * This function performs a `POST` to the `/v1/terminal/readers/{reader}/set_reader_display` endpoint.
     *
     * <p>Sets reader display to show cart details.</p>
     *
     * **Parameters:**
     *
     * * `reader: &str` -- The account's country.
     */
    pub async fn post_readers_reader_set_display(
        &self,
        reader: &str,
    ) -> ClientResult<crate::types::TerminalReader> {
        let url = self.client.url(
            &format!(
                "/v1/terminal/readers/{}/set_reader_display",
                crate::progenitor_support::encode_path(reader),
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
