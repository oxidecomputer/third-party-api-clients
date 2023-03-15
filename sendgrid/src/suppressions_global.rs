use crate::Client;
use crate::ClientResult;

pub struct SuppressionsGlobal {
    pub client: Client,
}

impl SuppressionsGlobal {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SuppressionsGlobal { client }
    }

    /**
     * Add recipient addresses to the global suppression group.
     *
     * This function performs a `POST` to the `/asm/suppressions/global` endpoint.
     *
     * **This endpoint allows you to add one or more email addresses to the global suppressions group.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_asm(
        &self,
        body: &crate::types::SuppressionsRequestBody,
    ) -> ClientResult<crate::types::SuppressionsRequestBody> {
        let url = self.client.url("/asm/suppressions/global", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve all global suppressions.
     *
     * This function performs a `GET` to the `/suppression/unsubscribes` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all email address that are globally suppressed.**
     *
     * **Parameters:**
     *
     * * `start_time: i64` -- Refers start of the time range in unix timestamp when an unsubscribe email was created (inclusive).
     * * `end_time: i64` -- Refers end of the time range in unix timestamp when an unsubscribe email was created (inclusive).
     * * `limit: i64` -- The number of results to display on each page.
     * * `offset: i64` -- The point in the list of results to begin displaying global suppressions.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_suppression_unsubscribes(
        &self,
        start_time: i64,
        end_time: i64,
        limit: i64,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::GetSuppressionUnsubscribesResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if end_time > 0 {
            query_args.push(("end_time".to_string(), end_time.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if start_time > 0 {
            query_args.push(("start_time".to_string(), start_time.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/suppression/unsubscribes?{}", query_), None);
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
     * Retrieve all global suppressions.
     *
     * This function performs a `GET` to the `/suppression/unsubscribes` endpoint.
     *
     * As opposed to `get_suppression_unsubscribes`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a list of all email address that are globally suppressed.**
     */
    pub async fn get_all_suppression_unsubscribes(
        &self,
        start_time: i64,
        end_time: i64,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::GetSuppressionUnsubscribesResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if end_time > 0 {
            query_args.push(("end_time".to_string(), end_time.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if start_time > 0 {
            query_args.push(("start_time".to_string(), start_time.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/suppression/unsubscribes?{}", query_), None);
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
     * Retrieve a Global Suppression.
     *
     * This function performs a `GET` to the `/asm/suppressions/global/{email}` endpoint.
     *
     * **This endpoint allows you to retrieve a global suppression. You can also use this endpoint to confirm if an email address is already globally suppresed.**
     *
     * If the email address you include in the URL path parameter `{email}` is already globally suppressed, the response will include that email address. If the address you enter for `{email}` is not globally suppressed, an empty JSON object `{}` will be returned.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_asm_email(
        &self,
        email: &str,
    ) -> ClientResult<crate::types::RetrieveAGlobalSuppressionResponse> {
        let url = self.client.url(
            &format!(
                "/asm/suppressions/global/{}",
                crate::progenitor_support::encode_path(email),
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
     * Delete a Global Suppression.
     *
     * This function performs a `DELETE` to the `/asm/suppressions/global/{email}` endpoint.
     *
     * **This endpoint allows you to remove an email address from the global suppressions group.**
     *
     * Deleting a suppression group will remove the suppression, meaning email will once again be sent to the previously suppressed addresses. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_asm_email(&self, email: &str) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/asm/suppressions/global/{}",
                crate::progenitor_support::encode_path(email),
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
