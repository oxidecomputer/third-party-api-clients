use crate::Client;
use crate::ClientResult;

pub struct SpamReportsApi {
    pub client: Client,
}

impl SpamReportsApi {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SpamReportsApi { client }
    }

    /**
     * Retrieve all spam reports.
     *
     * This function performs a `GET` to the `/suppression/spam_reports` endpoint.
     *
     * **This endpoint allows you to retrieve all spam reports.**
     *
     * **Parameters:**
     *
     * * `start_time: i64` -- The start of the time range when a spam report was created (inclusive). This is a unix timestamp.
     * * `end_time: i64` -- The end of the time range when a spam report was created (inclusive). This is a unix timestamp.
     * * `limit: i64` -- Limit the number of results to be displayed per page.
     * * `offset: i64` -- Paging offset. The point in the list to begin displaying results.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_suppression_spam_reports(
        &self,
        start_time: i64,
        end_time: i64,
        limit: i64,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::SpamReportsResponse>> {
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
            .url(&format!("/suppression/spam_reports?{}", query_), None);
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
     * Retrieve all spam reports.
     *
     * This function performs a `GET` to the `/suppression/spam_reports` endpoint.
     *
     * As opposed to `get_suppression_spam_reports`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all spam reports.**
     */
    pub async fn get_all_suppression_spam_reports(
        &self,
        start_time: i64,
        end_time: i64,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::SpamReportsResponse>> {
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
            .url(&format!("/suppression/spam_reports?{}", query_), None);
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
     * Delete spam reports.
     *
     * This function performs a `DELETE` to the `/suppression/spam_reports` endpoint.
     *
     * **This endpoint allows you to delete your spam reports.**
     *
     * Deleting a spam report will remove the suppression, meaning email will once again be sent to the previously suppressed address. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.
     *
     * There are two options for deleting spam reports:
     *
     * 1. You can delete all spam reports by setting the `delete_all` field to `true` in the request body.
     * 2. You can delete a list of select spam reports by specifying the email addresses in the `emails` array of the request body.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_suppression_spam_reports(
        &self,
        body: &crate::types::DeleteSuppressionBlocksRequest,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url("/suppression/spam_reports", None);
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieve a specific spam report.
     *
     * This function performs a `GET` to the `/suppression/spam_reports/{email}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific spam report by email address.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_suppression_spam_reports_email(
        &self,
        email: &str,
    ) -> ClientResult<Vec<crate::types::SpamReportsResponse>> {
        let url = self.client.url(
            &format!(
                "/suppression/spam_reports/{}",
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
     * Retrieve a specific spam report.
     *
     * This function performs a `GET` to the `/suppression/spam_reports/{email}` endpoint.
     *
     * As opposed to `get_suppression_spam_reports_email`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a specific spam report by email address.**
     */
    pub async fn get_all_suppression_spam_reports_email(
        &self,
        email: &str,
    ) -> ClientResult<Vec<crate::types::SpamReportsResponse>> {
        let url = self.client.url(
            &format!(
                "/suppression/spam_reports/{}",
                crate::progenitor_support::encode_path(email),
            ),
            None,
        );
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
     * Delete a specific spam report.
     *
     * This function performs a `DELETE` to the `/suppression/spam_reports/{email}` endpoint.
     *
     * **This endpoint allows you to delete a specific spam report by email address.**
     *
     * Deleting a spam report will remove the suppression, meaning email will once again be sent to the previously suppressed address. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_suppression_spam_reports_email(
        &self,
        email: &str,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/suppression/spam_reports/{}",
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
