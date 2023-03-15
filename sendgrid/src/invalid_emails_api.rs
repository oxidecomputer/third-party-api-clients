use crate::Client;
use crate::ClientResult;

pub struct InvalidEmailsApi {
    pub client: Client,
}

impl InvalidEmailsApi {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        InvalidEmailsApi { client }
    }

    /**
     * Retrieve all invalid emails.
     *
     * This function performs a `GET` to the `/suppression/invalid_emails` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all invalid email addresses.**
     *
     * **Parameters:**
     *
     * * `start_time: i64` -- Refers start of the time range in unix timestamp when an invalid email was created (inclusive).
     * * `end_time: i64` -- Refers end of the time range in unix timestamp when an invalid email was created (inclusive).
     * * `limit: i64` -- Limit the number of results to be displayed per page.
     * * `offset: i64` -- Paging offset. The point in the list to begin displaying results.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_suppression_invalid_emails(
        &self,
        start_time: i64,
        end_time: i64,
        limit: i64,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::InvalidEmail>> {
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
            .url(&format!("/suppression/invalid_emails?{}", query_), None);
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
     * Retrieve all invalid emails.
     *
     * This function performs a `GET` to the `/suppression/invalid_emails` endpoint.
     *
     * As opposed to `get_suppression_invalid_emails`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a list of all invalid email addresses.**
     */
    pub async fn get_all_suppression_invalid_emails(
        &self,
        start_time: i64,
        end_time: i64,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::InvalidEmail>> {
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
            .url(&format!("/suppression/invalid_emails?{}", query_), None);
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
     * Delete invalid emails.
     *
     * This function performs a `DELETE` to the `/suppression/invalid_emails` endpoint.
     *
     * **This endpoint allows you to remove email addresses from your invalid email address list.**
     *
     * There are two options for deleting invalid email addresses:
     *
     * 1) You can delete all invalid email addresses by setting `delete_all` to true in the request body.
     * 2) You can delete some invalid email addresses by specifying certain addresses in an array in the request body.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_suppression_invalid_emails(
        &self,
        body: &crate::types::DeleteSuppressionBlocksRequest,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url("/suppression/invalid_emails", None);
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
     * Retrieve a specific invalid email.
     *
     * This function performs a `GET` to the `/suppression/invalid_emails/{email}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific invalid email addresses.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_suppression_invalid_emails_email(
        &self,
        email: &str,
    ) -> ClientResult<Vec<crate::types::InvalidEmail>> {
        let url = self.client.url(
            &format!(
                "/suppression/invalid_emails/{}",
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
     * Retrieve a specific invalid email.
     *
     * This function performs a `GET` to the `/suppression/invalid_emails/{email}` endpoint.
     *
     * As opposed to `get_suppression_invalid_emails_email`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a specific invalid email addresses.**
     */
    pub async fn get_all_suppression_invalid_emails_email(
        &self,
        email: &str,
    ) -> ClientResult<Vec<crate::types::InvalidEmail>> {
        let url = self.client.url(
            &format!(
                "/suppression/invalid_emails/{}",
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
     * Delete a specific invalid email.
     *
     * This function performs a `DELETE` to the `/suppression/invalid_emails/{email}` endpoint.
     *
     * **This endpoint allows you to remove a specific email address from the invalid email address list.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_suppression_invalid_emails_email(
        &self,
        email: &str,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/suppression/invalid_emails/{}",
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
