use crate::Client;
use crate::ClientResult;

pub struct ApplePay {
    pub client: Client,
}

impl ApplePay {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ApplePay { client }
    }

    /**
     * This function performs a `GET` to the `/v1/apple_pay/domains` endpoint.
     *
     * <p>List apple pay domains.</p>
     *
     * **Parameters:**
     *
     * * `domain_name: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_domains(
        &self,
        domain_name: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApplePayDomain>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !domain_name.is_empty() {
            query_args.push(("domain_name".to_string(), domain_name.to_string()));
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
            .url(&format!("/v1/apple_pay/domains?{}", query_), None);
        let resp: crate::Response<crate::types::ApplePayDomainList> = self
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
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.data.to_vec(),
        ))
    }
    /**
     * This function performs a `GET` to the `/v1/apple_pay/domains` endpoint.
     *
     * As opposed to `get_domains`, this function returns all the pages of the request at once.
     *
     * <p>List apple pay domains.</p>
     */
    pub async fn get_all_domains(
        &self,
        domain_name: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApplePayDomain>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !domain_name.is_empty() {
            query_args.push(("domain_name".to_string(), domain_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/apple_pay/domains?{}", query_), None);
        let crate::Response::<crate::types::ApplePayDomainList> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut data = body.data;
        let mut has_more = body.has_more;
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
                crate::Response::<crate::types::ApplePayDomainList> {
                    status,
                    headers,
                    body,
                } = self
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
                crate::Response::<crate::types::ApplePayDomainList> {
                    status,
                    headers,
                    body,
                } = self
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

            data.append(&mut body.data);

            has_more = body.has_more;
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, data.to_vec()))
    }
    /**
     * This function performs a `POST` to the `/v1/apple_pay/domains` endpoint.
     *
     * <p>Create an apple pay domain.</p>
     */
    pub async fn post_domain(&self) -> ClientResult<crate::Response<crate::types::ApplePayDomain>> {
        let url = self.client.url("/v1/apple_pay/domains", None);
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
     * This function performs a `GET` to the `/v1/apple_pay/domains/{domain}` endpoint.
     *
     * <p>Retrieve an apple pay domain.</p>
     *
     * **Parameters:**
     *
     * * `domain: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_domains_domain(
        &self,
        domain: &str,
    ) -> ClientResult<crate::Response<crate::types::ApplePayDomain>> {
        let url = self.client.url(
            &format!(
                "/v1/apple_pay/domains/{}",
                crate::progenitor_support::encode_path(domain),
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
     * This function performs a `DELETE` to the `/v1/apple_pay/domains/{domain}` endpoint.
     *
     * <p>Delete an apple pay domain.</p>
     *
     * **Parameters:**
     *
     * * `domain: &str` -- The account's country.
     */
    pub async fn delete_domains_domain(
        &self,
        domain: &str,
    ) -> ClientResult<crate::Response<crate::types::DeletedApplePayDomain>> {
        let url = self.client.url(
            &format!(
                "/v1/apple_pay/domains/{}",
                crate::progenitor_support::encode_path(domain),
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
