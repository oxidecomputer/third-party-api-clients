use anyhow::Result;

use crate::Client;

pub struct TestHelpers {
    pub client: Client,
}

impl TestHelpers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TestHelpers { client }
    }

    /**
    * This function performs a `POST` to the `/v1/test_helpers/terminal/readers/{reader}/present_payment_method` endpoint.
    *
    * <p>Presents a payment method on a simulated reader. Can be used to simulate accepting a payment, saving a card or refunding a transaction.</p>
    *
    * **Parameters:**
    *
    * * `reader: &str` -- The account's country.
    */
    pub async fn post_terminal_readers_reader_present_payment_method(
        &self,
        reader: &str,
    ) -> Result<crate::types::TerminalReader> {
        let url = format!(
            "/v1/test_helpers/terminal/readers/{}/present_payment_method",
            crate::progenitor_support::encode_path(reader),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/test_helpers/test_clocks` endpoint.
    *
    * <p>Returns a list of your test clocks.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_clocks(
        &self,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> Result<Vec<crate::types::TestClock>> {
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
        let url = format!("/v1/test_helpers/test_clocks?{}", query_);

        let resp: crate::types::GetTestHelpersClocksResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }

    /**
    * This function performs a `GET` to the `/v1/test_helpers/test_clocks` endpoint.
    *
    * As opposed to `get_clocks`, this function returns all the pages of the request at once.
    *
    * <p>Returns a list of your test clocks.</p>
    */
    pub async fn get_all_clocks(&self) -> Result<Vec<crate::types::TestClock>> {
        let url = "/v1/test_helpers/test_clocks".to_string();
        let mut resp: crate::types::GetTestHelpersClocksResponse =
            self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(p) = o.get("id") {
                        if let serde_json::Value::String(s) = p {
                            page = s.to_string();
                        }
                    }
                }
            }

            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?startng_after={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&starting_after={}", url, page), None)
                    .await?;
            }

            data.append(&mut resp.data);

            has_more = resp.has_more;
        }

        // Return our response data.
        Ok(data.to_vec())
    }

    /**
    * This function performs a `POST` to the `/v1/test_helpers/test_clocks` endpoint.
    *
    * <p>Creates a new test clock that can be attached to new customers and quotes.</p>
    */
    pub async fn post_clock(&self) -> Result<crate::types::TestClock> {
        let url = "/v1/test_helpers/test_clocks".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/test_helpers/test_clocks/{test_clock}` endpoint.
    *
    * <p>Retrieves a test clock.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `test_clock: &str` -- The account's country.
    */
    pub async fn get_clocks_clock(&self, test_clock: &str) -> Result<crate::types::TestClock> {
        let url = format!(
            "/v1/test_helpers/test_clocks/{}",
            crate::progenitor_support::encode_path(test_clock),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/test_helpers/test_clocks/{test_clock}` endpoint.
    *
    * <p>Deletes a test clock.</p>
    *
    * **Parameters:**
    *
    * * `test_clock: &str` -- The account's country.
    */
    pub async fn delete_clocks_clock(
        &self,
        test_clock: &str,
    ) -> Result<crate::types::DeletedTestClock> {
        let url = format!(
            "/v1/test_helpers/test_clocks/{}",
            crate::progenitor_support::encode_path(test_clock),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/test_helpers/test_clocks/{test_clock}/advance` endpoint.
    *
    * <p>Starts advancing a test clock to a specified time in the future. Advancement is done when status changes to <code>Ready</code>.</p>
    *
    * **Parameters:**
    *
    * * `test_clock: &str` -- The account's country.
    */
    pub async fn post_clocks_clock_advance(
        &self,
        test_clock: &str,
    ) -> Result<crate::types::TestClock> {
        let url = format!(
            "/v1/test_helpers/test_clocks/{}/advance",
            crate::progenitor_support::encode_path(test_clock),
        );

        self.client.post(&url, None).await
    }
}
