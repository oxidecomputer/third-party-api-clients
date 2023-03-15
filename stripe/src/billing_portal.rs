use crate::Client;
use crate::ClientResult;

pub struct BillingPortal {
    pub client: Client,
}

impl BillingPortal {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        BillingPortal { client }
    }

    /**
     * This function performs a `GET` to the `/v1/billing_portal/configurations` endpoint.
     *
     * <p>Returns a list of configurations that describe the functionality of the customer portal.</p>
     *
     * **Parameters:**
     *
     * * `active: bool` -- Only return configurations that are active or inactive (e.g., pass `true` to only list active configurations).
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `is_default: bool` -- Only return the default or non-default configurations (e.g., pass `true` to only list the default configuration).
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_configurations(
        &self,
        active: bool,
        ending_before: &str,
        is_default: bool,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::PortalConfiguration>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if is_default {
            query_args.push(("is_default".to_string(), is_default.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/v1/billing_portal/configurations?{}", query_),
            None,
        );
        let resp: crate::types::GetBillingPortalConfigurationsResponse = self
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
     * This function performs a `GET` to the `/v1/billing_portal/configurations` endpoint.
     *
     * As opposed to `get_configurations`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of configurations that describe the functionality of the customer portal.</p>
     */
    pub async fn get_all_configurations(
        &self,
        active: bool,
        is_default: bool,
    ) -> ClientResult<Vec<crate::types::PortalConfiguration>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if is_default {
            query_args.push(("is_default".to_string(), is_default.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/v1/billing_portal/configurations?{}", query_),
            None,
        );
        let mut resp: crate::types::GetBillingPortalConfigurationsResponse = self
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
     * This function performs a `POST` to the `/v1/billing_portal/configurations` endpoint.
     *
     * <p>Creates a configuration that describes the functionality and behavior of a PortalSession</p>
     */
    pub async fn post_configuration(&self) -> ClientResult<crate::types::PortalConfiguration> {
        let url = self.client.url("/v1/billing_portal/configurations", None);
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
     * This function performs a `GET` to the `/v1/billing_portal/configurations/{configuration}` endpoint.
     *
     * <p>Retrieves a configuration that describes the functionality of the customer portal.</p>
     *
     * **Parameters:**
     *
     * * `configuration: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_configurations_configuration(
        &self,
        configuration: &str,
    ) -> ClientResult<crate::types::PortalConfiguration> {
        let url = self.client.url(
            &format!(
                "/v1/billing_portal/configurations/{}",
                crate::progenitor_support::encode_path(configuration),
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
     * This function performs a `POST` to the `/v1/billing_portal/configurations/{configuration}` endpoint.
     *
     * <p>Updates a configuration that describes the functionality of the customer portal.</p>
     *
     * **Parameters:**
     *
     * * `configuration: &str` -- The account's country.
     */
    pub async fn post_configurations_configuration(
        &self,
        configuration: &str,
    ) -> ClientResult<crate::types::PortalConfiguration> {
        let url = self.client.url(
            &format!(
                "/v1/billing_portal/configurations/{}",
                crate::progenitor_support::encode_path(configuration),
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
     * This function performs a `POST` to the `/v1/billing_portal/sessions` endpoint.
     *
     * <p>Creates a session of the customer portal.</p>
     */
    pub async fn post_session(&self) -> ClientResult<crate::types::PortalSession> {
        let url = self.client.url("/v1/billing_portal/sessions", None);
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
