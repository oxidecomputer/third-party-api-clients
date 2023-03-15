use crate::Client;
use crate::ClientResult;

pub struct CancelScheduledSends {
    pub client: Client,
}

impl CancelScheduledSends {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CancelScheduledSends { client }
    }

    /**
     * Create a batch ID.
     *
     * This function performs a `POST` to the `/mail/batch` endpoint.
     *
     * **This endpoint allows you to generate a new batch ID.**
     *
     * Once a `batch_id` is created, you can associate it with a scheduled send using the `/mail/send` endpoint. Passing the `batch_id` as a field in the `/mail/send` request body will assign the ID to the send you are creating.
     *
     * Once an ID is associated with a scheduled send, the send can be accessed and its send status can be modified using the `batch_id`.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_mail_batch(&self) -> ClientResult<crate::types::MailBatchId> {
        let url = self.client.url("/mail/batch", None);
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
     * Retrieve all scheduled sends.
     *
     * This function performs a `GET` to the `/user/scheduled_sends` endpoint.
     *
     * **This endpoint allows you to retrieve all cancelled and paused scheduled send information.**
     *
     * This endpoint will return only the scheduled sends that are associated with a `batch_id`. If you have scheduled a send using the `/mail/send` endpoint and the `send_at` field but no `batch_id`, the send will be scheduled for delivery; however, it will not be returned by this endpoint. For this reason, you should assign a `batch_id` to any scheduled send you may need to pause or cancel in the future.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_scheduled_sends(
        &self,
    ) -> ClientResult<Vec<crate::types::UserScheduledSendStatusAllOf>> {
        let url = self.client.url("/user/scheduled_sends", None);
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
     * Retrieve all scheduled sends.
     *
     * This function performs a `GET` to the `/user/scheduled_sends` endpoint.
     *
     * As opposed to `get_user_scheduled_sends`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all cancelled and paused scheduled send information.**
     *
     * This endpoint will return only the scheduled sends that are associated with a `batch_id`. If you have scheduled a send using the `/mail/send` endpoint and the `send_at` field but no `batch_id`, the send will be scheduled for delivery; however, it will not be returned by this endpoint. For this reason, you should assign a `batch_id` to any scheduled send you may need to pause or cancel in the future.
     */
    pub async fn get_all_user_scheduled_sends(
        &self,
    ) -> ClientResult<Vec<crate::types::UserScheduledSendStatusAllOf>> {
        let url = self.client.url("/user/scheduled_sends", None);
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
     * Cancel or pause a scheduled send.
     *
     * This function performs a `POST` to the `/user/scheduled_sends` endpoint.
     *
     * **This endpoint allows you to cancel or pause a scheduled send associated with a `batch_id`.**
     *
     * Passing this endpoint a `batch_id` and status will cancel or pause the scheduled send.
     *
     * Once a scheduled send is set to `pause` or `cancel` you must use the "Update a scheduled send" endpoint to change its status or the "Delete a cancellation or pause from a scheduled send" endpoint to remove the status. Passing a status change to a scheduled send that has already been paused or cancelled will result in a `400` level status code.
     *
     * If the maximum number of cancellations/pauses are added to a send, a `400` level status code will be returned.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_user_scheduled_send(
        &self,
        body: &crate::types::CancelPauseAScheduledSendRequest,
    ) -> ClientResult<crate::types::UserScheduledSendStatusAllOf> {
        let url = self.client.url("/user/scheduled_sends", None);
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
     * Validate batch ID.
     *
     * This function performs a `GET` to the `/mail/batch/{batch_id}` endpoint.
     *
     * **This endpoint allows you to validate a batch ID.**
     *
     * When you pass a valid `batch_id` to this endpoint, it will return a `200` status code and the batch ID itself.
     *
     * If you pass an invalid `batch_id` to the endpoint, you will receive a `400` level status code and an error message.
     *
     * A `batch_id` does not need to be assigned to a scheduled send to be considered valid. A successful response means only that the `batch_id` has been created, but it does not indicate that it has been associated with a send.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_mail_batch(&self, batch_id: &str) -> ClientResult<crate::types::MailBatchId> {
        let url = self.client.url(
            &format!(
                "/mail/batch/{}",
                crate::progenitor_support::encode_path(batch_id),
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
     * Retrieve scheduled send.
     *
     * This function performs a `GET` to the `/user/scheduled_sends/{batch_id}` endpoint.
     *
     * **This endpoint allows you to retrieve the cancel/paused scheduled send information for a specific `batch_id`.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_scheduled_sends_batch(
        &self,
        batch_id: &str,
    ) -> ClientResult<Vec<crate::types::UserScheduledSendStatusAllOf>> {
        let url = self.client.url(
            &format!(
                "/user/scheduled_sends/{}",
                crate::progenitor_support::encode_path(batch_id),
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
     * Retrieve scheduled send.
     *
     * This function performs a `GET` to the `/user/scheduled_sends/{batch_id}` endpoint.
     *
     * As opposed to `get_user_scheduled_sends_batch`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve the cancel/paused scheduled send information for a specific `batch_id`.**
     */
    pub async fn get_all_user_scheduled_sends_batch(
        &self,
        batch_id: &str,
    ) -> ClientResult<Vec<crate::types::UserScheduledSendStatusAllOf>> {
        let url = self.client.url(
            &format!(
                "/user/scheduled_sends/{}",
                crate::progenitor_support::encode_path(batch_id),
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
     * Delete a cancellation or pause from a scheduled send.
     *
     * This function performs a `DELETE` to the `/user/scheduled_sends/{batch_id}` endpoint.
     *
     * **This endpoint allows you to delete the cancellation/pause of a scheduled send.**
     *
     * Scheduled sends cancelled less than 10 minutes before the scheduled time are not guaranteed to be cancelled.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_user_scheduled_sends_batch(&self, batch_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/user/scheduled_sends/{}",
                crate::progenitor_support::encode_path(batch_id),
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
     * Update a scheduled send.
     *
     * This function performs a `PATCH` to the `/user/scheduled_sends/{batch_id}` endpoint.
     *
     * **This endpoint allows you to update the status of a scheduled send for the given `batch_id`.**
     *
     * If you have already set a `cancel` or `pause` status on a scheduled send using the "Cancel or pause a scheduled send" endpoint, you can update it's status using this endpoint. Attempting to update a status once it has been set with the "Cancel or pause a scheduled send" endpoint will result in a `400` error.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_user_scheduled_sends_batch(
        &self,
        batch_id: &str,
        body: &crate::types::UserScheduledSendStatus,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/user/scheduled_sends/{}",
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
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
}
