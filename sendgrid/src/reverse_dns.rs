use crate::Client;
use crate::ClientResult;

pub struct ReverseDns {
    pub client: Client,
}

impl ReverseDns {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ReverseDns { client }
    }

    /**
     * Retrieve all reverse DNS records.
     *
     * This function performs a `GET` to the `/whitelabel/ips` endpoint.
     *
     * **This endpoint allows you to retrieve all of the Reverse DNS records created by this account.**
     *
     * You may include a search key by using the `ip` query string parameter. This enables you to perform a prefix search for a given IP segment (e.g., `?ip="192."`).
     *
     * Use the `limit` query string parameter to reduce the number of records returned. All records will be returned if you have fewer records than the specified limit.
     *
     * The `offset` query string parameter allows you to specify a non-zero index from which records will be returned. For example, if you have ten records, `?offset=5` will return the last five records (at indexes 5 through 9). The list starts at index zero.
     *
     * **Parameters:**
     *
     * * `limit: i64` -- The maximum number of results to retrieve.
     * * `offset: i64` -- The point in the list of results to begin retrieving IP addresses from.
     * * `ip: &str` -- The IP address segment that you'd like to use in a prefix search.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_whitelabel_ips(
        &self,
        limit: i64,
        offset: i64,
        ip: &str,
    ) -> ClientResult<Vec<crate::types::ReverseDns>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ip.is_empty() {
            query_args.push(("ip".to_string(), ip.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/ips?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve all reverse DNS records.
     *
     * This function performs a `GET` to the `/whitelabel/ips` endpoint.
     *
     * As opposed to `get_whitelabel_ips`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all of the Reverse DNS records created by this account.**
     *
     * You may include a search key by using the `ip` query string parameter. This enables you to perform a prefix search for a given IP segment (e.g., `?ip="192."`).
     *
     * Use the `limit` query string parameter to reduce the number of records returned. All records will be returned if you have fewer records than the specified limit.
     *
     * The `offset` query string parameter allows you to specify a non-zero index from which records will be returned. For example, if you have ten records, `?offset=5` will return the last five records (at indexes 5 through 9). The list starts at index zero.
     */
    pub async fn get_all_whitelabel_ips(
        &self,
        offset: i64,
        ip: &str,
    ) -> ClientResult<Vec<crate::types::ReverseDns>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ip.is_empty() {
            query_args.push(("ip".to_string(), ip.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/whitelabel/ips?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Set up reverse DNS.
     *
     * This function performs a `POST` to the `/whitelabel/ips` endpoint.
     *
     * **This endpoint allows you to set up reverse DNS.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_whitelabel_ip(
        &self,
        body: &crate::types::PostWhitelabelIpsRequest,
    ) -> ClientResult<crate::types::ReverseDns> {
        let url = self.client.url("/whitelabel/ips", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Validate a reverse DNS record.
     *
     * This function performs a `POST` to the `/whitelabel/ips/{id}/validate` endpoint.
     *
     * **This endpoint allows you to validate a reverse DNS record.**
     *
     * Always check the `valid` property of the response’s `validation_results.a_record` object. This field will indicate whether it was possible to validate the reverse DNS record. If the `validation_results.a_record.valid` is `false`, this indicates only that Twilio SendGrid could not determine the validity your reverse DNS record — it may still be valid.
     *
     * If validity couldn’t be determined, you can check the value of `validation_results.a_record.reason` to find out why.
     *
     * You can retrieve the IDs associated with all your reverse DNS records using the "Retrieve all reverse DNS records" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_whitelabel_ips_validate(
        &self,
        id: &str,
    ) -> ClientResult<crate::types::PostWhitelabelIpsValidateResponse> {
        let url = self.client.url(
            &format!(
                "/whitelabel/ips/{}/validate",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a reverse DNS record.
     *
     * This function performs a `GET` to the `/whitelabel/ips/{id}` endpoint.
     *
     * **This endpoint allows you to retrieve a reverse DNS record.**
     *
     * You can retrieve the IDs associated with all your reverse DNS records using the "Retrieve all reverse DNS records" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_whitelabel_ip(&self, id: &str) -> ClientResult<crate::types::ReverseDns> {
        let url = self.client.url(
            &format!(
                "/whitelabel/ips/{}",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a reverse DNS record.
     *
     * This function performs a `DELETE` to the `/whitelabel/ips/{id}` endpoint.
     *
     * **This endpoint allows you to delete a reverse DNS record.**
     *
     * A call to this endpoint will respond with a 204 status code if the deletion was successful.
     *
     * You can retrieve the IDs associated with all your reverse DNS records using the "Retrieve all reverse DNS records" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_whitelabel_ips(&self, id: &str) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/whitelabel/ips/{}",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
