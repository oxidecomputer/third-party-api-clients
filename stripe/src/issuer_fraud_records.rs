use crate::Client;
use crate::ClientResult;

pub struct IssuerFraudRecords {
    pub client: Client,
}

impl IssuerFraudRecords {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        IssuerFraudRecords { client }
    }

    /**
     * This function performs a `GET` to the `/v1/issuer_fraud_records` endpoint.
     *
     * <p>Returns a list of issuer fraud records.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- Only return issuer fraud records for the charge specified by this charge ID.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        charge: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::IssuerFraudRecord>> {
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
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuer_fraud_records?{}", query_), None);
        let resp: crate::types::RadarIssuerFraudRecordList = self
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
     * This function performs a `GET` to the `/v1/issuer_fraud_records` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of issuer fraud records.</p>
     */
    pub async fn get_all(
        &self,
        charge: &str,
    ) -> ClientResult<Vec<crate::types::IssuerFraudRecord>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/issuer_fraud_records?{}", query_), None);
        let mut resp: crate::types::RadarIssuerFraudRecordList = self
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
     * This function performs a `GET` to the `/v1/issuer_fraud_records/{issuer_fraud_record}` endpoint.
     *
     * <p>Retrieves the details of an issuer fraud record that has previously been created. </p>
     *
     * <p>Please refer to the <a href="#issuer_fraud_record_object">issuer fraud record</a> object reference for more details.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `issuer_fraud_record: &str` -- The account's country.
     */
    pub async fn get_record(
        &self,
        issuer_fraud_record: &str,
    ) -> ClientResult<crate::types::IssuerFraudRecord> {
        let url = self.client.url(
            &format!(
                "/v1/issuer_fraud_records/{}",
                crate::progenitor_support::encode_path(issuer_fraud_record),
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
