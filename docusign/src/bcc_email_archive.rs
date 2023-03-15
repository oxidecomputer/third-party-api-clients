use crate::Client;
use crate::ClientResult;

pub struct BccEmailArchive {
    pub client: Client,
}

impl BccEmailArchive {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        BccEmailArchive { client }
    }

    /**
     * Gets the BCC email archive configurations for an account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/settings/bcc_email_archives` endpoint.
     *
     * This method retrieves all of the BCC email archive configurations associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- (Optional) The index position within the total result set from which to start returning values. The default value is `0`.
     */
    pub async fn get_list(
        &self,
        account_id: &str,
        count: &str,
        start_position: &str,
    ) -> ClientResult<crate::types::BccEmailArchiveList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/bcc_email_archives?{}",
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
     * Creates a BCC email archive configuration.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/settings/bcc_email_archives` endpoint.
     *
     * This method creates a BCC email archive configuration for an account (adds a BCC email address to the account for archiving the emails that DocuSign generates).
     *
     * The only property that you must set in the request body is the BCC email address that you want to use.
     *
     * **Note**: An account can have up to five active and pending email archive addresses combined, but you must use this method to add them to the account one at a time. Each email address is considered a separate BCC email archive configuration.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn post(
        &self,
        account_id: &str,
        body: &crate::types::BccEmailArchiveData,
    ) -> ClientResult<crate::types::BccEmailArchiveData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/bcc_email_archives",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
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
     * Gets a BCC email archive configuration and its history.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/settings/bcc_email_archives/{bccEmailArchiveId}` endpoint.
     *
     * This method returns a specific BCC email archive configuration for an account, as well as the history of changes to the email address.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `bcc_email_archive_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- (Optional) The index position within the total result set from which to start returning values. The default value is `0`.
     */
    pub async fn get_history_list(
        &self,
        account_id: &str,
        bcc_email_archive_id: &str,
        count: &str,
        start_position: &str,
    ) -> ClientResult<crate::types::BccEmailArchiveHistoryList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/bcc_email_archives/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(bcc_email_archive_id),
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
     * Deletes a BCC email archive configuration.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/settings/bcc_email_archives/{bccEmailArchiveId}` endpoint.
     *
     * This method deletes a BCC email archive configuration from an account.
     *
     * When you use this method, the status of the BCC email archive configuration switches to `closed` and the BCC email address is no longer used to archive DocuSign-generated email messages.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `bcc_email_archive_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete(&self, account_id: &str, bcc_email_archive_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/bcc_email_archives/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(bcc_email_archive_id),
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
