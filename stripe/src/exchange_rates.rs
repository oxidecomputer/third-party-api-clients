use crate::Client;
use crate::ClientResult;

pub struct ExchangeRates {
    pub client: Client,
}

impl ExchangeRates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ExchangeRates { client }
    }

    /**
     * This function performs a `GET` to the `/v1/exchange_rates` endpoint.
     *
     * <p>Returns a list of objects that contain the rates at which foreign currencies are converted to one another. Only shows the currencies for which Stripe supports.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is the currency that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with the exchange rate for currency X your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and total number of supported payout currencies, and the default is the max.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is the currency that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with the exchange rate for currency X, your subsequent call can include `starting_after=X` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::ExchangeRate>> {
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
            .url(&format!("/v1/exchange_rates?{}", query_), None);
        let resp: crate::types::GetExchangeRatesResponse = self
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
     * This function performs a `GET` to the `/v1/exchange_rates` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of objects that contain the rates at which foreign currencies are converted to one another. Only shows the currencies for which Stripe supports.</p>
     */
    pub async fn get_all(&self) -> ClientResult<Vec<crate::types::ExchangeRate>> {
        let url = self.client.url("/v1/exchange_rates", None);
        let mut resp: crate::types::GetExchangeRatesResponse = self
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
     * This function performs a `GET` to the `/v1/exchange_rates/{rate_id}` endpoint.
     *
     * <p>Retrieves the exchange rates from the given currency to every supported currency.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `rate_id: &str` -- The account's country.
     */
    pub async fn get_rate(&self, rate_id: &str) -> ClientResult<crate::types::ExchangeRate> {
        let url = self.client.url(
            &format!(
                "/v1/exchange_rates/{}",
                crate::progenitor_support::encode_path(rate_id),
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
}
