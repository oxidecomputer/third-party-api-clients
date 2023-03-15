use crate::Client;
use crate::ClientResult;

pub struct ConnectEvents {
    pub client: Client,
}

impl ConnectEvents {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ConnectEvents { client }
    }

    /**
     * Republishes Connect information for multiple envelopes.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/connect/envelopes/retry_queue` endpoint.
     *
     * Republishes Connect information for the  specified set of envelopes. The primary use is to republish Connect post failures by including envelope IDs for the envelopes that failed to post in the request. The list of envelope IDs that failed to post correctly can be retrieved by calling to [Connect::listEventLogs](https://developers.docusign.com/docs/esign-rest-api/reference/Connect/ConnectEvents/list) retrieve the failure log.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_publish_put_retry(
        &self,
        account_id: &str,
        body: &crate::types::ConnectFailureFilter,
    ) -> ClientResult<crate::types::ConnectFailureResults> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/connect/envelopes/retry_queue",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Republishes Connect information for the specified envelope.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/connect/envelopes/{envelopeId}/retry_queue` endpoint.
     *
     * Republishes Connect information for the specified envelope.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_publish_put_retry_envelope(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ConnectFailureResults> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/connect/envelopes/{}/retry_queue",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Gets the Connect failure log information.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/connect/failures` endpoint.
     *
     * Retrieves the Connect failure log information. You can use this log to determine which envelopes failed to post, in order to create a republish request.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `from_date: &str` -- The start date for a date range in UTC DateTime format.
     *   
     *   **Note**: If this property is null, no date filtering is applied.
     * * `to_date: &str` -- The end of a search date range in UTC DateTime format. When you use this parameter, only templates created up to this date and time are returned.
     *   
     *   **Note**: If this property is null, the value defaults to the current date.
     */
    pub async fn connect_failures_get_log(
        &self,
        account_id: &str,
        from_date: &str,
        to_date: &str,
    ) -> ClientResult<crate::types::ConnectLogs> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from_date.is_empty() {
            query_args.push(("from_date".to_string(), from_date.to_string()));
        }
        if !to_date.is_empty() {
            query_args.push(("to_date".to_string(), to_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/connect/failures?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
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
     * Deletes a Connect failure log entry.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/connect/failures/{failureId}` endpoint.
     *
     * Deletes the Connect failure log information for the specified entry.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `failure_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_failures_delete_failure_log(
        &self,
        account_id: &str,
        failure_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/connect/failures/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(failure_id),
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
    /**
     * Gets the Connect log.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/connect/logs` endpoint.
     *
     * Retrieves a list of connect log entries for your account.
     *
     * **Note**: The `enableLog` setting in the Connect configuration must be set to true to enable logging. If logging is not enabled, then no log entries are recorded.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `from_date: &str` -- The start date for a date range in UTC DateTime format.
     *   
     *   **Note**: If this property is null, no date filtering is applied.
     * * `to_date: &str` -- The end of a search date range in UTC DateTime format. When you use this parameter, only templates created up to this date and time are returned.
     *   
     *   **Note**: If this property is null, the value defaults to the current date.
     */
    pub async fn connect_log_get_log(
        &self,
        account_id: &str,
        from_date: &str,
        to_date: &str,
    ) -> ClientResult<crate::types::ConnectLogs> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from_date.is_empty() {
            query_args.push(("from_date".to_string(), from_date.to_string()));
        }
        if !to_date.is_empty() {
            query_args.push(("to_date".to_string(), to_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/connect/logs?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
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
     * Deletes a list of Connect log entries.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/connect/logs` endpoint.
     *
     * Deletes a list of Connect log entries for your account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_log_delete_logs(&self, account_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/connect/logs",
                crate::progenitor_support::encode_path(account_id),
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
    /**
     * Gets a Connect log entry.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/connect/logs/{logId}` endpoint.
     *
     * Retrieves the specified Connect log entry for your account.
     *
     * **Note**: The `enableLog` setting in the Connect configuration must be set to true to enable logging. If logging is not enabled, then no log entries are recorded.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `log_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `additional_info: &str` -- When set to **true**, the response includes the `connectDebugLog` information.
     */
    pub async fn connect_log_get(
        &self,
        account_id: &str,
        log_id: &str,
        additional_info: &str,
    ) -> ClientResult<crate::types::ConnectLog> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !additional_info.is_empty() {
            query_args.push(("additional_info".to_string(), additional_info.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/connect/logs/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(log_id),
                query_
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
     * Deletes a specified Connect log entry.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/connect/logs/{logId}` endpoint.
     *
     * Deletes a specified entry from the Connect Log.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `log_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_log_delete(&self, account_id: &str, log_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/connect/logs/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(log_id),
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
