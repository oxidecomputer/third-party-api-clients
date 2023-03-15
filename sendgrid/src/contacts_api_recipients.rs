use crate::Client;
use crate::ClientResult;

pub struct ContactsApiRecipients {
    pub client: Client,
}

impl ContactsApiRecipients {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ContactsApiRecipients { client }
    }

    /**
     * Retrieve recipients.
     *
     * This function performs a `GET` to the `/contactdb/recipients` endpoint.
     *
     * **This endpoint allows you to retrieve all of your Marketing Campaigns recipients.**
     *
     * Batch deletion of a page makes it possible to receive an empty page of recipients before reaching the end of
     * the list of recipients. To avoid this issue; iterate over pages until a 404 is retrieved.
     *
     * **Parameters:**
     *
     * * `page: i64` -- Page index of first recipients to return (must be a positive integer).
     * * `page_size: i64` -- Number of recipients to return at a time (must be a positive integer between 1 and 1000).
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_recipients(
        &self,
        page: i64,
        page_size: i64,
    ) -> ClientResult<crate::types::ListRecipientsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/contactdb/recipients?{}", query_), None);
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
     * Add recipients.
     *
     * This function performs a `POST` to the `/contactdb/recipients` endpoint.
     *
     * **This endpoint allows you to add a Marketing Campaigns recipient.**
     *
     * You can add custom field data as a parameter on this endpoint. We have provided an example using some of the default custom fields SendGrid provides.
     *
     * The rate limit is three requests every 2 seconds. You can upload 1000  contacts per request. So the maximum upload rate is 1500 recipients per second.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_contactdb_recipient(
        &self,
        body: &[crate::types::PostContactdbRecipientsRequest],
    ) -> ClientResult<crate::types::ContactDbRecipientResponse> {
        let url = self.client.url("/contactdb/recipients", None);
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
     * Delete Recipients.
     *
     * This function performs a `DELETE` to the `/contactdb/recipients` endpoint.
     *
     * **This endpoint allows you to deletes one or more recipients.**
     *
     * The body of an API call to this endpoint must include an array of recipient IDs of the recipients you want to delete.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_contactdb_recipients(
        &self,
        body: &[String],
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url("/contactdb/recipients", None);
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
     * Update Recipient.
     *
     * This function performs a `PATCH` to the `/contactdb/recipients` endpoint.
     *
     * **This endpoint allows you to update one or more recipients.**
     *
     * The body of an API call to this endpoint must include an array of one or more recipient objects.
     *
     * It is of note that you can add custom field data as parameters on recipient objects. We have provided an example using some of the default custom fields SendGrid provides.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_contactdb_recipients(
        &self,
        body: &[crate::types::PatchContactdbRecipientsRequest],
    ) -> ClientResult<crate::types::ContactDbRecipientResponse> {
        let url = self.client.url("/contactdb/recipients", None);
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get Recipient Upload Status.
     *
     * This function performs a `GET` to the `/contactdb/status` endpoint.
     *
     * **This endpoint allows you to check the upload status of a Marketing Campaigns recipient.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_statu(
        &self,
    ) -> ClientResult<crate::types::GetContactdbStatusResponseData> {
        let url = self.client.url("/contactdb/status", None);
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
     * Retrieve a single recipient.
     *
     * This function performs a `GET` to the `/contactdb/recipients/{recipient_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a single recipient by ID from your contact database.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_recipients_recipient(
        &self,
        recipient_id: &str,
    ) -> ClientResult<crate::types::ContactdbRecipient> {
        let url = self.client.url(
            &format!(
                "/contactdb/recipients/{}",
                crate::progenitor_support::encode_path(recipient_id),
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
     * Delete a Recipient.
     *
     * This function performs a `DELETE` to the `/contactdb/recipients/{recipient_id}` endpoint.
     *
     * **This endpoint allows you to delete a single recipient with the given ID from your contact database.**
     *
     * > Use this to permanently delete your recipients from all of your contact lists and all segments if required by applicable law.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_contactdb_recipients_recipient(
        &self,
        recipient_id: &str,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/contactdb/recipients/{}",
                crate::progenitor_support::encode_path(recipient_id),
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
     * Retrieve the lists that a recipient is on.
     *
     * This function performs a `GET` to the `/contactdb/recipients/{recipient_id}/lists` endpoint.
     *
     * **This endpoint allows you to retrieve the lists that a given recipient belongs to.**
     *
     * Each recipient can be on many lists. This endpoint gives you all of the lists that any one recipient has been added to.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_recipients_recipient_lists(
        &self,
        recipient_id: &str,
    ) -> ClientResult<crate::types::GetContactdbRecipientsRecipientListsResponse> {
        let url = self.client.url(
            &format!(
                "/contactdb/recipients/{}/lists",
                crate::progenitor_support::encode_path(recipient_id),
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
     * Retrieve the count of billable recipients.
     *
     * This function performs a `GET` to the `/contactdb/recipients/billable_count` endpoint.
     *
     * **This endpoint allows you to retrieve the number of Marketing Campaigns recipients that you will be billed for.**
     *
     * You are billed for marketing campaigns based on the highest number of recipients you have had in your account at one time. This endpoint will allow you to know the current billable count value.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_recipients_billable_count(
        &self,
    ) -> ClientResult<crate::types::ContactdbRecipientCount> {
        let url = self
            .client
            .url("/contactdb/recipients/billable_count", None);
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
     * Retrieve a Count of Recipients.
     *
     * This function performs a `GET` to the `/contactdb/recipients/count` endpoint.
     *
     * **This endpoint allows you to retrieve the total number of Marketing Campaigns recipients.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_recipients_count(
        &self,
    ) -> ClientResult<crate::types::ContactdbRecipientCount> {
        let url = self.client.url("/contactdb/recipients/count", None);
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
     * Search recipients.
     *
     * This function performs a `GET` to the `/contactdb/recipients/search` endpoint.
     *
     * **This endpoint allows you to perform a search on all of your Marketing Campaigns recipients.**
     *
     * field_name:
     *
     * * is a variable that is substituted for your actual custom field name from your recipient.
     * * Text fields must be url-encoded. Date fields are searchable only by unix timestamp (e.g. 2/2/2015 becomes 1422835200)
     * * If field_name is a 'reserved' date field, such as created_at or updated_at, the system will internally convert
     * your epoch time to a date range encompassing the entire day. For example, an epoch time of 1422835600 converts to
     * Mon, 02 Feb 2015 00:06:40 GMT, but internally the system will search from Mon, 02 Feb 2015 00:00:00 GMT through
     * Mon, 02 Feb 2015 23:59:59 GMT.
     *
     * **Parameters:**
     *
     * * `field_name: &str` -- The license key provided with your New Relic account.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_recipients_search(
        &self,
        field_name: &str,
    ) -> ClientResult<crate::types::GetContactdbRecipientsSearchResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !field_name.is_empty() {
            query_args.push(("{field_name}".to_string(), field_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/contactdb/recipients/search?{}", query_), None);
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
     * Search recipients.
     *
     * This function performs a `POST` to the `/contactdb/recipients/search` endpoint.
     *
     * <p>
     *   Search using segment conditions without actually creating a segment.
     *   Body contains a JSON object with <code>conditions</code>, a list of conditions as described below, and an optional <code>list_id</code>, which is a valid list ID for a list to limit the search on.
     * </p>
     *
     * <p>
     *   Valid operators for create and update depend on the type of the field for which you are searching.
     * </p>
     *
     * <ul>
     *   <li>Dates:
     *     <ul>
     *       <li>"eq", "ne", "lt" (before), "gt" (after)
     *         <ul>
     *           <li>You may use MM/DD/YYYY for day granularity or an epoch for second granularity.</li>
     *         </ul>
     *       </li>
     *       <li>"empty", "not_empty"</li>
     *       <li>"is within"
     *         <ul>
     *           <li>You may use an <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a> or the # of days.</li>
     *         </ul>
     *       </li>
     *     </ul>
     *   </li>
     *   <li>Text: "contains", "eq" (is - matches the full field), "ne" (is not - matches any field where the entire field is not the condition value), "empty", "not_empty"</li>
     *   <li>Numbers: "eq", "lt", "gt", "empty", "not_empty"</li>
     *   <li>Email Clicks and Opens: "eq" (opened), "ne" (not opened)</li>
     * </ul>
     *
     * <p>
     *   Field values must all be a string.
     * </p>
     *
     * <p>
     *   Search conditions using "eq" or "ne" for email clicks and opens should provide a "field" of either <code>clicks.campaign_identifier</code> or <code>opens.campaign_identifier</code>.
     *   The condition value should be a string containing the id of a completed campaign.
     * </p>
     *
     * <p>
     *   Search conditions list may contain multiple conditions, joined by an "and" or "or" in the "and_or" field.
     *   The first condition in the conditions list must have an empty "and_or", and subsequent conditions must all specify an "and_or".
     * </p>
     */
    pub async fn post_contactdb_recipients_search(
        &self,
        body: &crate::types::PostContactdbRecipientsSearchRequest,
    ) -> ClientResult<crate::types::PostContactdbRecipientsSearchResponseData> {
        let url = self.client.url("/contactdb/recipients/search", None);
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
}
