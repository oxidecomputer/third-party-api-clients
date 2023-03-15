use crate::Client;
use crate::ClientResult;

pub struct WebhookEndpoints {
    pub client: Client,
}

impl WebhookEndpoints {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        WebhookEndpoints { client }
    }

    /**
     * This function performs a `GET` to the `/v1/webhook_endpoints` endpoint.
     *
     * <p>Returns a list of your webhook endpoints.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::WebhookEndpoint>> {
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
            .url(&format!("/v1/webhook_endpoints?{}", query_), None);
        let resp: crate::types::GetWebhookEndpointsResponse = self
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
     * This function performs a `GET` to the `/v1/webhook_endpoints` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your webhook endpoints.</p>
     */
    pub async fn get_all(&self) -> ClientResult<Vec<crate::types::WebhookEndpoint>> {
        let url = self.client.url("/v1/webhook_endpoints", None);
        let mut resp: crate::types::GetWebhookEndpointsResponse = self
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
     * This function performs a `POST` to the `/v1/webhook_endpoints` endpoint.
     *
     * <p>A webhook endpoint must have a <code>url</code> and a list of <code>enabled_events</code>. You may optionally specify the Boolean <code>connect</code> parameter. If set to true, then a Connect webhook endpoint that notifies the specified <code>url</code> about events from all connected accounts is created; otherwise an account webhook endpoint that notifies the specified <code>url</code> only about events from your account is created. You can also create webhook endpoints in the <a href="https://dashboard.stripe.com/account/webhooks">webhooks settings</a> section of the Dashboard.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::WebhookEndpoint> {
        let url = self.client.url("/v1/webhook_endpoints", None);
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
     * This function performs a `GET` to the `/v1/webhook_endpoints/{webhook_endpoint}` endpoint.
     *
     * <p>Retrieves the webhook endpoint with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `webhook_endpoint: &str` -- The account's country.
     */
    pub async fn get_endpoint(
        &self,
        webhook_endpoint: &str,
    ) -> ClientResult<crate::types::WebhookEndpoint> {
        let url = self.client.url(
            &format!(
                "/v1/webhook_endpoints/{}",
                crate::progenitor_support::encode_path(webhook_endpoint),
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
     * This function performs a `POST` to the `/v1/webhook_endpoints/{webhook_endpoint}` endpoint.
     *
     * <p>Updates the webhook endpoint. You may edit the <code>url</code>, the list of <code>enabled_events</code>, and the status of your endpoint.</p>
     *
     * **Parameters:**
     *
     * * `webhook_endpoint: &str` -- The account's country.
     */
    pub async fn post_endpoint(
        &self,
        webhook_endpoint: &str,
    ) -> ClientResult<crate::types::WebhookEndpoint> {
        let url = self.client.url(
            &format!(
                "/v1/webhook_endpoints/{}",
                crate::progenitor_support::encode_path(webhook_endpoint),
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
     * This function performs a `DELETE` to the `/v1/webhook_endpoints/{webhook_endpoint}` endpoint.
     *
     * <p>You can also delete webhook endpoints via the <a href="https://dashboard.stripe.com/account/webhooks">webhook endpoint management</a> page of the Stripe dashboard.</p>
     *
     * **Parameters:**
     *
     * * `webhook_endpoint: &str` -- The account's country.
     */
    pub async fn delete_endpoint(
        &self,
        webhook_endpoint: &str,
    ) -> ClientResult<crate::types::DeletedWebhookEndpoint> {
        let url = self.client.url(
            &format!(
                "/v1/webhook_endpoints/{}",
                crate::progenitor_support::encode_path(webhook_endpoint),
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
